
/**
 * @file parser.h
 * @brief parser
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

#ifndef SIMPLECOMPILER_PARSER_H
#define SIMPLECOMPILER_PARSER_H

#include <functional>
#include <initializer_list>
#include <iostream>
#include <string>
#include <vector>

#include "ast.h"
#include "error.h"
#include "lexical.h"
#include "token.h"

extern Error *error;

// 语法分析
class Parser {
private:
  // 词法分析器
  Lexer &lexer;
  // 超前查看的 token
  token_base_t *token;
  // 获取下一个 token
  void next();
  // 匹配指定 token_base_t
  bool match_token(Tag _tag);

  // 程序
  ASTPtr program();
  // 语句
  ASTPtr statement();

  // 一元表达式
  ASTPtr unary();
  // 二元表达式
  ASTPtr binary(const std::function<ASTPtr()> &_parser,
                std::initializer_list<Operator> _ops);
  ASTPtr binary_add();
  ASTPtr binary_mul();
  ASTPtr binary_relation();
  ASTPtr binary_eq();
  ASTPtr binary_and();
  ASTPtr binary_or();

  // If then else
  ASTPtr if_else();
  // while loop
  ASTPtr while_loop();
  // block
  ASTPtr block();

  // var declare
  ASTPtr var_decl();
  // var definition
  ASTPtr var_def(bool _isConst);
  // initial value
  ASTPtr init_val();

  // function def
  ASTPtr function_def();

public:
  Parser(Lexer &_lex);
  ~Parser();
  // 进行解析
  ASTPtr parsing();
  bool is_done() const;
};

#endif /* SIMPLECOMPILER_PARSER_H */
