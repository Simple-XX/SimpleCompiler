
// This file is a part of MRNIU/SimpleCompiler (https://github.com/MRNIU/SimpleCompiler).
//
// parser.h for MRNIU/SimpleCompiler.

#ifndef _PARSER_H_
#define _PARSER_H_

#include "string"
#include "iostream"
#include "token.h"
#include "lexical.h"
#include "error.h"

using namespace std;

extern Error * error;

// 语法分析
class Parser {
private:
	// 词法分析器
	Lexer &lexer;
	// 超前查看的 token
	Token * token;
	// 获取下一个 token
	void next(void);
	// 匹配指定 Token
	bool match_token(Tag tag);

	// 程序
	void program(void);
	// 代码段
	void segment(void);

	//

public:
	Parser(Lexer &lex);
	~Parser(void);
	// 进行解析
	void parsing(void);
	bool is_done(void) const;


};

#endif  /* _PARSER_H_ */
