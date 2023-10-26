
/**
 * @file irparser.h
 * @brief irparser
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

#ifndef SIMPLECOMPILER_IRPARSER_H
#define SIMPLECOMPILER_IRPARSER_H

#include <iostream>

#include "ir.h"
#include "irast.h"
#include "irlexer.h"

class IRParser {
public:
  IRParser(std::istream &_cinstream) : lexer(_cinstream) {}

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

#endif /* SIMPLECOMPILER_IRPARSER_H */
