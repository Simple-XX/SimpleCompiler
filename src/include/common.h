
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
#include "typechecker.h"
#include "irgen.h"
#include "ir.h"
#include "irast.h"
#include "irlexer.h"
#include "irparser.h"
#include "lowirgen.h"
#include "codegen.h"
#include "utils.h"

class Init;
class Error;
class Lexer;
class Scanner;
class Parser;
class TypeChecker;
class IRGenerator;
class IRLexer;
class IRParser;
class LowIRGenerator;
class Codegen;

extern Error *error;

extern const char *tokenName[];

#endif /* _COMMON_H_ */
