
// This file is a part of MRNIU/SimpleCompiler (https://github.com/MRNIU/SimpleCompiler).
//
// main.cpp for MRNIU/SimpleCompiler.

#include "common.h"

// 源文件
std::vector<std::string> src_files;
// 输出文件
string dest_file = "";

int main(int argc, char * * argv) {
	// 初始化
	// 包括与命令行的交互、获取要操作的文件等
	Init initer;
	initer.init(argc, argv);
	// 逐个打开文件
	for(const auto &i:src_files) {
		Scanner scanner(i);
		while(scanner.is_open() ) {
			auto ch = scanner.scan();
			cout << ch;
		}
	}

	return 0;
}
