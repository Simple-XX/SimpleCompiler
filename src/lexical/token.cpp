
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

token_base_t::token_base_t(Tag _t) : tag(_t) {}

auto token_base_t::to_string() const -> const std::string {
  return tokenName[tag];
}

Id::Id(std::string _name) : token_base_t(ID), name(_name) {}

const std::string Id::to_string() const {
  return token_base_t::to_string() + "(" + name + ")";
}

token_num_t::token_num_t(int _num_val) : token_base_t(NUM), num_val(_num_val) {}

const std::string token_num_t::to_string() const {
  return token_base_t::to_string() + "(" + std::to_string(num_val) + ")";
}

token_char_t::token_char_t(char _char_val)
    : token_base_t(CHAR), char_val(_char_val) {}

const std::string token_char_t::to_string() const {
  return token_base_t::to_string() + "(" + std::to_string(char_val) + ")";
}

token_string_t::token_string_t(const std::string &_string_val)
    : token_base_t(STR), string_val(_string_val) {}

const std::string token_string_t::to_string() const {
  return token_base_t::to_string() + "(" + string_val + ")";
}

keywords_t::keywords_t() {
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

Tag keywords_t::get_tag(std::string _name) {
  return keywords.find(_name) != keywords.end() ? keywords[_name] : ID;
}
