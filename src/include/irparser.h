
// This file is a part of Simple-XX/SimpleCompiler
// (https://github.com/Simple-XX/SimpleCompiler).
//
// irparser.h for Simple-XX/SimpleCompiler.

#ifndef _IRPARSER_H_
#define _IRPARSER_H_

#include "ir.h"
#include "irast.h"
#include <iostream>
#include "irlexer.h"

class IRParser {
public:
    IRParser(std::istream &_cinstream): lexer(_cinstream) {}

    void NextIRToken();

    IRPtr ParseDecl();

    IRPtr ParseInit();

    IRPtr ParseFuncDef();

    IRPtr ParseStatements();

    IRPtr ParseAssign();

    IRPtr ParseCondGoto();

    IRPtr ParseLVal();

    IRPtr ParseGoto();

    IRPtr ParseLabel();

    IRPtr ParseParams();

    IRPtr ParseFuncCall();

    IRPtr ParseReturn();

    IRPtr ParseProgram();

private:
    IRLexer lexer;
    IRToken current;
};


#endif