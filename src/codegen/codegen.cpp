
/**
 * @file codegen.cpp
 * @brief 代码生成
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

#include <cstring>
#include <iostream>
#include <set>
#include <sstream>
#include <string>

#include "codegen.h"

void CodeGen::Generate(std::string &_code) {
  char stmt[1005];
  while (cinstream.getline(stmt, 1005)) {
    std::string str_stmt;
    int init_idx = 0;
    for (; stmt[init_idx] <= 32; init_idx++)
      ;
    int len = strlen(stmt);
    for (; init_idx < len; init_idx++) {
      str_stmt += stmt[init_idx];
    }
    if (str_stmt.empty()) {
      continue;
    }
    std::istringstream stream_stmt(str_stmt);
    std::string first;
    stream_stmt >> first;
    if (first[0] == 'f' && first[1] == '_') {
      std::string func_name = first.substr(2, first.length() - 2);
      int int1 = 0, int2 = 0;
      int idx = 0;
      for (; !(stmt[idx] >= '0' && stmt[idx] <= '9'); idx++)
        ;
      for (; stmt[idx] >= '0' && stmt[idx] <= '9'; idx++) {
        int1 = int1 * 10 + stmt[idx] - '0';
      }
      for (; !(stmt[idx] >= '0' && stmt[idx] <= '9'); idx++)
        ;
      for (; stmt[idx] >= '0' && stmt[idx] <= '9'; idx++) {
        int2 = int2 * 10 + stmt[idx] - '0';
      }
      funcStack[func_name] = 16 * (int2 / 4 + 1);
      if (16 * (int2 / 4 + 1) - 4 >= -2048 && 16 * (int2 / 4 + 1) <= 2047) {
        _code += "\t.text\n\t.align 2\n\t.global " + func_name + "\n\t.type " +
                 func_name + ", @function\n" + func_name +
                 ":\n\taddi sp, sp, -" + std::to_string(16 * (int2 / 4 + 1)) +
                 "\n\tsw ra, " + std::to_string(16 * (int2 / 4 + 1) - 4) +
                 "(sp)\n";
      } else {
        _code += "\t.text\n\t.align 2\n\t.global " + func_name + "\n\t.type " +
                 func_name + ", @function\n" + func_name +
                 ":\n\taddi sp, sp, -4\n\tsw ra, 0(sp)\n\tli s0, " +
                 std::to_string(16 * (int2 / 4 + 1) - 4) +
                 "\n\tsub sp, sp, s0\n";
      }
      currentFunc = func_name;
    } else if (first == "end") {
      std::string func_name;
      stream_stmt >> func_name;
      func_name = func_name.substr(2, func_name.length() - 2);
      _code += "\t.size " + func_name + ", .-" + func_name + "\n";
      currentFunc = "";
    } else if (first[0] == 'v') {
      std::string var_name, eq, init_val;
      var_name = first;
      stream_stmt >> eq >> init_val;
      if (init_val == "malloc") {
        std::string size;
        stream_stmt >> size;
        _code += "\t.comm " + var_name + ", " + size + ", 4\n";
      } else {
        _code += "\t.global " + var_name +
                 "\n\t.section .sdata\n\t.align 2\n\t.type " + var_name +
                 ", @object\n\t.size " + var_name + ", 4\n" + var_name +
                 ":\n\t.word " + init_val + "\n";
      }
    } else if (first == "goto") {
      std::string label;
      stream_stmt >> label;
      _code += "\tj ." + label + "\n";
    } else if (first[0] == 'l' && first[1] >= '0' && first[1] <= '9') {
      _code += "." + first + "\n";
    } else if (first == "call") {
      std::string func_name;
      stream_stmt >> func_name;
      func_name = func_name.substr(2, func_name.length() - 2);
      _code += "\tcall " + func_name + "\n";
    } else if (first == "return") {
      int stk = funcStack[currentFunc];
      if (stk - 4 >= -2048 && stk <= 2047) {
        _code += "\tlw ra, " + std::to_string(stk - 4) +
                 "(sp)\n\taddi sp, sp, " + std::to_string(stk) + "\n\tret\n";
      } else {
        _code += "\tli s0, " + std::to_string(stk - 4) +
                 "\n\tadd sp, sp, s0\n\tlw ra, 0(sp)\naddi sp, sp, 4\n\tret\n";
      }
    } else if (first == "store") {
      std::string reg;
      int int10;
      stream_stmt >> reg >> int10;
      if (int10 >= -512 && int10 <= 511) {
        _code += "\tsw " + reg + ", " + std::to_string(int10 * 4) + "(sp)\n";
      } else {
        _code += "\tli s0, " + std::to_string(int10) + "\n\tadd s0, s0, s0\n" +
                 "\tadd s0, s0, s0\n\tadd sp, sp, s0\n\tsw " + reg +
                 ", 0(sp)\n\tsub sp, sp, s0\n";
      }
    } else if (first == "load") {
      std::string var, reg;
      stream_stmt >> var >> reg;
      if (var[0] == 'v') {
        _code += "\tlui " + reg + ", %hi(" + var + ")\n\tlw " + reg + ", %lo(" +
                 var + ")(" + reg + ")\n";
      } else {
        int int10 = std::stoi(var);
        if (int10 >= -512 && int10 <= 511) {
          _code += "\tlw " + reg + ", " + std::to_string(int10 * 4) + "(sp)\n";
        } else {
          _code += "\tli s0, " + std::to_string(int10) +
                   "\n\tadd s0, s0, s0\n" +
                   "\tadd s0, s0, s0\n\tadd sp, sp, s0\n\tlw " + reg +
                   ", 0(sp)\n\tsub sp, sp, s0\n";
        }
      }
    } else if (first == "loadaddr") {
      std::string var, reg;
      stream_stmt >> var >> reg;
      if (var[0] == 'v') {
        _code += "\tla " + reg + ", " + var + "\n";
      } else {
        int int10 = std::stoi(var);
        if (int10 >= -512 && int10 <= 511) {
          _code +=
              "\taddi " + reg + ", sp, " + std::to_string(int10 * 4) + "\n";
        } else {
          _code += "\tli s0, " + var +
                   "\n\tadd s0, s0, s0\n\tadd s0, s0, s0\n\tadd " + reg +
                   ", sp, s0\n";
        }
      }
    } else if (first == "if") {
      std::string reg1, reg2, cmp, gt, label;
      stream_stmt >> reg1 >> cmp >> reg2 >> gt >> label;
      if (cmp == "<") {
        _code += "\tblt " + reg1 + ", " + reg2 + ", ." + label + "\n";
      } else if (cmp == ">") {
        _code += "\tbgt " + reg1 + ", " + reg2 + ", ." + label + "\n";
      } else if (cmp == "<=") {
        _code += "\tble " + reg1 + ", " + reg2 + ", ." + label + "\n";
      } else if (cmp == ">=") {
        _code += "\tbge " + reg1 + ", " + reg2 + ", ." + label + "\n";
      } else if (cmp == "!=") {
        _code += "\tbne " + reg1 + ", " + reg2 + ", ." + label + "\n";
      } else if (cmp == "==") {
        _code += "\tbeq " + reg1 + ", " + reg2 + ", ." + label + "\n";
      }
    } else {
      std::string lhs = first;
      std::string token;
      std::string op, var1, var2;
      std::set<std::string> ops{"+",  "-",  "*",  "/",  "%",  "<", ">",
                                "<=", ">=", "&&", "||", "!=", "=="};
      std::string unary_op_1, unary_op_2;
      while (stream_stmt >> token) {
        if (token[0] == '[') {
          if (var1.empty()) {
            lhs += token;
          } else if (var2.empty()) {
            var1 += token;
          } else {
            var2 += token;
          }
          continue;
        }
        if (token == "!") {
          if (var1.empty())
            unary_op_1 = token;
          else if (var2.empty())
            unary_op_2 = token;
          continue;
        }
        if (token == "-") {
          if (var1.empty()) {
            unary_op_1 = token;
            continue;
          }
        }
        if (token == "=")
          continue;
        if (ops.find(token) != ops.end()) {
          op = token;
          continue;
        }
        if (var1.empty()) {
          var1 = token;
          continue;
        }
        if (var2.empty())
          var2 = token;
      }
      var1 = unary_op_1 + var1;
      var2 = unary_op_2 + var2;
      if (op.empty()) {
        if (var1[0] == '!') {
          var1 = var1.substr(1, var1.length() - 1);
          _code += "\tseqz " + lhs + ", " + var1 + "\n";
        } else if (var1[0] == '-') {
          if (var1[1] >= '0' && var1[1] <= '9') {
            _code += "\tli " + lhs + ", " + var1 + "\n";
          } else {
            var1 = var1.substr(1, var1.length() - 1);
            _code += "\tneg " + lhs + ", " + var1 + "\n";
          }
        } else {
          auto llidx = lhs.find("[");
          auto rlidx = var1.find("[");
          if (llidx == std::string::npos && rlidx == std::string::npos) {
            if (!(var1[0] >= 'a' && var1[0] <= 'z')) {
              _code += "\tli " + lhs + ", " + var1 + "\n";
            } else {
              _code += "\tmv " + lhs + ", " + var1 + "\n";
            }
          } else if (llidx == std::string::npos) {
            int rridx = var1.find("]");
            int offset = std::stoi(var1.substr(rlidx + 1, rridx - rlidx - 1));
            if (offset >= -2048 && offset <= 2047) {
              _code += "\tlw " + lhs + ", " + std::to_string(offset) + "(" +
                       var1.substr(0, rlidx) + ")\n";
            } else {
              std::string array = var1.substr(0, rlidx);
              _code += "\tli s0, " + std::to_string(offset) + "\n\tadd " +
                       var1.substr(0, rlidx) + ", " + var1.substr(0, rlidx) +
                       ", s0\n\tlw " + lhs + ", 0(" + var1.substr(0, rlidx) +
                       ")\n\tsub " + var1.substr(0, rlidx) + ", " +
                       var1.substr(0, rlidx) + ", s0\n";
            }
          } else if (rlidx == std::string::npos) {
            int lridx = lhs.find("]");
            int offset = std::stoi(lhs.substr(llidx + 1, lridx - llidx - 1));
            if (offset >= -2048 && offset <= 2047) {
              _code += "\tsw " + var1 + ", " + std::to_string(offset) + "(" +
                       lhs.substr(0, llidx) + ")\n";
            } else {
              _code += "\tli s0, " + std::to_string(offset) + "\n\tadd " +
                       lhs.substr(0, llidx) + ", " + lhs.substr(0, llidx) +
                       ", s0\n\tsw " + var1 + ", 0(" + lhs.substr(0, llidx) +
                       ")\n\tsub " + lhs.substr(0, llidx) + ", " +
                       lhs.substr(0, llidx) + ", s0\n";
            }
          }
        }
      } else {
        if (op == "+") {
          if (!(var2[0] >= 'a' && var2[0] <= 'z')) {
            int int12 = std::stoi(var2);
            if (int12 >= -2048 && int12 <= 2047) {
              _code += "\taddi " + lhs + ", " + var1 + ", " + var2 + "\n";
            } else {
              _code += "\tli s0, " + var2 + "\n\tadd " + lhs + ", " + var1 +
                       ", s0\n";
            }
          } else {
            _code += "\tadd " + lhs + ", " + var1 + ", " + var2 + "\n";
          }
        } else if (op == "-") {
          if (!(var2[0] >= 'a' && var2[0] <= 'z')) {
            _code +=
                "\tli s0, " + var2 + "\n\tsub " + lhs + ", " + var1 + ", s0\n";
          } else {
            _code += "\tsub " + lhs + ", " + var1 + ", " + var2 + "\n";
          }
        } else if (op == "*") {
          if (!(var2[0] >= 'a' && var2[0] <= 'z')) {
            _code +=
                "\tli s0, " + var2 + "\n\tmul " + lhs + ", " + var1 + ", s0\n";
          } else {
            _code += "\tmul " + lhs + ", " + var1 + ", " + var2 + "\n";
          }
        } else if (op == "/") {
          if (!(var2[0] >= 'a' && var2[0] <= 'z')) {
            _code +=
                "\tli s0, " + var2 + "\n\tdiv " + lhs + ", " + var1 + ", s0\n";
          } else {
            _code += "\tdiv " + lhs + ", " + var1 + ", " + var2 + "\n";
          }
        } else if (op == "%") {
          if (!(var2[0] >= 'a' && var2[0] <= 'z')) {
            _code +=
                "\tli s0, " + var2 + "\n\trem " + lhs + ", " + var1 + ", s0\n";
          } else {
            _code += "\trem " + lhs + ", " + var1 + ", " + var2 + "\n";
          }
        } else if (op == "<") {
          if (!(var2[0] >= 'a' && var2[0] <= 'z')) {
            int int12 = std::stoi(var2);
            if (int12 >= -2048 && int12 <= 2047) {
              _code += "\tslti " + lhs + ", " + var1 + ", " + var2 + "\n";
            } else {
              _code += "\tli s0, " + var2 + "\n\tslt " + lhs + ", " + var1 +
                       ", s0\n";
            }
          } else {
            _code += "\tslt " + lhs + ", " + var1 + ", " + var2 + "\n";
          }
        } else if (op == ">") {
          if (!(var2[0] >= 'a' && var2[0] <= 'z')) {
            _code +=
                "\tli s0, " + var2 + "\n\tsgt " + lhs + ", " + var1 + ", s0\n";
          } else {
            _code += "\tsgt " + lhs + ", " + var1 + ", " + var2 + "\n";
          }
        } else if (op == "<=") {
          if (!(var2[0] >= 'a' && var2[0] <= 'z')) {
            _code += "\tli s0, " + var2 + "\n\tsgt " + lhs + ", " + var1 +
                     ", s0\n\tseqz " + lhs + ", " + lhs + "\n";
          } else {
            _code += "\tsgt " + lhs + ", " + var1 + ", " + var2 + "\n\tseqz " +
                     lhs + ", " + lhs + "\n";
          }
        } else if (op == ">=") {
          if (!(var2[0] >= 'a' && var2[0] <= 'z')) {
            _code += "\tli s0, " + var2 + "\n\tslt " + lhs + ", " + var1 +
                     ", s0\n\tseqz " + lhs + ", " + lhs + "\n";
          } else {
            _code += "\tslt " + lhs + ", " + var1 + ", " + var2 + "\n\tseqz " +
                     lhs + ", " + lhs + "\n";
          }
        } else if (op == "&&") {
          if (!(var2[0] >= 'a' && var2[0] <= 'z')) {
            int tmp = std::stoi(var2);
            if (tmp == 0) {
              _code += "\tli t0, " + var2 + "\n\tsnez " + lhs +
                       ", t0\n\tsnez s0, t0\n\tand " + lhs + ", " + lhs +
                       ", s0\n";
            }
          } else {
            _code += "\tsnez " + lhs + ", " + var1 + "\n\tsnez s0, " + var2 +
                     "\n\tand " + lhs + ", " + lhs + ", s0\n";
          }
        } else if (op == "||") {
          if (!(var2[0] >= 'a' && var2[0] <= 'z')) {
            _code += "\tli s0, " + var2 + "\n\tor " + lhs + ", " + var1 +
                     ", t0\n\tsnez " + lhs + ", " + lhs + "\n";
          } else {
            _code += "\tor " + lhs + ", " + var1 + ", " + var2 + "\n\tsnez " +
                     lhs + ", " + lhs + "\n";
          }
        } else if (op == "!=") {
          if (!(var2[0] >= 'a' && var2[0] <= 'z')) {
            _code += "\tli s0, " + var2 + "\n\txor " + lhs + ", " + var1 +
                     ", " + var2 + "\n\tsnez " + lhs + ", " + lhs + "\n";
          } else {
            _code += "\txor " + lhs + ", " + var1 + ", " + var2 + "\n\tsnez " +
                     lhs + ", " + lhs + "\n";
          }
        } else if (op == "==") {
          if (!(var2[0] >= 'a' && var2[0] <= 'z')) {
            _code += "\tli s0, " + var2 + "\n\txor " + lhs + ", " + var1 +
                     ", " + var2 + "\n\tseqz " + lhs + ", " + lhs + "\n";
          } else {
            _code += "\txor " + lhs + ", " + var1 + ", " + var2 + "\n\tseqz " +
                     lhs + ", " + lhs + "\n";
          }
        }
      }
    }
  }
}
