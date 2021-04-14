
// This file is a part of Simple-XX/SimpleCompiler
// (https://github.com/Simple-XX/SimpleCompiler).
//
// error.h for Simple-XX/SimpleCompiler.

#ifndef _ERROR_H_
#define _ERROR_H_

#include "iostream"
#include "string"

using namespace std;

class Pos {
public:
    Pos(unsigned int l, unsigned int c);
    ~Pos(void);
    // 保存当前行号
    unsigned int line;
    // 保存当前列号
    unsigned int col;
};

// 错误处理
class Error {
private:
    // 保存当前文件名
    const string filename;
    // 保存当前错误号
    int err_no;
    // 保存错误位置
    Pos *pos;

public:
    Error(const string &f);
    virtual ~Error();
    void         set_line(unsigned int l);
    void         set_col(unsigned int c);
    void         set_err_no(int e);
    int          get_err_no(void) const;
    Pos *        get_pos(void) const;
    virtual void display_err(void) const;
};

#endif /* _ERROR_H_ */
