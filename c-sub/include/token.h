
// This file is a part of MRNIU/SimpleCompiler (https://github.com/MRNIU/SimpleCompiler).
//
// token.h for MRNIU/SimpleCompiler.

#ifndef _TOKEN_H_
#define _TOKEN_H_

#include "string"
#include "unordered_map"

using namespace std;

enum Tag {
	ERR = -2,  // 错误
	END = EOF,  // 结束标识
	// 关键字 keyword
	KW_INT = 0,  // int
	KW_CHAR,  // char
	KW_VOID,  // void
	KW_IF,  // if
	KW_ELSE,  // else
	KW_WHILE,  // while
	KW_FOR,  // for
	KW_BREAK,  // break
	KW_CONTINUE,  // continue
	KW_RETURN,  // return
	// 类型标识
	ID,  // 标识符
	NUM,  // int
	CHAR,  // char
	STR,  // str
	// 操作符 operator
	ASSIGN,  // =
	ADD,  // +
	SUB,  // -
	MUL,  // *
	DIV,  // /
	MOD,  // %
	ORBIT,  // |
	ANDBIT,  // &
	EORBIT,  // ^
	AND,  // &&
	OR,  // ||
	NOT,  // !
	GT,  // >
	GE,  // >=
	LT,  // <
	LE,  // <=
	EQU,  // ==
	NEQU,  // !=
	// 分界符 separator
	LPAREN,  // (
	RPAREN,  // )
	LBRACE,  // {
	RBRACE,  // }
	COMMA,  // ,
	COLON,  // :
	SEMICON,  // ;
	// 字面值 literal
	// 1, 2, 3, a, b, c, true, flase, test, 233, 666, ...
};

// 基类
class Token {
public:
	Tag tag;
	Token(Tag t);
	virtual string toString();
	virtual ~Token();
};

// 标识符
class Id : public Token {
public:
	string name;
	Id(string n);
	virtual string toString();
};

// 数字
class Num : public Token {
public:
	int val;
	Num(int v);
	virtual string toString();
};

// 字符
class Char : public Token {
public:
	char ch;
	Char(char c);
	virtual string toString();
};

// 字符串
class Str : public Token {
public:
	string str;
	Str(string s);
	virtual string toString();
};

// 关键字
class Keywords {
private:
	unordered_map<string, Tag, hash<string> > keywords;

public:
	Keywords();
	Tag getTag(string name);
};

#endif /* _TOKEN_H_ */
