
// This file is a part of Simple-XX/SimpleCompiler
// (https://github.com/Simple-XX/SimpleCompiler).
//
// parser.h for Simple-XX/SimpleCompiler.

#ifndef _PARSER_H_
#define _PARSER_H_

#include "string"
#include "iostream"
#include "functional"
#include <functional>
#include <initializer_list>
#include <vector> 
#include "token.h"
#include "lexical.h"
#include "ast.h"
#include "error.h"

using namespace std;

extern Error *error;

// 语法分析
class Parser {
private:
    // 词法分析器
    Lexer &lexer;
    // 超前查看的 token
    Token *token;
    // 获取下一个 token
    void next(void);
    // 匹配指定 Token
    bool match_token(Tag tag);

    // 程序
    ASTPtr program(void);
    // 语句
    ASTPtr statement(void);
    // 一元表达式
    ASTPtr unary(void);
    // 二元表达式
    ASTPtr binary(const function<ASTPtr()> &parser, std::initializer_list<Operator> ops);
    ASTPtr binary_add(void);
    ASTPtr binary_mul(void);
    ASTPtr binary_relation(void);
    ASTPtr binary_eq(void);
    ASTPtr binary_and(void);
    ASTPtr binary_or(void);
    // If then else
    ASTPtr if_else(void);
public:
    Parser(Lexer &lex);
    ~Parser(void);
    // 进行解析
    ASTPtr parsing(void);
    bool is_done(void) const;
};

#endif /* _PARSER_H_ */
