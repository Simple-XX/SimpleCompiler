
// This file is a part of MRNIU/SimpleCompiler (https://github.com/MRNIU/SimpleCompiler).
//
// main.c for MRNIU/SimpleCompiler.

#include "common.h"

ifstream fin;
ofstream fout;

string finName;  // 输入文件名
bool showAss = false;  // 显示汇编信息

int main(int argc, char * argv[]) {
	// finName = argv[1];
	// showAss = (argv[2][0] == 'y');
	// fin = fopen( (finName + ".s").c_str(), "r");  // 输入文件
	// fout = fopen( (finName + ".t").c_str(), "w");  // 临时输出文件，供代码段使用
	// program();  // .text
	// obj.writeElf();  // 输出头
	// fclose(fin);
	// fclose(fout);
	// remove( (finName + ".t").c_str() );
	return 0;
}
