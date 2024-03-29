
/**
 * @file codegen.h
 * @brief codegen
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

#ifndef SIMPLECOMPILER_CODEGEN_H
#define SIMPLECOMPILER_CODEGEN_H

#include <iostream>
#include <map>
#include <string>

class CodeGen {
  std::map<std::string, int> funcStack;
  std::string currentFunc;
  std::istream &cinstream;

public:
  void Generate(std::string &_code);
  CodeGen(std::istream &_cinstream) : cinstream(_cinstream) {}
};

#endif /* SIMPLECOMPILER_CODEGEN_H */
