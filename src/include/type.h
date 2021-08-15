
// This file is a part of Simple-XX/SimpleCompiler
// (https://github.com/Simple-XX/SimpleCompiler).
//
// type.h for Simple-XX/SimpleCompiler.

#ifndef _TYPE_H_
#define _TYPE_H_

#include "token.h"

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
    orbit_op,
    andbit_op,
    eorbit_op,
    and_op,
    or_op,
    not_op,
    gt_op,
    ge_op,
    lt_op,
    le_op,
    equ_op,
    nequ_op,
    ERROR
};

enum Control {
    continue_c,
    break_c,
    return_c
};

Operator tag_to_op(Tag t);

string op_to_string(Operator p);

string type_to_string(Type t);

string vartype_to_string(VarType t);
#endif /* _TYPE_H_ */
