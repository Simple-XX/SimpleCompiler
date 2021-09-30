
// This file is a part of Simple-XX/SimpleCompiler
// (https://github.com/Simple-XX/SimpleCompiler).
//
// codegen.h for Simple-XX/SimpleCompiler.

#ifndef _CODEGEN_H_
#define _CODEGEN_H_

#include <map>
#include <string>
#include <iostream>

class CodeGen {
    std::map<std::string, int> funcStack;
    std::string currentFunc;
    std::istream &cinstream;
public:
    void Generate(std::string &code);
    CodeGen(std::istream &_cinstream): cinstream(_cinstream) {}
};

#endif