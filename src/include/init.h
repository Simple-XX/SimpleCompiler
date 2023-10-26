
/**
 * @file init.h
 * @brief init
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

#ifndef SIMPLECOMPILER_INIT_H
#define SIMPLECOMPILER_INIT_H

#include <string>
#include <vector>

// 源文件
extern std::vector<std::string> src_files;
// 输出文件
extern std::string dest_file;

class Init {
private:
  // 绝对路径
  std::string abs_path;
  // 路径缓存大小
  static const int PATH_BUFFER = 1024;
  // 路径缓存
  char abs_path_buffer[PATH_BUFFER];
  // 用于接收选项
  int index;
  int c;

public:
  Init();
  ~Init();
  int init(int &_argc, char **&_argv);
};

#endif /* SIMPLECOMPILER_INIT_H */
