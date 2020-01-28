
// This file is a part of MRNIU/SimpleCompiler (https://github.com/MRNIU/SimpleCompiler).
//
// scaner.cpp for MRNIU/SimpleCompiler.

#include "common.h"

// 扫描器
// 输入：程序文本
// 输出：线性字符序列
// 自动机设计
// 设计词法状态转移自动机
// 解析器（词法分析器）
// 输入：扫描器产生的字符序列
// 输出：词法记号序列

#define BUFLEN 80
int lineLen = 0;
int readPos = -1;
char line[BUFLEN];
int lineNum = 1;
int colNum = 0;
char ch = ' ';
char lastch = ' ';

char scan(FILE * file) {
	if(!file) {
		return -1;
	}
	if(readPos == lineLen - 1) {
		lineLen = fread(line, 1, BUFLEN, file);
		if(lineLen == 0) {
			lineLen = 1;
			line[0] = -1;
		}
		readPos = -1;
	}
	readPos++;
	ch = line[readPos];
	if(lastch == '\n') {
		lineNum++;
		colNum = 0;
	}
	if(ch == -1) {
		fclose(file);
		file = NULL;
	}
	else if(ch != '\n') {
		colNum++;
	}
	lastch = ch;
	return ch;
}
