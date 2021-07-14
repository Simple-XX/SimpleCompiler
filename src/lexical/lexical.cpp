
// This file is a part of Simple-XX/SimpleCompiler
// (https://github.com/Simple-XX/SimpleCompiler).
//
// lexical.cpp for Simple-XX/SimpleCompiler.

#include "iostream"
#include "string"
#include "lexical.h"

using namespace std;

Keywords Lexer::keywords;

Lexer::Lexer(Scanner &sc) : scanner(sc) {
    ch    = ' ';
    token = NULL;
    return;
}

Lexer::~Lexer() {
    if (token != NULL) {
        delete token;
    }
    return;
}

bool Lexer::scan(char need) {
    ch = scanner.scan();
    if (need) {
        if (ch != need)
            return false;
        ch = scanner.scan();
        return true;
    }
    return true;
}

void Lexer::blank() {
    Token *t = NULL;
    // 跳过空字符
    do {
        scan();
    } while (COND_BLANK);
    token = t;
    return;
}

void Lexer::identifier() {
    Token *t = NULL;
    // 标识符
    while (COND_IDENTIFIER) {
        string name = "";
        do {
            // 记录字符
            name.push_back(ch);
            scan();
        } while (COND_IDENTIFIER || (ch >= '0' && ch <= '9'));
        // 判断是不是关键字
        Tag tag = keywords.get_tag(name);
        // 如果不是，则为标识符
        if (tag == ID) {
            t = new Id(name);
        }
        // 如果是
        else {
            t = new Token(tag);
        }
    }
    token = t;
    return;
}

void Lexer::number() {
    Token *t   = NULL;
    int    val = 0;
    // 十进制数
    do {
        // 计算数字
        val = val * 10 + ch - '0';
    } while (scan(), (ch >= '0' && ch <= '9'));
    t     = new Num(val);
    token = t;
    return;
}

void Lexer::character() {
    Token *t = NULL;
    do {
        // 过滤掉第一个单引号
        if (ch == '\'') {
            continue;
        }
        // 文件结束或换行
        else if ((ch == '\n') || (ch == EOF) ) {
            t = new Token(ERR);
            error->set_err_no(ERR);
            error->display_err();
            break;
        }
        // 转义字符
        else if (ch == '\\') {
            scan();
            if (ch == 'n') {
                t = new Char('\n');
            }
            else if (ch == 't') {
                t = new Char('\t');
            }
            else if (ch == '0') {
                t = new Char('\0');
            }
            else if (ch == '\'') {
                t = new Char('\'');
            }
            else if (ch == '\\') {
                t = new Char('\\');
            }
            else if ((ch == EOF) || (ch == '\n')) {
                t = new Token(ERR);
                error->set_err_no(ERR);
                error->display_err();
                break;
            }
            // 其它的不转义
            else {
                t = new Char(ch);
            }
        }
        // 一般情况
        else {
            t = new Char(ch);
        }
    } while (scan('\'') == false);
    token = t;
    return;
}

void Lexer::str() {
    Token *t = NULL;
    string s = "";
    do {
        // 过滤掉第一个双引号
        if (ch == '"') {
            continue;
        }
        // 直接结束
        else if ((ch == '\n') || (ch == EOF)) {
            t = new Token(ERR);
            error->set_err_no(ERR);
            error->display_err();
            break;
        }
        // 转义字符
        if (ch == '\\') {
            scan();
            if (ch == 'n') {
                s.push_back('\n');
            }
            else if (ch == 't') {
                s.push_back('\t');
            }
            else if (ch == '0') {
                s.push_back('\0');
            }
            else if (ch == '"') {
                s.push_back('"');
            }
            else if (ch == '\\') {
                s.push_back('\\');
            }
            else if (ch == '\n') {
                ;
            }
            else if (ch == EOF) {
                t = new Token(ERR);
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
        t = new Str(s);
    }
    token = t;
    return;
}

void Lexer::separator() {
    Token *t = NULL;
    switch (ch) {
        case '(':
            t = new Token(LPAREN);
            break;
        case ')':
            t = new Token(RPAREN);
            break;
        case '{':
            t = new Token(LBRACE);
            break;
        case '}':
            t = new Token(RBRACE);
            break;
        case '[':
            t = new Token(LBRACKET);
            break;
        case ']':
            t = new Token(RBRACKET);
            break;
        case ',':
            t = new Token(COMMA);
            break;
        case ':':
            t = new Token(COLON);
            break;
        case ';':
            t = new Token(SEMICON);
            break;
        default:
            t = new Token(ERR); // 错误的词法记号
    }
    scan();
    token = t;
    return;
}

void Lexer::operation() {
    Token *t = NULL;
    switch (ch) {
        case '=':
            t = new Token(scan('=') ? EQU : ASSIGN);
            break;
        case '+':
            t = new Token(ADD);
            scan();
            break;
        case '-':
            t = new Token(SUB);
            scan();
            break;
        case '*':
            t = new Token(MUL);
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
                    cout << "多行注释未正常结束" << endl;
                }
            }
            // 否则为除号
            else
                t = new Token(DIV);
            break;
        case '%':
            t = new Token(MOD);
            scan();
            break;
        case '|':
            t = new Token(scan('|') ? OR : ORBIT);
            break;
        case '&':
            t = new Token(scan('&') ? AND : ANDBIT);
            break;
        case '^':
            t = new Token(EORBIT);
            scan();
            break;
        case '!':
            t = new Token(scan('=') ? NEQU : NOT);
            break;
        case '>':
            t = new Token(scan('=') ? GE : GT);
            break;
        case '<':
            t = new Token(scan('=') ? LE : LT);
            break;
        // 除此以外是错误的
        default:
            t = new Token(ERR);
            error->set_err_no(ERR);
            error->display_err();
            scan();
    }
    token = t;
    return;
}

Token *Lexer::lexing() {
    // 字符不为空且没有出错时
    while ((is_done() == false)) {
        if (COND_BLANK)
            blank();
        else if (COND_IDENTIFIER)
            identifier();
        else if (COND_NUMBER) {
            number();
        }
        else if ((ch == '\'')) {
            character();
        }
        else if (ch == '"') {
            this->str();
        }
        else if (COND_SEPARATOR) {
            separator();
        }
        else if (COND_OPERATION) {
            operation();
        }
        else {
            token = new Token(ERR);
            error->set_err_no(ERR);
            error->display_err();
            return token;
        }
        // 更新 token 内容
        if (token != NULL && token->tag != ERR) {
            return token;
        }
    }
    token = new Token(END);
    return token;
}

bool Lexer::is_done() const {
    return scanner.is_done() || (error->get_err_no() < 0);
}
