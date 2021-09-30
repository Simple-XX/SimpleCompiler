
// This file is a part of Simple-XX/SimpleCompiler
// (https://github.com/Simple-XX/SimpleCompiler).
//
// irlexer.h for Simple-XX/SimpleCompiler.

#ifndef _IRLEXER_H
#define _IRLEXER_H

#include <string>
#include <iostream>
#include "ir.h"
#include "type.h"

class IRLexer {
public:
    std::istream &cinstream;
    IRLexer(std::istream & _cinstream): cinstream(_cinstream) {};
    ~IRLexer() = default;

    IRToken NextIRToken();
    Operator getOp() { return op; }
    std::string getName() { return name; }
    int getVal() const { return value; }
    int getLineno() const { return lineno; }

private:
    int value;
    std::string name;
    Operator op;
    int lineno;

    IRToken ParseNum();
    IRToken ParseSymbol();
    IRToken ParseComment();
};

#endif