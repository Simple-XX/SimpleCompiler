
// This file is a part of MRNIU/SimpleCompiler (https://github.com/MRNIU/SimpleCompiler).
//
// scanner.cpp for MRNIU/SimpleCompiler.

#include "scanner.h"

// 扫描器
// 输入：程序文本
// 输出：线性字符序列
// 自动机设计
// 设计词法状态转移自动机
// 解析器（词法分析器）
// 输入：扫描器产生的字符序列
// 输出：词法记号序列

Scanner::Scanner(const std::string &filename) {
	fin.open(filename, ios::in);
	prev_char = ' ';
	curr_char = ' ';
	next_char = ' ';
	real_buf_len = 0;
	pos_read_buf = -1;
	return;
}

Scanner::~Scanner() {
	if(fin.is_open() ) {
		fin.close();
	}
	return;
}

// 扫描
char Scanner::scan() {
	// 缓冲区已经读取完
	if(pos_read_buf == real_buf_len - 1) {
		// 重新读取
		fin.read(scan_buf, SCAN_BUFFER);
		// 读到了多少数据
		real_buf_len = fin.gcount();
		// 文件读完了
		if(real_buf_len == 0) {
			fin.close();
			// 重置读取位置
			pos_read_buf = -1;
			return -1;
		}
		// 重置读取位置
		pos_read_buf = -1;
	}
	// 读取位置++
	pos_read_buf++;
	// 获取对应位置的字符
	curr_char = scan_buf[pos_read_buf];
	// 如果为结束符则返回
	if(curr_char == EOF) {
		fin.close();
		return -1;
	}
	// 否则设置 prev_char
	prev_char = curr_char;
	// 然后返回
	return curr_char;
}

char Scanner::get_prev_char() {
	return prev_char;
}

bool Scanner::is_open() {
	return fin.is_open();
}
