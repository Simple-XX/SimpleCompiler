
// This file is a part of Simple-XX/SimpleCompiler
// (https://github.com/Simple-XX/SimpleCompiler).
//
// type.h for Simple-XX/SimpleCompiler.

#ifndef _TYPE_H_
#define _TYPE_H_

using namespace std;

enum Type {
    int_t,
    char_t,
    void_t
};

enum VarType {
    array_t,
    var_t
};

enum Operator {
    add_op,
    sub_op,
    mul_op,
    div_op,
    mod_op,
    orb_op,
    andb_op,
    eorb_op,
    and_op,
    or_op,
    not_op,
    gt_op,
    ge_op,
    lt_op,
    le_op,
    equ_op,
    nequ_op
};

enum Control {
    continue_c,
    break_c,
    return_c
};

#endif /* _TYPE_H_ */
