
// This file is a part of MRNIU/SimpleCompiler (https://github.com/MRNIU/SimpleCompiler).
//
// symbol.cpp for MRNIU/SimpleCompiler.

#include "symbol.h"

// void
Variable::Variable(void) {
	extern_flag = false;
	const_flag = false;
	scope = Scope_Global;
	ptr_flag = true;
	type = KW_VOID;
	name = "<void>";
	data = nullptr;
	lv_flag = false;
	literal_flag = false;
	return;
}

// 匿名变量
Variable::Variable(Token * _token) {
	extern_flag = false;
	const_flag = false;
	scope = Scope_Global;
	ptr_flag = true;
	type = KW_VOID;
	name = "<void>";
	data = nullptr;
	lv_flag = false;
	literal_flag = true;
	switch(_token->tag) {
	    case NUM:
		    type = KW_INT;
		    name = "<int>";
		    data = (void *)( (Num *)_token)->val;
		    break;
	    case CHAR:
		    type = KW_CHAR;
		    name = "<char>";
		    data = (void *)( (Char *)_token)->ch;
		    break;
	    case STR:
		    type = KW_CHAR;
		    name = "<>";
		    data =
		        ptr_flag = true;

		    break;
	    default:
		    break;
	}
	return;
}
// 作用域，类型，名称，数据
Variable::Variable(scope_t _scope, type_t _type, name_t _name, data_t _data) {
	extern_flag = false;
	const_flag = false;
	scope = _scope;
	ptr_flag = false;
	type = _type;
	name = _name;
	data = _data;
	lv_flag = true;
	literal_flag = false;
	return;
}

// 作用域，类型，名称，数据，指针标识
Variable::Variable(scope_t _scope, type_t _type, name_t _name, data_t _data, bool _ptr_flag) {
	extern_flag = false;
	const_flag = false;
	scope = _scope;
	ptr_flag = _ptr_flag;
	type = _type;
	name = _name;
	data = _data;
	lv_flag = true;
	literal_flag = false;
	return;
}

// 作用域，类型，名称，数据，指针标识，外部标识，常量标识
Variable::Variable(scope_t _scope, type_t _type, name_t _name, data_t _data, bool _ptr_flag, bool _extern_flag, bool _const_flag) {
	extern_flag = _extern_flag;
	const_flag = _const_flag;
	scope = _scope;
	ptr_flag = _ptr_flag;
	type = _type;
	name = _name;
	data = _data;
	lv_flag = true;
	literal_flag = false;
	return;
}

Variable::~Variable(void) {
	return;
}

// 是否为外部
bool Variable::is_extern(void) {
	return extern_flag;
}

// 是否为常量
bool Variable::is_const(void) {
	return const_flag;
}

// 是否为指针
bool Variable::is_ptr(void) {
	return ptr_flag;
}

// 是否能作为左值
bool Variable::is_lv(void) {
	return lv_flag;
}

// 是否为字面值
bool Variable::is_literal(void) {
	return literal_flag;
}

// 获取类型
type_t Variable::get_type(void) {
	return type;
}

// 获取数据
void * Variable::get_data(void) {
	return data;
}

// 获取变量名
name_t Variable::get_name(void) {
	return name;
}

// 获取作用域
scope_t Variable::get_scope(void) {
	return scope;
}

// 设置外部标识
void Variable::set_extern(bool _is_extern) {
	extern_flag = _is_extern;
	return;
}

// 设置指针标识
void Variable::set_ptr(bool _is_ptr) {
	ptr_flag = _is_ptr;
	return;
}

// 设置常量标识
void Variable::set_const(bool _is_const) {
	const_flag = _is_const;
	return;
}

// 设置是否能作为左值
void Variable::set_lv(bool _is_lv) {
	lv_flag = _is_lv;
	return;
}

// 设置是否为字面值
void Variable::set_literal(bool _is_literal) {
	literal_flag = _is_literal;
	return;
}

// 设置类型
void Variable::set_type(type_t _new_type) {
	type = _new_type;
	return;
}

// 设置数据
void Variable::set_data(data_t _new_data) {
	data = _new_data;
	return;
}

// 设置变量名
void Variable::set_name(name_t _new_name) {
	name = _new_name;
	return;
}

// 外部标识，返回类型，函数名，参数列表
Function::Function(bool _extern_flag, type_t _return_type, name_t _fun_name, paralist_t &_paralist) {
	extern_flag = _extern_flag;
	return_type = _return_type;
	fun_name = _fun_name;
	paralist = _paralist;
	return;
}

Function::~Function(void) {
	return;
}
