
// This file is a part of MRNIU/SimpleCompiler (https://github.com/MRNIU/SimpleCompiler).
//
// common.h for MRNIU/SimpleCompiler.

#ifndef _COMMON_H_
#define _COMMON_H_

#include "iostream"
#include "fstream"
#include "string.h"
#include "string"
#include "vector"
#include "unistd.h"
#include "getopt.h"

using namespace std;

// 读入文件流
extern ifstream fin;
// 输出文件流
extern ofstream fout;

// 路径缓存大小
const int PATH_BUFFER = 1024;
// 当前工作目录
extern string abs_path;
// 源文件
extern vector<string> src_files;
// 输出文件
extern string dest_file;

int main(int argc, char * * argv);
int init(int &argc, char * * &argv);
char scanner(ifstream &in);


#endif  /* _COMMON_H_ */
