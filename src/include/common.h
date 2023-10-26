
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
