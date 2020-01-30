
// This file is a part of MRNIU/SimpleCompiler (https://github.com/MRNIU/SimpleCompiler).
//
// lexical.cpp for MRNIU/SimpleCompiler.

#include "iostream"
#include "string"
#include "lexical.h"

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

bool Lexer::scan(char need) {
	ch = scanner.scan();
	if(need) {
		if(ch != need)
			return false;
		ch = scanner.scan();
		return true;
	}
	return true;
}

void Lexer::blank() {
	Token * t = NULL;
	// 跳过空字符
	do {
		scan();
	} while( (ch == ' ') || (ch == '\n') || (ch == '\t') );
	token = t;
	return;
}

void Lexer::identifier() {
	Token * t = NULL;
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
	token = t;
	return;
}

void Lexer::number() {
	Token * t = NULL;
	int val = 0;
	// 十进制数
	while(
	    ( (ch >= '1') && (ch <= '9') ) ) {
		do {
			// 计算数字
			val = val * 10 + ch - '0';
			scan();
		} while(
		    (ch >= '0' && ch <= '9') );
		t = new Num(val);
	}
	token = t;
	return;
}

void Lexer::character() {
	Token * t = NULL;
	do {
		// 过滤掉第一个单引号
		if(ch == '\'') {
			continue;
		}
		// 文件结束或换行
		else if( (ch == '\n') || (ch == EOF) ) {
			t = new Token(ERR);
			break;
		}
		// 转义字符
		else if(ch == '\\') {
			scan();
			if(ch == 'n') {
				t = new Char('\n');
			}
			else if(ch == 't') {
				t = new Char('\t');
			}
			else if(ch == '0') {
				t = new Char('\0');
			}
			else if(ch == '\'') {
				t = new Char('\'');
			}
			else if(ch == '\\') {
				t = new Char('\\');
			}
			else if( (ch == EOF) || (ch == '\n') ) {
				t = new Token(ERR);
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
	} while(scan(), ch != '\'');
	token = t;
	return;
}

void Lexer::str() {
	Token * t = NULL;
	string s = "";
	do {
		// 过滤掉第一个双引号
		if(ch == '"') {
			continue;
		}
		// 直接结束
		else if( (ch == '\n') || (ch == EOF) ) {
			t = new Token(ERR);
			break;
		}
		// 转义字符
		if(ch == '\\') {
			scan();
			if(ch == 'n') {
				s.push_back('\n');
			}
			else if(ch == 't') {
				s.push_back('\t');
			}
			else if(ch == '0') {
				s.push_back('\0');
			}
			else if(ch == '"') {
				s.push_back('"');
			}
			else if(ch == '\\') {
				s.push_back('\\');
			}
			else if(ch == '\n') {
				;
			}
			else if(ch == EOF) {
				t = new Token(ERR);
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
	} while(scan(), ch != '"');
	// 最终字符串
	if(t == NULL) {
		t = new Str(s);
	}
	token = t;
	return;
}

void Lexer::separator() {
	Token * t = NULL;
	switch(ch) {
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
		    t = new Token(ERR);// 错误的词法记号
	}
	scan();
	token = t;
	return;
}

void Lexer::operation() {
	Token * t = NULL;
	switch(ch) {
	    case '=':
		    t = new Token(scan('=') ? EQU : ASSIGN);
		    break;
	    case '+':
		    t = new Token(ADD);
		    break;
	    case '-':
		    t = new Token(SUB);
		    break;
	    case '*':
		    t = new Token(MUL);
		    break;
	    case '/':
		    scan();
		    // 单行注释
		    if(ch == '/') {
			    while(ch != '\n' && ch != EOF)
				    scan();
		    }
		    // 多行注释
		    else if(ch == '*') {
			    while(scan(EOF) == false) {
				    if(ch == '*') {
					    if(scan('/') )
						    break;
				    }
			    }
			    // 没有闭合
			    if(ch == EOF) {
				    cout << "多行注释未正常结束" << endl;
			    }
		    }
		    // 否则为除号
		    else
			    t = new Token(DIV);
		    break;
	    case '%':
		    t = new Token(MOD);
		    break;
	    case '|':
		    t = new Token(scan('|') ? OR : ORBIT);
		    break;
	    case '&':
		    t = new Token(scan('&') ? AND : ANDBIT);
		    break;
	    case '^':
		    t = new Token(EORBIT);
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
	}
	scan();
	token = t;
	return;
}

Token * Lexer::lexing() {
	// 字符不为空时
	while(is_done() == false) {
		if( (ch == ' ') || (ch == '\n') || (ch == '\t') )
			blank();
		if( (ch >= 'a' && ch <= 'z') ||
		    (ch >= 'A' && ch <= 'Z') ||
		    (ch == '_') )
			identifier();
		if( ( (ch >= '1') && (ch <= '9') ) )
			number();
		if( (ch == '\'') ) {
			character();
		}
		if(ch == '"')
			this->str();
		if(
		    (ch == '(') || (ch == ')') || (ch == '{') || (ch == '}') || (ch == ',')
		    || (ch == ':') || (ch == ';') ) {
			separator();
		}
		if(
		    (ch == '=') || (ch == '+') || (ch == '-') || (ch == '*') || (ch == '/')
		    || (ch == '%') || (ch == '|') || (ch == '&') || (ch == '^')
		    || (ch == '!')
		    || (ch == '>') || (ch == '<') ) {
			operation();
		}
		// 更新 token 内容
		if(token != NULL && token->tag != ERR) {
			return token;
		}
	}
	token = new Token(END);
	return token;
}

bool Lexer::is_done() {
	return scanner.is_done();
}
