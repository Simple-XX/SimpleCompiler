
/**
 * @file error.h
 * @brief error
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

#ifndef SIMPLECOMPILER_ERROR_H
#define SIMPLECOMPILER_ERROR_H

#include <iostream>
#include <string>

class Pos {
public:
  Pos(unsigned int _l, unsigned int _c);
  ~Pos();
  // 保存当前行号
  unsigned int line;
  // 保存当前列号
  unsigned int col;
};

// 错误处理
class Error {
private:
  // 保存当前文件名
  const std::string filename;
  // 保存当前错误号
  int err_no;
  // 保存错误位置
  Pos *pos;

public:
  Error(const std::string &_f);
  virtual ~Error();
  void set_line(unsigned int _l);
  void set_col(unsigned int _c);
  void set_err_no(int _e);
  int get_err_no() const;
  Pos *get_pos() const;
  virtual void display_err() const;
};

#endif /* SIMPLECOMPILER_ERROR_H */
