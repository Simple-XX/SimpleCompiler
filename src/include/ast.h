
// This file is a part of Simple-XX/SimpleCompiler
// (https://github.com/Simple-XX/SimpleCompiler).
//
// ast.h for Simple-XX/SimpleCompiler.

#ifndef _AST_H_
#define _AST_H_

#include <memory>
#include <string>
#include <vector>
#include "type.h"

using namespace std;

class MetaAST;

typedef std::unique_ptr<MetaAST> ASTPtr;
typedef std::vector<ASTPtr> ASTPtrList;

class MetaAST {
    public:
        virtual ~MetaAST() = default;
};

// Compile Unit 编译单元
class CompUnitAST : public MetaAST {
    private:
        ASTPtrList units; 
    public:
        CompUnitAST(ASTPtrList u) : units(move(u)) {}
        // construction
        ~CompUnitAST() override {
            for (auto &unit : units) {
                if (unit) unit.reset();
            }
        }
        // destruction
};

// Statement 语句
class StmtAST : public MetaAST {
    private:
        ASTPtr stmt; 
    public:
        StmtAST(ASTPtr s) : stmt(move(s)) {}
        // construction
        ~StmtAST() override {
            if (stmt) stmt.reset();
        }
        // destruction
};

// FunctionDefinition 函数定义
class FuncDefAST : public MetaAST {
    private:
        Type type; 
        // function return type 函数返回类型
        string name; 
        // function name 函数名
        ASTPtrList params; 
        // function params 函数参数列表
        ASTPtr body; 
        // function body 函数体
    public:
        FuncDefAST(Type t, const string &n, ASTPtrList p, ASTPtr b) : type(t), name(n), params(move(p)), body(move(b)) {}
        // construction
        ~FuncDefAST() override {
            for (auto &param : params) {
                if (param) param.reset();
            }
            if (body) body.reset();
        }
        // destruction
};

// FunctionCall 函数调用
class FuncCallAST : public MetaAST {
    private:
        string name;
        ASTPtrList args;
    public:
        FuncCallAST(const string &n, ASTPtrList a) : name(n), args(move(a)) {}
        // construction
        ~FuncCallAST() override {
            for (auto &arg: args) {
                if (arg) arg.reset();
            }
        }
        // destruction
};

// VarDeclaration 变量声明
class VarDeclAST : public MetaAST {
    private:
        ASTPtrList vars;
        // many vars, for example: int a,b,c,d;
        bool isConst;
        // const or not
    public:
        VarDeclAST(ASTPtrList v, bool i) : vars(move(v)), isConst(i) {}
        // construction
        ~VarDeclAST() override {
            for (auto &var: vars) {
                if (var) var.reset();
            }
        }
        // destruction
};

// VarDefinition 变量定义
class VarDefAST : public MetaAST {
    private:
        ASTPtr var;
        // Ident
        ASTPtr initVal; 
        // initial value
        bool isConst;
        // const or not
    public:
        VarDefAST(ASTPtr v, ASTPtr init, bool i) : var(move(v)), initVal(move(init)), isConst(i) {}
        // construction
        ~VarDefAST() override {
            if (var) var.reset();
            if (initVal) initVal.reset();
        }
        // destruction
};

// Ident 变量
class IdAST : public MetaAST {
    private:
        string name;
        VarType type;
        bool isConst;
    public:
        IdAST(const string &n, VarType t, bool i) : name(n), type(t), isConst(i) {}
        // construction
        ~IdAST() override {}
        // destruction
};

// InitialValue 初始值
class InitValAST : public MetaAST {
    private:
        VarType type;
        ASTPtrList values;
    public:
        InitValAST(VarType t ,ASTPtrList v) : type(t), values(move(v)) {}
        // construction
        ~InitValAST() override {
            for (auto &value: values) {
                if (value) value.reset();
            }
        }
        // destruction
};

// Block 块作用域
class BlockAST : public MetaAST {
    private:
        ASTPtrList stmts; 
        // block statements 一串语句
    public:
        BlockAST(ASTPtrList s) : stmts(move(s)) {}
        // construction
        ~BlockAST() override {
            for (auto &s : stmts) {
                if (s) s.reset();
            }
        }
        // destruction
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
        BinaryAST(Operator o, ASTPtr l, ASTPtr r) : op(o), left(move(l)), right(move(r)) {}
        // construction
        ~BinaryAST() override {
            if (left) left.reset();
            if (right) right.reset();
        }
        // destruction
};

// UnaryExpression 一元表达式 (op A)
class UnaryAST : public MetaAST {
    private:
        Operator op;
        // operator
        ASTPtr exp;

        // expression
    public:
        UnaryAST(Operator o, ASTPtr e) : op(o), exp(move(e)) {}
        // construction
        ~UnaryAST() override {
            if (exp) exp.reset();
        }
        // destruction
};

// Number 数字（int）
class NumAST : public MetaAST {
    private:
        int val;
        // number value
    public:
        NumAST(int v) : val(v) {}
        // construction
        ~NumAST() override {}
        // destruction
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
        IfAST(ASTPtr c, ASTPtr t, ASTPtr e) : conditionExp(move(c)), thenAST(move(t)), elseAST(move(e)) {}
        // construction
        ~IfAST() override {
            if (conditionExp) conditionExp.reset();
            if (thenAST) thenAST.reset();
            if (elseAST) elseAST.reset();
        }
        // destruction
};

// While 循环
class WhileAST : public MetaAST {
    private:
        ASTPtr conditionExp;
        // condition expression, decide whether to continue or not
        ASTPtr body;
        // loop body
    public:
        WhileAST(ASTPtr c, ASTPtr b) : conditionExp(move(c)), body(move(b)) {}
        // construction
        ~WhileAST() override {
            if (conditionExp) conditionExp.reset();
            if (body) body.reset();
        }
        // destruction
};

// Control 控制语句 (break continue return)
class ControlAST : public MetaAST {
    private:
        Control type;
        // control type: break_c continue_c return_c
        ASTPtr returnStmt;
        // to which statement (destination)
    public:
        ControlAST(Control t, ASTPtr r) : type(t), returnStmt(move(r)) {}
        // construction
        ~ControlAST() override {
            if (returnStmt) returnStmt.reset();
        }
        // destruction
};

// Assignment 赋值语句 (break continue return)
class AssignAST : public MetaAST {
    private:
        ASTPtr left;
        // LVal
        ASTPtr right;
        // Expression
    public:
        AssignAST(ASTPtr l, ASTPtr r) : left(move(l)), right(move(r)) {}
        // construction
        ~AssignAST() override {
            if (left) left.reset();
            if (right) right.reset();
        }
        // destruction
};

// LeftValue 左值
class LValAST : public MetaAST {
    private:
        string name;
        VarType type;
        ASTPtrList position;
    public:
        LValAST(const string &n, VarType t ,ASTPtrList p) : name(n), type(t), position(move(p)) {}
        // construction
        ~LValAST() override {
            for (auto &pos: position) {
                if (pos) pos.reset();
            }
        }
        // destruction
};

#endif /* _AST_H_ */