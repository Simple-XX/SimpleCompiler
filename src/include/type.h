
/**
 * @file type.h
 * @brief type
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

#ifndef _TYPE_H_
#define _TYPE_H_

#include "token.h"

using namespace std;

enum Type { int_t, char_t, void_t };

enum VarType { array_t, var_t };

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

enum Control { continue_c, break_c, return_c };

Operator tag_to_op(Tag t);

string op_to_string(Operator p);

string type_to_string(Type t);

string vartype_to_string(VarType t);
#endif /* _TYPE_H_ */
