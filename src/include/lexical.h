
/**
 * @file lexical.h
 * @brief lexical
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

#ifndef SIMPLECOMPILER_LEXICAL_H
#define SIMPLECOMPILER_LEXICAL_H

#include "error.h"
#include "scanner.h"
#include "token.h"

extern Error *error;

// blank 条件
#define COND_BLANK ((ch == ' ') || (ch == '\n') || (ch == '\t') || (ch == '\r'))
// identifier 条件
#define COND_IDENTIFIER                                                        \
  ((ch >= 'a' && ch <= 'z') || (ch >= 'A' && ch <= 'Z') || (ch == '_'))
// number 条件
#define COND_NUMBER (((ch >= '0') && (ch <= '9')))
// separator 条件
#define COND_SEPARATOR                                                         \
  ((ch == '(') || (ch == ')') || (ch == '{') || (ch == '}') || (ch == ',') ||  \
   (ch == ':') || (ch == ';') || (ch == '[') || (ch == ']'))
// operation 条件
#define COND_OPERATION                                                         \
  ((ch == '=') || (ch == '+') || (ch == '-') || (ch == '*') || (ch == '/') ||  \
   (ch == '%') || (ch == '|') || (ch == '&') || (ch == '^') || (ch == '!') ||  \
   (ch == '>') || (ch == '<'))

// 词法分析
class Lexer {
private:
  // 扫描器对象，用于从文件中获取字符
  Scanner &scanner;
  // 关键字
  static Keywords keywords;
  // 当前字符
  char ch;
  // 保存结果
  Token *token;
  // 扫描
  bool scan(char _need = 0);
  // 返回错误号
  int err();
  // 空白字符
  void blank();
  // 标识符，包括关键字
  void identifier();
  // 数字
  void number();
  // 字符
  void character();
  // 字符串
  void str();
  // 分界符
  void separator();
  // 操作符
  void operation();

public:
  Lexer(Scanner &_sc);
  ~Lexer();

  Token *lexing();
  bool is_done() const;
};

#endif /* SIMPLECOMPILER_LEXICAL_H */
