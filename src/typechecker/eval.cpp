
/**
 * @file eval.cpp
 * @brief eval
 * @author Zone.N (Zone.Niuzh@hotmail.com)
 * @version 1.0
 * @date 2023-10-26
 * @copyright MIT LICENSE
 * https://github.com/Simple-XX/SimpleCompiler
 * @par change log:
 * <table>
 * <tr><th>Date<th>Author<th>Description
 * <tr><td>2023-10-26<td>Zone.N<td>迁移到 doxygen
 * </table>
 */

#include <functional>
#include <string>
#include <vector>

#include "ast.h"
#include "irgen.h"
#include "typechecker.h"

ASTPtr FuncDefAST::Eval(TypeCheck &_checker) {
  return _checker.EvalFuncDef(*this);
}

ASTPtr BlockAST::Eval(TypeCheck &_checker) { return _checker.EvalBlock(*this); }

ASTPtr BinaryAST::Eval(TypeCheck &_checker) {
  std::initializer_list<Operator> opAdd = {Operator::add_op, Operator::sub_op};
  std::initializer_list<Operator> opMul = {Operator::mul_op, Operator::div_op,
                                           Operator::mod_op};
  std::initializer_list<Operator> opRel = {Operator::le_op, Operator::ge_op,
                                           Operator::lt_op, Operator::gt_op};
  std::initializer_list<Operator> opEq = {Operator::equ_op, Operator::nequ_op};
  std::initializer_list<Operator> opLAnd = {Operator::and_op};
  std::initializer_list<Operator> opLOr = {Operator::or_op};

  if (std::find(opLOr.begin(), opLOr.end(), op) != opLOr.end()) {
    return _checker.EvalLOrExp(*this);
  } else if (std::find(opLAnd.begin(), opLAnd.end(), op) != opLAnd.end()) {
    return _checker.EvalLAndExp(*this);
  } else if (std::find(opEq.begin(), opEq.end(), op) != opEq.end()) {
    return _checker.EvalEqExp(*this);
  } else if (std::find(opRel.begin(), opRel.end(), op) != opRel.end()) {
    return _checker.EvalRelExp(*this);
  } else if (std::find(opAdd.begin(), opAdd.end(), op) != opAdd.end()) {
    return _checker.EvalAddExp(*this);
  } else if (std::find(opMul.begin(), opMul.end(), op) != opMul.end()) {
    return _checker.EvalMulExp(*this);
  } else {
    return nullptr;
  }
}

ASTPtr IfAST::Eval(TypeCheck &_checker) { return _checker.EvalIfElse(*this); }

ASTPtr WhileAST::Eval(TypeCheck &_checker) { return _checker.EvalWhile(*this); }

ASTPtr NumAST::Eval(TypeCheck &_checker) { return _checker.EvalNumber(*this); }

ASTPtr ProcessedIdAST::Eval(TypeCheck &_checker) {
  return _checker.EvalProcessedId(*this);
}

ASTPtr IdAST::Eval(TypeCheck &_checker) { return _checker.EvalId(*this); }

ASTPtr UnaryAST::Eval(TypeCheck &_checker) {
  return _checker.EvalUnaryExp(*this);
}

ASTPtr ControlAST::Eval(TypeCheck &_checker) {
  return _checker.EvalControl(*this);
}

ASTPtr AssignAST::Eval(TypeCheck &_checker) { return _checker.EvalAssign(*this); }

ASTPtr StmtAST::Eval(TypeCheck &_checker) { return _checker.EvalStmt(*this); }

ASTPtr LValAST::Eval(TypeCheck &_checker) { return _checker.EvalLVal(*this); }

ASTPtr FuncCallAST::Eval(TypeCheck &_checker) {
  return _checker.EvalFuncCall(*this);
}

ASTPtr VarDeclAST::Eval(TypeCheck &_checker) {
  return _checker.EvalVarDecl(*this);
}

ASTPtr VarDefAST::Eval(TypeCheck &_checker) { return _checker.EvalVarDef(*this); }

ASTPtr InitValAST::Eval(TypeCheck &_checker) {
  return _checker.EvalInitVal(*this);
}

ASTPtr CompUnitAST::Eval(TypeCheck &_checker) {
  return _checker.EvalCompUnit(*this);
}

ASTPtr EmptyAST::Eval(TypeCheck &_checker) { return _checker.EvalEmpty(); }