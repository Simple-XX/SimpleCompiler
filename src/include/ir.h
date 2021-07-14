
// This file is a part of Simple-XX/SimpleCompiler
// (https://github.com/Simple-XX/SimpleCompiler).
//
// ir.h for Simple-XX/SimpleCompiler.

#ifndef _IR_H_
#define _IR_H_

// 操作类型
enum Operator {
    // 占位指令,默认值
    OP_NOP,
    // 声明指令
    OP_DEC, // eg: DEC arg1 => (int/char */[]) arg1
    // 函数入口和出口
    OP_ENTRY,
    OP_EXIT,
    // 赋值运算
    OP_AS, // 赋值 eg: AS result,arg1 => result=arg1
    // 算数运算
    OP_ADD,
    OP_SUB,
    OP_MUL,
    OP_DIV,
    OP_MOD, // 加减乘除模 eg: ADD result,arg1,arg2 => result=arg1 + arg2
    OP_NEG, // 负 eg: NEG result,arg1 => result = -arg1
    // 比较运算
    OP_GT,
    OP_GE,
    OP_LT,
    OP_LE,
    OP_EQU,
    OP_NE, // 大小等 eg: GT result,arg1,arg2 => result=arg1 > arg2
    // 逻辑运算
    OP_NOT, // 非 eg: NOT result,arg1 => result=!arg1
    OP_AND,
    OP_OR, // 与或 eg: AND result,arg1,arg2 => result=arg1 && arg2
    // 数组运算
    // OP_INDL,//索引作为左值 eg: INDL result,arg1,arg2 => arg1[arg2]=result
    // OP_INDR,//索引作为右值 eg: INDR result,arg1,arg2 => result=arg1[arg2]
    // 指针运算
    OP_LEA, // 取址 eg: LEA result,arg1 => result=&arg1
    OP_SET, // 设置左值 eg: SET result,arg1 => *arg1=result
    OP_GET, // 取右值 eg: GET result,arg1 => result=*arg1
    // 跳转
    OP_JMP, // 无条件跳转 eg: JMP result => goto result
    OP_JT,  // 真跳转	 eg: JT result,arg1 => if(arg1) goto result
    OP_JF,  // 假跳转	 eg: JF result,arg1 => if(!arg1) goto result
    /*OP_JG,OP_JGE,OP_JL,OP_JLE,OP_JE,*/ OP_JNE, // 跳转 eg:JG result,arg1,arg2
                                                 // => if(arg1 > arg2) goto
                                                 // result
    // 函数调用
    OP_ARG,  // 参数传递 eg: ARG arg1 => 传递参数arg1
    OP_PROC, // 调用过程 eg: PROC fun => 调用fun函数,fun()
    OP_CALL, // 调用函数 eg: CALL result,fun => 调用fun函数,返回值result=fun()
    OP_RET, // 直接返回 eg: RET => return
    OP_RETV // 带数据返回 eg:RET arg1 => return arg1
};

#endif /* _IR_H_ */
