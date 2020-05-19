
// This file is a part of MRNIU/SimpleCompiler (https://github.com/MRNIU/SimpleCompiler).
//
// symbol.h for MRNIU/SimpleCompiler.

#ifndef _SYMBOL_H_
#define _SYMBOL_H_

#include "token.h"

// 符号定义

// 变量
class Variable {
private:
	// 标识符
	bool extern_flag;
	bool const_flag;
	// 作用域
	int scope;
	// 变量类型
	Tag type;
	// 指针标识
	bool ptr_flag;
	// 变量名
	string name;
	// 数据
	void * data;

public:
	// void
	Variable(void);
	// int 数据
	Variable(int data);
	// 作用域，int 数据
	Variable(int sp, int data);
	// 作用域，前缀，int 数据
	Variable(int sp, int pre_flag, int data);
	// 作用域，前缀，变量名，int 数据
	Variable(int sp, int pre_flag, string data_name, int data);
	// 作用域，前缀，指针标识，变量名，int 数据
	Variable(int sp, int pre_flag, bool isptr, string data_name, int data);
	~Variable(void);

	// 获取前缀
	bool get_flag(void);
	// 获取类型
	Tag get_type(void);
	// 获取变量名
	string get_name(void);
	// 是否为指针
	bool if_ptr(void);
	// 获取作用域
	int get_scope(void);
	// 获取数据
	void * get_data(void);
};

// 函数
class Function {
private:
public:
};

#endif /* _SYMBOL_H_ */
