
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

  bool FillInValue(int *memory, InitValAST *init, std::vector<int> &dim,
                   size_t i);

  std::unique_ptr<VarDeclAST> EvalVarDecl(VarDeclAST &varDecl);

  std::unique_ptr<ProcessedIdAST> EvalId(IdAST &id);

  std::unique_ptr<VarDefAST> EvalVarDef(VarDefAST &varDef);

  std::unique_ptr<FuncCallAST> EvalFuncCall(FuncCallAST &func);

  std::unique_ptr<BlockAST> EvalBlock(BlockAST &block);

  std::unique_ptr<IfAST> EvalIfElse(IfAST &stmt);

  std::unique_ptr<WhileAST> EvalWhile(WhileAST &stmt);

  std::unique_ptr<ControlAST> EvalControl(ControlAST &stmt);

  std::unique_ptr<AssignAST> EvalAssign(AssignAST &assign);

  ASTPtr EvalLVal(LValAST &lval);

  ASTPtr EvalAddExp(BinaryAST &exp);

  ASTPtr EvalMulExp(BinaryAST &exp);

  ASTPtr EvalUnaryExp(UnaryAST &exp);

  std::unique_ptr<FuncDefAST> EvalFuncDef(FuncDefAST &funcDef);

  ASTPtr EvalRelExp(BinaryAST &exp);

  ASTPtr EvalLAndExp(BinaryAST &exp);

  ASTPtr EvalLOrExp(BinaryAST &exp);

  std::unique_ptr<CompUnitAST> EvalCompUnit(CompUnitAST &unit);

  ASTPtr EvalEqExp(BinaryAST &exp);

  std::unique_ptr<StmtAST> EvalStmt(StmtAST &stmt);

  std::unique_ptr<InitValAST> EvalInitVal(InitValAST &init);

  std::unique_ptr<NumAST> EvalNumber(NumAST &num);

  std::unique_ptr<ProcessedIdAST> EvalProcessedId(ProcessedIdAST &id);

  std::unique_ptr<EmptyAST> EvalEmpty();
};

#endif /* SIMPLECOMPILER_TYCK_H */
