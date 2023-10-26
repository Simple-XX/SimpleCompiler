
/**
 * @file ir.h
 * @brief ir
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

#ifndef SIMPLECOMPILER_IR_H
#define SIMPLECOMPILER_IR_H

enum IRToken {
  VARDECL_IR,
  NUMBER_IR,
  SYMBOL_IR,
  LSB_IR,
  RSB_IR,
  ASSIGN_IR,
  FUNCEND_IR,
  END_IR,
  GOTO_IR,
  IF_IR,
  PARAM_IR,
  CALL_IR,
  RETURN_IR,
  OP_IR,
  COLON_IR,
  LOGICOP_IR,
  COMMENT_IR
};

#endif /* SIMPLECOMPILER_IR_H */