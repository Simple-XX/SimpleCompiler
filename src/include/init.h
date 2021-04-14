
// This file is a part of Simple-XX/SimpleCompiler
// (https://github.com/Simple-XX/SimpleCompiler).
//
// init.h for Simple-XX/SimpleCompiler.

#ifndef _INIT_H_
#define _INIT_H_

#include "string"
#include "vector"

using namespace std;

// 源文件
extern std::vector<std::string> src_files;
// 输出文件
extern string dest_file;

class Init {
private:
    // 绝对路径
    string abs_path;
    // 路径缓存大小
    static const int PATH_BUFFER = 1024;
    // 路径缓存
    char abs_path_buffer[PATH_BUFFER];
    // 用于接收选项
    int index;
    int c;

public:
    Init(void);
    ~Init(void);
    int init(int &argc, char **&argv);
};

#endif /* _INIT_H_ */
