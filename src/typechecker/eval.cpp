
// This file is a part of Simple-XX/SimpleCompiler
// (https://github.com/Simple-XX/SimpleCompiler).
//
// eval.cpp for Simple-XX/SimpleCompiler.

#include "ast.h"
#include "typechecker.h"
#include "irgen.h"
#include <functional>
#include <string>
#include <vector>

using namespace std;

ASTPtr FuncDefAST::Eval(TypeCheck &checker) {
    return checker.EvalFuncDef(*this);
}

ASTPtr BlockAST::Eval(TypeCheck &checker) {
    return checker.EvalBlock(*this);
}

ASTPtr BinaryAST::Eval(TypeCheck &checker) {
    initializer_list<Operator> opAdd = {Operator::add_op, Operator::sub_op};
    initializer_list<Operator> opMul = {Operator::mul_op, Operator::div_op, Operator::mod_op};
    initializer_list<Operator> opRel = {Operator::le_op, Operator::ge_op, Operator::lt_op, Operator::gt_op};
    initializer_list<Operator> opEq = {Operator::equ_op, Operator::nequ_op};
    initializer_list<Operator> opLAnd = {Operator::and_op};
    initializer_list<Operator> opLOr = {Operator::or_op};

    if (find(opLOr.begin(), opLOr.end(), op) != opLOr.end()) {
        return checker.EvalLOrExp(*this);
    } else if (find(opLAnd.begin(), opLAnd.end(), op) != opLAnd.end()) {
        return checker.EvalLAndExp(*this);
    } else if (find(opEq.begin(), opEq.end(), op) != opEq.end()) {
        return checker.EvalEqExp(*this);
    } else if (find(opRel.begin(), opRel.end(), op) != opRel.end()) {
        return checker.EvalRelExp(*this);
    } else if (find(opAdd.begin(), opAdd.end(), op) != opAdd.end()) {
        return checker.EvalAddExp(*this);
    } else if (find(opMul.begin(), opMul.end(), op) != opMul.end()) {
        return checker.EvalMulExp(*this);
    } else {
        return nullptr;
    }
}

ASTPtr IfAST::Eval(TypeCheck &checker) {
    return checker.EvalIfElse(*this);
}

ASTPtr WhileAST::Eval(TypeCheck &checker) {
    return checker.EvalWhile(*this);
}

ASTPtr NumAST::Eval(TypeCheck &checker) {
    return checker.EvalNumber(*this);
}

ASTPtr ProcessedIdAST::Eval(TypeCheck &checker) {
    return checker.EvalProcessedId(*this);
}

ASTPtr IdAST::Eval(TypeCheck &checker) {
    return checker.EvalId(*this);
}

ASTPtr UnaryAST::Eval(TypeCheck &checker) {
    return checker.EvalUnaryExp(*this);
}

ASTPtr ControlAST::Eval(TypeCheck &checker) {
    return checker.EvalControl(*this);
}

ASTPtr AssignAST::Eval(TypeCheck &checker) {
    return checker.EvalAssign(*this);
}

ASTPtr StmtAST::Eval(TypeCheck &checker) {
    return checker.EvalStmt(*this);
}

ASTPtr LValAST::Eval(TypeCheck &checker) {
    return checker.EvalLVal(*this);
}

ASTPtr FuncCallAST::Eval(TypeCheck &checker) {
    return checker.EvalFuncCall(*this);
}

ASTPtr VarDeclAST::Eval(TypeCheck &checker) {
    return checker.EvalVarDecl(*this);
}

ASTPtr VarDefAST::Eval(TypeCheck &checker) {
    return checker.EvalVarDef(*this);
}

ASTPtr InitValAST::Eval(TypeCheck &checker) {
    return checker.EvalInitVal(*this);
}

ASTPtr CompUnitAST::Eval(TypeCheck &checker) {
    return checker.EvalCompUnit(*this);
}

ASTPtr EmptyAST::Eval(TypeCheck &checker) {
    return checker.EvalEmpty();
}