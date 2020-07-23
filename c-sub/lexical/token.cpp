
// This file is a part of MRNIU/SimpleCompiler (https://github.com/MRNIU/SimpleCompiler).
//
// token.cpp for MRNIU/SimpleCompiler.

#include "token.h"

// 初始化
Keywords::Keywords() {
	keywords["int"] = KW_INT;
	keywords["char"] = KW_CHAR;
	keywords["void"] = KW_VOID;
	keywords["if"] = KW_IF;
	keywords["else"] = KW_ELSE;
	keywords["while"] = KW_WHILE;
	keywords["for"] = KW_FOR;
	keywords["break"] = KW_BREAK;
	keywords["continue"] = KW_CONTINUE;
	keywords["return"] = KW_RETURN;
}

Tag Keywords::getTag(std::string name) {
	return keywords.find(name) != keywords.end() ? keywords[name] : ID;
}

const char * tokenName[] = {
	"INT", "CHAR", "VOID",
	"IF", "ELSE",
	"WHILE", "FOR",
	"BREAK", "CONTINUE", "RETURN",
	"ID", "NUM", "CH", "STR",
	"ASSIGN",
	"ADD", "SUB", "MUL", "DIV", "MOD",
	"ORBIT", "ANDBIT", "EORBIT",
	"AND", "OR", "NOT",
	"GT", "GE", "LT", "LE",
	"EQU", "NEQU",
	"LPAREN", "RPAREN",
	"LBRACE", "RBRACE",
	"COMMA", "COLON", "SEMICON",
};

Token::Token(Tag t) : tag(t) {
	return;
}

std::string Token::toString() {
	return tokenName[tag];
}

Token::~Token() {
	return;
}

Id::Id(std::string n) : Token(ID), name(n) {
	return;
}

std::string Id::toString() {
	return Token::toString() + "(" + name + ")";
}

Num::Num(int v) : Token(NUM), val(v) {
	return;
}

std::string Num::toString() {
	return Token::toString() + "(" + std::to_string(val) + ")";
}

Char::Char(char c) : Token(CHAR), ch(c) {
	return;
}

std::string Char::toString() {
	return Token::toString() + "(" + std::to_string(ch) + ")";
}

Str::Str(std::string s) : Token(STR), str(s) {
	return;
}

std::string Str::toString() {
	return Token::toString() + "("  + str + ")";
}
