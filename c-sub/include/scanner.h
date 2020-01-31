
// This file is a part of MRNIU/SimpleCompiler (https://github.com/MRNIU/SimpleCompiler).
//
// scanner.h for MRNIU/SimpleCompiler.

#ifndef _SCANNER_H_
#define _SCANNER_H_

#include "string"
#include "fstream"
#include "error.h"

using namespace std;

class Scanner : public Error {
private:
	// 输入流
	ifstream fin;
	// 前一个读到的字符
	char prev_char;
	// 当前读到的字符
	char curr_char;
	// 扫描缓冲区长度
	static const int SCAN_BUFFER = 128;
	// 扫描缓冲区
	char scan_buf[SCAN_BUFFER];
	// 实际读取到的字节数
	int real_buf_len;
	// 缓冲区读取位置
	int pos_read_buf;

public:
	Scanner(const std::string & filename);
	~Scanner(void);
	// 扫描并返回字符
	char scan(void);
	// 返回前一个字符
	char get_prev_char(void);
	bool is_done(void);
};


#endif /* _SCANNER_H_ */
