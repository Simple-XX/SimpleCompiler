
/**
 * @file main.cpp
 * @brief 入口
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

#include <iostream>
#include <sstream>
#include <string>
#include <vector>

#include "common.h"

// TODO: 由 init 处理
// 源文件
std::vector<std::string> src_files;
// 输出文件
std::string dest_file = "";

Error *error = NULL;

int main(int _argc, char **_argv) {
  // 初始化
  // 包括与命令行的交互、获取要操作的文件等
  Init initer;
  initer.init(_argc, _argv);
  // 逐个打开文件
  for (const auto &i : src_files) {
    std::cout << "Open file: " << i << std::endl;
    error = new Error(i);
    Scanner scanner(i);
    Lexer lexer(scanner);
    Parser parser(lexer);
    ASTPtr prog = parser.parsing();
    std::cout << "[AST]:" << std::endl << prog->to_string() << std::endl;
    TypeCheck checker = TypeCheck();
    ASTPtr root = prog->Eval(checker);
    if (!root) {
      std::cerr << "Type check error\n";
      exit(2);
    }
    std::map<std::string, Function> FuncTable = checker.FuncTable;
    std::map<int, std::map<std::string, Var>> BlockVars = checker.BlockVars;

    IRGenerator generator =
        IRGenerator(std::move(FuncTable), std::move(BlockVars));
    std::string irout;
    root->GenerateIR(generator, irout);
    std::cout << "[IR]:" << std::endl << irout << std::endl; // ir

    std::istringstream stream_stmt(irout);
    IRParser irparser = IRParser(stream_stmt);
    IRPtr irroot = irparser.ParseProgram();
    LowIRGenerator lowirgenerator = LowIRGenerator();
    std::string lowirout;
    irroot->Generate(lowirgenerator, lowirout);
    std::cout << "[LowIR]:" << std::endl << lowirout << std::endl; // low ir

    std::istringstream stream_stmt2(lowirout);
    CodeGen codegenerator = CodeGen(stream_stmt2);
    std::string riscV;
    codegenerator.Generate(riscV);
    std::cout << "[RiscV]:" << std::endl << riscV << std::endl;
  }

  return 0;
}
