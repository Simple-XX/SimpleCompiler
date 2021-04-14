
// This file is a part of Simple-XX/SimpleCompiler
// (https://github.com/Simple-XX/SimpleCompiler).
//
// main.cpp for Simple-XX/SimpleCompiler.

#include "iostream"
#include "string"
#include "vector"
#include "common.h"

using namespace std;

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
        Lexer   lexer(scanner);
        SymTab  symtab(void);
        // Parser parser(lexer);
        while (lexer.is_done() == false) {
            auto ch = lexer.lexing();
            if (ch->tag < 0) {
                if (ch->tag == EOF) {
                    cout << "File done: " << i << endl;
                }
            }
            else {
                TOKEN_CAST(ch);
                cout << ch->to_string() << endl;
            }
        }
    }

    return 0;
}
