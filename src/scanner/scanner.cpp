
/**
 * @file scanner.cpp
 * @brief 读入
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

#include "log.h"
#include "scanner.h"

Scanner::Scanner(const std::string &_filename) {
  fin.open(_filename, std::ios::in);
  if (fin.is_open() == false) {
    SPDLOG_LOGGER_ERROR(SCLOG, "Open file failed");
  }
  prev_char = ' ';
  curr_char = ' ';
  real_buf_len = 0;
  pos_read_buf = -1;
  return;
}

Scanner::~Scanner() {
  if (fin.is_open()) {
    fin.close();
  }
  return;
}

// 扫描
char Scanner::scan() {
  // 缓冲区已经读取完
  if (pos_read_buf == real_buf_len - 1) {
    // 重新读取
    fin.read(scan_buf, SCAN_BUFFER);
    // 读到了多少数据
    real_buf_len = fin.gcount();
    // 文件读完了
    if (real_buf_len == 0) {
      fin.close();
      // 重置读取位置
      pos_read_buf = -1;
      return EOF;
    }
    // 重置读取位置
    pos_read_buf = -1;
  }
  // 读取位置++
  pos_read_buf++;
  // 获取对应位置的字符
  curr_char = scan_buf[pos_read_buf];
  // 如果为结束符则返回
  if (curr_char == EOF) {
    fin.close();
    return EOF;
  }
  // 如果是换行符，就把当前行 +1，列重置
  if (curr_char == '\n') {
    error->set_line(++(error->get_pos()->line));
    error->set_col(1);
  }
  // 否则列 +1
  else {
    error->set_col(++(error->get_pos()->col));
  }
  // 否则设置 prev_char
  prev_char = curr_char;
  // 然后返回
  return curr_char;
}

char Scanner::get_prev_char() { return prev_char; }

// 若文件关闭则视为已完成
// 已完成返回 true
bool Scanner::is_done() { return !fin.is_open(); }
