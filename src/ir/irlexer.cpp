
/**
 * @file irlexer.cpp
 * @brief ir lexer
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

#include <iostream>

#include "irlexer.h"
#include "irtoken.h"

IRToken IRLexer::ParseNum() {
  char c = cinstream.peek();
  int val = 0;
  if (c == '0') {
    c = cinstream.get();
    c = cinstream.peek();
    if (c == 'x' || c == 'X') {
      c = cinstream.get();
      while (true) {
        c = cinstream.peek();
        if (c >= '0' && c <= '9') {
          c = cinstream.get();
          val = val * 16 + c - '0';
        } else if (c >= 'A' && c <= 'F') {
          c = cinstream.get();
          val = val * 16 + c - 'A' + 10;
        } else if (c >= 'a' && c <= 'f') {
          c = cinstream.get();
          val = val * 16 + c - 'a' + 10;
        } else {
          value = val;
          return IRToken::NUMBER_IR;
        }
      }
    } else {
      while (true) {
        if (c >= '0' && c <= '7') {
          c = cinstream.get();
          val = val * 8 + c - '0';
        } else {
          value = val;
          return IRToken::NUMBER_IR;
        }
        c = cinstream.peek();
      }
    }
  } else {
    while (c >= '0' && c <= '9') {
      c = cinstream.get();
      val = val * 10 + (int)(c - '0');
      c = cinstream.peek();
    }
    value = val;
    return IRToken::NUMBER_IR;
  }
}

IRToken IRLexer::ParseSymbol() {
  std::string s;
  char c;
  while (true) {
    c = cinstream.get();
    s += c;
    c = cinstream.peek();
    if (!((c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || (c == '_') ||
          (c >= '0' && c <= '9'))) {
      if (s == "var") {
        return IRToken::VARDECL_IR;
      }
      if (s == "end") {
        return IRToken::FUNCEND_IR;
      }
      if (s == "if") {
        return IRToken::IF_IR;
      }
      if (s == "goto") {
        return IRToken::GOTO_IR;
      }
      if (s == "return") {
        return IRToken::RETURN_IR;
      }
      if (s == "call") {
        return IRToken::CALL_IR;
      }
      if (s == "param") {
        return IRToken::PARAM_IR;
      }
      name = s;
      return IRToken::SYMBOL_IR;
    }
  }
}

IRToken IRLexer::ParseComment() {
  char c;
  c = cinstream.get();
  while ((c = cinstream.peek()) != '\n') {
    c = cinstream.get();
  }
  return IRToken::COMMENT_IR;
}

IRToken IRLexer::NextIRToken() {
  char c;
  while (true) {
    c = cinstream.peek();
    if (c >= '0' && c <= '9') {
      return ParseNum();
    } else if ((c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || (c == '_')) {
      return ParseSymbol();
    } else if (c == EOF) {
      return IRToken::END_IR;
    } else if (c <= 32) {
      if (c == '\n') {
        lineno++;
      }
      c = cinstream.get();
      continue;
    } else {
      switch (c) {
      case '+': {
        c = cinstream.get();
        op = Operator::add_op;
        return IRToken::OP_IR;
      }
      case '-': {
        c = cinstream.get();
        op = Operator::sub_op;
        return IRToken::OP_IR;
      }
      case '*': {
        c = cinstream.get();
        op = Operator::mul_op;
        return IRToken::OP_IR;
      }
      case '/': {
        c = cinstream.get();
        c = cinstream.peek();
        if (c == '/') {
          return ParseComment();
        }
        op = Operator::div_op;
        return IRToken::OP_IR;
      }
      case '%': {
        c = cinstream.get();
        op = Operator::mod_op;
        return IRToken::OP_IR;
      }
      case '>': {
        c = cinstream.get();
        c = cinstream.peek();
        if (c == '=') {
          c = cinstream.get();
          op = Operator::ge_op;
          return IRToken::LOGICOP_IR;
        } else {
          op = Operator::gt_op;
          return IRToken::LOGICOP_IR;
        }
      }
      case '<': {
        c = cinstream.get();
        c = cinstream.peek();
        if (c == '=') {
          c = cinstream.get();
          op = Operator::le_op;
          return IRToken::LOGICOP_IR;
        } else {
          op = Operator::lt_op;
          return IRToken::LOGICOP_IR;
        }
      }
      case '=': {
        c = cinstream.get();
        c = cinstream.peek();
        if (c == '=') {
          c = cinstream.get();
          op = Operator::equ_op;
          return IRToken::LOGICOP_IR;
        } else {
          return IRToken::ASSIGN_IR;
        }
      }
      case '!': {
        c = cinstream.get();
        c = cinstream.peek();
        if (c == '=') {
          c = cinstream.get();
          op = Operator::nequ_op;
          return IRToken::LOGICOP_IR;
        } else {
          op = Operator::not_op;
          return IRToken::OP_IR;
        }
      }
      case '[': {
        c = cinstream.get();
        return IRToken::LSB_IR;
      }
      case ']': {
        c = cinstream.get();
        return IRToken::RSB_IR;
      }
      case ':': {
        c = cinstream.get();
        return IRToken::COLON_IR;
      }
      default:
        exit(23);
      }
    }
  }
}
