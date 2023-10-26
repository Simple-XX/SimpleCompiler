
/**
 * @file irgen.cpp
 * @brief ir 生成
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

#include <functional>
#include <vector>

#include "ast.h"
#include "irgen.h"

void IRGenerator::GenerateValue(const std::string &varName, int &idx, int indx,
                                InitValAST *init, std::vector<int> dim, int i,
                                std::string &code) {
  int elem = 1;
  for (int j = i + 1; j < (int)dim.size(); j++) {
    elem *= dim[j];
  }
  if (init) {
    int i_idx = 0;
    int index = 0;
    for (const auto &initval : init->getValues()) {
      if (dynamic_cast<NumAST *>(initval.get())) {
        index++;
        if (index == elem) {
          index = 0;
          i_idx++;
        }
        for (int j = 0; j < currentDepth; j++) {
          code += "\t";
        }
        code +=
            (varName + "[" + std::to_string(idx * 4) + "] = " +
             std::to_string(dynamic_cast<NumAST *>(initval.get())->getVal()) +
             "\n");
        idx++;
      } else if (dynamic_cast<InitValAST *>(initval.get())) {
        if (dynamic_cast<InitValAST *>(initval.get())->getType() ==
            VarType::var_t) {
          if (dynamic_cast<NumAST *>(dynamic_cast<InitValAST *>(initval.get())
                                         ->getValues()[0]
                                         .get())) {
            index++;
            if (index == elem) {
              index = 0;
              i_idx++;
            }
            for (int j = 0; j < currentDepth; j++) {
              code += "\t";
            }
            code +=
                (varName + "[" + std::to_string(idx * 4) + "] = " +
                 std::to_string(dynamic_cast<NumAST *>(
                                    dynamic_cast<InitValAST *>(initval.get())
                                        ->getValues()[0]
                                        .get())
                                    ->getVal()) +
                 "\n");
            idx++;
          } else {
            std::string res = dynamic_cast<InitValAST *>(initval.get())
                                  ->getValues()[0]
                                  ->GenerateIR(*this, code);
            for (int j = 0; j < currentDepth; j++) {
              code += "\t";
            }
            code += ("t" + std::to_string(t_num) + " = " + res + "\n");
            res = "t" + std::to_string(t_num);
            t_num++;
            index++;
            if (index == elem) {
              index = 0;
              i_idx++;
            }
            for (int j = 0; j < currentDepth; j++) {
              code += "\t";
            }
            code +=
                (varName + "[" + std::to_string(idx * 4) + "] = " + res + "\n");
            idx++;
          }
        } else {
          i_idx++;
          GenerateValue(varName, idx, 0,
                        dynamic_cast<InitValAST *>(initval.get()), dim, i + 1,
                        code);
        }
      }
    }
    for (int j = i_idx; j < dim[i]; j++) {
      GenerateValue(varName, idx, index, nullptr, dim, i + 1, code);
    }
  } else {
    if (i == (int)dim.size()) {
      for (int k = 0; k < currentDepth; k++) {
        code += "\t";
      }
      code += (varName + "[" + std::to_string(idx * 4) + "] = 0\n");
      idx++;
    } else if (i == (int)dim.size() - 1) {
      for (int j = indx; j < dim[i]; j++) {
        for (int k = 0; k < currentDepth; k++) {
          code += "\t";
        }
        code += (varName + "[" + std::to_string(idx * 4) + "] = 0\n");
        idx++;
      }
    } else {
      for (int j = indx; j < dim[i]; j++) {
        GenerateValue(varName, idx, 0, nullptr, dim, i + 1, code);
      }
    }
  }
}

std::string IRGenerator::op2char(Operator op) {
  std::string c;
  switch (op) {
  case Operator::add_op:
    c = "+";
    break;
  case Operator::sub_op:
    c = "-";
    break;
  case Operator::div_op:
    c = "/";
    break;
  case Operator::mod_op:
    c = "%";
    break;
  case Operator::mul_op:
    c = "*";
    break;
  case Operator::and_op:
    c = "&&";
    break;
  case Operator::equ_op:
    c = "==";
    break;
  case Operator::ge_op:
    c = ">=";
    break;
  case Operator::nequ_op:
    c = "!=";
    break;
  case Operator::le_op:
    c = "<=";
    break;
  case Operator::lt_op:
    c = "<";
    break;
  case Operator::gt_op:
    c = ">";
    break;
  case Operator::or_op:
    c = "||";
    break;
  case Operator::ERROR:
    c = "";
    break;
  case Operator::not_op:
    c = "!";
    break;
  default:
    break;
  }
  return c;
}

std::string IRGenerator::GenBinaryExp(BinaryAST &exp, std::string &code) {
  std::string t1 = exp.getLHS()->GenerateIR(*this, code);
  for (int j = 0; j < currentDepth; j++) {
    code += "\t";
  }
  code += ("t" + std::to_string(t_num) + " = " + t1 + "\n");
  t1 = "t" + std::to_string(t_num);
  t_num++;
  std::string t2 = exp.getRHS()->GenerateIR(*this, code);
  for (int j = 0; j < currentDepth; j++) {
    code += "\t";
  }
  code += ("t" + std::to_string(t_num) + " = " + t2 + "\n");
  t2 = "t" + std::to_string(t_num);
  t_num++;
  std::string res = "t" + std::to_string(t_num++);
  for (int j = 0; j < currentDepth; j++) {
    code += "\t";
  }
  code += (res + " = " + t1 + " " + op2char(exp.getOp()) + " " + t2 + "\n");
  return res;
}

std::string IRGenerator::GenNumber(NumAST &num, std::string &code) {
  return std::to_string(num.getVal());
}

std::string IRGenerator::GenVarDef(VarDefAST &varDef, std::string &code) {
  std::string var = varDef.getVar()->GenerateIR(*this, code);
  if (varDef.getInitVal()) {
    varDef.getInitVal()->GenerateIR(*this, code);
  } else {
    if (dynamic_cast<ProcessedIdAST *>(varDef.getVar().get())->getType() ==
        VarType::var_t) {
      if (currentFunc.empty()) {
        for (int i = 0; i < currentDepth; i++) {
          code += "\t";
        }
        code += (var + " = 0\n");
      }
    } else {
      if (currentFunc.empty()) {
        int idx = 0;
        GenerateValue(
            var, idx, 0, nullptr,
            dynamic_cast<ProcessedIdAST *>(varDef.getVar().get())->getDim(), 0,
            code);
      }
    }
  }
  return {};
}

std::string IRGenerator::GenId(ProcessedIdAST &id, std::string &code) {
  std::map<std::string, GenVar>::iterator iter;
  int tmpCurrentBlock = currentBlock;
  while (tmpCurrentBlock != -1) {
    iter = BlockSymbolTable[tmpCurrentBlock].find(id.getName());
    if (iter != BlockSymbolTable[tmpCurrentBlock].end()) {
      break;
    }
    tmpCurrentBlock = parentBlock[tmpCurrentBlock];
  }
  if (iter->second.id.empty()) {
    iter->second.id = "T" + std::to_string(T_num++);
    ReverseSymbolTable.push_back(iter->second);
  }
  return iter->second.id;
}

std::string IRGenerator::GenInitVal(InitValAST &init, std::string &code) {
  if (init.getType() == VarType::var_t) {
    std::string res = init.getValues()[0]->GenerateIR(*this, code);
    for (int i = 0; i < currentDepth; i++) {
      code += "\t";
    }
    code += ("T" + std::to_string(T_num - 1) + " = " + res + "\n");
  } else {
    int idx = 0;
    GenerateValue("T" + std::to_string(T_num - 1), idx, 0, &init,
                  init.getDims(), 0, code);
  }
  return "T" + std::to_string(T_num - 1);
}

std::string IRGenerator::GenAssign(AssignAST &assign, std::string &code) {
  std::string l = assign.getLeft()->GenerateIR(*this, code);
  std::string r = assign.getRight()->GenerateIR(*this, code);
  for (int j = 0; j < currentDepth; j++) {
    code += "\t";
  }
  code += ("t" + std::to_string(t_num) + " = " + r + "\n");
  r = "t" + std::to_string(t_num);
  t_num++;
  for (int i = 0; i < currentDepth; i++) {
    code += "\t";
  }
  code += (l + " = " + r + "\n");
  return l;
}

void IRGenerator::GenVarDecl(VarDeclAST &varDecl, std::string &code) {
  for (const auto &varDef : varDecl.getVarDefs()) {
    varDef->GenerateIR(*this, code);
  }
}

void IRGenerator::GenCompUnit(CompUnitAST &unit, std::string &code) {
  std::string str;
  int tmp = T_num;
  parentBlock.push_back(-1);
  for (const auto &node : unit.getNodes()) {
    if (dynamic_cast<VarDeclAST *>(node.get())) {
      currentDepth += 1;
      node->GenerateIR(*this, str);
      currentDepth -= 1;
    }
  }
  for (int i = tmp; i < T_num; i++) {
    if (ReverseSymbolTable[i].argType == VarType::array_t) {
      std::vector<int> dim = ReverseSymbolTable[i].dims;
      int size = 4;
      for (int d : dim) {
        size *= d;
      }
      for (int j = 0; j < currentDepth; j++) {
        code += "\t";
      }
      code += ("var " + std::to_string(size) + " T" + std::to_string(i) + "\n");
    } else {
      for (int j = 0; j < currentDepth; j++) {
        code += "\t";
      }
      code += ("var T" + std::to_string(i) + "\n");
    }
  }
  for (const auto &node : unit.getNodes()) {
    if (!dynamic_cast<VarDeclAST *>(node.get())) {
      if (dynamic_cast<FuncDefAST *>(node.get()) &&
          dynamic_cast<FuncDefAST *>(node.get())->getName() == "main") {
        code += "f_main [0]\n";
        code += str;
      }
      node->GenerateIR(*this, code);
    }
  }
}

std::string IRGenerator::GenFuncCall(FuncCallAST &func, std::string &code) {
  std::vector<std::string> args;
  for (const auto &arg : func.getArgs()) {
    std::string res = arg->GenerateIR(*this, code);
    for (int j = 0; j < currentDepth; j++) {
      code += "\t";
    }
    code += ("t" + std::to_string(t_num) + " = " + res + "\n");
    res = "t" + std::to_string(t_num);
    t_num++;
    args.push_back(res);
  }
  for (const auto &res : args) {
    for (int j = 0; j < currentDepth; j++) {
      code += "\t";
    }
    code += ("param " + res + "\n");
  }
  if (FuncTable[func.getName()].funcType == Type::void_t) {
    for (int j = 0; j < currentDepth; j++) {
      code += "\t";
    }
    code += ("call f_" + func.getName() + "\n");
    return {};
  } else {
    for (int j = 0; j < currentDepth; j++) {
      code += "\t";
    }
    code +=
        ("t" + std::to_string(t_num) + " = call f_" + func.getName() + "\n");
    std::string res = "t" + std::to_string(t_num);
    t_num++;
    return res;
  }
}

std::string IRGenerator::GenUnaryExp(UnaryAST &exp, std::string &code) {
  std::string ret;
  if (exp.getOp() != Operator::add_op) {
    ret += op2char(exp.getOp());
  }
  std::string res = exp.getNode()->GenerateIR(*this, code);
  for (int j = 0; j < currentDepth; j++) {
    code += "\t";
  }
  code += ("t" + std::to_string(t_num) + " = " + res + "\n");
  res = "t" + std::to_string(t_num);
  t_num++;
  ret += res;
  if (exp.getOp() != Operator::ERROR) {
    for (int j = 0; j < currentDepth; j++) {
      code += "\t";
    }
    code += ("t" + std::to_string(t_num) + " = " + ret + "\n");
    ret = "t" + std::to_string(t_num);
    t_num++;
  }
  return ret;
}

std::string IRGenerator::GenLVal(LValAST &lval, std::string &code) {
  std::map<std::string, GenVar>::iterator iter;
  int tmpCurrentBlock = currentBlock;
  while (tmpCurrentBlock != -1) {
    iter = BlockSymbolTable[tmpCurrentBlock].find(lval.getName());
    if (iter != BlockSymbolTable[tmpCurrentBlock].end() &&
        !iter->second.id.empty()) {
      break;
    }
    tmpCurrentBlock = parentBlock[tmpCurrentBlock];
  }
  if (lval.getType() == VarType::var_t) {
    return iter->second.id;
  } else {
    std::string name = iter->second.id;
    std::vector<int> dim = iter->second.dims;
    int tmp;
    for (size_t i = 0; i < lval.getPosition().size(); i++) {
      std::string var = lval.getPosition()[i]->GenerateIR(*this, code);
      for (int j = 0; j < currentDepth; j++) {
        code += "\t";
      }
      code += ("t" + std::to_string(t_num) + " = " + var + "\n");
      var = "t" + std::to_string(t_num);
      t_num++;
      if (i < lval.getPosition().size() - 1) {
        for (int j = 0; j < currentDepth; j++) {
          code += "\t";
        }
        code += ("t" + std::to_string(t_num) + " = " + var + " * " +
                 std::to_string(dim[i + 1]) + "\n");
        for (int k = i + 2; k < lval.getPosition().size(); k++) {
          for (int j = 0; j < currentDepth; j++) {
            code += "\t";
          }
          code +=
              ("t" + std::to_string(t_num) + " = " + "t" +
               std::to_string(t_num) + " * " + std::to_string(dim[k]) + "\n");
        }
      } else {
        for (int j = 0; j < currentDepth; j++) {
          code += "\t";
        }
        code += ("t" + std::to_string(t_num) + " = " + var + "\n");
      }
      if (i > 0) {
        for (int j = 0; j < currentDepth; j++) {
          code += "\t";
        }
        code += ("t" + std::to_string(t_num) + " = t" + std::to_string(tmp) +
                 " + t" + std::to_string(t_num) + "\n");
      }
      tmp = t_num++;
    }
    for (int j = 0; j < currentDepth; j++) {
      code += "\t";
    }
    code +=
        ("t" + std::to_string(tmp) + " = 4 * t" + std::to_string(tmp) + "\n");
    std::string res = name + "[t" + std::to_string(tmp) + "]";
    return res;
  }
}

void IRGenerator::GenFuncDef(FuncDefAST &funcDef, std::string &code) {
  currentFunc = funcDef.getName();
  for (size_t i = 0; i < funcDef.getArgs().size(); i++) {
    BlockSymbolTable[parentBlock.size()]
                    [dynamic_cast<ProcessedIdAST *>(funcDef.getArgs()[i].get())
                         ->getName()]
                        .id = "p" + std::to_string(i);
  }
  if (funcDef.getName() != "main") {
    code += ("f_" + funcDef.getName() + " [" +
             std::to_string(funcDef.getArgs().size()) + "]\n");
  }
  int T_tmp = T_num;
  int t_tmp = t_num;
  std::string code2;
  funcDef.getBody()->GenerateIR(*this, code2);
  for (int i = T_tmp; i < T_num; i++) {
    if (ReverseSymbolTable[i].argType == VarType::var_t) {
      for (int j = 0; j < currentDepth + 1; j++) {
        code += "\t";
      }
      code += ("var T" + std::to_string(i) + "\n");
    } else {
      std::vector<int> dim = ReverseSymbolTable[i].dims;
      int size = 4;
      for (int d : dim) {
        size *= d;
      }
      for (int j = 0; j < currentDepth + 1; j++) {
        code += "\t";
      }
      code += ("var " + std::to_string(size) + " " + ReverseSymbolTable[i].id +
               "\n");
    }
  }
  for (int i = t_tmp; i < t_num; i++) {
    for (int j = 0; j < currentDepth + 1; j++) {
      code += "\t";
    }
    code += ("var t" + std::to_string(i) + "\n");
  }
  code += code2;
  for (int j = 0; j < currentDepth + 1; j++) {
    code += "\t";
  }
  code += "return";
  code += (funcDef.getType() == Type::int_t ? " 0\n" : " \n");
  code += "\nend f_" + funcDef.getName() + "\n";
}

void IRGenerator::GenStmt(StmtAST &stmt, std::string &code) {
  if (stmt.getStmt()) {
    stmt.getStmt()->GenerateIR(*this, code);
  }
}

void IRGenerator::GenBlock(BlockAST &block, std::string &code) {
  ++currentDepth;
  parentBlock.push_back(currentBlock);
  currentBlock = parentBlock.size() - 1;
  for (const auto &stmt : block.getStmts()) {
    stmt->GenerateIR(*this, code);
  }
  --currentDepth;
  currentBlock = parentBlock[currentBlock];
}

void IRGenerator::GenIfElse(IfAST &stmt, std::string &code) {
  std::string cond = stmt.getCond()->GenerateIR(*this, code);
  int tmp1 = l_num;
  if (cond.find("[") != std::string::npos) {
    for (int j = 0; j < currentDepth; j++) {
      code += "\t";
    }
    code += ("t" + std::to_string(t_num) + " = " + cond + "\n");
    cond = "t" + std::to_string(t_num);
    t_num++;
  }
  if (!(cond[0] >= 'a' && cond[0] <= 'z')) {
    for (int j = 0; j < currentDepth; j++) {
      code += "\t";
    }
    code += ("t" + std::to_string(t_num) + " = " + cond + "\n");
    cond = "t" + std::to_string(t_num);
    t_num++;
  }
  for (int j = 0; j < currentDepth; j++) {
    code += "\t";
  }
  code += ("if " + cond + " == 0 goto l" + std::to_string(tmp1) + "\n");
  l_num++;
  stmt.getThenStmt()->GenerateIR(*this, code);
  if (stmt.getElseStmt()) {
    int tmp = tmp1;
    std::string branch;
    stmt.getElseStmt()->GenerateIR(*this, branch);
    for (int j = 0; j < currentDepth + 1; j++) {
      code += "\t";
    }
    code += ("goto l" + std::to_string(l_num) + "\n");
    code += ("l" + std::to_string(tmp) + ":\n");
    code += branch;
    code += ("l" + std::to_string(l_num) + ":\n");
    l_num++;
  } else {
    code += ("l" + std::to_string(tmp1) + ":\n");
  }
}

void IRGenerator::GenWhile(WhileAST &stmt, std::string &code) {
  int old_break = cur_break_l;
  int old_continue = cur_continue_l;

  int begin_loop = l_num;
  code += ("l" + std::to_string(begin_loop) + ":\n");
  l_num++;
  std::string cond = stmt.getCond()->GenerateIR(*this, code);
  int out_loop = l_num;
  l_num++;

  cur_break_l = out_loop;
  cur_continue_l = begin_loop;

  for (int j = 0; j < currentDepth + 1; j++) {
    code += "\t";
  }
  code += ("if " + cond + " == 0 goto l" + std::to_string(out_loop) + "\n");
  stmt.getStmt()->GenerateIR(*this, code);
  for (int j = 0; j < currentDepth + 1; j++) {
    code += "\t";
  }
  code += ("goto l" + std::to_string(begin_loop) + "\n");
  code += ("l" + std::to_string(out_loop) + ":\n");

  cur_break_l = old_break;
  cur_continue_l = old_continue;
}

void IRGenerator::GenControl(ControlAST &stmt, std::string &code) {
  switch (stmt.getControl()) {
  case Control::continue_c: {
    for (int j = 0; j < currentDepth; j++) {
      code += "\t";
    }
    code += ("goto l" + std::to_string(cur_continue_l) + "\n");
    break;
  }
  case Control::break_c: {
    for (int j = 0; j < currentDepth; j++) {
      code += "\t";
    }
    code += ("goto l" + std::to_string(cur_break_l) + "\n");
    break;
  }
  case Control::return_c: {
    if (stmt.getReturnExp()) {
      std::string ret = stmt.getReturnExp()->GenerateIR(*this, code);
      for (int j = 0; j < currentDepth; j++) {
        code += "\t";
      }
      code += ("t" + std::to_string(t_num) + " = " + ret + "\n");
      ret = ("t" + std::to_string(t_num));
      t_num++;
      for (int j = 0; j < currentDepth; j++) {
        code += "\t";
      }
      code += ("return " + ret + "\n");
    } else {
      for (int j = 0; j < currentDepth; j++) {
        code += "\t";
      }
      code += ("return \n");
    }
  }
  default:
    break;
  }
}

std::string IRGenerator::GenLAndExp(BinaryAST &exp, std::string &code) {
  std::string t1 = exp.getLHS()->GenerateIR(*this, code);
  for (int j = 0; j < currentDepth; j++) {
    code += "\t";
  }
  code += ("t" + std::to_string(t_num) + " = " + t1 + "\n");
  t1 = "t" + std::to_string(t_num);
  t_num++;
  int shorthand = l_num;
  for (int j = 0; j < currentDepth; j++) {
    code += "\t";
  }
  code += ("if " + t1 + " == 0 goto l" + std::to_string(l_num++) + "\n");
  std::string t2 = exp.getRHS()->GenerateIR(*this, code);
  for (int j = 0; j < currentDepth; j++) {
    code += "\t";
  }
  code += ("t" + std::to_string(t_num) + " = " + t2 + "\n");
  t2 = "t" + std::to_string(t_num);
  t_num++;
  code += ("goto l" + std::to_string(l_num++) + "\n");
  code += ("l" + std::to_string(shorthand) + ":\n");
  for (int j = 0; j < currentDepth; j++) {
    code += "\t";
  }
  code += (t2 + " = 0\n");
  for (int j = 0; j < currentDepth; j++) {
    code += "\t";
  }
  code += ("l" + std::to_string(l_num - 1) + ":\n");
  return t2;
}

std::string IRGenerator::GenLOrExp(BinaryAST &exp, std::string &code) {
  std::string t1 = exp.getLHS()->GenerateIR(*this, code);
  for (int j = 0; j < currentDepth; j++) {
    code += "\t";
  }
  code += ("t" + std::to_string(t_num) + " = " + t1 + "\n");
  t1 = "t" + std::to_string(t_num);
  t_num++;
  int shorthand = l_num;
  for (int j = 0; j < currentDepth; j++) {
    code += "\t";
  }
  code += ("if " + t1 + " == 1 goto l" + std::to_string(l_num++) + "\n");
  std::string t2 = exp.getRHS()->GenerateIR(*this, code);
  for (int j = 0; j < currentDepth; j++) {
    code += "\t";
  }
  code += ("t" + std::to_string(t_num) + " = " + t2 + "\n");
  t2 = "t" + std::to_string(t_num);
  t_num++;
  code += ("goto l" + std::to_string(l_num++) + "\n");
  code += ("l" + std::to_string(shorthand) + ":\n");
  for (int j = 0; j < currentDepth; j++) {
    code += "\t";
  }
  code += (t2 + " = 1\n");
  code += ("l" + std::to_string(l_num - 1) + ":\n");
  return t2;
}

std::string FuncDefAST::GenerateIR(IRGenerator &gen, std::string &code) {
  gen.GenFuncDef(*this, code);
  return {};
}

std::string BlockAST::GenerateIR(IRGenerator &gen, std::string &code) {
  gen.GenBlock(*this, code);
  return {};
}

std::string BinaryAST::GenerateIR(IRGenerator &gen, std::string &code) {
  if (this->getOp() == Operator::and_op) {
    return gen.GenLAndExp(*this, code);
  } else if (this->getOp() == Operator::or_op) {
    return gen.GenLOrExp(*this, code);
  } else {
    return gen.GenBinaryExp(*this, code);
  }
}

std::string IfAST::GenerateIR(IRGenerator &gen, std::string &code) {
  gen.GenIfElse(*this, code);
  return {};
}

std::string WhileAST::GenerateIR(IRGenerator &gen, std::string &code) {
  gen.GenWhile(*this, code);
  return {};
}

std::string NumAST::GenerateIR(IRGenerator &gen, std::string &code) {
  return gen.GenNumber(*this, code);
}

std::string IdAST::GenerateIR(IRGenerator &gen, std::string &code) {
  return this->getName();
}

std::string ProcessedIdAST::GenerateIR(IRGenerator &gen, std::string &code) {
  return gen.GenId(*this, code);
}

std::string UnaryAST::GenerateIR(IRGenerator &gen, std::string &code) {
  return gen.GenUnaryExp(*this, code);
}

std::string ControlAST::GenerateIR(IRGenerator &gen, std::string &code) {
  gen.GenControl(*this, code);
  return {};
}

std::string AssignAST::GenerateIR(IRGenerator &gen, std::string &code) {
  return gen.GenAssign(*this, code);
}

std::string StmtAST::GenerateIR(IRGenerator &gen, std::string &code) {
  gen.GenStmt(*this, code);
  return {};
}

std::string LValAST::GenerateIR(IRGenerator &gen, std::string &code) {
  return gen.GenLVal(*this, code);
}

std::string FuncCallAST::GenerateIR(IRGenerator &gen, std::string &code) {
  return gen.GenFuncCall(*this, code);
}

std::string VarDeclAST::GenerateIR(IRGenerator &gen, std::string &code) {
  gen.GenVarDecl(*this, code);
  return {};
}

std::string VarDefAST::GenerateIR(IRGenerator &gen, std::string &code) {
  return gen.GenVarDef(*this, code);
}

std::string InitValAST::GenerateIR(IRGenerator &gen, std::string &code) {
  return gen.GenInitVal(*this, code);
}

std::string EmptyAST::GenerateIR(IRGenerator &gen, std::string &code) {
  return {};
}

std::string CompUnitAST::GenerateIR(IRGenerator &gen, std::string &code) {
  gen.GenCompUnit(*this, code);
  return {};
}