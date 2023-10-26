
/**
 * @file common.h
 * @brief common
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

#ifndef SIMPLECOMPILER_COMMON_H
#define SIMPLECOMPILER_COMMON_H

#include "ast.h"
#include "codegen.h"
#include "error.h"
#include "init.h"
#include "ir.h"
#include "irast.h"
#include "irgen.h"
#include "irlexer.h"
#include "irparser.h"
#include "lexical.h"
#include "lowirgen.h"
#include "parser.h"
#include "scanner.h"
#include "typechecker.h"
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

#endif /* SIMPLECOMPILER_COMMON_H */
