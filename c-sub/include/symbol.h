
// This file is a part of MRNIU/SimpleCompiler (https://github.com/MRNIU/SimpleCompiler).
//
// symbol.h for MRNIU/SimpleCompiler.

#ifndef _SYMBOL_H_
#define _SYMBOL_H_

#include "string"
#include "vector"
#include "token.h"

// 作用域
#define Scope_Global 0

typedef int scope_t;
typedef Tag type_t;
typedef void * data_t;
typedef std::string name_t;

// 符号定义
// 变量
class Variable {
private:
	// 外部标识
	bool extern_flag;
	// 常量标识
	bool const_flag;
	// 作用域
	scope_t scope;
	// 指针标识
	bool ptr_flag;
	// 变量类型
	type_t type;
	// 变量名
	name_t name;
	// 数据
	data_t data;
	// 是否能作为左值
	bool lv_flag;
	// 是否为字面量
	bool literal_flag;
	// 是否为数组
	bool array_flag;
	//

public:
	// void
	Variable(void);
	// 匿名变量
	Variable(Token * _token);
	// 作用域，类型，名称，数据
	Variable(scope_t _scope, type_t _type, name_t _name, data_t _data);
	// 作用域，类型，名称，数据，指针标识
	Variable(scope_t _scope, type_t _type, name_t _name, data_t _data, bool _ptr_flag);
	// 作用域，类型，名称，数据，指针标识，外部标识，常量标识
	Variable(scope_t _scope, type_t _type, name_t _name, data_t _data, bool _ptr_flag, bool _extern_flag, bool _const_flag);
	~Variable(void);

	// 是否为外部
	bool is_extern(void);
	// 是否为常量
	bool is_const(void);
	// 是否为指针
	bool is_ptr(void);
	// 是否能作为左值
	bool is_lv(void);
	// 是否为字面值
	bool is_literal(void);
	// 获取类型
	type_t get_type(void);
	// 获取数据
	data_t get_data(void);
	// 获取变量名
	name_t get_name(void);
	// 获取作用域
	scope_t get_scope(void);
	// 设置外部标识
	void set_extern(bool _is_extern);
	// 设置指针标识
	void set_ptr(bool _is_ptr);
	// 设置常量标识
	void set_const(bool _is_const);
	// 设置是否能作为左值
	void set_lv(bool _is_lv);
	// 设置是否为字面值
	void set_literal(bool _is_literal);
	// 设置类型
	void set_type(type_t _new_type);
	// 设置数据
	void set_data(data_t _new_data);
	// 设置变量名
	void set_name(name_t _new_name);
};

typedef std::vector<Variable *> paralist_t;

// 函数
class Function {
private:
	// 标识符
	bool extern_flag;
	// 返回值类型
	type_t return_type;
	// 函数名
	name_t fun_name;
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
	Function(bool _extern_flag, type_t _return_type, name_t _fun_name, paralist_t &_paralist);
	~Function(void);
};

#endif /* _SYMBOL_H_ */
