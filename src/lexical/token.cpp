
/**
 * @file token.cpp
 * @brief token
 * @author Zone.N (Zone.Niuzh@hotmail.com)
 * @version 1.0
 * @date 2023-10-26
 * @copyright MIT LICENSE
 * https://github.com/Simple-XX/SimpleCompiler
 * @par change log:
 * <table>
 * <tr><th>Date<th>Author<th>Description
 * <tr><td>2023-10-26<td>Zone.N<td>迁移到 doxygen
 * </table>
 */

#include "token.h"

static const std::string tokenName[] = {
    "INT",    "CHAR",   "VOID",     "CONST",    "IF",    "ELSE",   "WHILE",
    "FOR",    "BREAK",  "CONTINUE", "RETURN",   "ID",    "NUM",    "CH",
    "STR",    "ASSIGN", "ADD",      "SUB",      "MUL",   "DIV",    "MOD",
    "ORBIT",  "ANDBIT", "EORBIT",   "AND",      "OR",    "NOT",    "GT",
    "GE",     "LT",     "LE",       "EQU",      "NEQU",  "LPAREN", "RPAREN",
    "LBRACE", "RBRACE", "LBRACKET", "RBRACKET", "COMMA", "COLON",  "SEMICON",
};

Token::Token(Tag _t) : tag(_t) { ; }

auto Token::to_string() -> const std::string { return tokenName[tag]; }

Id::Id(std::string _n) : Token(ID), name(_n) {}

const std::string Id::to_string() {
  return Token::to_string() + "(" + name + ")";
}

Num::Num(int _v) : Token(NUM), val(_v) {}

const std::string Num::to_string() {
  return Token::to_string() + "(" + std::to_string(val) + ")";
}

Char::Char(char _c) : Token(CHAR), ch(_c) {}

const std::string Char::to_string() {
  return Token::to_string() + "(" + std::to_string(ch) + ")";
}

Str::Str(const std::string &_string) : Token(STR), str(_string) {}

const std::string Str::to_string() {
  return Token::to_string() + "(" + str + ")";
}

Keywords::Keywords() {
  keywords["int"] = KW_INT;
  keywords["char"] = KW_CHAR;
  keywords["void"] = KW_VOID;
  keywords["const"] = KW_CONST;
  keywords["if"] = KW_IF;
  keywords["else"] = KW_ELSE;
  keywords["while"] = KW_WHILE;
  keywords["for"] = KW_FOR;
  keywords["break"] = KW_BREAK;
  keywords["continue"] = KW_CONTINUE;
  keywords["return"] = KW_RETURN;
}

Tag Keywords::get_tag(std::string _name) {
  return keywords.find(_name) != keywords.end() ? keywords[_name] : ID;
}
