
/**
 * @file scanner.h
 * @brief scanner
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

#ifndef _SCANNER_H_
#define _SCANNER_H_

#include "string"
#include "fstream"
#include "error.h"

using namespace std;

extern Error *error;

// 扫描器
class Scanner {
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
    // 读一个文件
    Scanner(const std::string &filename);
    ~Scanner(void);
    // 扫描并返回字符
    char scan(void);
    // 返回前一个字符
    char get_prev_char(void);
    // 文件是否结束
    bool is_done(void);
};

#endif /* _SCANNER_H_ */
