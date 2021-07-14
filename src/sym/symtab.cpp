
// This file is a part of Simple-XX/SimpleCompiler
// (https://github.com/Simple-XX/SimpleCompiler).
//
// symtab.cpp for Simple-XX/SimpleCompiler.

#include "symbol.h"
#include "symtab.h"

SymTab::SymTab(void) {
    curr_fun = nullptr;
    curr_scope.push_back(Scope_Global);
    return;
}

SymTab::~SymTab(void) {
    return;
}

// 添加变量
void SymTab::add_var(Variable *_var) {
    // 如果当前变量表中没有同名变量
    if (vartab.find(_var->get_name()) == vartab.end()) {
        // 添加链表
        vartab[_var->get_name()] = new vars_t;
        // 添加链表元素
        vartab[_var->get_name()]->push_back(_var);
        // 添加顺序
        varlist.push_back(_var->get_name());
    }
    // 如果有
    else {
        // 先判断同作用域下有无冲突
        vars_t &list = *vartab[_var->get_name()];
        size_t  i    = 0;
        for (; i < list.size(); i++) {
            // 有冲突
            if (list[i]->get_scope().back() == _var->get_scope().back()) {
                break;
            }
        }
        // 查完了或者 _var 为匿名对象，则没有冲突
        if (i == list.size() || _var->get_name()[0] == '<') {
            list.push_back(_var);
        }
        // 出现冲突
        else {
            error->set_err_no(ERR);
            error->display_err();
        }
    }
    return;
}

// 添加字符串
void SymTab::add_str(Variable *_var) {
    strtab[_var->get_name()] = _var;
    return;
}

// 获取变量
Variable *SymTab::get_var(name_t _name) {
    Variable *var = nullptr;
    // 首先寻找有没有
    if (vartab.find(_name) != vartab.end()) {
        vars_t &list        = *vartab[_name];
        size_t  scope_depth = curr_scope.size();
        size_t  maxlen      = 0;
        for (size_t i = 0; i < list.size(); i++) {
            size_t len = list[i]->get_scope().size();
            if ((len <= scope_depth) &&
                (list[i]->get_scope()[len - 1] == curr_scope[len - 1])) {
                if (len > maxlen) {
                    maxlen = len;
                    var    = list[i];
                }
            }
        }
    }
    else {
        error->set_err_no(ERR);
        error->display_err();
    }
    return var;
}

// 声明函数
void SymTab::dec_fun(Function *_fun) {
    _fun->set_extern_flag(true);
    // 判断是否存在
    if (funtab.find(_fun->get_name()) == funtab.end()) {
        // 没有的话就加进去
        funtab[_fun->get_name()] = _fun;
        funlist.push_back(_fun->get_name());
    }
    else {
        error->set_err_no(ERR);
        error->display_err();
    }
    return;
}

// 定义函数
void SymTab::def_fun(Function *_fun) {
    // 定义不能有 extern
    if (_fun->get_extern_flag()) {
        error->set_err_no(ERR);
        error->display_err();
    }
    // 没有同名函数
    if (funtab.find(_fun->get_name()) == funtab.end()) {
        funtab[_fun->get_name()] = _fun;
        funlist.push_back(_fun->get_name());
    }
    // 出现过声明
    else {
        Function *fun = funtab[_fun->get_name()];
        if (fun->get_extern_flag()) {
            // 声明与定义不匹配
            if (fun->match(_fun) == false) {
                error->set_err_no(ERR);
                error->display_err();
            }
            fun->define(_fun);
        }
    }
    return;
}

// 获取函数
Function *SymTab::get_fun(name_t _name, paralist_t &_paralist) {
    Function *fun = nullptr;
    if (funtab.find(_name) != funtab.end()) {
        fun = funtab[_name];
        if (fun->match(_paralist) == false) {
            error->set_err_no(ERR);
            error->display_err();
            fun = nullptr;
        }
    }
    else {
        error->set_err_no(ERR);
        error->display_err();
    }
    return fun;
}
