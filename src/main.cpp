
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

using namespace std;

// TODO: 由 init 处理
// 源文件
std::vector<std::string> src_files;
// 输出文件
string dest_file = "";

Error *error = NULL;

int main(int argc, char **argv) {
  // 初始化
  // 包括与命令行的交互、获取要操作的文件等
  Init initer;
  initer.init(argc, argv);
  // 逐个打开文件
  for (const auto &i : src_files) {
    cout << "Open file: " << i << endl;
    error = new Error(i);
    Scanner scanner(i);
    Lexer lexer(scanner);
    Parser parser(lexer);
    ASTPtr prog = parser.parsing();
    cout << "[AST]:" << endl << prog->to_string() << endl;
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
    string irout;
    root->GenerateIR(generator, irout);
    cout << "[IR]:" << endl << irout << endl; // ir

    std::istringstream stream_stmt(irout);
    IRParser irparser = IRParser(stream_stmt);
    IRPtr irroot = irparser.ParseProgram();
    LowIRGenerator lowirgenerator = LowIRGenerator();
    string lowirout;
    irroot->Generate(lowirgenerator, lowirout);
    cout << "[LowIR]:" << endl << lowirout << endl; // low ir

    std::istringstream stream_stmt2(lowirout);
    CodeGen codegenerator = CodeGen(stream_stmt2);
    string riscV;
    codegenerator.Generate(riscV);
    cout << "[RiscV]:" << endl << riscV << std::endl;
  }

  return 0;
}
