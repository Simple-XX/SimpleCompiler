
// This file is a part of MRNIU/SimpleCompiler (https://github.com/MRNIU/SimpleCompiler).
//
// lexical.h for MRNIU/SimpleCompiler.

#ifndef _LEXICAL_H_
#define _LEXICAL_H_

#include "string"
#include <ext/hash_map>

using namespace std;
using namespace __gnu_cxx;

enum Tag {
	// 关键字
	KW_INT,  // int
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
	ERR,  // 错误
	END,  // 结束标识
	ID,  // 变量名
	NUM,  // int
	CH,  // char
	// 符号
	ADD,  // +
	SUB,  // -
	MUL,  // *
	DIV,  // /
	MOD,  // %
	GT,  // >
	GE,  // >=
	LT,  // <
	LE,  // <=
	EQU,  // ==
	NEQU,  // !=
	AND,  // &&
	OR,  // ||
	NOT,  // !
	LPAREN,  // (
	RPAREN,  // )
	LBRACE,  // {
	RBRACE,  // }
	COMMA,  // ,
	COLON,  // :
	SEMICON,  // ;
	ASSIGN,  // =
};

class Token {
public:
	Tag tag;
	Token(Tag t);
	virtual string toString();
	virtual ~Token();
};

class Id : public Token {
public:
	string name;
	Id(string n);
	virtual string toString();
};

class Num : public Token {
public:
	int val;
	Num(int v);
	virtual string toString();
};

class Char : public Token {
public:
	char ch;
	Char(char c);
	virtual string toString();
};

class Str : public Token {
public:
	string str;
	Str(string s);
	virtual string toString();
};

class Keywords {
private:
	struct string_hash {
size_t operator() (const string & str) const {
	return __stl_hash_string(str.c_str() );
};
	};

	hash_map<string, Tag, string_hash> keywords;

public:
	Keywords();
	Tag getTag(string name);
};

#endif /* _LEXICAL_H_ */
