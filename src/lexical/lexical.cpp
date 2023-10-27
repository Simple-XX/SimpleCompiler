
/**
 * @file lexical.cpp
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

#include <iostream>
#include <string>

#include "lexical.h"
#include "log.h"

keywords_t Lexer::keywords;

Lexer::Lexer(Scanner &_sc) : scanner(_sc) {
  ch = ' ';
  token = NULL;
  return;
}

Lexer::~Lexer() {
  if (token != NULL) {
    delete token;
  }
  return;
}

bool Lexer::scan(char _need) {
  ch = scanner.scan();
  if (_need) {
    if (ch != _need)
      return false;
    ch = scanner.scan();
    return true;
  }
  return true;
}

void Lexer::blank() {
  token_base_t *t = NULL;
  // 跳过空字符
  do {
    scan();
  } while (COND_BLANK);
  token = t;
  return;
}

void Lexer::identifier() {
  token_base_t *t = NULL;
  // 标识符
  while (COND_IDENTIFIER) {
    std::string name = "";
    do {
      // 记录字符
      name.push_back(ch);
      scan();
    } while (COND_IDENTIFIER || (ch >= '0' && ch <= '9'));
    // 判断是不是关键字
    tag_t tag = keywords.get_tag(name);
    // 如果不是，则为标识符
    if (tag == ID) {
      t = new token_identifier_t(name);
    }
    // 如果是
    else {
      t = new token_base_t(tag);
    }
  }
  token = t;
  return;
}

void Lexer::number() {
  token_base_t *t = NULL;
  int val = 0;
  // 十进制数
  do {
    // 计算数字
    val = val * 10 + ch - '0';
  } while (scan(), (ch >= '0' && ch <= '9'));
  t = new token_num_t(val);
  token = t;
  return;
}

void Lexer::character() {
  token_base_t *t = NULL;
  do {
    // 过滤掉第一个单引号
    if (ch == '\'') {
      continue;
    }
    // 文件结束或换行
    else if ((ch == '\n') || (ch == EOF)) {
      t = new token_base_t(ERR);
      error->set_err_no(ERR);
      error->display_err();
      break;
    }
    // 转义字符
    else if (ch == '\\') {
      scan();
      if (ch == 'n') {
        t = new token_char_t('\n');
      } else if (ch == 't') {
        t = new token_char_t('\t');
      } else if (ch == '0') {
        t = new token_char_t('\0');
      } else if (ch == '\'') {
        t = new token_char_t('\'');
      } else if (ch == '\\') {
        t = new token_char_t('\\');
      } else if ((ch == EOF) || (ch == '\n')) {
        t = new token_base_t(ERR);
        error->set_err_no(ERR);
        error->display_err();
        break;
      }
      // 其它的不转义
      else {
        t = new token_char_t(ch);
      }
    }
    // 一般情况
    else {
      t = new token_char_t(ch);
    }
  } while (scan('\'') == false);
  token = t;
  return;
}

void Lexer::str() {
  token_base_t *t = NULL;
  std::string s = "";
  do {
    // 过滤掉第一个双引号
    if (ch == '"') {
      continue;
    }
    // 直接结束
    else if ((ch == '\n') || (ch == EOF)) {
      t = new token_base_t(ERR);
      error->set_err_no(ERR);
      error->display_err();
      break;
    }
    // 转义字符
    if (ch == '\\') {
      scan();
      if (ch == 'n') {
        s.push_back('\n');
      } else if (ch == 't') {
        s.push_back('\t');
      } else if (ch == '0') {
        s.push_back('\0');
      } else if (ch == '"') {
        s.push_back('"');
      } else if (ch == '\\') {
        s.push_back('\\');
      } else if (ch == '\n') {
        ;
      } else if (ch == EOF) {
        t = new token_base_t(ERR);
        error->set_err_no(ERR);
        error->display_err();
        break;
      }
      // 其它的不转义
      else {
        s.push_back(ch);
      }
    }
    // 一般情况
    else {
      s.push_back(ch);
    }
  } while (scan('"') == false);
  // 最终字符串
  if (t == NULL) {
    t = new token_string_t(s);
  }
  token = t;
  return;
}

void Lexer::separator() {
  token_base_t *t = NULL;
  switch (ch) {
  case '(':
    t = new token_base_t(LPAREN);
    break;
  case ')':
    t = new token_base_t(RPAREN);
    break;
  case '{':
    t = new token_base_t(LBRACE);
    break;
  case '}':
    t = new token_base_t(RBRACE);
    break;
  case '[':
    t = new token_base_t(LBRACKET);
    break;
  case ']':
    t = new token_base_t(RBRACKET);
    break;
  case ',':
    t = new token_base_t(COMMA);
    break;
  case ':':
    t = new token_base_t(COLON);
    break;
  case ';':
    t = new token_base_t(SEMICON);
    break;
  default:
    t = new token_base_t(ERR); // 错误的词法记号
  }
  scan();
  token = t;
  return;
}

void Lexer::operation() {
  token_base_t *t = NULL;
  switch (ch) {
  case '=':
    t = new token_base_t(scan('=') ? EQU : ASSIGN);
    break;
  case '+':
    t = new token_base_t(ADD);
    scan();
    break;
  case '-':
    t = new token_base_t(SUB);
    scan();
    break;
  case '*':
    t = new token_base_t(MUL);
    scan();
    break;
  case '/':
    scan();
    // 单行注释
    if (ch == '/') {
      while (ch != '\n' && ch != EOF)
        scan();
    }
    // 多行注释
    else if (ch == '*') {
      while (scan(EOF) == false) {
        if (ch == '*') {
          if (scan('/'))
            break;
        }
      }
      // 没有闭合
      if (ch == EOF) {
        SPDLOG_LOGGER_ERROR(SCLOG, "多行注释未正常结束");
      }
    }
    // 否则为除号
    else
      t = new token_base_t(DIV);
    break;
  case '%':
    t = new token_base_t(MOD);
    scan();
    break;
  case '|':
    t = new token_base_t(scan('|') ? OR : ORBIT);
    break;
  case '&':
    t = new token_base_t(scan('&') ? AND : ANDBIT);
    break;
  case '^':
    t = new token_base_t(EORBIT);
    scan();
    break;
  case '!':
    t = new token_base_t(scan('=') ? NEQU : NOT);
    break;
  case '>':
    t = new token_base_t(scan('=') ? GE : GT);
    break;
  case '<':
    t = new token_base_t(scan('=') ? LE : LT);
    break;
  // 除此以外是错误的
  default:
    t = new token_base_t(ERR);
    error->set_err_no(ERR);
    error->display_err();
    scan();
  }
  token = t;
  return;
}

token_base_t *Lexer::lexing() {
  // 字符不为空且没有出错时
  while ((is_done() == false)) {
    if (COND_BLANK)
      blank();
    else if (COND_IDENTIFIER)
      identifier();
    else if (COND_NUMBER) {
      number();
    } else if (ch == '\'') {
      character();
    } else if (ch == '"') {
      this->str();
    } else if (COND_SEPARATOR) {
      separator();
    } else if (COND_OPERATION) {
      operation();
    } else {
      token = new token_base_t(ERR);
      error->set_err_no(ERR);
      error->display_err();
      return token;
    }
    // 更新 token 内容
    if (token != NULL && token->tag != ERR) {
      return token;
    }
  }
  token = new token_base_t(END);
  return token;
}

bool Lexer::is_done() const {
  return scanner.is_done() || (error->get_err_no() < 0);
}
