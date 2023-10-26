
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

#ifndef SIMPLECOMPILER_IRGEN_H
#define SIMPLECOMPILER_IRGEN_H

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
  IRGenerator(std::map<std::string, Function> _FuncTable,
              const std::map<int, std::map<std::string, Var>> &_BlockVars)
      : FuncTable(std::move(_FuncTable)) {
    for (auto &item1 : _BlockVars) {
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

  void GenerateValue(const std::string &_varName, int &_idx, int _indx,
                     InitValAST *_init, std::vector<int> _dim, int _i,
                     std::string &_code);

  void GenVarDecl(VarDeclAST &_varDecl, std::string &_code);

  std::string GenId(ProcessedIdAST &_id, std::string &_code);

  std::string GenNumber(NumAST &_num, std::string &_code);

  std::string GenVarDef(VarDefAST &_varDef, std::string &_code);

  std::string GenAssign(AssignAST &_assign, std::string &_code);

  std::string GenBinaryExp(BinaryAST &_exp, std::string &_code);

  std::string GenInitVal(InitValAST &_init, std::string &_code);

  static std::string op2char(Operator _op);

  void GenBlock(BlockAST &_block, std::string &_code);

  std::string GenFuncCall(FuncCallAST &_func, std::string &_code);

  std::string GenLVal(LValAST &_lval, std::string &_code);

  std::string GenUnaryExp(UnaryAST &_exp, std::string &_code);

  std::string GenLAndExp(BinaryAST &_exp, std::string &_code);

  std::string GenLOrExp(BinaryAST &_exp, std::string &_code);

  void GenFuncDef(FuncDefAST &_funcDef, std::string &_code);

  void GenCompUnit(CompUnitAST &_unit, std::string &_code);

  void GenIfElse(IfAST &_stmt, std::string &_code);

  void GenWhile(WhileAST &_stmt, std::string &_code);

  void GenControl(ControlAST &_stmt, std::string &_code);

  void GenStmt(StmtAST &_stmt, std::string &_code);
};

#endif /* SIMPLECOMPILER_IRGEN_H */
