
/**
 * @file typechecker.h
 * @brief typechecker
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

#ifndef SIMPLECOMPILER_TYCK_H
#define SIMPLECOMPILER_TYCK_H

#include <map>
#include <set>
#include <string>
#include <utility>
#include <vector>

#include "ast.h"
#include "token.h"
#include "utils.h"

class TypeCheck {
private:
  int currentBlock;
  std::string currentFunc;

public:
  std::vector<int> parentBlock;

  std::map<std::string, Function> FuncTable;
  std::map<int, std::map<std::string, Var>> BlockVars;

  TypeCheck() {
    currentFunc = "";
    currentBlock = 0;

    FuncTable["getint"] = Function("getint", Type::int_t, std::vector<Var>{});
    FuncTable["getch"] = Function("getch", Type::int_t, std::vector<Var>{});
    FuncTable["getarray"] =
        Function("getarray", Type::int_t,
                 std::vector<Var>{
                     Var("a", VarType::array_t, false, std::vector<int>{0})});
    FuncTable["putint"] =
        Function("putint", Type::void_t,
                 std::vector<Var>{Var("a", VarType::var_t, false)});
    FuncTable["putch"] =
        Function("putch", Type::void_t,
                 std::vector<Var>{Var("a", VarType::var_t, false)});
    FuncTable["putarray"] =
        Function("putarray", Type::void_t,
                 std::vector<Var>{
                     Var("a", VarType::var_t, false),
                     Var("b", VarType::array_t, false, std::vector<int>{0})});
  }

  ~TypeCheck() = default;

  bool FillInValue(int *_memory, InitValAST *_init, std::vector<int> &_dim,
                   size_t _i);

  std::unique_ptr<VarDeclAST> EvalVarDecl(VarDeclAST &_varDecl);

  std::unique_ptr<ProcessedIdAST> EvalId(IdAST &_id);

  std::unique_ptr<VarDefAST> EvalVarDef(VarDefAST &_varDef);

  std::unique_ptr<FuncCallAST> EvalFuncCall(FuncCallAST &_func);

  std::unique_ptr<BlockAST> EvalBlock(BlockAST &_block);

  std::unique_ptr<IfAST> EvalIfElse(IfAST &_stmt);

  std::unique_ptr<WhileAST> EvalWhile(WhileAST &_stmt);

  std::unique_ptr<ControlAST> EvalControl(ControlAST &_stmt);

  std::unique_ptr<AssignAST> EvalAssign(AssignAST &_assign);

  ASTPtr EvalLVal(LValAST &_lval);

  ASTPtr EvalAddExp(BinaryAST &_exp);

  ASTPtr EvalMulExp(BinaryAST &_exp);

  ASTPtr EvalUnaryExp(UnaryAST &_exp);

  std::unique_ptr<FuncDefAST> EvalFuncDef(FuncDefAST &_funcDef);

  ASTPtr EvalRelExp(BinaryAST &_exp);

  ASTPtr EvalLAndExp(BinaryAST &_exp);

  ASTPtr EvalLOrExp(BinaryAST &_exp);

  std::unique_ptr<CompUnitAST> EvalCompUnit(CompUnitAST &_unit);

  ASTPtr EvalEqExp(BinaryAST &_exp);

  std::unique_ptr<StmtAST> EvalStmt(StmtAST &_stmt);

  std::unique_ptr<InitValAST> EvalInitVal(InitValAST &_init);

  std::unique_ptr<NumAST> EvalNumber(NumAST &_num);

  std::unique_ptr<ProcessedIdAST> EvalProcessedId(ProcessedIdAST &_id);

  std::unique_ptr<EmptyAST> EvalEmpty();
};

#endif /* SIMPLECOMPILER_TYCK_H */
