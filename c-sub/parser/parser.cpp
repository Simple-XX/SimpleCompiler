
// This file is a part of MRNIU/SimpleCompiler (https://github.com/MRNIU/SimpleCompiler).
//
// parser.cpp for MRNIU/SimpleCompiler.

#include "parser.h"

Parser::Parser(Lexer &lex) : lexer(lex)  {
	return;
}

Parser::~Parser() {
	return;
}

// 获取下一个 token
void Parser::next(void) {
	token = lexer.lexing();
	return;
}
// 匹配指定 Token
bool Parser::match_token(Tag tag) {
	if(token->tag == tag) {
		// 如果匹配到了就读入下一个
		next();
		return true;
	}
	else {
		return false;
	}
}

// 进行解析，返回解析结果(AST)
void Parser::parsing(void) {

}

bool Parser::is_done(void) const {
	return lexer.is_done();
}
