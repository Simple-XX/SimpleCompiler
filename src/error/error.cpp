
/**
 * @file error.cpp
 * @brief 错误处理
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

#include "error.h"

Pos::Pos(unsigned int l, unsigned int c) : line(l), col(c) { return; }

Pos::~Pos() { return; }

Error::Error(const std::string &f) : filename(f) {
  err_no = 0;
  pos = new Pos(1, 1);
  return;
}

Error::~Error() {
  if (pos != NULL) {
    delete pos;
  }
  return;
}

void Error::set_line(unsigned int l) {
  pos->line = l;
  return;
}

void Error::set_col(unsigned int c) {
  pos->col = c;
  return;
}

void Error::set_err_no(int e) {
  err_no = e;
  return;
}

int Error::get_err_no() const { return err_no; }

Pos *Error::get_pos() const { return pos; }

void Error::display_err() const {
  std::cout << "\033[;31mErr:\033[0m " << err_no << ", \033[;31mFile:\033[0m "
            << filename << ", \033[;31mLine:\033[0m " << pos->line
            << ", \033[;31mCOL:\033[0m " << pos->col << std::endl;
  return;
}
