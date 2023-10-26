
/**
 * @file init.cpp
 * @brief 初始化
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

#include <cstring>
#include <getopt.h>
#include <iostream>
#include <unistd.h>

#include "init.h"

// 命令行参数解析相关定义
extern int optind, opterr, optopt;
extern char *optarg;
static const int LEXICAL_OPT = 256;
static struct option long_options[] = {
    {"help", no_argument, NULL, 'h'},
    {"version", no_argument, NULL, 'v'},
    {"output", required_argument, NULL, 'o'},
    {"lexical", optional_argument, NULL, LEXICAL_OPT},
};

Init::Init() {
  // 初始化目录信息
  abs_path = getcwd(abs_path_buffer, 256);
  abs_path += "/";
  index = 0;
  c = 0;
  return;
}

Init::~Init() { return; }

int Init::init(int &_argc, char **&_argv) {
  while ((c = getopt_long(_argc, _argv, "hvo:", long_options, &index)) != EOF) {
    switch (c) {
    // 显示帮助信息
    case 'h':
      std::cout << "c-sub v0.01\nCopyright(C) Simple-XX 2020\n"
                << "命令格式：[源文件[源文件] -o 输出文件 [选项]][-h|-v]\n"
                << "\t源文件\t\t必须是以.c结尾的文件\n"
                << "\t-o\t\t指定输出文件\n"
                << "\t--lexical[指定文件(可选)]\t显示词法分析过程\n"
                << "\t-h\t\t显示帮助信息\n"
                << "\t-v\t\t显示版本信息" << std::endl;
      break;
    // 显示版本信息
    case 'v':
      std::cout << "c-sub v0.01\nCopyright(C) Simple-XX 2020\n"
                << "简单的 C 语言子集编译器" << std::endl;
      break;
    case 'o':
      // 寻找源文件，寻找区间 _argv[1:-2]
      for (int i = 1; i < _argc - 2; i++) {
        // 添加源文件
        if (strstr(_argv[i], ".c")) {
          src_files.push_back(abs_path + _argv[i]);
        }
      }
      // 设置输出文件
      dest_file = abs_path + optarg;
      break;

    case LEXICAL_OPT:
      std::cout << "输出词法分析结果，可指定输出到文件\n"
                << "[--lexical 输出文件]" << std::endl;
      break;
    // 表示选项不支持
    case '?':
      std::cout << "unknow option" << std::endl;
      break;
    default:
      std::cout << "bikbib" << std::endl;
      break;
    }
  }
  return 0;
}
