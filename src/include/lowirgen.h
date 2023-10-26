
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

#ifndef _LOWIRGEN_H_
#define _LOWIRGEN_H_

#include "ir.h"
#include "irast.h"
#include <map>
#include <utility>

class LowIRGenerator {
private:
    std::string currentFunc;
    int v_num;
    int reg_num;

    struct Variable {
        int v_num;
        VarType varType;
        Variable() {}
        Variable(int num, VarType type): v_num(num), varType(type) {}
    };

    struct StackVar {
        int pos_min;
        int pos_max;
        VarType type;
        std::string func;
        StackVar() {}
        StackVar(int _p_min, int _p_max, std::string _f, VarType _t): pos_min(_p_min), pos_max(_p_max), func(std::move(_f)), type(_t) {}
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

    std::string op2char(Operator op);

    std::string GenDecl(DeclIR &decl, std::string &code);

    std::string GenInit(InitIR &init, std::string &code);

    std::string GenFuncDef(FuncDefIR &funcDef, std::string &code);

    std::string GenStatements(StatementsIR &stmts, std::string &code);

    std::string GenBinaryExp(BinaryExpIR &binary, std::string &code);

    std::string GenUnaryExp(UnaryExpIR &unary, std::string &code);

    std::string GenAssign(AssignIR &assign, std::string &code);

    std::string GenCondGoto(CondGotoIR &cond, std::string &code);

    std::string GenLVal(LValIR &lval, std::string &code);

    std::string GenGoto(GotoIR &gt, std::string &code);

    std::string GenLabel(Label &label, std::string &code);

    std::string GenParamList(ParamListIR &params, std::string &code);

    std::string GenFuncCall(FuncCallIR &funcCall, std::string &code);

    std::string GenReturn(ReturnIR &ret, std::string &code);

    std::string GenRightVal(RightValIR &rightval, std::string &code);

    std::string GenProgram(ProgramIR &program, std::string &code);
};

#endif /* _LOWIRGEN_H_ */
