
/**
 * @file lowirgen.h
 * @brief lowirgen
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

#ifndef SIMPLECOMPILER_LOWIRGEN_H
#define SIMPLECOMPILER_LOWIRGEN_H

#include <map>
#include <utility>

#include "ir.h"
#include "irast.h"

class LowIRGenerator {
private:
  std::string currentFunc;
  int v_num;
  int reg_num;

  struct Variable {
    int v_num;
    VarType varType;
    Variable() {}
    Variable(int num, VarType type) : v_num(num), varType(type) {}
  };

  struct StackVar {
    int pos_min;
    int pos_max;
    VarType type;
    std::string func;
    StackVar() {}
    StackVar(int _p_min, int _p_max, std::string _f, VarType _t)
        : pos_min(_p_min), pos_max(_p_max), func(std::move(_f)), type(_t) {}
  };

  std::map<std::string, Variable> globalVars;
  std::map<std::string, int> funcStack;
  std::map<std::string, StackVar> varStack;

public:
  LowIRGenerator() {
    v_num = 0;
    currentFunc = "";
    reg_num = 1;
  }

  std::string op2char(Operator _op);

  std::string GenDecl(DeclIR &_decl, std::string &_code);

  std::string GenInit(InitIR &_init, std::string &_code);

  std::string GenFuncDef(FuncDefIR &_funcDef, std::string &_code);

  std::string GenStatements(StatementsIR &_stmts, std::string &_code);

  std::string GenBinaryExp(BinaryExpIR &_binary, std::string &_code);

  std::string GenUnaryExp(UnaryExpIR &_unary, std::string &_code);

  std::string GenAssign(AssignIR &_assign, std::string &_code);

  std::string GenCondGoto(CondGotoIR &_cond, std::string &_code);

  std::string GenLVal(LValIR &_lval, std::string &_code);

  std::string GenGoto(GotoIR &_gt, std::string &_code);

  std::string GenLabel(Label &_label, std::string &_code);

  std::string GenParamList(ParamListIR &_params, std::string &_code);

  std::string GenFuncCall(FuncCallIR &_funcCall, std::string &_code);

  std::string GenReturn(ReturnIR &_ret, std::string &_code);

  std::string GenRightVal(RightValIR &_rightval, std::string &_code);

  std::string GenProgram(ProgramIR &_program, std::string &_code);
};

#endif /* SIMPLECOMPILER_LOWIRGEN_H */
