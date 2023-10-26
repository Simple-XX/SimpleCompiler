
/**
 * @file irgen.h
 * @brief irgen
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

#ifndef _IRGEN_H_

#include <iostream>
#include <map>
#include <utility>

#include "ast.h"
#include "utils.h"

using namespace std;

class IRGenerator {
private:
  int t_num;
  int T_num;
  int l_num;
  int cur_break_l;
  int cur_continue_l;
  int currentDepth;
  int currentBlock;
  string currentFunc;

  map<int, map<string, GenVar>> BlockSymbolTable;
  vector<int> parentBlock;
  map<string, Function> FuncTable;
  vector<GenVar> ReverseSymbolTable;

public:
  IRGenerator(map<string, Function> __FuncTable,
              const map<int, map<string, Var>> &BlockVars)
      : FuncTable(move(__FuncTable)) {
    for (auto &item1 : BlockVars) {
      for (auto &item2 : item1.second) {
        if (item2.second.isConst && item2.second.type == VarType::var_t)
          continue;
        BlockSymbolTable[item1.first][item2.first] =
            GenVar(item2.second.name, item2.second.type, item2.second.dims);
      }
    }

    t_num = 0;
    T_num = 0;
    l_num = 0;
    cur_break_l = -1;
    cur_continue_l = -1;
    currentDepth = 0;
    currentBlock = 0;
    currentFunc = "";
  }

  void GenerateValue(const string &varName, int &idx, int indx,
                     InitValAST *init, vector<int> dim, int i, string &code);

  void GenVarDecl(VarDeclAST &varDecl, string &code);

  string GenId(ProcessedIdAST &id, string &code);

  string GenNumber(NumAST &num, string &code);

  string GenVarDef(VarDefAST &varDef, string &code);

  string GenAssign(AssignAST &assign, string &code);

  string GenBinaryExp(BinaryAST &exp, string &code);

  string GenInitVal(InitValAST &init, string &code);

  static string op2char(Operator op);

  void GenBlock(BlockAST &block, string &code);

  string GenFuncCall(FuncCallAST &func, string &code);

  string GenLVal(LValAST &lval, string &code);

  string GenUnaryExp(UnaryAST &exp, string &code);

  string GenLAndExp(BinaryAST &exp, string &code);

  string GenLOrExp(BinaryAST &exp, string &code);

  void GenFuncDef(FuncDefAST &funcDef, string &code);

  void GenCompUnit(CompUnitAST &unit, string &code);

  void GenIfElse(IfAST &stmt, string &code);

  void GenWhile(WhileAST &stmt, string &code);

  void GenControl(ControlAST &stmt, string &code);

  void GenStmt(StmtAST &stmt, string &code);
};

#define _IRGEN_H_

#endif /* _IRGEN_H_ */
