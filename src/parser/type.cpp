// This file is a part of Simple-XX/SimpleCompiler
// (https://github.com/Simple-XX/SimpleCompiler).
//
// type.cpp for Simple-XX/SimpleCompiler.


#include <type.h>

// convert tag to operator

Operator tag_to_op(Tag t) {
    switch (t) 
    {
        case Tag::ADD:
            return add_op;
        case Tag::SUB:
            return sub_op;
        case Tag::MUL:
            return mul_op;
        case Tag::DIV:
            return div_op;
        case Tag::MOD:
            return mod_op;
        case Tag::ORBIT:
            return orbit_op;
        case Tag::ANDBIT:
            return andbit_op;
        case Tag::EORBIT:
            return eorbit_op;
        case Tag::AND:
            return and_op;
        case Tag::OR:
            return or_op;
        case Tag::NOT:
            return not_op;
        case Tag::GT:
            return gt_op;
        case Tag::GE:
            return ge_op;
        case Tag::LT:
            return lt_op;
        case Tag::LE:
            return le_op;
        case Tag::EQU:
            return equ_op;
        case Tag::NEQU:
            return nequ_op;
        default:
            return ERROR;
        }
}

string op_to_string(Operator p) {
    switch (p) 
    {
        case Operator::add_op:
            return "add";
        case Operator::sub_op:
            return "sub";
        case Operator::mul_op:
            return "mul";
        case Operator::div_op:
            return "div";
        case Operator::mod_op:
            return "mod";
        case Operator::orbit_op:
            return "orbit";
        case Operator::andbit_op:
            return "andbit";
        case Operator::eorbit_op:
            return "eorbit";
        case Operator::and_op:
            return "and";
        case Operator::or_op:
            return "or";
        case Operator::not_op:
            return "not";
        case Operator::gt_op:
            return "gt";
        case Operator::ge_op:
            return "ge";
        case Operator::lt_op:
            return "lt";
        case Operator::le_op:
            return "le";
        case Operator::equ_op:
            return "equ";
        case Operator::nequ_op:
            return "nequ";
        default:
            exit(97);
        }
}