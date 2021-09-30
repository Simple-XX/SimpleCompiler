
// This file is a part of Simple-XX/SimpleCompiler
// (https://github.com/Simple-XX/SimpleCompiler).
//
// irast.h for Simple-XX/SimpleCompiler.

#ifndef _IRAST_H_
#define _IRAST_H_

#include <iostream>
#include <utility>
#include <vector>
#include <memory>
#include <optional>
#include <string>
#include <set>
#include "utils.h"
#include "ir.h"

class LowIRGenerator;

class BaseIR {
public:
    int lineno;
    BaseIR(int l): lineno(l) {}
    virtual ~BaseIR() = default;
    virtual std::string Generate(LowIRGenerator &generator, std::string &code) = 0;
};

using IRPtr = std::unique_ptr<BaseIR>;
using IRPtrList = std::vector<IRPtr>;


class DeclIR: public BaseIR {
    VarType varType;
    int size;
    std::string name;

public:
    DeclIR(VarType _type, int _size, std::string _name, int line): BaseIR(line), varType(_type), size(_size), name(std::move(_name)) {}

    const VarType getType() const { return varType; }
    const int getSize() const { return size; }
    const std::string getName() const { return name; }

    virtual std::string Generate(LowIRGenerator &generator, std::string &code);
};

class InitIR: public BaseIR {
    VarType varType;
    std::string name;
    int pos;
    int val;

public:
    InitIR(VarType _type, std::string _name, int _p, int _v, int line): BaseIR(line), varType(_type), name(std::move(_name)), pos(_p), val(_v) {}

    const VarType getType() const { return varType; }
    const int getPos() const { return pos; }
    const int getVal() const { return val; }
    const std::string getName() const { return name; }

    virtual std::string Generate(LowIRGenerator &generator, std::string &code);
};

class FuncDefIR: public BaseIR {
    std::string name;
    int paramNum;
    IRPtr body;

public:
    FuncDefIR(std::string _name, int param, IRPtr body, int line): BaseIR(line), name(std::move(_name)), paramNum(param), body(std::move(body)) {}

    const std::string getName() const { return name; }
    const int getParamNum() const { return paramNum; }
    const IRPtr &getBody() const { return body; }

    virtual std::string Generate(LowIRGenerator &generator, std::string &code);
};

class StatementsIR: public BaseIR {
    IRPtrList stmts;
public:
    StatementsIR(IRPtrList _stmt, int line): BaseIR(line), stmts(std::move(_stmt)) {}

    const IRPtrList &getStmts() const { return stmts; }

    virtual std::string Generate(LowIRGenerator &generator, std::string &code);
};

class BinaryExpIR: public BaseIR {
    IRPtr lhs; // right value
    IRPtr rhs; // right value
    Operator op;
public:
    BinaryExpIR(IRPtr _l, IRPtr _r, Operator _op, int line): BaseIR(line), lhs(std::move(_l)), rhs(std::move(_r)), op(_op) {}

    const IRPtr &getLHS() const { return lhs; }
    const IRPtr &getRHS() const { return rhs; }
    const Operator getOp() const { return op; }

    virtual std::string Generate(LowIRGenerator &generator, std::string &code);
};

class UnaryExpIR: public BaseIR {
    Operator op;
    IRPtr exp; // right value;

public:
    UnaryExpIR(IRPtr _exp, Operator _op, int line): BaseIR(line), exp(std::move(_exp)), op(_op) {}

    const IRPtr &getExp() const { return exp; }
    const Operator &getOp() const { return op; }

    virtual std::string Generate(LowIRGenerator &generator, std::string &code);
};

class AssignIR: public BaseIR {
    IRPtr lhs;
    IRPtr rhs;

public:
    AssignIR(IRPtr _l, IRPtr _r, int line): BaseIR(line), lhs(std::move(_l)), rhs(std::move(_r)) {}

    const IRPtr &getLHS() const { return lhs; }
    const IRPtr &getRHS() const { return rhs; }

    virtual std::string Generate(LowIRGenerator &generator, std::string &code);
};

class CondGotoIR: public BaseIR {
    IRPtr cond;
    int labelNum; // label

public:
    CondGotoIR(IRPtr _cond, int num, int line): BaseIR(line), cond(std::move(_cond)), labelNum(num) {}

    const IRPtr &getCond() const { return cond; }
    const int getLabel() const { return labelNum; }

    virtual std::string Generate(LowIRGenerator &generator, std::string &code);
};

class LValIR: public BaseIR {
    VarType varType;
    std::string name;
    IRPtr pos;

public:
    LValIR(VarType var, std::string _n, int line, IRPtr _p = nullptr): BaseIR(line), varType(var), name(std::move(_n)), pos(std::move(_p)) {}

    const VarType getType() const { return varType; }
    const IRPtr &getPos() const { return pos; }
    const std::string getName() const { return name; }

    virtual std::string Generate(LowIRGenerator &generator, std::string &code);
};

class GotoIR: public BaseIR {
    int label; // label

public:
    GotoIR(int l, int line): BaseIR(line), label(l) {}

    const int getLabel() const { return label; }

    virtual std::string Generate(LowIRGenerator &generator, std::string &code);
};

class Label: public BaseIR {
    int num;

public:
    Label(int _n, int line): BaseIR(line), num(_n) {}

    const int getNum() const { return num; }

    virtual std::string Generate(LowIRGenerator &generator, std::string &code);
};

class ParamListIR: public BaseIR {
    IRPtrList params; // right value;

public:
    ParamListIR(IRPtrList p, int line): BaseIR(line), params(std::move(p)) {}

    const IRPtrList &getParams() const { return params; }

    virtual std::string Generate(LowIRGenerator &generator, std::string &code);
};

class FuncCallIR: public BaseIR {
    std::string funcName;

public:
    FuncCallIR(std::string name, int line): BaseIR(line), funcName(std::move(name)) {}

    const std::string getName() const { return funcName; }

    virtual std::string Generate(LowIRGenerator &generator, std::string &code);
};

class ReturnIR: public BaseIR {
    IRPtr ret; // right value or null

public:
    ReturnIR(IRPtr r, int line): BaseIR(line), ret(std::move(r)) {}

    const IRPtr &getReturnValue() const { return ret; }

    virtual std::string Generate(LowIRGenerator &generator, std::string &code);
};

class RightValIR: public BaseIR {
    IRToken type;
    std::string name;
    int value;

public:
    RightValIR(IRToken _t, std::string _name, int line): BaseIR(line), type(_t), name(std::move(_name)) {}
    RightValIR(IRToken _t, int val, int line): BaseIR(line), type(_t), value(val) {}

    const IRToken getType() const { return type; }
    const int getVal() const { return value; }
    const std::string getName() const { return name; }

    virtual std::string Generate(LowIRGenerator &generator, std::string &code);
};

class ProgramIR: public BaseIR {
    IRPtrList nodes;

public:
    ProgramIR(IRPtrList _n, int line): BaseIR(line), nodes(std::move(_n)) {}

    const IRPtrList &getNodes() const { return nodes; }

    virtual std::string Generate(LowIRGenerator &generator, std::string &code);
};

#endif /* _IRAST_H_ */
