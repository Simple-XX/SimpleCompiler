
// This file is a part of Simple-XX/SimpleCompiler
// (https://github.com/Simple-XX/SimpleCompiler).
//
// common.h for Simple-XX/SimpleCompiler.

#ifndef _COMMON_H_
#define _COMMON_H_

#include "init.h"
#include "error.h"
#include "scanner.h"
#include "lexical.h"
#include "parser.h"
#include "ast.h"

class Init;
class Error;
class Lexer;
class Scanner;
class Parser;
class Variable;
class Function;
class SymTab;

extern Error *error;

extern const char *tokenName[];

#endif /* _COMMON_H_ */
