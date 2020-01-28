
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
	KW_INT,  // int
	KW_CHAR,  // char
	KW_VOID,  // void
	KW_EXTERN,  // extern
	KW_IF,  // if
	KW_ELSE,  // else
	KW_SWITCH,  // switch
	KW_CASE,  // case
	KW_DEFAULT,  // default
	KW_WHILE,  // while
	KW_DO,  // do
	KW_FOR,  // for
	KW_BREAK,  // break
	KW_CONTINUE,  // continue
	KW_RETURN,  // return
	ERR,
	END,
	ID,
	NUM,
	CH,
	STR,
	NOT,  // !
	LEA,
	ADD,  // +
	SUB,  // -
	MUL,  // *
	DIV,  // /
	MOD,  // %
	INC,  // ++
	DEC,  // --
	GT,
	GE,
	LT,
	LE,
	EQU,  // ==
	NEQU,  // !=
	AND,  // &&
	OR,  // ||
	LPAREN,  // (
	RPAREN,  // )
	LBRACK,  // [
	RBRACK,  // ]
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

struct string_hash {
size_t operator()(const string & str) const {
	return __stl_hash_string(str.c_str() );
};
};

class Keywords {
private:
hash_map<string, Tag, string_hash> keywords;
public:
Keywords();
Tag getTag(string name);
};

#endif /* _LEXICAL_H_ */
