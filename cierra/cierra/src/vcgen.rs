use std::{
    collections::HashMap,
    convert::TryFrom,
    iter,
    iter::{Map, Successors},
};

use itertools::Itertools;
use sexp::SexpDisplay;
use tracing::trace;

use crate::{
    subst::Substitutable,
    taurus::{
        concrete::{Assign, DeclStmt, Expr, ForInit, FuncDef, LocalTy, Stmt},
        guarded::{
            decl_to_gc,
            GCBlock,
            GuardedCommand,
            GuardedCommand::{Assert, Assume, Choice, Havoc},
        },
        logic::{
            self,
            ArithTerm,
            ArithTerm::{Store, Var},
            Binder,
            LoopAnnot,
            Pred,
            PredCmp,
            Quantifier,
        },
        CmpOp,
        Symbol,
    },
    utils::boxed,
};

pub struct Context {
    env: HashMap<Symbol, LocalTy>,
    fvs: Map<Successors<i32, fn(&i32) -> Option<i32>>, fn(i32) -> Symbol>,
}

impl Default for Context {
    fn default() -> Self {
        const SUCC: fn(&i32) -> Option<i32> = |i| Some(i + 1);
        const SYM: fn(i32) -> Symbol = |i| Symbol::new(&format!("_t{}", i));
        Self { env: HashMap::new(), fvs: iter::successors(Some(0), SUCC).map(SYM) }
    }
}

impl Context {
    fn next_fv(&mut self) -> Symbol {
        self.fvs.next().unwrap()
    }

    fn next_fvs<const N: usize>(&mut self) -> [Symbol; N] {
        array_init::from_iter(self.fvs.by_ref().take(N)).unwrap()
    }

    fn ty(&self, x: Symbol) -> LocalTy {
        self.env[&x]
    }
}

pub fn stmts_to_gc(ctx: &mut Context, stmts: impl IntoIterator<Item = Stmt>) -> GCBlock {
    stmts.into_iter().flat_map(|stmt| stmt_to_gc(ctx, stmt)).collect()
}

#[allow(clippy::too_many_lines)]
pub fn stmt_to_gc(ctx: &mut Context, stmt: Stmt) -> GCBlock {
    match stmt {
        Stmt::Empty => vec![],
        Stmt::Expr(_) => {
            // We assume all expressions have no side effect.
            vec![]
        }
        Stmt::Assign(Assign::Var(x, e)) => {
            let tmp = ctx.next_fv();
            vec![
                Assume(Pred::Cmp(PredCmp::Cmp(
                    CmpOp::Eq,
                    boxed(PredCmp::ArithTerm(Var(tmp))),
                    Var(x),
                ))),
                Havoc(x),
                Assume(Pred::Cmp(PredCmp::Cmp(
                    CmpOp::Eq,
                    boxed(PredCmp::ArithTerm(Var(x))),
                    ArithTerm::from(e).subst(Var(tmp), x),
                ))),
            ]
        }
        Stmt::Assign(Assign::Sub(a, Expr::Var(idx), Expr::Var(v))) => {
            let [a_, i, j] = ctx.next_fvs();
            let array_eq = |a1, a2, i| {
                Pred::Quant(
                    Quantifier::Forall,
                    vec![Binder(logic::AtomTy::Int, i)],
                    boxed(Pred::Cmp(PredCmp::Cmp(CmpOp::Eq, boxed(PredCmp::ArithTerm(a1)), a2))),
                )
            };
            let store = Store(boxed(Var(a_)), boxed(Var(idx)), boxed(Var(v)));
            vec![Assume(array_eq(Var(a_), Var(a), i)), Havoc(a), Assume(array_eq(Var(a), store, j))]
        }
        Stmt::Assign(Assign::Sub(a, ei, ev)) => {
            let [i, v] = ctx.next_fvs();
            stmts_to_gc(
                ctx,
                [
                    Stmt::Assign(Assign::Var(i, ei)),
                    Stmt::Assign(Assign::Var(v, ev)),
                    Stmt::Assign(Assign::Sub(a, Expr::Var(i), Expr::Var(v))),
                ],
            )
        }
        Stmt::Assign(Assign::Mem(..)) => todo!("Struct assignment"),
        Stmt::If(b, c1, c2) => {
            let (gc1, gc2) = (stmt_to_gc(ctx, *c1), stmt_to_gc(ctx, *c2));
            let bs = b.binary_to_pred();
            vec![Choice(
                vect!([Assume(bs.clone()), ..gc1]),
                vect!([Assume(Pred::Not(boxed(bs))), ..gc2]),
            )]
        }
        Stmt::While(LoopAnnot { invariants, .. }, b, c) => {
            let bs = b.binary_to_pred();
            let inv = Pred::from_conjuncts(invariants);
            let modified = c.modified();
            let havocs = modified.into_iter().map(Havoc);
            let gc = stmt_to_gc(ctx, *c);
            vect!([
                Assert(inv.clone()),
                ..havocs,
                Assume(inv.clone()),
                Choice(
                    vect!([Assume(bs.clone()), ..gc, Assert(inv), Assume(Pred::Bot)]),
                    vect!([Assume(Pred::Not(boxed(bs)))])
                )
            ])
        }
        Stmt::Do(LoopAnnot { invariants, .. }, b, c) => {
            let bs = b.binary_to_pred();
            let inv = Pred::from_conjuncts(invariants);
            let modified = c.modified();
            let havocs: Vec<_> = modified.into_iter().map(Havoc).collect();
            let gc = stmt_to_gc(ctx, *c);
            vect!([
                ..havocs.clone(),
                Assert(inv.clone()),
                ..havocs,
                Assume(inv.clone()),
                Choice(
                    vect!([Assume(bs.clone()), ..gc, Assert(inv), Assume(Pred::Bot)]),
                    vect!([Assume(Pred::Not(boxed(bs)))])
                )
            ])
        }
        Stmt::For(LoopAnnot { invariants, .. }, init, b, acc, c) => {
            let bs = b.map_or(Pred::TT, |b| b.binary_to_pred());
            let inv = Pred::from_conjuncts(invariants);
            let modified = c.modified();
            let havocs = modified.into_iter().map(Havoc);
            let gc = stmt_to_gc(ctx, *c);
            let gi = match init {
                ForInit::Decl(decl) => decl_to_gc(decl).into_iter().collect(),
                ForInit::Assign(assign) => stmt_to_gc(ctx, Stmt::Assign(assign)),
            };
            let ga = acc.clone().map(|acc| stmt_to_gc(ctx, *acc)).unwrap_or_default();
            let acc_havocs: Vec<_> = acc
                .map(|acc| acc.modified())
                .map(|modified| modified.into_iter().map(Havoc).collect())
                .unwrap_or_default();
            vect!([
                ..gi,
                Assert(inv.clone()),
                ..havocs,
                ..acc_havocs,
                Assume(inv.clone()),
                Choice(
                    vect!([Assume(bs.clone()), ..gc, ..ga, Assert(inv), Assume(Pred::Bot)]),
                    vect!([Assume(Pred::Not(boxed(bs)))])
                )
            ])
        }
        Stmt::Break => todo!("Control flow"),
        Stmt::Continue => todo!("Control flow"),
        Stmt::Ret(_) => todo!("Control flow"),
        Stmt::Assert(pred) => vec![Assert(pred)],
        Stmt::Block(cs) => cs
            .into_iter()
            .flat_map(|c| match c {
                DeclStmt::Decl(d) => decl_to_gc(d).into_iter().collect(),
                DeclStmt::Stmt(s) => stmt_to_gc(ctx, s),
            })
            .collect(),
    }
}

pub fn weakest_post(ctx: &mut Context, acc: Pred, gc: GuardedCommand) -> Pred {
    match gc {
        Assert(Pred::TT) => {
            trace!("post condition is true");
            acc
        }
        Assert(p) => {
            trace!("&& {}", p.plain_display());
            Pred::Con(boxed(p), boxed(acc))
        }
        Assume(Pred::Bot) => {
            trace!("ex falso quodlibet");
            Pred::TT
        }
        Assume(p) => {
            trace!("=> {}", p.plain_display());
            Pred::Imply(boxed(p), boxed(acc))
        }
        Havoc(x) => {
            let tmp = ctx.next_fv();
            let q = acc.subst(Var(tmp), x);
            trace!(
                "replace {} with {} in {} ==> {}",
                x,
                tmp,
                acc.plain_display(),
                q.pretty_display()
            );
            q
        }
        Choice(gc1, gc2) => {
            let p1 = weakest_post_block(ctx, acc.clone(), gc1);
            let p2 = weakest_post_block(ctx, acc, gc2);
            trace!("choice {} or {}", p1.pretty_display(), p2.pretty_display());
            Pred::Con(boxed(p1), boxed(p2))
        }
    }
}

pub fn weakest_post_block(ctx: &mut Context, acc: Pred, gcs: GCBlock) -> Pred {
    gcs.into_iter().rfold(acc, |acc, gc| weakest_post(ctx, acc, gc))
}

// TODO should return var list
pub fn func_to_vc(ctx: &mut Context, f: FuncDef) -> Pred {
    let gcs = stmts_to_gc(ctx, f.decl_stmts.into_iter().filter_map(|ds| Stmt::try_from(ds).ok()));
    trace!(gc=%gcs.iter().map(sexp::SexpDisplay::pretty_display).join("\n"), "guarded commands");
    let pre = Pred::from_conjuncts(f.contract.requires);
    let post = Pred::from_conjuncts(f.contract.ensures);

    let weakest = weakest_post_block(ctx, post, gcs);
    Pred::Not(boxed(Pred::Imply(boxed(pre), boxed(weakest))))
}
