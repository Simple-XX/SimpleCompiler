
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

#ifndef _TYCK_H_
#define _TYCK_H_

#include <map>
#include <set>
#include <string>
#include <utility>
#include <vector>
#include "token.h"
#include "ast.h"
#include "utils.h"

using namespace std;

class TypeCheck {
    private:
        int currentBlock;
        string currentFunc;
    public:

        vector<int> parentBlock;

        map<string, Function> FuncTable;
        map<int, map<string, Var>> BlockVars;

        TypeCheck() {
            currentFunc = "";
            currentBlock = 0;

            FuncTable["getint"] = Function("getint", Type::int_t, vector<Var>{});
            FuncTable["getch"] = Function("getch", Type::int_t, vector<Var>{});
            FuncTable["getarray"] = Function("getarray", Type::int_t,
                                             vector<Var>{Var("a", VarType::array_t, false, vector<int>{0})});
            FuncTable["putint"] = Function("putint", Type::void_t, vector<Var>{Var("a", VarType::var_t, false)});
            FuncTable["putch"] = Function("putch", Type::void_t, vector<Var>{Var("a", VarType::var_t, false)});
            FuncTable["putarray"] = Function("putarray", Type::void_t, vector<Var>{Var("a", VarType::var_t, false),
                                                                                      Var("b", VarType::array_t, false,
                                                                                          vector<int>{0})});
        }

        ~TypeCheck() = default;

        bool FillInValue(int *memory, InitValAST *init, vector<int> &dim, size_t i);

        unique_ptr<VarDeclAST> EvalVarDecl(VarDeclAST &varDecl);

        unique_ptr<ProcessedIdAST> EvalId(IdAST &id);

        unique_ptr<VarDefAST> EvalVarDef(VarDefAST &varDef);

        unique_ptr<FuncCallAST> EvalFuncCall(FuncCallAST &func);

        unique_ptr<BlockAST> EvalBlock(BlockAST &block);

        unique_ptr<IfAST> EvalIfElse(IfAST &stmt);

        unique_ptr<WhileAST> EvalWhile(WhileAST &stmt);

        unique_ptr<ControlAST> EvalControl(ControlAST &stmt);

        unique_ptr<AssignAST> EvalAssign(AssignAST &assign);

        ASTPtr EvalLVal(LValAST &lval);

        ASTPtr EvalAddExp(BinaryAST &exp);

        ASTPtr EvalMulExp(BinaryAST &exp);

        ASTPtr EvalUnaryExp(UnaryAST &exp);

        unique_ptr<FuncDefAST> EvalFuncDef(FuncDefAST &funcDef);

        ASTPtr EvalRelExp(BinaryAST &exp);

        ASTPtr EvalLAndExp(BinaryAST &exp);

        ASTPtr EvalLOrExp(BinaryAST &exp);

        unique_ptr<CompUnitAST> EvalCompUnit(CompUnitAST &unit);

        ASTPtr EvalEqExp(BinaryAST &exp);

        unique_ptr<StmtAST> EvalStmt(StmtAST &stmt);

        unique_ptr<InitValAST> EvalInitVal(InitValAST &init);

        unique_ptr<NumAST> EvalNumber(NumAST &num);

        unique_ptr<ProcessedIdAST> EvalProcessedId(ProcessedIdAST &id);

        unique_ptr<EmptyAST> EvalEmpty();
};

#endif /* _TYCK_H_ */