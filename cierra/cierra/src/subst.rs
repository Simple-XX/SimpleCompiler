use crate::{
    taurus::{
        logic::{ArithTerm, Binder, Pred, PredCmp},
        Symbol,
    },
    utils::{boxed, mk_subst},
};

pub trait Substitutable {
    /// Returns `self[y/x]`.
    #[must_use]
    fn subst(&self, y: ArithTerm, x: Symbol) -> Self;
}

impl Substitutable for Pred {
    fn subst(&self, y: ArithTerm, x: Symbol) -> Self {
        let c_subst = mk_subst(y.clone(), x);
        let p_subst = mk_subst(y, x);
        match self {
            Self::TT => Self::TT,
            Self::Bot => Self::Bot,
            Self::Cmp(c) => Self::Cmp(c_subst(c)),
            Self::Call(..) => todo!("Inter-procedural substitution"),
            Self::Con(p1, p2) => Self::Con(boxed(p_subst(p1)), boxed(p_subst(p2))),
            Self::Dis(p1, p2) => Self::Dis(boxed(p_subst(p1)), boxed(p_subst(p2))),
            Self::Imply(p1, p2) => Self::Imply(boxed(p_subst(p1)), boxed(p_subst(p2))),
            Self::Iff(p1, p2) => Self::Iff(boxed(p_subst(p1)), boxed(p_subst(p2))),
            Self::Not(p) => Self::Not(boxed(p_subst(p))),
            Self::Xor(p1, p2) => Self::Xor(boxed(p_subst(p1)), boxed(p_subst(p2))),
            Self::Length { .. } => todo!("Length substitution"),
            Self::Quant(q, xs, p) => Self::Quant(
                *q,
                xs.clone(),
                if xs.iter().any(|Binder(_ty, x_)| *x_ == x) {
                    p.clone()
                } else {
                    boxed(p_subst(p))
                },
            ),
        }
    }
}

impl Substitutable for PredCmp {
    fn subst(&self, y: ArithTerm, x: Symbol) -> Self {
        let subst = mk_subst(y.clone(), x);
        let subst_ = mk_subst(y, x);
        match self {
            Self::Cmp(op, e1, e2) => Self::Cmp(*op, boxed(subst(e1)), subst_(e2)),
            Self::ArithTerm(t) => Self::ArithTerm(subst_(t)),
        }
    }
}

impl Substitutable for ArithTerm {
    #[allow(clippy::many_single_char_names)]
    fn subst(&self, y: ArithTerm, x: Symbol) -> Self {
        use ArithTerm::{Binary, Const, Mem, Read, Res, Store, Unary, Var};
        let subst = mk_subst(y.clone(), x);
        match self {
            Var(x_) if x == *x_ => y,
            Var(_) | Res | Const(_) => self.clone(),
            Read(e, i) => Read(boxed(subst(e)), boxed(subst(i))),
            Store(e, i, v) => Store(boxed(subst(e)), boxed(subst(i)), boxed(subst(v))),
            Mem(e, m) => Mem(boxed(subst(e)), *m),
            Unary(op, e) => Unary(*op, boxed(subst(e))),
            Binary(op, e1, e2) => Binary(*op, boxed(subst(e1)), boxed(subst(e2))),
        }
    }
}
