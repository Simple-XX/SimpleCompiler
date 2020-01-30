
// This file is a part of MRNIU/SimpleCompiler (https://github.com/MRNIU/SimpleCompiler).
//
// lexical.h for MRNIU/SimpleCompiler.

#ifndef _LEXICAL_H_
#define _LEXICAL_H_

#include "token.h"
#include "scanner.h"

using namespace std;

#define TAG_KW \
	(KW_INT || KW_CHAR || KW_VOID || KW_IF || KW_ELSE || KW_WHILE || KW_FOR \
	|| KW_BREAK || KW_CONTINUE || KW_RETURN)

#define TAG_TYPE \
	(ID || NUM || CH)

#define TAG_OP \
	(ASSIGN || ADD || SUB || MUL || DIV || MOD || ORBIT || ANDBIT || EORBIT || \
	AND || OR || NOT || GT || GE || LT || LE || EQU || NEQU)

#define TAG_SEP \
	(LPAREN || RPAREN || LBRACE || RBRACE || COMMA || COLON || SEMICON)


// 判断 token 是否为 t 类型
#define IS_TAG(token, t)  (token->tag == t)
// 将 token 转换为实际的类型
#define TOKEN_CAST(token) \
	if(IS_TAG(token, ID) ) { \
		token = (Id *)token; \
	} \
	else if(IS_TAG(token, NUM) ) { \
		token = (Num *)token; \
	} \
	else if(IS_TAG(token, CH) ) { \
		token = (Char *)token; \
	} \
	else if(IS_TAG(token, STR) ) { \
		token = (Str *)token; \
	}

class Lexer {
private:
	// 扫描器对象
	Scanner &scanner;
	// 关键字
	static Keywords keywords;
	// 当前字符
	char ch;
	// 扫描
	bool scan(void);
	Token * token;

public:
	Lexer(Scanner &sc);
	~Lexer(void);
	Token * lexing(void);
	bool is_done(void);
};

#endif /* _LEXICAL_H_ */
