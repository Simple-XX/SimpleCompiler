use std::ops::{Div, Mul, Neg, Not, Rem};

use antlr_rust::{common_token_stream::CommonTokenStream, InputStream};

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
    pub fn parse(data: &str) -> Self {
        let lexer = TaurusLexer::new(InputStream::new(data));
        let tokens = CommonTokenStream::new(lexer);
        let mut parser = TaurusParser::new(tokens);
        let ctx = parser.main().unwrap();
        parse_main(&ctx)
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
    Struct(String),
    Array(Atom),
}

#[derive(Debug, Copy, Clone)]
pub enum Const {
    Int(isize),
    Float(f32),
    True,
    False,
}

#[derive(Debug, Copy, Clone)]
pub enum UnaryOp {
    Not,
    Neg,
}

#[derive(Debug, Copy, Clone)]
pub enum ArithOp {
    Mul,
    Div,
    Mod,
    Plus,
    Minus,
}

#[derive(Debug, Copy, Clone)]
pub enum CmpOp {
    Lt,
    Le,
    Gt,
    Ge,
    Eq,
    Neq,
}

#[derive(Debug, Copy, Clone)]
pub enum LogicOp {
    Cmp(CmpOp),
    And,
    Or,
}

#[derive(Debug, Copy, Clone)]
pub enum BinaryOp {
    Arith(ArithOp),
    Logic(LogicOp),
}

pub mod concrete {
    use crate::taurus::{logic, BinaryOp, Const, UnaryOp};

    #[derive(Debug, Clone)]
    pub struct FuncDef {
        pub name: String,
        pub contract: logic::Contract,
        pub ret_ty: RetTy,
        pub vars: Vec<(ParaTy, String)>,
        pub decl_stmts: Vec<DeclStmt>,
    }

    #[derive(Debug, Clone)]
    pub struct StructDef {
        pub name: String,
        pub members: Vec<(ParaTy, String)>,
    }

    #[derive(Debug, Clone)]
    pub enum Stmt {
        Empty,
        Expr(Expr),
        Assign(Assign),
        If {
            cond: Expr,
            then: Box<Stmt>,
            els: Box<Stmt>,
        },
        While {
            annot: logic::LoopAnnot,
            cond: Expr,
            body: Box<Stmt>,
        },
        Do {
            annot: logic::LoopAnnot,
            cond: Expr,
            body: Box<Stmt>,
        },
        For {
            annot: logic::LoopAnnot,
            init: ForInit,
            cond: Option<Expr>,
            update: Option<ForIter>,
            body: Box<Stmt>,
        },
        Break,
        Continue,
        Ret(Option<Expr>),
        Assert(logic::Pred),
        Block(Vec<DeclStmt>),
    }

    #[derive(Debug, Clone)]
    pub enum DeclStmt {
        Decl(Decl),
        Stmt(Stmt),
    }

    #[derive(Debug, Clone)]
    pub enum ForInit {
        Decl(Decl),
        Assign(Assign),
    }

    #[derive(Debug, Clone)]
    pub enum ForIter {
        Assign(Assign),
        Expr(Expr),
    }

    #[derive(Debug, Clone)]
    pub struct Decl {
        pub ty: LocalTy,
        pub name: String,
        pub value: Option<Expr>,
    }

    #[derive(Debug, Clone)]
    pub enum Expr {
        Ident(String),
        Const(Const),
        Call { callee: String, params: Vec<Expr> },
        ArrAccess { array: Box<Expr>, idx: Box<Expr> },
        Mem { obj: Box<Expr>, member: String },
        Unary(UnaryOp, Box<Expr>),
        Binary(BinaryOp, Box<Expr>, Box<Expr>),
    }

    #[derive(Debug, Clone)]
    pub enum Assign {
        Var(String, Expr),
        Sub {
            array: String,
            idx: Expr,
            rhs: Expr,
        },
        Mem {
            obj: String,
            member: String,
            rhs: Expr,
        },
    }

    #[derive(Debug, Copy, Clone)]
    pub enum AtomTy {
        Int,
        Float,
    }

    pub type ParaTy = super::ParaTy<AtomTy>;

    #[derive(Debug, Clone)]
    pub enum RetTy {
        Atom(AtomTy),
        Struct(String),
        Array(AtomTy),
        Void,
    }

    #[derive(Debug, Clone)]
    pub enum LocalTy {
        Atom(AtomTy),
        Struct(String),
        Array(AtomTy, isize),
    }
}

pub mod logic {
    use crate::taurus::{ArithOp, CmpOp, Const, LogicOp, UnaryOp};

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

    #[derive(Debug, Copy, Clone)]
    pub enum AtomTy {
        Int,
        Real,
        Bool,
    }

    #[derive(Debug, Clone)]
    pub enum ArithTerm {
        Ident(String),
        Res,
        Const(Const),
        ArrUpd {
            array: Box<ArithTerm>,
            idx: Box<ArithTerm>,
            value: Box<ArithTerm>,
        },
        ArrAccess {
            array: Box<ArithTerm>,
            idx: Box<ArithTerm>,
        },
        Mem {
            obj: Box<ArithTerm>,
            member: String,
        },
        Unary(UnaryOp, Box<ArithTerm>),
        Binary(ArithOp, Box<ArithTerm>, Box<ArithTerm>),
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
        Call {
            callee: String,
            params: Vec<Term>,
        },
        Con(Box<Pred>, Box<Pred>),
        Dis(Box<Pred>, Box<Pred>),
        Imply(Box<Pred>, Box<Pred>),
        Iff(Box<Pred>, Box<Pred>),
        Not(Box<Pred>),
        Xor(Box<Pred>, Box<Pred>),
        // We assume that ranges of location and tset are closed.
        Length {
            base: String,
            start: isize,
            end: ArithTerm,
        },
        Quantifier {
            kind: Quantifier,
            vars: Vec<Binder>,
            pred: Box<Pred>,
        },
    }

    #[derive(Debug, Clone)]
    pub enum PredCmp {
        Cmp(CmpOp, Box<PredCmp>, ArithTerm),
        ArithTerm(ArithTerm),
    }

    #[derive(Debug, Clone)]
    pub struct PredDef {
        pub name: String,
        pub vars: Vec<(ParaTy, String)>,
        pub body: Pred,
    }

    #[derive(Debug, Clone)]
    pub struct Binder {
        pub ty: AtomTy,
        pub(crate) name: String,
    }

    #[derive(Debug, Copy, Clone)]
    pub enum Quantifier {
        Forall,
        Exists,
    }

    pub type ParaTy = super::ParaTy<AtomTy>;
}
