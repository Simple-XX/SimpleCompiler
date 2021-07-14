
// This file is a part of Simple-XX/SimpleCompiler
// (https://github.com/Simple-XX/SimpleCompiler).
//
// symtab.h for Simple-XX/SimpleCompiler.

#ifndef _SYMTAB_H_
#define _SYMTAB_H_

#include "vector"
#include "string"
#include "unordered_map"

typedef std::vector<std::string> varlist_t;
typedef std::vector<std::string> funlist_t;
typedef std::vector<Variable *>  vars_t;
typedef std::unordered_map<std::string, vars_t *, std::hash<std::string>>
    vartab_t;
typedef std::unordered_map<std::string, Variable *, std::hash<std::string>>
    strtab_t;
typedef std::unordered_map<std::string, Function *, std::hash<std::string>>
    funtab_t;

// 符号表
class SymTab {
private:
    // 变量顺序
    varlist_t varlist;
    // 函数顺序
    funlist_t funlist;
    // 变量表
    vartab_t vartab;
    // 字符串表
    strtab_t strtab;
    // 函数表
    funtab_t funtab;

    // 当前所在函数
    Function *curr_fun;
    // 当前变量/函数作用域
    scope_t curr_scope;

public:
    SymTab(void);
    ~SymTab(void);
    // 添加变量
    void add_var(Variable *_var);
    // 添加字符串
    void add_str(Variable *_var);
    // 获取变量
    Variable *get_var(name_t _name);

    // 声明函数
    void dec_fun(Function *_fun);
    // 定义函数
    void def_fun(Function *_fun);
    // 获取函数
    Function *get_fun(name_t _name, paralist_t &_paralist);
};

#endif /* _SYMTAB_H_ */
