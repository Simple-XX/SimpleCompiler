
/**
 * @file irlexer.h
 * @brief irlexer
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