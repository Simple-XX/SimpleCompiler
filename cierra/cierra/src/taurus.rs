use std::{
    borrow::Borrow,
    convert::TryFrom,
    fmt::{Debug, Formatter},
    str::FromStr,
};

use antlr_rust::{common_token_stream::CommonTokenStream, InputStream};
use derive_more::Display;
use internment::Intern;
use sexp::{Serializer, SexpDisplay};
use sexp_macro::sexp;

use crate::parser::{
    cst2ast::parse_main,
    taurus::{tauruslexer::TaurusLexer, taurusparser::TaurusParser},
};

#[derive(Debug, Clone)]
pub struct Taurus {
    pub defs: Vec<Def>,
}

impl Taurus {
    /// Parses input and returns AST.
    ///
    /// # Panics
    /// Panics if input is invalid.
    #[must_use]
    pub fn parse(data: &str) -> Self {
        let lexer = TaurusLexer::new(InputStream::new(data));
        let tokens = CommonTokenStream::new(lexer);
        let mut parser = TaurusParser::new(tokens);
        let ctx = parser.main().unwrap();
        parse_main(&ctx)
    }

    #[must_use]
    pub fn find_func(&self, name: Symbol) -> Option<&concrete::FuncDef> {
        self.defs
            .iter()
            .filter_map(|def| match def {
                Def::Func(f) => Some(f),
                Def::Struct(_) | Def::Pred(_) => None,
            })
            .find(|f| f.name == name)
    }
}

#[derive(Display, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Symbol(Intern<String>);

impl SexpDisplay for Symbol {
    fn fmt(&self, s: &mut Serializer) {
        sexp!(s, ,(self.0));
    }
}

impl Debug for Symbol {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Symbol").field(&self.0.to_string()).finish()
    }
}

impl Symbol {
    #[must_use]
    pub fn new(s: &str) -> Self {
        Self(Intern::from_ref(s))
    }
}

impl FromStr for Symbol {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self::new(s))
    }
}

impl<T> From<T> for Symbol
where
    T: Borrow<str>,
{
    fn from(s: T) -> Self {
        Self::new(s.borrow())
    }
}

#[derive(Debug, Clone)]
pub enum Def {
    Func(concrete::FuncDef),
    Struct(concrete::StructDef),
    Pred(logic::PredDef),
}

#[derive(Debug, Clone)]
pub enum ParaTy<Atom> {
    Atom(Atom),
    Struct(Symbol),
    Array(Atom),
}

// TODO should display of True and False be 1/0 or true/false?
#[derive(Debug, Display, Copy, Clone)]
pub enum Const {
    Int(isize),
    Float(f32),
    True,
    False,
}

impl SexpDisplay for Const {
    fn fmt(&self, s: &mut Serializer) {
        sexp!(s, ,self);
    }
}

impl TryFrom<Const> for bool {
    type Error = ();

    fn try_from(value: Const) -> Result<Self, Self::Error> {
        match value {
            Const::True => Ok(true),
            Const::False => Ok(false),
            _ => Err(()),
        }
    }
}

impl Const {
    #[must_use]
    pub fn to_sem_bool(self) -> bool {
        match self {
            Self::Int(i) if i == 1 => true,
            Self::Float(f) if (f - 1.0).abs() < f32::EPSILON => true,
            Self::Int(_) | Self::Float(_) => false,
            Self::True | Self::False => bool::try_from(self).unwrap(),
        }
    }
}

#[derive(Debug, Display, Copy, Clone)]
pub enum UnaryOp {
    // TODO is this so? this is used in ArithTerm so you may want to change this when you
    // implement bool in logic variables
    #[display(fmt = "!")]
    Not,
    #[display(fmt = "-")]
    Neg,
}

impl SexpDisplay for UnaryOp {
    fn fmt(&self, s: &mut Serializer) {
        sexp!(s, ,(self.to_string()));
    }
}

#[derive(Debug, Display, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ArithOp {
    #[display(fmt = "*")]
    Mul,
    #[display(fmt = "div")]
    Div,
    #[display(fmt = "mod")]
    Mod,
    #[display(fmt = "+")]
    Plus,
    #[display(fmt = "-")]
    Minus,
}

impl SexpDisplay for ArithOp {
    fn fmt(&self, s: &mut Serializer) {
        sexp!(s, ,(self.to_string()));
    }
}

#[derive(Debug, Display, Copy, Clone, Eq, PartialEq, Hash)]
pub enum CmpOp {
    #[display(fmt = "<")]
    Lt,
    #[display(fmt = "<=")]
    Le,
    #[display(fmt = ">")]
    Gt,
    #[display(fmt = ">=")]
    Ge,
    #[display(fmt = "=")]
    Eq,
    #[display(fmt = "!=")]
    Neq,
}

impl SexpDisplay for CmpOp {
    fn fmt(&self, s: &mut Serializer) {
        sexp!(s, ,(self.to_string()));
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum LogicOp {
    Cmp(CmpOp),
    And,
    Or,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum BinaryOp {
    Arith(ArithOp),
    Logic(LogicOp),
}

pub mod concrete {
    use std::convert::TryFrom;

    use crate::{
        taurus::{logic, BinaryOp, CmpOp, Const, LogicOp, Symbol, UnaryOp},
        utils::boxed,
    };

    #[derive(Debug, Clone)]
    pub struct FuncDef {
        pub name: Symbol,
        pub contract: logic::Contract,
        pub ret_ty: RetTy,
        pub vars: Vec<(ParaTy, Symbol)>,
        pub decl_stmts: Vec<DeclStmt>,
    }

    #[derive(Debug, Clone)]
    pub struct StructDef {
        pub name: Symbol,
        pub members: Vec<(ParaTy, Symbol)>,
    }

    #[derive(Debug, Clone)]
    pub enum Stmt {
        Empty,
        Expr(Expr),
        Assign(Assign),
        If(Expr, Box<Stmt>, Box<Stmt>),
        While(logic::LoopAnnot, Expr, Box<Stmt>),
        Do(logic::LoopAnnot, Expr, Box<Stmt>),
        For(logic::LoopAnnot, ForInit, Option<Expr>, Option<Box<Stmt>>, Box<Stmt>),
        Break,
        Continue,
        Ret(Option<Expr>),
        Assert(logic::Pred),
        Block(Vec<DeclStmt>),
    }

    impl Stmt {
        #[must_use]
        pub fn modified(&self) -> Vec<Symbol> {
            match self {
                Self::Empty
                | Self::Expr(_)
                | Self::Break
                | Self::Continue
                | Self::Ret(_)
                | Self::Assert(_) => vec![],
                Self::Assign(Assign::Var(x, _) | Assign::Mem(x, ..)) => vec![*x],
                Self::Assign(Assign::Sub(..)) => todo!("Struct support"),
                Self::If(_, s1, s2) => vect!([..s1.modified(), ..s2.modified()]),
                Self::While(_, _, s) | Self::Do(_, _, s) | Self::For(_, _, _, _, s) => s.modified(),
                Self::Block(stmts) => stmts
                    .iter()
                    .flat_map(|s| match s {
                        DeclStmt::Decl(Decl { name, .. }) => vec![*name],
                        DeclStmt::Stmt(s) => s.modified(),
                    })
                    .collect(),
            }
        }
    }

    #[derive(Debug, Clone)]
    pub enum DeclStmt {
        Decl(Decl),
        Stmt(Stmt),
    }

    impl TryFrom<DeclStmt> for Stmt {
        type Error = ();

        fn try_from(value: DeclStmt) -> Result<Self, Self::Error> {
            match value {
                DeclStmt::Decl(Decl { ty: _, name, value: Some(value) }) => {
                    Ok(Self::Assign(Assign::Var(name, value)))
                }
                DeclStmt::Decl { .. } => Err(()),
                DeclStmt::Stmt(s) => Ok(s),
            }
        }
    }

    #[derive(Debug, Clone)]
    pub enum ForInit {
        Decl(Decl),
        Assign(Assign),
    }

    #[derive(Debug, Clone)]
    pub struct Decl {
        pub ty: LocalTy,
        pub name: Symbol,
        pub value: Option<Expr>,
    }

    #[derive(Debug, Clone)]
    pub enum Expr {
        Var(Symbol),
        Const(Const),
        Call(Symbol, Vec<Expr>),
        /// Array access.
        Read(Box<Expr>, Box<Expr>),
        /// Struct access.
        Mem(Box<Expr>, Symbol),
        Unary(UnaryOp, Box<Expr>),
        Binary(BinaryOp, Box<Expr>, Box<Expr>),
    }

    impl Expr {
        #[must_use]
        pub fn binary_to_pred(&self) -> logic::Pred {
            use logic::{Pred, PredCmp};
            match self {
                Self::Var(_) => Pred::Cmp(PredCmp::Cmp(
                    CmpOp::Eq,
                    boxed(PredCmp::ArithTerm(self.clone().into())),
                    logic::ArithTerm::Const(Const::Int(1)),
                )),
                Self::Const(c) => c.to_sem_bool().into(),
                Self::Call(..) => todo!(),
                Self::Unary(op, e) => {
                    let e = e.binary_to_pred();
                    match op {
                        UnaryOp::Not | UnaryOp::Neg => Pred::Not(boxed(e)),
                    }
                }
                Self::Binary(BinaryOp::Logic(op), e1, e2) => match op {
                    LogicOp::And => {
                        Pred::Con(boxed(e1.binary_to_pred()), boxed(e2.binary_to_pred()))
                    }
                    LogicOp::Or => {
                        Pred::Dis(boxed(e1.binary_to_pred()), boxed(e2.binary_to_pred()))
                    }
                    LogicOp::Cmp(op) => Pred::Cmp(PredCmp::Cmp(
                        *op,
                        boxed(PredCmp::ArithTerm((*e1.clone()).into())),
                        (*e2.clone()).into(),
                    )),
                },
                Self::Read(..) | Self::Mem(..) | Self::Binary(..) => Pred::Cmp(PredCmp::Cmp(
                    CmpOp::Eq,
                    boxed(PredCmp::ArithTerm(self.clone().into())),
                    logic::ArithTerm::Const(Const::Int(1)),
                )),
            }
        }
    }

    #[derive(Debug, Clone)]
    pub enum Assign {
        Var(/* lhs */ Symbol, /* rhs */ Expr),
        Sub(/* array */ Symbol, /* idx */ Expr, /* rhs */ Expr),
        Mem(/* struct */ Symbol, /* member */ Symbol, /* rhs */ Expr),
    }

    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub enum AtomTy {
        Int,
        Float,
    }

    pub type ParaTy = super::ParaTy<AtomTy>;

    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub enum RetTy {
        Atom(AtomTy),
        Struct(Symbol),
        Array(AtomTy),
        Void,
    }

    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub enum LocalTy {
        Atom(AtomTy),
        Struct(Symbol),
        Array(AtomTy, isize),
    }
}

pub mod logic {
    use derive_more::Display;
    use sexp::{Serializer, SexpDisplay};
    use sexp_macro::sexp;

    use crate::{
        taurus::{concrete, ArithOp, BinaryOp, CmpOp, Const, LogicOp, Symbol, UnaryOp},
        utils::boxed,
    };

    #[derive(Debug, Clone)]
    pub struct Contract {
        pub requires: Vec<Pred>,
        // Decrease clauses are omitted
        pub ensures: Vec<Pred>,
    }

    #[derive(Debug, Clone)]
    pub struct LoopAnnot {
        pub invariants: Vec<Pred>,
        pub variants: Vec<ArithTerm>,
    }

    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub enum AtomTy {
        Int,
        Real,
        Bool,
    }

    impl From<concrete::AtomTy> for AtomTy {
        fn from(ty: concrete::AtomTy) -> Self {
            match ty {
                concrete::AtomTy::Int => Self::Int,
                concrete::AtomTy::Float => Self::Real,
            }
        }
    }

    #[derive(Debug, Clone)]
    pub enum ArithTerm {
        Var(Symbol),
        Res,
        Const(Const),
        Store(Box<ArithTerm>, Box<ArithTerm>, Box<ArithTerm>),
        Read(Box<ArithTerm>, Box<ArithTerm>),
        Mem(Box<ArithTerm>, Symbol),
        Unary(UnaryOp, Box<ArithTerm>),
        Binary(ArithOp, Box<ArithTerm>, Box<ArithTerm>),
    }

    impl SexpDisplay for ArithTerm {
        fn fmt(&self, s: &mut Serializer) {
            match self {
                Self::Var(x) => {
                    sexp!(s, %x);
                }
                Self::Res => {
                    unreachable!()
                }
                Self::Const(c) => {
                    sexp!(s, %c);
                }
                Self::Store(a, i, v) => {
                    sexp!(s, (store % a % i % v));
                }
                Self::Read(a, i) => {
                    sexp!(s, (select % a % i));
                }
                Self::Mem(..) => {
                    unimplemented!()
                }
                Self::Unary(op, e) => {
                    sexp!(s, (%op %e));
                }
                Self::Binary(op, e1, e2) => {
                    sexp!(s, (%op %e1 %e2));
                }
            }
        }
    }

    impl From<concrete::Expr> for ArithTerm {
        fn from(e: concrete::Expr) -> Self {
            use concrete::Expr::{Binary, Call, Const, Mem, Read, Unary, Var};
            match e {
                Var(s) => Self::Var(s),
                Const(c) => Self::Const(c),
                Call(..) => todo!("Inter-procedural analysis"),
                Read(e, i) => Self::Read(boxed((*e).into()), boxed((*i).into())),
                Mem(e, s) => Self::Mem(boxed((*e).into()), s),
                Unary(op, e) => Self::Unary(op, boxed((*e).into())),
                Binary(BinaryOp::Arith(op), e1, e2) => {
                    Self::Binary(op, boxed((*e1).into()), boxed((*e2).into()))
                }
                Binary(BinaryOp::Logic(_op), _e1, _e2) => {
                    todo!("Boolean expression on logic variables is not supported.")
                }
            }
        }
    }

    #[derive(Debug, Clone)]
    pub enum Term {
        Arith(ArithTerm),
        Binary(LogicOp, Box<Term>, Box<Term>),
    }

    #[derive(Debug, Clone)]
    pub enum Pred {
        TT,
        Bot,
        Cmp(PredCmp),
        Call(Symbol, Vec<Term>),
        Con(Box<Pred>, Box<Pred>),
        Dis(Box<Pred>, Box<Pred>),
        Imply(Box<Pred>, Box<Pred>),
        Iff(Box<Pred>, Box<Pred>),
        Not(Box<Pred>),
        Xor(Box<Pred>, Box<Pred>),
        // We assume that ranges of location and tset are closed.
        Length { base: Symbol, start: isize, end: ArithTerm },
        // #[display(fmt = "({} [{}] {})", _0, "unword(_1)", _2)]
        Quant(Quantifier, Vec<Binder>, Box<Pred>),
    }

    impl SexpDisplay for Pred {
        fn fmt(&self, s: &mut Serializer) {
            match self {
                Self::TT => {
                    sexp!(s, true);
                }
                Self::Bot => {
                    sexp!(s, false);
                }
                Self::Cmp(c) => {
                    sexp!(s, %c);
                }
                Self::Call(..) => {
                    unimplemented!()
                }
                Self::Con(p1, p2) => {
                    sexp!(s, (and % p1 % p2));
                }
                Self::Dis(p1, p2) => {
                    sexp!(s, (or % p1 % p2));
                }
                Self::Imply(p1, p2) => {
                    sexp!(s, (=> %p1 %p2));
                }
                Self::Iff(p1, p2) => {
                    sexp!(s, (= %p1 %p2));
                }
                Self::Not(p) => {
                    sexp!(s, (not % p));
                }
                Self::Xor(p1, p2) => {
                    sexp!(s, (xor % p1 % p2));
                }
                Self::Length { .. } => {
                    unimplemented!()
                }
                Self::Quant(q, bs, p) => {
                    sexp!(s, (%q ..bs %p));
                }
            }
        }
    }

    impl Pred {
        pub fn from_conjuncts(conjuncts: impl IntoIterator<Item = Self>) -> Self {
            conjuncts.into_iter().fold(Self::TT, |acc, p| Self::Con(boxed(acc), boxed(p)))
        }
    }

    impl From<bool> for Pred {
        fn from(b: bool) -> Self {
            if b { Self::TT } else { Self::Bot }
        }
    }

    #[derive(Debug, Clone)]
    pub enum PredCmp {
        Cmp(CmpOp, Box<PredCmp>, ArithTerm),
        ArithTerm(ArithTerm),
    }

    impl SexpDisplay for PredCmp {
        fn fmt(&self, s: &mut Serializer) {
            match self {
                Self::Cmp(op, p, t) => {
                    sexp!(s, (%op %p %t));
                }
                Self::ArithTerm(t) => {
                    sexp!(s, %t);
                }
            }
        }
    }

    #[derive(Debug, Clone)]
    pub struct PredDef {
        pub name: Symbol,
        pub vars: Vec<(ParaTy, Symbol)>,
        pub body: Pred,
    }

    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct Binder(pub AtomTy, pub Symbol);

    impl SexpDisplay for Binder {
        fn fmt(&self, s: &mut Serializer) {
            sexp!(s, %(self.1));
        }
    }

    #[derive(Debug, Display, Copy, Clone, Eq, PartialEq, Hash)]
    pub enum Quantifier {
        #[display(fmt = "forall")]
        Forall,
        #[display(fmt = "exists")]
        Exists,
    }

    impl SexpDisplay for Quantifier {
        fn fmt(&self, s: &mut Serializer) {
            match self {
                Self::Forall => {
                    sexp!(s, forall);
                }
                Self::Exists => {
                    sexp!(s, exists);
                }
            }
        }
    }

    pub type ParaTy = super::ParaTy<AtomTy>;
}

pub mod guarded {
    use sexp::{Serializer, SexpDisplay};
    use sexp_macro::sexp;

    use crate::{
        taurus::{
            concrete,
            concrete::Decl,
            logic::{ArithTerm, Pred, PredCmp},
            CmpOp,
            Symbol,
        },
        utils::boxed,
    };

    // TODO newtype
    pub type GCBlock = Vec<GuardedCommand>;

    #[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
    pub struct Typed(pub Symbol, pub concrete::LocalTy);

    #[derive(Debug, Clone)]
    pub enum GuardedCommand {
        Assert(Pred),
        Assume(Pred),
        Havoc(Symbol),
        Choice(GCBlock, GCBlock),
    }

    impl SexpDisplay for GuardedCommand {
        fn fmt(&self, s: &mut Serializer) {
            match self {
                Self::Assert(p) => {
                    sexp!(s, (assert % p));
                }
                Self::Assume(p) => {
                    sexp!(s, (assume % p));
                }
                Self::Havoc(x) => {
                    sexp!(s, (havoc % x));
                }
                Self::Choice(gc1, gc2) => {
                    sexp!(s, (choice (...gc1) (...gc2)));
                }
            }
        }
    }

    #[must_use]
    pub fn decl_to_gc(Decl { name, value, .. }: Decl) -> Option<GuardedCommand> {
        value.map(|value| {
            GuardedCommand::Assume(Pred::Cmp(PredCmp::Cmp(
                CmpOp::Eq,
                boxed(PredCmp::ArithTerm(ArithTerm::Var(name))),
                ArithTerm::from(value),
            )))
        })
    }
}
