use antlr_rust::{tree::ParseTree};

use crate::{
    parser::taurus::taurusparser::{
        AddExprContextAttrs,
        AddOpContextAll,
        AddTermContextAttrs,
        AndExprContextAttrs,
        AndTermContextAttrs,
        AriTermContextAttrs,
        ArithTermContextAll,
        ArrAccessExprContextAttrs,
        ArrAccessTermContextAttrs,
        ArrLocalVarContextAttrs,
        ArrLogicParaVarContextAttrs,
        ArrParaVarContextAttrs,
        ArrRetTyContextAttrs,
        ArrUpdTermContextAttrs,
        AssertStmtContextAttrs,
        AssertionContextAttrs,
        AssignContextAll,
        AssignStmtContextAttrs,
        AtomLocalVarContextAttrs,
        AtomLogicParaVarContextAttrs,
        AtomParaVarContextAttrs,
        AtomRetTyContextAttrs,
        AtomicTypeContextAll,
        BinderContextAll,
        BinderContextAttrs,
        BlockStmtContextAttrs,
        CallExprContextAttrs,
        CallPredContextAttrs,
        CmpOpContextAll,
        CmpPredContextAttrs,
        ConPredContextAttrs,
        ConstExprContextAttrs,
        ConstTermContextAttrs,
        ConstantContextAll,
        DeclContextAll,
        DeclContextAttrs,
        DeclStmtContextAll,
        DeclarationContextAttrs,
        DefContextAll,
        DisPredContextAttrs,
        DoStmtContextAttrs,
        EnsuresClauseContextAttrs,
        EqCmpOpContextAttrs,
        EqExprContextAttrs,
        EqOpContextAll,
        EqTermContextAttrs,
        ExprContextAll,
        ExprStmtContextAttrs,
        FloatConstContextAttrs,
        ForInitAssignContextAttrs,
        ForInitContextAll,
        ForInitLocalVarContextAttrs,
        ForIterAssignContextAttrs,
        ForIterContextAll,
        ForIterExprContextAttrs,
        ForStmtContextAttrs,
        FuncContractContextAll,
        FuncContractContextAttrs,
        FuncDefContextAll,
        FuncDefContextAttrs,
        FunctionContextAttrs,
        IdentExprContextAttrs,
        IdentTermContextAttrs,
        IfStmtContextAttrs,
        IffPredContextAttrs,
        ImpPredContextAttrs,
        IntConstContextAttrs,
        LengthPredContextAttrs,
        LocalVarContextAll,
        LogicAtomicTypeContextAll,
        LogicConstantContextAll,
        LogicFloatConstContextAttrs,
        LogicIntConstContextAttrs,
        LogicParaVarContextAll,
        LoopAnnotContextAll,
        LoopAnnotContextAttrs,
        MainContextAll,
        MainContextAttrs,
        MemAssignContextAttrs,
        MemExprContextAttrs,
        MemTermContextAttrs,
        MulExprContextAttrs,
        MulOpContextAll,
        MulTermContextAttrs,
        NotPredContextAttrs,
        OrExprContextAttrs,
        OrTermContextAttrs,
        OrdCmpOpContextAttrs,
        OrdExprContextAttrs,
        OrdOpContextAll,
        OrdTermContextAttrs,
        ParArithTermContextAttrs,
        ParExprContextAttrs,
        ParPredContextAttrs,
        ParTermContextAttrs,
        ParaVarContextAll,
        PredContextAll,
        PredDefContextAll,
        PredDefContextAttrs,
        PredDefsContextAttrs,
        PredicateContextAttrs,
        QuantiPredContextAttrs,
        QuantifierContextAll,
        RequiresClauseContextAttrs,
        RetTyContextAll,
        RetVarContextAttrs,
        ReturnStmtContextAttrs,
        StatementContextAttrs,
        StmtContextAll,
        StructDefContextAll,
        StructDefContextAttrs,
        StructLocalVarContextAttrs,
        StructLogicParaVarContextAttrs,
        StructParaVarContextAttrs,
        StructRetTyContextAttrs,
        StructureContextAttrs,
        SubAssignContextAttrs,
        TermContextAll,
        UnaryExprContextAttrs,
        UnaryOpContextAll,
        UnaryTermContextAttrs,
        VarAssignContextAttrs,
        WhileStmtContextAttrs,
        XorPredContextAttrs,
    },
    taurus::{
        concrete,
        concrete::{
            Assign,
            Decl,
            DeclStmt,
            Expr,
            ForInit,
            ForIter,
            FuncDef,
            LocalTy,
            RetTy,
            Stmt,
            StructDef,
        },
        logic,
        logic::{ArithTerm, Binder, Contract, LoopAnnot, Pred, PredCmp, PredDef, Quantifier, Term},
        ArithOp,
        BinaryOp,
        CmpOp,
        Const,
        Def,
        LogicOp,
        Taurus,
        UnaryOp,
    },
};

pub fn parse_main(ctx: &MainContextAll) -> Taurus {
    let defs = ctx
        .def_all()
        .into_iter()
        .flat_map(|ctx| parse_def(&ctx))
        .collect();
    Taurus { defs }
}

fn parse_def(ctx: &DefContextAll) -> Vec<Def> {
    match ctx {
        DefContextAll::FunctionContext(ctx) => {
            vec![Def::Func(parse_func(&ctx.funcDef().unwrap()))]
        }
        DefContextAll::StructureContext(ctx) => {
            vec![Def::Struct(parse_struct(&ctx.structDef().unwrap()))]
        }
        DefContextAll::PredicateContext(ctx) => ctx
            .predDefs()
            .unwrap()
            .predDef_all()
            .into_iter()
            .map(|ctx| Def::Pred(parse_pred_def(&ctx)))
            .collect(),
        DefContextAll::Error(_) => unreachable!(),
    }
}

fn parse_func(ctx: &FuncDefContextAll) -> FuncDef {
    let name = ctx.retVar().unwrap().IDENT().unwrap().get_text();
    let contract = parse_contract(&ctx.funcContract().unwrap());
    let ret_ty = parse_ret_ty(&ctx.retVar().unwrap().retTy().unwrap());
    let vars = ctx
        .paraVar_all()
        .into_iter()
        .map(|ctx| parse_para_var(&ctx))
        .collect();
    let decl_stmts = ctx
        .declStmt_all()
        .into_iter()
        .map(|ctx| parse_decl_stmt(&ctx))
        .collect();
    FuncDef {
        name,
        contract,
        ret_ty,
        vars,
        decl_stmts,
    }
}

fn parse_struct(ctx: &StructDefContextAll) -> StructDef {
    let name = ctx.IDENT().unwrap().get_text();
    let members = ctx
        .paraVar_all()
        .into_iter()
        .map(|ctx| parse_para_var(&ctx))
        .collect();
    StructDef { name, members }
}

fn parse_pred_def(ctx: &PredDefContextAll) -> PredDef {
    let name = ctx.IDENT().unwrap().get_text();
    let vars = ctx
        .logicParaVar_all()
        .into_iter()
        .map(|ctx| parse_logic_para_var(&ctx))
        .collect();
    let body = parse_pred(&ctx.pred().unwrap());
    PredDef { name, vars, body }
}

fn parse_pred(ctx: &PredContextAll) -> Pred {
    match ctx {
        PredContextAll::NotPredContext(ctx) => {
            Pred::Not(Box::new(parse_pred(&ctx.pred().unwrap())))
        }
        PredContextAll::DisPredContext(ctx) => Pred::Dis(
            Box::new(parse_pred(&ctx.pred(0).unwrap())),
            Box::new(parse_pred(&ctx.pred(1).unwrap())),
        ),
        PredContextAll::LengthPredContext(ctx) => Pred::Length {
            base: ctx.IDENT().unwrap().get_text(),
            start: ctx.INT_CONSTANT().unwrap().get_text().parse().unwrap(),
            end: parse_arith_term(&ctx.arithTerm().unwrap()),
        },
        PredContextAll::QuantiPredContext(ctx) => Pred::Quantifier {
            kind: parse_quantifier(&ctx.quantifier().unwrap()),
            vars: ctx
                .binder_all()
                .into_iter()
                .flat_map(|ctx| parse_binder(&ctx))
                .collect(),
            pred: Box::new(parse_pred(&ctx.pred().unwrap())),
        },
        PredContextAll::XorPredContext(ctx) => Pred::Xor(
            Box::new(parse_pred(&ctx.pred(0).unwrap())),
            Box::new(parse_pred(&ctx.pred(1).unwrap())),
        ),
        PredContextAll::ParPredContext(ctx) => parse_pred(&ctx.pred().unwrap()),
        PredContextAll::TruePredContext(_) => Pred::TT,
        PredContextAll::CallPredContext(ctx) => Pred::Call {
            callee: ctx.IDENT().unwrap().get_text(),
            params: ctx
                .term_all()
                .into_iter()
                .map(|ctx| parse_term(&ctx))
                .collect(),
        },
        PredContextAll::IffPredContext(ctx) => Pred::Iff(
            Box::new(parse_pred(&ctx.pred(0).unwrap())),
            Box::new(parse_pred(&ctx.pred(1).unwrap())),
        ),
        PredContextAll::CmpPredContext(ctx) => {
            let mut terms = ctx
                .arithTerm_all()
                .into_iter()
                .map(|ctx| parse_arith_term(&ctx));
            let ops = ctx.cmpOp_all().into_iter().map(|ctx| parse_cmp_op(&ctx));
            let initial = terms.next().unwrap();
            Pred::Cmp(
                ops.zip(terms)
                    .fold(PredCmp::ArithTerm(initial), |acc, (op, term)| {
                        PredCmp::Cmp(op, Box::new(acc), term)
                    }),
            )
        }
        PredContextAll::FalsePredContext(_) => Pred::Bot,
        PredContextAll::ImpPredContext(ctx) => Pred::Imply(
            Box::new(parse_pred(&ctx.pred(0).unwrap())),
            Box::new(parse_pred(&ctx.pred(1).unwrap())),
        ),
        PredContextAll::ConPredContext(ctx) => Pred::Con(
            Box::new(parse_pred(&ctx.pred(0).unwrap())),
            Box::new(parse_pred(&ctx.pred(1).unwrap())),
        ),
        PredContextAll::Error(_) => unreachable!(),
    }
}

fn parse_term(ctx: &TermContextAll) -> Term {
    match ctx {
        TermContextAll::AndTermContext(ctx) => Term::Binary(
            LogicOp::And,
            Box::new(parse_term(&ctx.term(0).unwrap())),
            Box::new(parse_term(&ctx.term(1).unwrap())),
        ),
        TermContextAll::OrTermContext(ctx) => Term::Binary(
            LogicOp::Or,
            Box::new(parse_term(&ctx.term(0).unwrap())),
            Box::new(parse_term(&ctx.term(1).unwrap())),
        ),
        TermContextAll::AriTermContext(ctx) => {
            Term::Arith(parse_arith_term(&ctx.arithTerm().unwrap()))
        }
        TermContextAll::OrdTermContext(ctx) => Term::Binary(
            LogicOp::Cmp(parse_ord_op(&ctx.ordOp().unwrap())),
            Box::new(parse_term(&ctx.term(0).unwrap())),
            Box::new(parse_term(&ctx.term(1).unwrap())),
        ),
        TermContextAll::EqTermContext(ctx) => Term::Binary(
            LogicOp::Cmp(parse_eq_op(&ctx.eqOp().unwrap())),
            Box::new(parse_term(&ctx.term(0).unwrap())),
            Box::new(parse_term(&ctx.term(1).unwrap())),
        ),
        TermContextAll::ParTermContext(ctx) => parse_term(&ctx.term().unwrap()),
        TermContextAll::Error(_) => unreachable!(),
    }
}

fn parse_arith_term(ctx: &ArithTermContextAll) -> ArithTerm {
    match ctx {
        ArithTermContextAll::ParArithTermContext(ctx) => {
            parse_arith_term(&ctx.arithTerm().unwrap())
        }
        ArithTermContextAll::MulTermContext(ctx) => ArithTerm::Binary(
            parse_mul_op(&ctx.mulOp().unwrap()),
            Box::new(parse_arith_term(&ctx.arithTerm(0).unwrap())),
            Box::new(parse_arith_term(&ctx.arithTerm(1).unwrap())),
        ),
        ArithTermContextAll::MemTermContext(ctx) => ArithTerm::Mem {
            obj: Box::new(parse_arith_term(&ctx.arithTerm().unwrap())),
            member: ctx.IDENT().unwrap().get_text(),
        },
        ArithTermContextAll::IdentTermContext(ctx) => {
            ArithTerm::Ident(ctx.IDENT().unwrap().get_text())
        }
        ArithTermContextAll::UnaryTermContext(ctx) => ArithTerm::Unary(
            parse_unary_op(&ctx.unaryOp().unwrap()),
            Box::new(parse_arith_term(&ctx.arithTerm().unwrap())),
        ),
        ArithTermContextAll::ArrAccessTermContext(ctx) => ArithTerm::ArrAccess {
            array: Box::new(parse_arith_term(&ctx.arithTerm(0).unwrap())),
            idx: Box::new(parse_arith_term(&ctx.arithTerm(1).unwrap())),
        },
        ArithTermContextAll::ResTermContext(_) => ArithTerm::Res,
        ArithTermContextAll::ArrUpdTermContext(ctx) => ArithTerm::ArrUpd {
            array: Box::new(parse_arith_term(&ctx.arithTerm(0).unwrap())),
            idx: Box::new(parse_arith_term(&ctx.arithTerm(1).unwrap())),
            value: Box::new(parse_arith_term(&ctx.arithTerm(2).unwrap())),
        },
        ArithTermContextAll::ConstTermContext(ctx) => {
            ArithTerm::Const(parse_logic_const(&ctx.logicConstant().unwrap()))
        }
        ArithTermContextAll::AddTermContext(ctx) => ArithTerm::Binary(
            parse_add_op(&ctx.addOp().unwrap()),
            Box::new(parse_arith_term(&ctx.arithTerm(0).unwrap())),
            Box::new(parse_arith_term(&ctx.arithTerm(1).unwrap())),
        ),
        ArithTermContextAll::Error(_) => unreachable!(),
    }
}

fn parse_contract(ctx: &FuncContractContextAll) -> Contract {
    let requires = ctx
        .requiresClause_all()
        .into_iter()
        .map(|ctx| parse_pred(&ctx.pred().unwrap()))
        .collect();
    let ensures = ctx
        .ensuresClause_all()
        .into_iter()
        .map(|ctx| parse_pred(&ctx.pred().unwrap()))
        .collect();
    Contract { requires, ensures }
}

fn parse_ret_ty(ctx: &RetTyContextAll) -> RetTy {
    match ctx {
        RetTyContextAll::VoidRetTyContext(_) => RetTy::Void,
        RetTyContextAll::AtomRetTyContext(ctx) => {
            RetTy::Atom(parse_atom_ty(&ctx.atomicType().unwrap()))
        }
        RetTyContextAll::ArrRetTyContext(ctx) => {
            RetTy::Array(parse_atom_ty(&ctx.atomicType().unwrap()))
        }
        RetTyContextAll::StructRetTyContext(ctx) => RetTy::Struct(ctx.IDENT().unwrap().get_text()),
        RetTyContextAll::Error(_) => unreachable!(),
    }
}

fn parse_para_var(ctx: &ParaVarContextAll) -> (concrete::ParaTy, String) {
    match ctx {
        ParaVarContextAll::AtomParaVarContext(ctx) => (
            concrete::ParaTy::Atom(parse_atom_ty(&ctx.atomicType().unwrap())),
            ctx.IDENT().unwrap().get_text(),
        ),
        ParaVarContextAll::ArrParaVarContext(ctx) => (
            concrete::ParaTy::Array(parse_atom_ty(&ctx.atomicType().unwrap())),
            ctx.IDENT().unwrap().get_text(),
        ),
        ParaVarContextAll::StructParaVarContext(ctx) => (
            concrete::ParaTy::Struct(ctx.IDENT(0).unwrap().get_text()),
            ctx.IDENT(1).unwrap().get_text(),
        ),
        ParaVarContextAll::Error(_) => unreachable!(),
    }
}

fn parse_logic_para_var(ctx: &LogicParaVarContextAll) -> (logic::ParaTy, String) {
    match ctx {
        LogicParaVarContextAll::AtomLogicParaVarContext(ctx) => (
            logic::ParaTy::Atom(parse_logic_atom_ty(&ctx.logicAtomicType().unwrap())),
            ctx.IDENT().unwrap().get_text(),
        ),
        LogicParaVarContextAll::ArrLogicParaVarContext(ctx) => (
            logic::ParaTy::Array(parse_logic_atom_ty(&ctx.logicAtomicType().unwrap())),
            ctx.IDENT().unwrap().get_text(),
        ),
        LogicParaVarContextAll::StructLogicParaVarContext(ctx) => (
            logic::ParaTy::Struct(ctx.IDENT(0).unwrap().get_text()),
            ctx.IDENT(1).unwrap().get_text(),
        ),
        LogicParaVarContextAll::Error(_) => unreachable!(),
    }
}

fn parse_local_var(ctx: &LocalVarContextAll) -> (LocalTy, String) {
    match ctx {
        LocalVarContextAll::AtomLocalVarContext(ctx) => (
            LocalTy::Atom(parse_atom_ty(&ctx.atomicType().unwrap())),
            ctx.IDENT().unwrap().get_text(),
        ),
        LocalVarContextAll::ArrLocalVarContext(ctx) => (
            LocalTy::Array(
                parse_atom_ty(&ctx.atomicType().unwrap()),
                ctx.INT_CONSTANT().unwrap().get_text().parse().unwrap(),
            ),
            ctx.IDENT().unwrap().get_text(),
        ),
        LocalVarContextAll::StructLocalVarContext(ctx) => (
            LocalTy::Struct(ctx.IDENT(0).unwrap().get_text()),
            ctx.IDENT(1).unwrap().get_text(),
        ),
        LocalVarContextAll::Error(_) => unreachable!(),
    }
}

fn parse_binder(ctx: &BinderContextAll) -> Vec<Binder> {
    let ty = parse_logic_atom_ty(&ctx.logicAtomicType().unwrap());
    ctx.IDENT_all()
        .into_iter()
        .map(|ident| Binder {
            ty,
            name: ident.get_text(),
        })
        .collect()
}

fn parse_decl(ctx: &DeclContextAll) -> Decl {
    let (ty, name) = parse_local_var(&ctx.localVar().unwrap());
    let value = ctx.expr().map(|ctx| parse_expr(&ctx));
    Decl { ty, name, value }
}

fn parse_expr(ctx: &ExprContextAll) -> Expr {
    match ctx {
        ExprContextAll::MulExprContext(ctx) => Expr::Binary(
            BinaryOp::Arith(ArithOp::Mul),
            Box::new(parse_expr(&ctx.expr(0).unwrap())),
            Box::new(parse_expr(&ctx.expr(1).unwrap())),
        ),
        ExprContextAll::AndExprContext(ctx) => Expr::Binary(
            BinaryOp::Logic(LogicOp::And),
            Box::new(parse_expr(&ctx.expr(0).unwrap())),
            Box::new(parse_expr(&ctx.expr(1).unwrap())),
        ),
        ExprContextAll::OrdExprContext(ctx) => Expr::Binary(
            BinaryOp::Logic(LogicOp::Cmp(parse_ord_op(&ctx.ordOp().unwrap()))),
            Box::new(parse_expr(&ctx.expr(0).unwrap())),
            Box::new(parse_expr(&ctx.expr(1).unwrap())),
        ),
        ExprContextAll::EqExprContext(ctx) => Expr::Binary(
            BinaryOp::Logic(LogicOp::Cmp(parse_eq_op(&ctx.eqOp().unwrap()))),
            Box::new(parse_expr(&ctx.expr(0).unwrap())),
            Box::new(parse_expr(&ctx.expr(1).unwrap())),
        ),
        ExprContextAll::IdentExprContext(ctx) => Expr::Ident(ctx.IDENT().unwrap().get_text()),
        ExprContextAll::ConstExprContext(ctx) => Expr::Const(parse_const(&ctx.constant().unwrap())),
        ExprContextAll::AddExprContext(ctx) => Expr::Binary(
            BinaryOp::Arith(parse_add_op(&ctx.addOp().unwrap())),
            Box::new(parse_expr(&ctx.expr(0).unwrap())),
            Box::new(parse_expr(&ctx.expr(1).unwrap())),
        ),
        ExprContextAll::ArrAccessExprContext(ctx) => Expr::ArrAccess {
            array: Box::new(parse_expr(&ctx.expr(0).unwrap())),
            idx: Box::new(parse_expr(&ctx.expr(1).unwrap())),
        },
        ExprContextAll::UnaryExprContext(ctx) => Expr::Unary(
            parse_unary_op(&ctx.unaryOp().unwrap()),
            Box::new(parse_expr(&ctx.expr().unwrap())),
        ),
        ExprContextAll::OrExprContext(ctx) => Expr::Binary(
            BinaryOp::Logic(LogicOp::Or),
            Box::new(parse_expr(&ctx.expr(0).unwrap())),
            Box::new(parse_expr(&ctx.expr(1).unwrap())),
        ),
        ExprContextAll::MemExprContext(ctx) => Expr::Mem {
            obj: Box::new(parse_expr(&ctx.expr().unwrap())),
            member: ctx.IDENT().unwrap().get_text(),
        },
        ExprContextAll::ParExprContext(ctx) => parse_expr(&ctx.expr().unwrap()),
        ExprContextAll::CallExprContext(ctx) => Expr::Call {
            callee: ctx.IDENT().unwrap().get_text(),
            params: ctx
                .expr_all()
                .into_iter()
                .map(|ctx| parse_expr(&ctx))
                .collect(),
        },
        ExprContextAll::Error(_) => unreachable!(),
    }
}

fn parse_stmt(ctx: &StmtContextAll) -> Stmt {
    match ctx {
        StmtContextAll::IfStmtContext(ctx) => Stmt::If {
            cond: parse_expr(&ctx.expr().unwrap()),
            then: Box::new(parse_stmt(&ctx.stmt(0).unwrap())),
            els: Box::new(parse_stmt(&ctx.stmt(1).unwrap())),
        },
        StmtContextAll::ExprStmtContext(ctx) => Stmt::Expr(parse_expr(&ctx.expr().unwrap())),
        StmtContextAll::WhileStmtContext(ctx) => Stmt::While {
            annot: parse_loop_annot(&ctx.loopAnnot().unwrap()),
            cond: parse_expr(&ctx.expr().unwrap()),
            body: Box::new(parse_stmt(&ctx.stmt().unwrap())),
        },
        StmtContextAll::AssignStmtContext(ctx) => {
            Stmt::Assign(parse_assign(&ctx.assign().unwrap()))
        }
        StmtContextAll::BreakStmtContext(_) => Stmt::Break,
        StmtContextAll::BlockStmtContext(ctx) => Stmt::Block(
            ctx.declStmt_all()
                .into_iter()
                .map(|ctx| parse_decl_stmt(&ctx))
                .collect(),
        ),
        StmtContextAll::EmptyStmtContext(_) => Stmt::Empty,
        StmtContextAll::ContStmtContext(_) => Stmt::Continue,
        StmtContextAll::AssertStmtContext(ctx) => {
            Stmt::Assert(parse_pred(&ctx.assertion().unwrap().pred().unwrap()))
        }
        StmtContextAll::ForStmtContext(ctx) => Stmt::For {
            annot: parse_loop_annot(&ctx.loopAnnot().unwrap()),
            init: parse_for_init(&ctx.forInit().unwrap()),
            cond: ctx.expr().map(|ctx| parse_expr(&ctx)),
            update: ctx.forIter().map(|ctx| parse_for_iter(&ctx)),
            body: Box::new(parse_stmt(&ctx.stmt().unwrap())),
        },
        StmtContextAll::ReturnStmtContext(ctx) => Stmt::Ret(ctx.expr().map(|ctx| parse_expr(&ctx))),
        StmtContextAll::DoStmtContext(ctx) => Stmt::Do {
            annot: parse_loop_annot(&ctx.loopAnnot().unwrap()),
            cond: parse_expr(&ctx.expr().unwrap()),
            body: Box::new(parse_stmt(&ctx.stmt().unwrap())),
        },
        StmtContextAll::Error(_) => unreachable!(),
    }
}

fn parse_decl_stmt(ctx: &DeclStmtContextAll) -> DeclStmt {
    match ctx {
        DeclStmtContextAll::StatementContext(ctx) => {
            DeclStmt::Stmt(parse_stmt(&ctx.stmt().unwrap()))
        }
        DeclStmtContextAll::DeclarationContext(ctx) => {
            DeclStmt::Decl(parse_decl(&ctx.decl().unwrap()))
        }
        DeclStmtContextAll::Error(_) => unreachable!(),
    }
}

fn parse_for_init(ctx: &ForInitContextAll) -> ForInit {
    match ctx {
        ForInitContextAll::ForInitLocalVarContext(ctx) => {
            let (ty, name) = parse_local_var(&ctx.localVar().unwrap());
            let value = ctx.expr().map(|ctx| parse_expr(&ctx));
            ForInit::Decl(Decl { ty, name, value })
        }
        ForInitContextAll::ForInitAssignContext(ctx) => {
            ForInit::Assign(parse_assign(&ctx.assign().unwrap()))
        }
        ForInitContextAll::Error(_) => unreachable!(),
    }
}

fn parse_for_iter(ctx: &ForIterContextAll) -> ForIter {
    match ctx {
        ForIterContextAll::ForIterExprContext(ctx) => {
            ForIter::Expr(parse_expr(&ctx.expr().unwrap()))
        }
        ForIterContextAll::ForIterAssignContext(ctx) => {
            ForIter::Assign(parse_assign(&ctx.assign().unwrap()))
        }
        ForIterContextAll::Error(_) => unreachable!(),
    }
}

fn parse_assign(ctx: &AssignContextAll) -> Assign {
    match ctx {
        AssignContextAll::MemAssignContext(ctx) => Assign::Mem {
            obj: ctx.IDENT(0).unwrap().get_text(),
            member: ctx.IDENT(1).unwrap().get_text(),
            rhs: parse_expr(&ctx.expr().unwrap()),
        },
        AssignContextAll::VarAssignContext(ctx) => Assign::Var(
            ctx.IDENT().unwrap().get_text(),
            parse_expr(&ctx.expr().unwrap()),
        ),
        AssignContextAll::SubAssignContext(ctx) => Assign::Sub {
            array: ctx.IDENT().unwrap().get_text(),
            idx: parse_expr(&ctx.expr(0).unwrap()),
            rhs: parse_expr(&ctx.expr(1).unwrap()),
        },
        AssignContextAll::Error(_) => unreachable!(),
    }
}

fn parse_loop_annot(ctx: &LoopAnnotContextAll) -> LoopAnnot {
    LoopAnnot {
        invariants: ctx
            .pred_all()
            .into_iter()
            .map(|ctx| parse_pred(&ctx))
            .collect(),
        variants: ctx
            .arithTerm_all()
            .into_iter()
            .map(|ctx| parse_arith_term(&ctx))
            .collect(),
    }
}

fn parse_quantifier(ctx: &QuantifierContextAll) -> Quantifier {
    match ctx {
        QuantifierContextAll::UniversalContext(_) => Quantifier::Forall,
        QuantifierContextAll::ExistentialContext(_) => Quantifier::Exists,
        QuantifierContextAll::Error(_) => unreachable!(),
    }
}

fn parse_atom_ty(ctx: &AtomicTypeContextAll) -> concrete::AtomTy {
    match &*ctx {
        AtomicTypeContextAll::AtomFloatContext(_) => concrete::AtomTy::Float,
        AtomicTypeContextAll::AtomIntContext(_) => concrete::AtomTy::Int,
        AtomicTypeContextAll::Error(_) => unreachable!(),
    }
}

fn parse_logic_atom_ty(ctx: &LogicAtomicTypeContextAll) -> logic::AtomTy {
    match &*ctx {
        LogicAtomicTypeContextAll::LogicAtomBoolContext(_) => logic::AtomTy::Bool,
        LogicAtomicTypeContextAll::LogicAtomRealContext(_) => logic::AtomTy::Real,
        LogicAtomicTypeContextAll::LogicAtomIntContext(_) => logic::AtomTy::Int,
        LogicAtomicTypeContextAll::Error(_) => unreachable!(),
    }
}

fn parse_const(ctx: &ConstantContextAll) -> Const {
    match ctx {
        ConstantContextAll::TrueConstContext(_) => Const::True,
        ConstantContextAll::FalseConstContext(_) => Const::False,
        ConstantContextAll::IntConstContext(ctx) => {
            Const::Int(ctx.INT_CONSTANT().unwrap().get_text().parse().unwrap())
        }
        ConstantContextAll::FloatConstContext(ctx) => {
            Const::Float(ctx.FLOAT_CONSTANT().unwrap().get_text().parse().unwrap())
        }
        ConstantContextAll::Error(_) => unreachable!(),
    }
}

fn parse_logic_const(ctx: &LogicConstantContextAll) -> Const {
    match ctx {
        LogicConstantContextAll::LogicTrueConstContext(_) => Const::True,
        LogicConstantContextAll::LogicFalseConstContext(_) => Const::False,
        LogicConstantContextAll::LogicIntConstContext(ctx) => {
            Const::Int(ctx.INT_CONSTANT().unwrap().get_text().parse().unwrap())
        }
        LogicConstantContextAll::LogicFloatConstContext(ctx) => {
            Const::Float(ctx.FLOAT_CONSTANT().unwrap().get_text().parse().unwrap())
        }
        LogicConstantContextAll::Error(_) => unreachable!(),
    }
}

fn parse_cmp_op(ctx: &CmpOpContextAll) -> CmpOp {
    match ctx {
        CmpOpContextAll::OrdCmpOpContext(ctx) => parse_ord_op(&ctx.ordOp().unwrap()),
        CmpOpContextAll::EqCmpOpContext(ctx) => parse_eq_op(&ctx.eqOp().unwrap()),
        CmpOpContextAll::Error(_) => unreachable!(),
    }
}

fn parse_ord_op(ctx: &OrdOpContextAll) -> CmpOp {
    match ctx {
        OrdOpContextAll::LtContext(_) => CmpOp::Lt,
        OrdOpContextAll::LeContext(_) => CmpOp::Le,
        OrdOpContextAll::GtContext(_) => CmpOp::Gt,
        OrdOpContextAll::GeContext(_) => CmpOp::Ge,
        OrdOpContextAll::Error(_) => unreachable!(),
    }
}

fn parse_eq_op(ctx: &EqOpContextAll) -> CmpOp {
    match ctx {
        EqOpContextAll::NeqContext(_) => CmpOp::Neq,
        EqOpContextAll::EqContext(_) => CmpOp::Eq,
        EqOpContextAll::Error(_) => unreachable!(),
    }
}

fn parse_add_op(ctx: &AddOpContextAll) -> ArithOp {
    match ctx {
        AddOpContextAll::PlusContext(_) => ArithOp::Plus,
        AddOpContextAll::MinusContext(_) => ArithOp::Minus,
        AddOpContextAll::Error(_) => unreachable!(),
    }
}

fn parse_mul_op(ctx: &MulOpContextAll) -> ArithOp {
    match ctx {
        MulOpContextAll::MulContext(_) => ArithOp::Mul,
        MulOpContextAll::DivContext(_) => ArithOp::Div,
        MulOpContextAll::ModContext(_) => ArithOp::Mod,
        MulOpContextAll::Error(_) => unreachable!(),
    }
}

fn parse_unary_op(ctx: &UnaryOpContextAll) -> UnaryOp {
    match ctx {
        UnaryOpContextAll::NegContext(_) => UnaryOp::Neg,
        UnaryOpContextAll::NotContext(_) => UnaryOp::Not,
        UnaryOpContextAll::Error(_) => unreachable!(),
    }
}
