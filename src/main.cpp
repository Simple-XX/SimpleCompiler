
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
#include "log.h"

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
    SPDLOG_LOGGER_INFO(SCLOG, "打开文件：{}", i);
    error = new Error(i);
    Scanner scanner(i);
    Lexer lexer(scanner);
    Parser parser(lexer);
    ASTPtr prog = parser.parsing();
    SPDLOG_LOGGER_INFO(SCLOG, "[AST]:\n{}", prog->to_string());
    TypeCheck checker = TypeCheck();
    ASTPtr root = prog->Eval(checker);
    if (!root) {
      SPDLOG_LOGGER_ERROR(SCLOG, "Type check error");
      exit(2);
    }
    std::map<std::string, Function> FuncTable = checker.FuncTable;
    std::map<int, std::map<std::string, Var>> BlockVars = checker.BlockVars;

    IRGenerator generator =
        IRGenerator(std::move(FuncTable), std::move(BlockVars));
    std::string irout;
    root->GenerateIR(generator, irout);
    SPDLOG_LOGGER_INFO(SCLOG, "[IR]:\n{}", irout);

    std::istringstream stream_stmt(irout);
    IRParser irparser = IRParser(stream_stmt);
    IRPtr irroot = irparser.ParseProgram();
    LowIRGenerator lowirgenerator = LowIRGenerator();
    std::string lowirout;
    irroot->Generate(lowirgenerator, lowirout);
    SPDLOG_LOGGER_INFO(SCLOG, "[LOW IR]:\n{}", lowirout);

    std::istringstream stream_stmt2(lowirout);
    CodeGen codegenerator = CodeGen(stream_stmt2);
    std::string riscV;
    codegenerator.Generate(riscV);
    SPDLOG_LOGGER_INFO(SCLOG, "[RISCV]:\n{}", riscV);
  }

  return 0;
}
