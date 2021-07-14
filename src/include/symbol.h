
// This file is a part of Simple-XX/SimpleCompiler
// (https://github.com/Simple-XX/SimpleCompiler).
//
// symbol.h for Simple-XX/SimpleCompiler.

#ifndef _SYMBOL_H_
#define _SYMBOL_H_

#include "string"
#include "vector"
#include "common.h"
#include "token.h"

// 作用域
#define Scope_Global 0

typedef std::vector<int> scope_t;
typedef Tag              type_t;
typedef void *           data_t;
typedef std::string      name_t;

// 符号定义
// 变量
class Variable {
private:
    // 是否为外部变量
    bool extern_flag;
    // 是否为常量
    bool const_flag;
    // 是否能作为左值
    bool lv_flag;
    // 是否为字面量
    bool literal_flag;
    // 是否初始化
    bool init_flag;
    // 初始化数据
    Variable *init_data;
    // 是否为数组
    bool array_flag;
    // 数组长度
    size_t array_size;
    // 是否为指针
    bool ptr_flag;
    // 作用域
    scope_t scope;
    // 变量类型
    type_t type;
    // 变量名
    name_t name;
    // 数据
    union {
        char        char_data;
        int         int_data;
        void *      ptr_data;
        std::string string_data;
    };
    // 此变量大小
    size_t size;
    // 是否活跃
    bool live_flag;
    // 默认 flag 设置
    void set_default(void);

public:
    // void
    Variable(void);
    // 匿名变量
    Variable(Token *_token);
    // 数组
    Variable(scope_t &_scope, bool _extern_flag, type_t _type, name_t _name,
             size_t _array_size);
    // 整数
    Variable(int _int_data);
    // 字符
    Variable(char _char_data);
    // 一般变量
    Variable(scope_t &_scope, bool _extern_flag, type_t _type, bool _ptr_flag,
             name_t _name, Variable *_init_data);
    // 拷贝构造
    Variable(scope_t &_scope, Variable *v);
    ~Variable(void);

    // 是否为外部
    bool get_extern_flag(void);
    // 设置外部标识
    void set_extern_flag(bool _is_extern);
    // 是否为常量
    bool get_const_flag(void);
    // 设置常量标识
    void set_const_flag(bool _is_const);
    // 是否为指针
    bool get_ptr_flag(void);
    // 设置指针标识
    void set_ptr_flag(bool _is_ptr);
    // 是否能作为左值
    bool get_lv_flag(void);
    // 设置是否能作为左值
    void set_lv_flag(bool _is_lv);
    // 是否为字面值
    bool get_literal_flag(void);
    // 设置是否为字面值
    void set_literal_flag(bool _is_literal);
    // 获取类型
    type_t get_type(void);
    // 设置类型
    void set_type(type_t _new_type);
    // 获取数据
    data_t get_data(void);
    // 设置数据
    void set_data(data_t _new_data);
    // 获取变量名
    name_t get_name(void);
    // 修改变量名涉及到其它组件，先不考虑
    // 获取作用域
    scope_t get_scope(void);
    // 设置作用域
    void set_scope(scope_t _new_scope);
    // 获取变量信息
    void to_string(std::string &_str);
};

typedef std::vector<Variable *> paralist_t;

// 函数
class Function {
private:
    // 标识符
    bool extern_flag;
    // 返回值类型
    type_t type;
    // 是否为指针
    bool ptr_flag;
    // 函数名
    name_t name;
    // 参数表
    paralist_t paralist;
    // 栈大小
    int stack_size;
    // 当前 sp
    int esp;
    // 中间代码
    // 优化后的中间代码
    // 返回地址

public:
    // 外部标识，返回类型，函数名，参数列表
    Function(bool _extern_flag, type_t _return_type, name_t _fun_name,
             paralist_t &_paralist);
    ~Function(void);

    void define(Function *_fun);
    bool match(Function *_fun);
    bool match(paralist_t &_paralist);

    // 是否为外部
    bool get_extern_flag(void);
    // 设置外部标识
    void set_extern_flag(bool _is_extern);
    // 是否为指针
    bool get_ptr_flag(void);
    // 设置指针标识
    void set_ptr_flag(bool _is_ptr);
    // 获取类型
    type_t get_type(void);
    // 设置类型
    void set_type(type_t _new_type);
    // 获取变量名
    name_t get_name(void);
    // 修改变量名涉及到其它组件，先不考虑
    // 获取变量信息
    void to_string(std::string &_str);
};

#endif /* _SYMBOL_H_ */
