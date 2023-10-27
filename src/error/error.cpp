
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
#include "log.h"

Pos::Pos(unsigned int _l, unsigned int _c) : line(_l), col(_c) { return; }

Pos::~Pos() { return; }

Error::Error(const std::string &_f) : filename(_f) {
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

void Error::set_line(unsigned int _l) {
  pos->line = _l;
  return;
}

void Error::set_col(unsigned int _c) {
  pos->col = _c;
  return;
}

void Error::set_err_no(int _e) {
  err_no = _e;
  return;
}

int Error::get_err_no() const { return err_no; }

Pos *Error::get_pos() const { return pos; }

void Error::display_err() const {
  SPDLOG_LOGGER_ERROR(SCLOG, "[ERROR] File: {}, Line: {}, Pos: {}", filename,
                      pos->line, pos->col);
  return;
}
