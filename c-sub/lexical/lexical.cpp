
// This file is a part of MRNIU/SimpleCompiler (https://github.com/MRNIU/SimpleCompiler).
//
// lexical.cpp for MRNIU/SimpleCompiler.

#include "lexical.h"
#include "iostream"

using namespace std;

Keywords Lexer::keywords;

Lexer::Lexer(Scanner &sc) : scanner(sc) {
	ch = ' ';
	token = NULL;
	return;
}

Lexer::~Lexer() {
	if(token != NULL) {
		delete token;
	}
	return;
}

bool Lexer::scan() {
	ch = scanner.scan();
	return scanner.is_done();
}

Token * Lexer::lexing() {
	// 字符不为空时
	while(is_done() == false) {
		Token * t = NULL;
		// 跳过空字符
		while(ch == ' ' || ch == '\n' || ch == '\t') {
			scan();
		}
		// 标识符
		while(
		    (ch >= 'a' && ch <= 'z') ||
		    (ch >= 'A' && ch <= 'Z') ||
		    (ch == '_') ) {
			string name = "";
			do {
				// 记录字符
				name.push_back(ch);
				scan();
			} while(
			    (ch >= 'a' && ch <= 'z') ||
			    (ch >= 'A' && ch <= 'Z') ||
			    (ch == '_') ||
			    (ch >= '0' && ch <= '9') );
			// 判断是不是关键字
			Tag tag = keywords.getTag(name);
			// 如果不是，则为标识符
			if(tag == ID) {
				t = new Id(name);
			}
			// 如果是
			else {
				t = new Token(tag);
			}
		}
		// 更新 token 内容
		token = t;
		if(token && token->tag != ERR) {
			return token;
		}
	}
	token = new Token(END);
	return token;
}

bool Lexer::is_done() {
	return scanner.is_done();
}
