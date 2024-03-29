
/**
 * @file ast.h
 * @brief ast
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

#ifndef SIMPLECOMPILER_AST_H
#define SIMPLECOMPILER_AST_H

#include <memory>
#include <string>
#include <vector>

#include "type.h"

class MetaAST;
class TypeCheck;
class IRGenerator;

using ASTPtr = std::unique_ptr<MetaAST>;
using ASTPtrList = std::vector<ASTPtr>;

class MetaAST {
public:
  virtual ~MetaAST() = default;
  virtual std::string to_string() = 0;
  virtual ASTPtr Eval(TypeCheck &_checker) = 0;
  virtual std::string GenerateIR(IRGenerator &_gen, std::string &_code) = 0;
};

// Compile Unit 编译单元
class CompUnitAST : public MetaAST {
private:
  ASTPtrList units;

public:
  CompUnitAST(ASTPtrList _u) : units(std::move(_u)) {}
  // construction
  ~CompUnitAST() override {
    for (auto &unit : units) {
      if (unit)
        unit.reset();
    }
  }
  // destruction
  std::string to_string() override {
    std::string output;
    for (auto &unit : units) {
      output = output + "\n" + unit->to_string();
    }
    return "CompUnit: [" + output + "]\n";
  }

  ASTPtr Eval(TypeCheck &_checker) override;

  std::string GenerateIR(IRGenerator &_gen, std::string &_code) override;

  const ASTPtrList &getNodes() const { return units; }
};

// Statement 语句
class StmtAST : public MetaAST {
private:
  ASTPtr stmt;

public:
  StmtAST(ASTPtr _s) : stmt(std::move(_s)) {}
  // construction
  ~StmtAST() override {
    if (stmt)
      stmt.reset();
  }
  // destruction
  std::string to_string() override {
    return "Statement: {" + stmt->to_string() + "}\n";
  }

  ASTPtr Eval(TypeCheck &_checker) override;

  std::string GenerateIR(IRGenerator &_gen, std::string &_code) override;

  const ASTPtr &getStmt() const { return stmt; }
};

// FunctionDefinition 函数定义
class FuncDefAST : public MetaAST {
private:
  Type type;
  // function return type 函数返回类型
  std::string name;
  // function name 函数名
  ASTPtrList params;
  // function params 函数参数列表
  ASTPtr body;
  // function body 函数体
public:
  FuncDefAST(Type t, const std::string &_n, ASTPtrList _p, ASTPtr _b)
      : type(t), name(_n), params(std::move(_p)), body(std::move(_b)) {}
  // construction
  ~FuncDefAST() override {
    for (auto &param : params) {
      if (param)
        param.reset();
    }
    if (body)
      body.reset();
  }
  // destruction
  std::string to_string() override {
    std::string output =
        "FunctionDef(" + type_to_string(type) + "): " + name + ' ';
    for (auto &param : params) {
      if (param)
        output = output + param->to_string();
    }
    if (body)
      output = output + body->to_string();
    return output;
  }

  ASTPtr Eval(TypeCheck &_checker) override;

  std::string GenerateIR(IRGenerator &_gen, std::string &_code) override;

  Type getType() const { return type; }

  const std::string &getName() const { return name; }

  const ASTPtrList &getArgs() const { return params; }

  const ASTPtr &getBody() const { return body; }
};

// FunctionCall 函数调用
class FuncCallAST : public MetaAST {
private:
  std::string name;
  ASTPtrList args;

public:
  FuncCallAST(const std::string &_n, ASTPtrList _a = ASTPtrList{})
      : name(_n), args(std::move(_a)) {}
  // construction
  ~FuncCallAST() override {
    for (auto &arg : args) {
      if (arg)
        arg.reset();
    }
  }
  // destruction
  std::string to_string() override { return "FuncCallAST"; }

  ASTPtr Eval(TypeCheck &_checker) override;

  std::string GenerateIR(IRGenerator &_gen, std::string &_code) override;

  const std::string &getName() const { return name; }

  const ASTPtrList &getArgs() const { return args; }
};

// VarDeclaration 变量声明
class VarDeclAST : public MetaAST {
private:
  ASTPtrList vars;
  // many vars, for example: int a,b,c,d;
  bool Const;
  // const or not
public:
  VarDeclAST(bool _i, ASTPtrList _v) : vars(std::move(_v)), Const(_i) {}
  // construction
  ~VarDeclAST() override {
    for (auto &var : vars) {
      if (var)
        var.reset();
    }
  }
  // destruction
  std::string to_string() override {
    std::string output;
    for (auto &unit : vars) {
      output = output + "\n" + unit->to_string();
    }
    if (Const) {
      return "VarDeclAST (CONST): {" + output + "}";
    }
    return "VarDeclAST: {" + output + "}";
  }

  ASTPtr Eval(TypeCheck &_checker) override;

  std::string GenerateIR(IRGenerator &_gen, std::string &_code) override;

  bool isConst() const { return Const; }

  const ASTPtrList &getVarDefs() const { return vars; }
};

// VarDefinition 变量定义
class VarDefAST : public MetaAST {
private:
  ASTPtr var;
  // Ident
  ASTPtr initVal;
  // initial value
  bool Const;
  // const or not
public:
  VarDefAST(bool _i, ASTPtr _v, ASTPtr _init = nullptr)
      : var(std::move(_v)), initVal(std::move(_init)), Const(_i) {}
  // construction
  ~VarDefAST() override {
    if (var)
      var.reset();
    if (initVal)
      initVal.reset();
  }
  // destruction
  std::string to_string() override {
    if (Const) {
      return "VarDefAST (CONST): {" + var->to_string() + "}";
    }
    return "VarDefAST: { " + var->to_string() + " }";
  }

  ASTPtr Eval(TypeCheck &_checker) override;

  std::string GenerateIR(IRGenerator &_gen, std::string &_code) override;

  const ASTPtr &getVar() const { return var; }

  const ASTPtr &getInitVal() const { return initVal; }

  bool isConst() const { return Const; }
};

// Ident 变量
class IdAST : public MetaAST {
private:
  std::string name;
  VarType type;
  ASTPtrList dim;
  bool Const;

public:
  IdAST(const std::string &_n, VarType _t, bool _i,
        ASTPtrList _d = ASTPtrList{})
      : name(_n), type(_t), dim(std::move(_d)), Const(_i) {}
  // construction
  ~IdAST() override {
    for (auto &d : dim) {
      d.reset();
    }
  }
  // destruction
  std::string to_string() override {
    if (Const) {
      return "IdAST (CONST) (" + vartype_to_string(type) + "): " + name;
    }
    return "IdAST(" + vartype_to_string(type) + "): " + name;
  }

  ASTPtr Eval(TypeCheck &_checker) override;

  std::string GenerateIR(IRGenerator &_gen, std::string &_code) override;

  const std::string getName() const { return name; }

  VarType getType() const { return type; }

  const ASTPtrList &getDim() const { return dim; }

  bool isConst() const { return Const; }
};

// Ident 变量 (processed)
class ProcessedIdAST : public MetaAST {
private:
  std::string name;
  VarType type;
  std::vector<int> dim;
  bool Const;

public:
  ProcessedIdAST(const std::string &_n, VarType _t, bool _i,
                 std::vector<int> _d = std::vector<int>{})
      : name(_n), type(_t), dim(std::move(_d)), Const(_i) {}
  // construction
  ~ProcessedIdAST() override {}
  // destruction
  std::string to_string() override {
    if (Const) {
      return "ProcessedIdAST (CONST) (" + vartype_to_string(type) +
             "): " + name;
    }
    return "ProcessedIdAST(" + vartype_to_string(type) + "): " + name;
  }

  ASTPtr Eval(TypeCheck &_checker) override;

  std::string GenerateIR(IRGenerator &_gen, std::string &_code) override;

  const std::string &getName() const { return std::move(name); }

  const VarType &getType() const { return type; }

  const std::vector<int> &getDim() const { return dim; }

  bool isConst() const { return Const; }
};

// InitialValue 初始值
class InitValAST : public MetaAST {
private:
  VarType type;
  ASTPtrList values;
  std::vector<int> dims;

public:
  InitValAST(VarType _t, ASTPtrList _v,
             std::vector<int> _d = std::vector<int>{})
      : type(_t), values(std::move(_v)), dims(_d) {}
  // construction
  ~InitValAST() override {
    for (auto &value : values) {
      if (value)
        value.reset();
    }
  }
  // destruction
  std::string to_string() override {
    return "InitValAST(" + vartype_to_string(type) + ")";
  }

  ASTPtr Eval(TypeCheck &_checker) override;

  std::string GenerateIR(IRGenerator &_gen, std::string &_code) override;

  bool setDim(std::vector<int> _dims) {
    dims = std::move(_dims);
    return true;
  }

  VarType getType() const { return type; }

  const ASTPtrList &getValues() const { return values; }

  const std::vector<int> &getDims() const { return dims; }
};

// Block 块作用域
class BlockAST : public MetaAST {
private:
  ASTPtrList stmts;
  // block statements 一串语句
public:
  BlockAST(ASTPtrList _s) : stmts(std::move(_s)) {}
  // construction
  ~BlockAST() override {
    for (auto &s : stmts) {
      if (s)
        s.reset();
    }
  }
  // destruction
  std::string to_string() override {
    std::string output;
    for (auto &unit : stmts) {
      output = output + "\n" + unit->to_string();
    }
    return "BlockAST: {" + output + "}";
  }

  ASTPtr Eval(TypeCheck &_checker) override;

  std::string GenerateIR(IRGenerator &_gen, std::string &_code) override;

  const ASTPtrList &getStmts() const { return stmts; };
};

// BinaryExpression 二元表达式 (A op B)
class BinaryAST : public MetaAST {
private:
  Operator op;
  // operator
  ASTPtr left;
  // left expression
  ASTPtr right;
  // right expression
public:
  BinaryAST(Operator _o, ASTPtr _l, ASTPtr _r)
      : op(_o), left(std::move(_l)), right(std::move(_r)) {}
  // construction
  ~BinaryAST() override {
    if (left)
      left.reset();
    if (right)
      right.reset();
  }
  // destruction
  std::string to_string() override {
    return '(' + (left->to_string()) + ' ' + op_to_string(op) + ' ' +
           (right->to_string()) + ')';
  }

  ASTPtr Eval(TypeCheck &_checker) override;

  std::string GenerateIR(IRGenerator &_gen, std::string &_code) override;

  const Operator &getOp() const { return op; }

  const ASTPtr &getLHS() const { return left; }

  const ASTPtr &getRHS() const { return right; }
};

// UnaryExpression 一元表达式 (op A)
class UnaryAST : public MetaAST {
private:
  Operator op;
  // operator
  ASTPtr exp;

  // expression
public:
  UnaryAST(ASTPtr _e, Operator _o = Operator::ERROR)
      : op(_o), exp(std::move(_e)) {}
  // construction
  ~UnaryAST() override {
    if (exp)
      exp.reset();
  }
  // destruction
  std::string to_string() override {
    return '(' + op_to_string(op) + ' ' + exp->to_string() + ')';
  }

  ASTPtr Eval(TypeCheck &_checker) override;

  std::string GenerateIR(IRGenerator &_gen, std::string &_code) override;

  const ASTPtr &getNode() const { return exp; }

  Operator getOp() const { return op; }
};

// Number 数字（int）
class NumAST : public MetaAST {
private:
  int val;
  // number value
public:
  NumAST(int _v) : val(_v) {}
  // construction
  ~NumAST() override {}
  // destruction
  std::string to_string() override { return std::to_string(val); }

  ASTPtr Eval(TypeCheck &_checker) override;

  std::string GenerateIR(IRGenerator &_gen, std::string &_code) override;

  const int &getVal() const { return val; }
};

// If 条件表达式
class IfAST : public MetaAST {
private:
  ASTPtr conditionExp;
  // condition expression, decide which branch (then or else) to eval
  ASTPtr thenAST;
  // then branch
  ASTPtr elseAST;
  // else branch
public:
  IfAST(ASTPtr _c, ASTPtr _t, ASTPtr _e = nullptr)
      : conditionExp(std::move(_c)), thenAST(std::move(_t)),
        elseAST(std::move(_e)) {}
  // construction
  ~IfAST() override {
    if (conditionExp)
      conditionExp.reset();
    if (thenAST)
      thenAST.reset();
    if (elseAST)
      elseAST.reset();
  }
  // destruction
  std::string to_string() override {
    if (elseAST)
      return "IfAST: { if (" + conditionExp->to_string() + " ) then ( " +
             thenAST->to_string() + ") else (" + elseAST->to_string() + " ) }";
    else
      return "IfAST: { if (" + conditionExp->to_string() + " ) then ( " +
             thenAST->to_string() + " ) }";
  }

  ASTPtr Eval(TypeCheck &_checker) override;

  std::string GenerateIR(IRGenerator &_gen, std::string &_code) override;

  const ASTPtr &getCond() const { return conditionExp; }

  const ASTPtr &getThenStmt() const { return thenAST; }

  const ASTPtr &getElseStmt() const { return elseAST; }
};

// While 循环
class WhileAST : public MetaAST {
private:
  ASTPtr conditionExp;
  // condition expression, decide whether to continue or not
  ASTPtr body;
  // loop body
public:
  WhileAST(ASTPtr _c, ASTPtr b)
      : conditionExp(std::move(_c)), body(std::move(b)) {}
  // construction
  ~WhileAST() override {
    if (conditionExp)
      conditionExp.reset();
    if (body)
      body.reset();
  }
  // destruction
  std::string to_string() override {
    return "WhileAST: { while (" + conditionExp->to_string() + " ) do ( " +
           body->to_string() + " ) }";
  }

  ASTPtr Eval(TypeCheck &_checker) override;

  std::string GenerateIR(IRGenerator &_gen, std::string &_code) override;

  const ASTPtr &getCond() const { return conditionExp; }

  const ASTPtr &getStmt() const { return body; }
};

// Control 控制语句 (break continue return)
class ControlAST : public MetaAST {
private:
  Control type;
  // control type: break_c continue_c return_c
  ASTPtr returnStmt;
  // to which statement (destination)
public:
  ControlAST(Control _t, ASTPtr _r = nullptr)
      : type(_t), returnStmt(std::move(_r)) {}
  // construction
  ~ControlAST() override {
    if (returnStmt)
      returnStmt.reset();
  }
  // destruction
  std::string to_string() override {
    if (type == Control::break_c) {
      return "ControlAST: BREAK";
    }
    if (type == Control::continue_c) {
      return "ControlAST: CONTINUE";
    }
    if (type == Control::return_c) {
      if (returnStmt) {
        return "ControlAST: RETURN (" + returnStmt->to_string() + ")";
      }
      return "ControlAST: RETURN ";
    }
    return "ERROR";
  }

  ASTPtr Eval(TypeCheck &_checker) override;

  std::string GenerateIR(IRGenerator &_gen, std::string &_code) override;

  Control getControl() const { return type; }

  const ASTPtr &getReturnExp() const { return returnStmt; }
};

// Assignment 赋值语句 (break continue return)
class AssignAST : public MetaAST {
private:
  ASTPtr left;
  // LVal
  ASTPtr right;
  // Expression
public:
  AssignAST(ASTPtr _l, ASTPtr _r) : left(std::move(_l)), right(std::move(_r)) {}
  // construction
  ~AssignAST() override {
    if (left)
      left.reset();
    if (right)
      right.reset();
  }
  // destruction
  std::string to_string() override {
    return " AssignAST: { " + left->to_string() + " = " + right->to_string() +
           " }";
  }

  ASTPtr Eval(TypeCheck &_checker) override;

  std::string GenerateIR(IRGenerator &_gen, std::string &_code) override;

  const ASTPtr &getLeft() const { return left; }

  const ASTPtr &getRight() const { return right; }
};

// LeftValue 左值
class LValAST : public MetaAST {
private:
  std::string name;
  VarType type;
  ASTPtrList position;

public:
  LValAST(const std::string &_n, VarType _t, ASTPtrList _p = ASTPtrList{})
      : name(_n), type(_t), position(std::move(_p)) {}
  // construction
  ~LValAST() override {
    for (auto &pos : position) {
      if (pos)
        pos.reset();
    }
  }
  // destruction
  std::string to_string() override {
    return "LValAST:(" + vartype_to_string(type) + "):  { " + name + " }";
  }

  ASTPtr Eval(TypeCheck &_checker) override;

  std::string GenerateIR(IRGenerator &_gen, std::string &_code) override;

  const ASTPtrList &getPosition() const { return position; }

  const std::string &getName() const { return name; }

  VarType getType() const { return type; }
};

// 空指令
class EmptyAST : public MetaAST {
public:
  EmptyAST() {}
  // construction
  ~EmptyAST() override {}
  // destruction
  std::string to_string() override { return "EmptyAST"; }

  ASTPtr Eval(TypeCheck &_checker) override;

  std::string GenerateIR(IRGenerator &_gen, std::string &_code) override;
};

#endif /* SIMPLECOMPILER_AST_H */
