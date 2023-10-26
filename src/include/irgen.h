
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

class IRGenerator {
private:
  int t_num;
  int T_num;
  int l_num;
  int cur_break_l;
  int cur_continue_l;
  int currentDepth;
  int currentBlock;
  std::string currentFunc;

  std::map<int, std::map<std::string, GenVar>> BlockSymbolTable;
  std::vector<int> parentBlock;
  std::map<std::string, Function> FuncTable;
  std::vector<GenVar> ReverseSymbolTable;

public:
  IRGenerator(std::map<std::string, Function> __FuncTable,
              const std::map<int, std::map<std::string, Var>> &BlockVars)
      : FuncTable(std::move(__FuncTable)) {
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

  void GenerateValue(const std::string &varName, int &idx, int indx,
                     InitValAST *init, std::vector<int> dim, int i,
                     std::string &code);

  void GenVarDecl(VarDeclAST &varDecl, std::string &code);

  std::string GenId(ProcessedIdAST &id, std::string &code);

  std::string GenNumber(NumAST &num, std::string &code);

  std::string GenVarDef(VarDefAST &varDef, std::string &code);

  std::string GenAssign(AssignAST &assign, std::string &code);

  std::string GenBinaryExp(BinaryAST &exp, std::string &code);

  std::string GenInitVal(InitValAST &init, std::string &code);

  static std::string op2char(Operator op);

  void GenBlock(BlockAST &block, std::string &code);

  std::string GenFuncCall(FuncCallAST &func, std::string &code);

  std::string GenLVal(LValAST &lval, std::string &code);

  std::string GenUnaryExp(UnaryAST &exp, std::string &code);

  std::string GenLAndExp(BinaryAST &exp, std::string &code);

  std::string GenLOrExp(BinaryAST &exp, std::string &code);

  void GenFuncDef(FuncDefAST &funcDef, std::string &code);

  void GenCompUnit(CompUnitAST &unit, std::string &code);

  void GenIfElse(IfAST &stmt, std::string &code);

  void GenWhile(WhileAST &stmt, std::string &code);

  void GenControl(ControlAST &stmt, std::string &code);

  void GenStmt(StmtAST &stmt, std::string &code);
};

#define _IRGEN_H_

#endif /* _IRGEN_H_ */
