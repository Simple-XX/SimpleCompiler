
// This file is a part of Simple-XX/SimpleCompiler
// (https://github.com/Simple-XX/SimpleCompiler).
//
// symbol.cpp for Simple-XX/SimpleCompiler.

#include "symbol.h"

// 默认 flag 设置
void Variable::set_default(void) {
    extern_flag  = false;
    const_flag   = false;
    lv_flag      = true;
    literal_flag = false;
    init_flag    = false;
    init_data    = nullptr;
    array_flag   = false;
    array_size   = 0;
    ptr_flag     = false;
    scope.push_back(Scope_Global);
    type        = KW_VOID;
    name        = "default";
    string_data = "";
    ptr_data    = nullptr;
    int_data    = 0;
    char_data   = '\0';
    size        = 0;
    live_flag   = true;
    return;
}

// void
Variable::Variable(void) {
    set_default();
    lv_flag      = false;
    literal_flag = false;
    ptr_flag     = true;
    type         = KW_VOID;
    name         = "<void>";
    return;
}

// 匿名变量
Variable::Variable(Token *_token) {
    set_default();
    lv_flag      = false;
    literal_flag = true;
    switch (_token->tag) {
        case NUM:
            type     = KW_INT;
            name     = "<int>";
            int_data = ((Num *)_token)->val;
            break;
        case CHAR:
            type      = KW_CHAR;
            name      = "<char>";
            char_data = ((Char *)_token)->ch;
            break;
        case STR:
            type = KW_CHAR;
            // 需要由 symtab 生成一个名字以供索引
            name        = "";
            string_data = ((Str *)_token)->str;
            ptr_flag    = true;
            array_flag  = true;
            array_size  = string_data.size() + 1;
            break;
        default:
            break;
    }
    return;
}

// 数组
Variable::Variable(scope_t &_scope, bool _extern_flag, type_t _type,
                   name_t _name, size_t _array_size) {
    set_default();
    scope       = _scope;
    extern_flag = _extern_flag;
    type        = _type;
    name        = _name;
    array_flag  = true;
    array_size  = _array_size;
    return;
}

// 整数
Variable::Variable(int _int_data) {
    set_default();
    name         = "<int>";
    literal_flag = true;
    lv_flag      = false;
    type         = KW_INT;
    int_data     = _int_data;
    return;
}

// 字符
Variable::Variable(char _char_data) {
    set_default();
    name         = "<char>";
    literal_flag = true;
    lv_flag      = false;
    type         = KW_CHAR;
    int_data     = _char_data;
    return;
}

// 一般变量
Variable::Variable(scope_t &_scope, bool _extern_flag, type_t _type,
                   bool _ptr_flag, name_t _name, Variable *_init_data) {
    set_default();
    scope       = _scope;
    extern_flag = _extern_flag;
    type        = _type;
    ptr_flag    = _ptr_flag;
    name        = _name;
    init_data   = _init_data;
    return;
}

Variable::~Variable(void) {
    return;
}

// 是否为外部
bool Variable::get_extern_flag(void) {
    return extern_flag;
}

// 设置外部标识
void Variable::set_extern_flag(bool _extern_flag) {
    extern_flag = _extern_flag;
    return;
}

// 是否为常量
bool Variable::get_const_flag(void) {
    return const_flag;
}

// 设置常量标识
void Variable::set_const_flag(bool _const_flag) {
    const_flag = _const_flag;
    return;
}

// 是否为指针
bool Variable::get_ptr_flag(void) {
    return ptr_flag;
}

// 设置指针标识
void Variable::set_ptr_flag(bool _ptr_flag) {
    ptr_flag = _ptr_flag;
    return;
}

// 是否能作为左值
bool Variable::get_lv_flag(void) {
    return lv_flag;
}

// 设置是否能作为左值
void Variable::set_lv_flag(bool _lv_flag) {
    lv_flag = _lv_flag;
    return;
}

// 是否为字面值
bool Variable::get_literal_flag(void) {
    return literal_flag;
}

// 设置是否为字面值
void Variable::set_literal_flag(bool _literal_flag) {
    literal_flag = _literal_flag;
    return;
}

// 获取类型
type_t Variable::get_type(void) {
    return type;
}

// 设置类型
void Variable::set_type(type_t _new_type) {
    type = _new_type;
    return;
}

// 获取变量名
name_t Variable::get_name(void) {
    return name;
}

// 获取作用域
scope_t Variable::get_scope(void) {
    return scope;
}

// 设置作用域
void Variable::set_scope(scope_t _new_scope) {
    scope = _new_scope;
    return;
}

// 输出变量信息
void Variable::to_string(std::string &_str) {
    if (extern_flag) {
        _str += "extern ";
    }
    _str += tokenName[type];
    if (ptr_flag) {
        _str += " * ";
    }
    _str += name;
    if (array_flag) {
        _str += " [";
        _str += std::to_string(array_size);
        _str += "] ";
    }
    if (init_flag) {
        _str += "= ";
        switch (type) {
            case KW_INT:
                _str += std::to_string(int_data);
                break;
            case KW_CHAR:
                if (ptr_flag) {
                    _str += string_data;
                }
                else {
                    _str += std::to_string(char_data);
                }
                break;
            default:
                break;
        }
    }
    _str += ";";
    _str += " size = ";
    _str += size;
    _str += " scope = ";
    for (auto i : scope) {
        _str += i;
        _str += "/";
    }

    return;
}

// 外部标识，返回类型，函数名，参数列表
Function::Function(bool _extern_flag, type_t _type, name_t _name,
                   paralist_t &_paralist) {
    extern_flag = _extern_flag;
    type        = _type;
    name        = _name;
    paralist    = _paralist;
    return;
}

Function::~Function(void) {
    return;
}

void Function::define(Function *_fun) {
    extern_flag = false;
    paralist    = _fun->paralist;
    return;
}

bool Function::match(Function *_fun) {
    // 名称匹配
    if (name != _fun->name) {
        return false;
    }
    // 参数个数匹配
    if (paralist.size() != _fun->paralist.size()) {
        return false;
    }
    // 参数类型匹配
    size_t len = paralist.size();
    for (size_t i = 0; i < len; i++) {
        if (0) {
            return false;
        }
    }
    // 返回类型匹配
    if (type != _fun->type) {
        error->set_err_no(ERR);
        error->display_err();
    }
    return true;
}

bool Function::match(paralist_t &_paralist) {
    // 检查参数个数
    if (paralist.size() != _paralist.size()) {
        return false;
    }
    // 检查参数类型
    size_t len = paralist.size();
    for (size_t i = 0; i < len; i++) {
        if (0) {
            return false;
        }
    }
    return true;
}

// 是否为外部
bool Function::get_extern_flag(void) {
    return extern_flag;
}

// 设置外部标识
void Function::set_extern_flag(bool _extern_flag) {
    extern_flag = _extern_flag;
    return;
}

// 是否为指针
bool Function::get_ptr_flag(void) {
    return ptr_flag;
}

// 设置指针标识
void Function::set_ptr_flag(bool _ptr_flag) {
    ptr_flag = _ptr_flag;
    return;
}

// 获取类型
type_t Function::get_type(void) {
    return type;
}

// 获取变量名
name_t Function::get_name(void) {
    return name;
}

void Function::to_string(std::string &_str) {
    if (extern_flag) {
        _str += "extern ";
    }
    _str += tokenName[type];
    _str += name;
    _str += "(";
    for (size_t i = 0; i < paralist.size(); i++) {
        _str += paralist[i]->get_name();
        if (i != paralist.size() - 1) {
            _str += ", ";
        }
    }
    _str += ")";
    _str += ";";
    return;
}
