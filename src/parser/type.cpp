
/**
 * @file type.cpp
 * @brief 类型处理
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

#include "type.h"

// convert tag to operator

Operator tag_to_op(tag_t _t) {
  switch (_t) {
  case tag_t::ADD:
    return add_op;
  case tag_t::SUB:
    return sub_op;
  case tag_t::MUL:
    return mul_op;
  case tag_t::DIV:
    return div_op;
  case tag_t::MOD:
    return mod_op;
  case tag_t::ORBIT:
    return orbit_op;
  case tag_t::ANDBIT:
    return andbit_op;
  case tag_t::EORBIT:
    return eorbit_op;
  case tag_t::AND:
    return and_op;
  case tag_t::OR:
    return or_op;
  case tag_t::NOT:
    return not_op;
  case tag_t::GT:
    return gt_op;
  case tag_t::GE:
    return ge_op;
  case tag_t::LT:
    return lt_op;
  case tag_t::LE:
    return le_op;
  case tag_t::EQU:
    return equ_op;
  case tag_t::NEQU:
    return nequ_op;
  default:
    return ERROR;
  }
}

std::string type_to_string(Type _t) {
  switch (_t) {
  case Type::int_t:
    return "INT";
  case Type::char_t:
    return "CHAR";
  case Type::void_t:
    return "VOID";
  default:
    return "ERROR";
  }
}

std::string vartype_to_string(VarType _t) {
  switch (_t) {
  case VarType::var_t:
    return "var";
  case VarType::array_t:
    return "array";
  default:
    return "ERROR";
  }
}

std::string op_to_string(Operator _p) {
  switch (_p) {
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