
/**
 * @file irast.h
 * @brief irast
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

#ifndef SIMPLECOMPILER_IRAST_H
#define SIMPLECOMPILER_IRAST_H

#include <iostream>
#include <memory>
#include <optional>
#include <set>
#include <string>
#include <utility>
#include <vector>

#include "ir.h"
#include "utils.h"

class LowIRGenerator;

class BaseIR {
public:
  int lineno;
  BaseIR(int l) : lineno(l) {}
  virtual ~BaseIR() = default;
  virtual std::string Generate(LowIRGenerator &_generator,
                               std::string &_code) = 0;
};

using IRPtr = std::unique_ptr<BaseIR>;
using IRPtrList = std::vector<IRPtr>;

class DeclIR : public BaseIR {
  VarType varType;
  int size;
  std::string name;

public:
  DeclIR(VarType _type, int _size, std::string _name, int _line)
      : BaseIR(_line), varType(_type), size(_size), name(std::move(_name)) {}

  VarType getType() const { return varType; }
  int getSize() const { return size; }
  const std::string &getName() const { return name; }

  virtual std::string Generate(LowIRGenerator &_generator, std::string &_code);
};

class InitIR : public BaseIR {
  VarType varType;
  std::string name;
  int pos;
  int val;

public:
  InitIR(VarType _type, std::string _name, int _p, int _v, int _line)
      : BaseIR(_line), varType(_type), name(std::move(_name)), pos(_p),
        val(_v) {}

  VarType getType() const { return varType; }
  int getPos() const { return pos; }
  int getVal() const { return val; }
  const std::string &getName() const { return name; }

  virtual std::string Generate(LowIRGenerator &_generator, std::string &_code);
};

class FuncDefIR : public BaseIR {
  std::string name;
  int paramNum;
  IRPtr body;

public:
  FuncDefIR(std::string _name, int _param, IRPtr _body, int _line)
      : BaseIR(_line), name(std::move(_name)), paramNum(_param),
        body(std::move(_body)) {}

  const std::string &getName() const { return name; }
  int getParamNum() const { return paramNum; }
  const IRPtr &getBody() const { return body; }

  virtual std::string Generate(LowIRGenerator &generator, std::string &code);
};

class StatementsIR : public BaseIR {
  IRPtrList stmts;

public:
  StatementsIR(IRPtrList _stmt, int _line)
      : BaseIR(_line), stmts(std::move(_stmt)) {}

  const IRPtrList &getStmts() const { return stmts; }

  virtual std::string Generate(LowIRGenerator &_generator, std::string &_code);
};

class BinaryExpIR : public BaseIR {
  IRPtr lhs; // right value
  IRPtr rhs; // right value
  Operator op;

public:
  BinaryExpIR(IRPtr _l, IRPtr _r, Operator _op, int _line)
      : BaseIR(_line), lhs(std::move(_l)), rhs(std::move(_r)), op(_op) {}

  const IRPtr &getLHS() const { return lhs; }
  const IRPtr &getRHS() const { return rhs; }
  const Operator &getOp() const { return op; }

  virtual std::string Generate(LowIRGenerator &_generator, std::string &_code);
};

class UnaryExpIR : public BaseIR {
  Operator op;
  IRPtr exp; // right value;

public:
  UnaryExpIR(IRPtr _exp, Operator _op, int _line)
      : BaseIR(_line), op(_op), exp(std::move(_exp)) {}

  const IRPtr &getExp() const { return exp; }
  const Operator &getOp() const { return op; }

  virtual std::string Generate(LowIRGenerator &_generator, std::string &_code);
};

class AssignIR : public BaseIR {
  IRPtr lhs;
  IRPtr rhs;

public:
  AssignIR(IRPtr _l, IRPtr _r, int _line)
      : BaseIR(_line), lhs(std::move(_l)), rhs(std::move(_r)) {}

  const IRPtr &getLHS() const { return lhs; }
  const IRPtr &getRHS() const { return rhs; }

  virtual std::string Generate(LowIRGenerator &_generator, std::string &_code);
};

class CondGotoIR : public BaseIR {
  IRPtr cond;
  int labelNum; // label

public:
  CondGotoIR(IRPtr _cond, int _num, int _line)
      : BaseIR(_line), cond(std::move(_cond)), labelNum(_num) {}

  const IRPtr &getCond() const { return cond; }
  int getLabel() const { return labelNum; }

  virtual std::string Generate(LowIRGenerator &_generator, std::string &_code);
};

class LValIR : public BaseIR {
  VarType varType;
  std::string name;
  IRPtr pos;

public:
  LValIR(VarType _var, std::string _n, int _line, IRPtr _p = nullptr)
      : BaseIR(_line), varType(_var), name(std::move(_n)), pos(std::move(_p)) {}

  VarType getType() const { return varType; }
  const IRPtr &getPos() const { return pos; }
  const std::string &getName() const { return name; }

  virtual std::string Generate(LowIRGenerator &_generator, std::string &_code);
};

class GotoIR : public BaseIR {
  int label; // label

public:
  GotoIR(int _l, int _line) : BaseIR(_line), label(_l) {}

  int getLabel() const { return label; }

  virtual std::string Generate(LowIRGenerator &_generator, std::string &_code);
};

class Label : public BaseIR {
  int num;

public:
  Label(int _n, int _line) : BaseIR(_line), num(_n) {}

  int getNum() const { return num; }

  virtual std::string Generate(LowIRGenerator &_generator, std::string &_code);
};

class ParamListIR : public BaseIR {
  IRPtrList params; // right value;

public:
  ParamListIR(IRPtrList _p, int _line) : BaseIR(_line), params(std::move(_p)) {}

  const IRPtrList &getParams() const { return params; }

  virtual std::string Generate(LowIRGenerator &_generator, std::string &_code);
};

class FuncCallIR : public BaseIR {
  std::string funcName;

public:
  FuncCallIR(std::string _name, int _line)
      : BaseIR(_line), funcName(std::move(_name)) {}

  const std::string &getName() const { return funcName; }

  virtual std::string Generate(LowIRGenerator &_generator, std::string &_code);
};

class ReturnIR : public BaseIR {
  IRPtr ret; // right value or null

public:
  ReturnIR(IRPtr _r, int _line) : BaseIR(_line), ret(std::move(_r)) {}

  const IRPtr &getReturnValue() const { return ret; }

  virtual std::string Generate(LowIRGenerator &_generator, std::string &_code);
};

class RightValIR : public BaseIR {
  IRToken type;
  std::string name;
  int value;

public:
  RightValIR(IRToken _t, std::string _name, int _line)
      : BaseIR(_line), type(_t), name(std::move(_name)) {}
  RightValIR(IRToken _t, int val, int line)
      : BaseIR(line), type(_t), value(val) {}

  const IRToken &getType() const { return type; }
  int getVal() const { return value; }
  const std::string &getName() const { return name; }

  virtual std::string Generate(LowIRGenerator &_generator, std::string &_code);
};

class ProgramIR : public BaseIR {
  IRPtrList nodes;

public:
  ProgramIR(IRPtrList _n, int _line) : BaseIR(_line), nodes(std::move(_n)) {}

  const IRPtrList &getNodes() const { return nodes; }

  virtual std::string Generate(LowIRGenerator &_generator, std::string &_code);
};

#endif /* SIMPLECOMPILER_IRAST_H */
