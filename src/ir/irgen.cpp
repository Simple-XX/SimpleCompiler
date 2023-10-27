
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

void IRGenerator::GenerateValue(const std::string &_varName, int &_idx,
                                int _indx, InitValAST *_init,
                                std::vector<int> _dim, int _i,
                                std::string &_code) {
  int elem = 1;
  for (int j = _i + 1; j < (int)_dim.size(); j++) {
    elem *= _dim[j];
  }
  if (_init) {
    int i_idx = 0;
    int index = 0;
    for (const auto &initval : _init->getValues()) {
      if (dynamic_cast<NumAST *>(initval.get())) {
        index++;
        if (index == elem) {
          index = 0;
          i_idx++;
        }
        for (int j = 0; j < currentDepth; j++) {
          _code += "\t";
        }
        _code +=
            (_varName + "[" + std::to_string(_idx * 4) + "] = " +
             std::to_string(dynamic_cast<NumAST *>(initval.get())->getVal()) +
             "\n");
        _idx++;
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
              _code += "\t";
            }
            _code +=
                (_varName + "[" + std::to_string(_idx * 4) + "] = " +
                 std::to_string(dynamic_cast<NumAST *>(
                                    dynamic_cast<InitValAST *>(initval.get())
                                        ->getValues()[0]
                                        .get())
                                    ->getVal()) +
                 "\n");
            _idx++;
          } else {
            std::string res = dynamic_cast<InitValAST *>(initval.get())
                                  ->getValues()[0]
                                  ->GenerateIR(*this, _code);
            for (int j = 0; j < currentDepth; j++) {
              _code += "\t";
            }
            _code += ("t" + std::to_string(t_num) + " = " + res + "\n");
            res = "t" + std::to_string(t_num);
            t_num++;
            index++;
            if (index == elem) {
              index = 0;
              i_idx++;
            }
            for (int j = 0; j < currentDepth; j++) {
              _code += "\t";
            }
            _code += (_varName + "[" + std::to_string(_idx * 4) + "] = " + res +
                      "\n");
            _idx++;
          }
        } else {
          i_idx++;
          GenerateValue(_varName, _idx, 0,
                        dynamic_cast<InitValAST *>(initval.get()), _dim, _i + 1,
                        _code);
        }
      }
    }
    for (int j = i_idx; j < _dim[_i]; j++) {
      GenerateValue(_varName, _idx, index, nullptr, _dim, _i + 1, _code);
    }
  } else {
    if (_i == (int)_dim.size()) {
      for (int k = 0; k < currentDepth; k++) {
        _code += "\t";
      }
      _code += (_varName + "[" + std::to_string(_idx * 4) + "] = 0\n");
      _idx++;
    } else if (_i == (int)_dim.size() - 1) {
      for (int j = _indx; j < _dim[_i]; j++) {
        for (int k = 0; k < currentDepth; k++) {
          _code += "\t";
        }
        _code += (_varName + "[" + std::to_string(_idx * 4) + "] = 0\n");
        _idx++;
      }
    } else {
      for (int j = _indx; j < _dim[_i]; j++) {
        GenerateValue(_varName, _idx, 0, nullptr, _dim, _i + 1, _code);
      }
    }
  }
}

std::string IRGenerator::op2char(Operator _op) {
  std::string c;
  switch (_op) {
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

std::string IRGenerator::GenBinaryExp(BinaryAST &_exp, std::string &_code) {
  std::string t1 = _exp.getLHS()->GenerateIR(*this, _code);
  for (int j = 0; j < currentDepth; j++) {
    _code += "\t";
  }
  _code += ("t" + std::to_string(t_num) + " = " + t1 + "\n");
  t1 = "t" + std::to_string(t_num);
  t_num++;
  std::string t2 = _exp.getRHS()->GenerateIR(*this, _code);
  for (int j = 0; j < currentDepth; j++) {
    _code += "\t";
  }
  _code += ("t" + std::to_string(t_num) + " = " + t2 + "\n");
  t2 = "t" + std::to_string(t_num);
  t_num++;
  std::string res = "t" + std::to_string(t_num++);
  for (int j = 0; j < currentDepth; j++) {
    _code += "\t";
  }
  _code += (res + " = " + t1 + " " + op2char(_exp.getOp()) + " " + t2 + "\n");
  return res;
}

std::string IRGenerator::GenNumber(NumAST &_num, std::string &_code) {
  return std::to_string(_num.getVal());
}

std::string IRGenerator::GenVarDef(VarDefAST &_varDef, std::string &_code) {
  std::string var = _varDef.getVar()->GenerateIR(*this, _code);
  if (_varDef.getInitVal()) {
    _varDef.getInitVal()->GenerateIR(*this, _code);
  } else {
    if (dynamic_cast<ProcessedIdAST *>(_varDef.getVar().get())->getType() ==
        VarType::var_t) {
      if (currentFunc.empty()) {
        for (int i = 0; i < currentDepth; i++) {
          _code += "\t";
        }
        _code += (var + " = 0\n");
      }
    } else {
      if (currentFunc.empty()) {
        int idx = 0;
        GenerateValue(
            var, idx, 0, nullptr,
            dynamic_cast<ProcessedIdAST *>(_varDef.getVar().get())->getDim(), 0,
            _code);
      }
    }
  }
  return {};
}

std::string IRGenerator::GenId(ProcessedIdAST &_id, std::string &_code) {
  std::map<std::string, GenVar>::iterator iter;
  int tmpCurrentBlock = currentBlock;
  while (tmpCurrentBlock != -1) {
    iter = BlockSymbolTable[tmpCurrentBlock].find(_id.getName());
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

std::string IRGenerator::GenInitVal(InitValAST &_init, std::string &_code) {
  if (_init.getType() == VarType::var_t) {
    std::string res = _init.getValues()[0]->GenerateIR(*this, _code);
    for (int i = 0; i < currentDepth; i++) {
      _code += "\t";
    }
    _code += ("T" + std::to_string(T_num - 1) + " = " + res + "\n");
  } else {
    int idx = 0;
    GenerateValue("T" + std::to_string(T_num - 1), idx, 0, &_init,
                  _init.getDims(), 0, _code);
  }
  return "T" + std::to_string(T_num - 1);
}

std::string IRGenerator::GenAssign(AssignAST &_assign, std::string &_code) {
  std::string l = _assign.getLeft()->GenerateIR(*this, _code);
  std::string r = _assign.getRight()->GenerateIR(*this, _code);
  for (int j = 0; j < currentDepth; j++) {
    _code += "\t";
  }
  _code += ("t" + std::to_string(t_num) + " = " + r + "\n");
  r = "t" + std::to_string(t_num);
  t_num++;
  for (int i = 0; i < currentDepth; i++) {
    _code += "\t";
  }
  _code += (l + " = " + r + "\n");
  return l;
}

void IRGenerator::GenVarDecl(VarDeclAST &_varDecl, std::string &_code) {
  for (const auto &varDef : _varDecl.getVarDefs()) {
    varDef->GenerateIR(*this, _code);
  }
}

void IRGenerator::GenCompUnit(CompUnitAST &_unit, std::string &_code) {
  std::string str;
  int tmp = T_num;
  parentBlock.push_back(-1);
  for (const auto &node : _unit.getNodes()) {
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
        _code += "\t";
      }
      _code +=
          ("var " + std::to_string(size) + " T" + std::to_string(i) + "\n");
    } else {
      for (int j = 0; j < currentDepth; j++) {
        _code += "\t";
      }
      _code += ("var T" + std::to_string(i) + "\n");
    }
  }
  for (const auto &node : _unit.getNodes()) {
    if (!dynamic_cast<VarDeclAST *>(node.get())) {
      if (dynamic_cast<FuncDefAST *>(node.get()) &&
          dynamic_cast<FuncDefAST *>(node.get())->getName() == "main") {
        _code += "f_main [0]\n";
        _code += str;
      }
      node->GenerateIR(*this, _code);
    }
  }
}

std::string IRGenerator::GenFuncCall(FuncCallAST &_func, std::string &_code) {
  std::vector<std::string> args;
  for (const auto &arg : _func.getArgs()) {
    std::string res = arg->GenerateIR(*this, _code);
    for (int j = 0; j < currentDepth; j++) {
      _code += "\t";
    }
    _code += ("t" + std::to_string(t_num) + " = " + res + "\n");
    res = "t" + std::to_string(t_num);
    t_num++;
    args.push_back(res);
  }
  for (const auto &res : args) {
    for (int j = 0; j < currentDepth; j++) {
      _code += "\t";
    }
    _code += ("param " + res + "\n");
  }
  if (FuncTable[_func.getName()].funcType == Type::void_t) {
    for (int j = 0; j < currentDepth; j++) {
      _code += "\t";
    }
    _code += ("call f_" + _func.getName() + "\n");
    return {};
  } else {
    for (int j = 0; j < currentDepth; j++) {
      _code += "\t";
    }
    _code +=
        ("t" + std::to_string(t_num) + " = call f_" + _func.getName() + "\n");
    std::string res = "t" + std::to_string(t_num);
    t_num++;
    return res;
  }
}

std::string IRGenerator::GenUnaryExp(UnaryAST &_exp, std::string &_code) {
  std::string ret;
  if (_exp.getOp() != Operator::add_op) {
    ret += op2char(_exp.getOp());
  }
  std::string res = _exp.getNode()->GenerateIR(*this, _code);
  for (int j = 0; j < currentDepth; j++) {
    _code += "\t";
  }
  _code += ("t" + std::to_string(t_num) + " = " + res + "\n");
  res = "t" + std::to_string(t_num);
  t_num++;
  ret += res;
  if (_exp.getOp() != Operator::ERROR) {
    for (int j = 0; j < currentDepth; j++) {
      _code += "\t";
    }
    _code += ("t" + std::to_string(t_num) + " = " + ret + "\n");
    ret = "t" + std::to_string(t_num);
    t_num++;
  }
  return ret;
}

std::string IRGenerator::GenLVal(LValAST &_lval, std::string &_code) {
  std::map<std::string, GenVar>::iterator iter;
  int tmpCurrentBlock = currentBlock;
  while (tmpCurrentBlock != -1) {
    iter = BlockSymbolTable[tmpCurrentBlock].find(_lval.getName());
    if (iter != BlockSymbolTable[tmpCurrentBlock].end() &&
        !iter->second.id.empty()) {
      break;
    }
    tmpCurrentBlock = parentBlock[tmpCurrentBlock];
  }
  if (_lval.getType() == VarType::var_t) {
    return iter->second.id;
  } else {
    std::string name = iter->second.id;
    std::vector<int> dim = iter->second.dims;
    int tmp;
    for (size_t i = 0; i < _lval.getPosition().size(); i++) {
      std::string var = _lval.getPosition()[i]->GenerateIR(*this, _code);
      for (int j = 0; j < currentDepth; j++) {
        _code += "\t";
      }
      _code += ("t" + std::to_string(t_num) + " = " + var + "\n");
      var = "t" + std::to_string(t_num);
      t_num++;
      if (i < _lval.getPosition().size() - 1) {
        for (int j = 0; j < currentDepth; j++) {
          _code += "\t";
        }
        _code += ("t" + std::to_string(t_num) + " = " + var + " * " +
                  std::to_string(dim[i + 1]) + "\n");
        for (int k = i + 2; k < _lval.getPosition().size(); k++) {
          for (int j = 0; j < currentDepth; j++) {
            _code += "\t";
          }
          _code +=
              ("t" + std::to_string(t_num) + " = " + "t" +
               std::to_string(t_num) + " * " + std::to_string(dim[k]) + "\n");
        }
      } else {
        for (int j = 0; j < currentDepth; j++) {
          _code += "\t";
        }
        _code += ("t" + std::to_string(t_num) + " = " + var + "\n");
      }
      if (i > 0) {
        for (int j = 0; j < currentDepth; j++) {
          _code += "\t";
        }
        _code += ("t" + std::to_string(t_num) + " = t" + std::to_string(tmp) +
                  " + t" + std::to_string(t_num) + "\n");
      }
      tmp = t_num++;
    }
    for (int j = 0; j < currentDepth; j++) {
      _code += "\t";
    }
    _code +=
        ("t" + std::to_string(tmp) + " = 4 * t" + std::to_string(tmp) + "\n");
    std::string res = name + "[t" + std::to_string(tmp) + "]";
    return res;
  }
}

void IRGenerator::GenFuncDef(FuncDefAST &_funcDef, std::string &_code) {
  currentFunc = _funcDef.getName();
  for (size_t i = 0; i < _funcDef.getArgs().size(); i++) {
    BlockSymbolTable[parentBlock.size()]
                    [dynamic_cast<ProcessedIdAST *>(_funcDef.getArgs()[i].get())
                         ->getName()]
                        .id = "p" + std::to_string(i);
  }
  if (_funcDef.getName() != "main") {
    _code += ("f_" + _funcDef.getName() + " [" +
              std::to_string(_funcDef.getArgs().size()) + "]\n");
  }
  int T_tmp = T_num;
  int t_tmp = t_num;
  std::string code2;
  _funcDef.getBody()->GenerateIR(*this, code2);
  for (int i = T_tmp; i < T_num; i++) {
    if (ReverseSymbolTable[i].argType == VarType::var_t) {
      for (int j = 0; j < currentDepth + 1; j++) {
        _code += "\t";
      }
      _code += ("var T" + std::to_string(i) + "\n");
    } else {
      std::vector<int> dim = ReverseSymbolTable[i].dims;
      int size = 4;
      for (int d : dim) {
        size *= d;
      }
      for (int j = 0; j < currentDepth + 1; j++) {
        _code += "\t";
      }
      _code += ("var " + std::to_string(size) + " " + ReverseSymbolTable[i].id +
                "\n");
    }
  }
  for (int i = t_tmp; i < t_num; i++) {
    for (int j = 0; j < currentDepth + 1; j++) {
      _code += "\t";
    }
    _code += ("var t" + std::to_string(i) + "\n");
  }
  _code += code2;
  for (int j = 0; j < currentDepth + 1; j++) {
    _code += "\t";
  }
  _code += "return";
  _code += (_funcDef.getType() == Type::int_t ? " 0\n" : " \n");
  _code += "\nend f_" + _funcDef.getName() + "\n";
}

void IRGenerator::GenStmt(StmtAST &_stmt, std::string &_code) {
  if (_stmt.getStmt()) {
    _stmt.getStmt()->GenerateIR(*this, _code);
  }
}

void IRGenerator::GenBlock(BlockAST &_block, std::string &_code) {
  ++currentDepth;
  parentBlock.push_back(currentBlock);
  currentBlock = parentBlock.size() - 1;
  for (const auto &stmt : _block.getStmts()) {
    stmt->GenerateIR(*this, _code);
  }
  --currentDepth;
  currentBlock = parentBlock[currentBlock];
}

void IRGenerator::GenIfElse(IfAST &_stmt, std::string &_code) {
  std::string cond = _stmt.getCond()->GenerateIR(*this, _code);
  int tmp1 = l_num;
  if (cond.find("[") != std::string::npos) {
    for (int j = 0; j < currentDepth; j++) {
      _code += "\t";
    }
    _code += ("t" + std::to_string(t_num) + " = " + cond + "\n");
    cond = "t" + std::to_string(t_num);
    t_num++;
  }
  if (!(cond[0] >= 'a' && cond[0] <= 'z')) {
    for (int j = 0; j < currentDepth; j++) {
      _code += "\t";
    }
    _code += ("t" + std::to_string(t_num) + " = " + cond + "\n");
    cond = "t" + std::to_string(t_num);
    t_num++;
  }
  for (int j = 0; j < currentDepth; j++) {
    _code += "\t";
  }
  _code += ("if " + cond + " == 0 goto l" + std::to_string(tmp1) + "\n");
  l_num++;
  _stmt.getThenStmt()->GenerateIR(*this, _code);
  if (_stmt.getElseStmt()) {
    int tmp = tmp1;
    std::string branch;
    _stmt.getElseStmt()->GenerateIR(*this, branch);
    for (int j = 0; j < currentDepth + 1; j++) {
      _code += "\t";
    }
    _code += ("goto l" + std::to_string(l_num) + "\n");
    _code += ("l" + std::to_string(tmp) + ":\n");
    _code += branch;
    _code += ("l" + std::to_string(l_num) + ":\n");
    l_num++;
  } else {
    _code += ("l" + std::to_string(tmp1) + ":\n");
  }
}

void IRGenerator::GenWhile(WhileAST &_stmt, std::string &_code) {
  int old_break = cur_break_l;
  int old_continue = cur_continue_l;

  int begin_loop = l_num;
  _code += ("l" + std::to_string(begin_loop) + ":\n");
  l_num++;
  std::string cond = _stmt.getCond()->GenerateIR(*this, _code);
  int out_loop = l_num;
  l_num++;

  cur_break_l = out_loop;
  cur_continue_l = begin_loop;

  for (int j = 0; j < currentDepth + 1; j++) {
    _code += "\t";
  }
  _code += ("if " + cond + " == 0 goto l" + std::to_string(out_loop) + "\n");
  _stmt.getStmt()->GenerateIR(*this, _code);
  for (int j = 0; j < currentDepth + 1; j++) {
    _code += "\t";
  }
  _code += ("goto l" + std::to_string(begin_loop) + "\n");
  _code += ("l" + std::to_string(out_loop) + ":\n");

  cur_break_l = old_break;
  cur_continue_l = old_continue;
}

void IRGenerator::GenControl(ControlAST &_stmt, std::string &_code) {
  switch (_stmt.getControl()) {
  case Control::continue_c: {
    for (int j = 0; j < currentDepth; j++) {
      _code += "\t";
    }
    _code += ("goto l" + std::to_string(cur_continue_l) + "\n");
    break;
  }
  case Control::break_c: {
    for (int j = 0; j < currentDepth; j++) {
      _code += "\t";
    }
    _code += ("goto l" + std::to_string(cur_break_l) + "\n");
    break;
  }
  case Control::return_c: {
    if (_stmt.getReturnExp()) {
      std::string ret = _stmt.getReturnExp()->GenerateIR(*this, _code);
      for (int j = 0; j < currentDepth; j++) {
        _code += "\t";
      }
      _code += ("t" + std::to_string(t_num) + " = " + ret + "\n");
      ret = ("t" + std::to_string(t_num));
      t_num++;
      for (int j = 0; j < currentDepth; j++) {
        _code += "\t";
      }
      _code += ("return " + ret + "\n");
    } else {
      for (int j = 0; j < currentDepth; j++) {
        _code += "\t";
      }
      _code += ("return \n");
    }
  }
  default:
    break;
  }
}

std::string IRGenerator::GenLAndExp(BinaryAST &_exp, std::string &_code) {
  std::string t1 = _exp.getLHS()->GenerateIR(*this, _code);
  for (int j = 0; j < currentDepth; j++) {
    _code += "\t";
  }
  _code += ("t" + std::to_string(t_num) + " = " + t1 + "\n");
  t1 = "t" + std::to_string(t_num);
  t_num++;
  int shorthand = l_num;
  for (int j = 0; j < currentDepth; j++) {
    _code += "\t";
  }
  _code += ("if " + t1 + " == 0 goto l" + std::to_string(l_num++) + "\n");
  std::string t2 = _exp.getRHS()->GenerateIR(*this, _code);
  for (int j = 0; j < currentDepth; j++) {
    _code += "\t";
  }
  _code += ("t" + std::to_string(t_num) + " = " + t2 + "\n");
  t2 = "t" + std::to_string(t_num);
  t_num++;
  _code += ("goto l" + std::to_string(l_num++) + "\n");
  _code += ("l" + std::to_string(shorthand) + ":\n");
  for (int j = 0; j < currentDepth; j++) {
    _code += "\t";
  }
  _code += (t2 + " = 0\n");
  for (int j = 0; j < currentDepth; j++) {
    _code += "\t";
  }
  _code += ("l" + std::to_string(l_num - 1) + ":\n");
  return t2;
}

std::string IRGenerator::GenLOrExp(BinaryAST &_exp, std::string &_code) {
  std::string t1 = _exp.getLHS()->GenerateIR(*this, _code);
  for (int j = 0; j < currentDepth; j++) {
    _code += "\t";
  }
  _code += ("t" + std::to_string(t_num) + " = " + t1 + "\n");
  t1 = "t" + std::to_string(t_num);
  t_num++;
  int shorthand = l_num;
  for (int j = 0; j < currentDepth; j++) {
    _code += "\t";
  }
  _code += ("if " + t1 + " == 1 goto l" + std::to_string(l_num++) + "\n");
  std::string t2 = _exp.getRHS()->GenerateIR(*this, _code);
  for (int j = 0; j < currentDepth; j++) {
    _code += "\t";
  }
  _code += ("t" + std::to_string(t_num) + " = " + t2 + "\n");
  t2 = "t" + std::to_string(t_num);
  t_num++;
  _code += ("goto l" + std::to_string(l_num++) + "\n");
  _code += ("l" + std::to_string(shorthand) + ":\n");
  for (int j = 0; j < currentDepth; j++) {
    _code += "\t";
  }
  _code += (t2 + " = 1\n");
  _code += ("l" + std::to_string(l_num - 1) + ":\n");
  return t2;
}

std::string FuncDefAST::GenerateIR(IRGenerator &_gen, std::string &_code) {
  _gen.GenFuncDef(*this, _code);
  return {};
}

std::string BlockAST::GenerateIR(IRGenerator &_gen, std::string &_code) {
  _gen.GenBlock(*this, _code);
  return {};
}

std::string BinaryAST::GenerateIR(IRGenerator &_gen, std::string &_code) {
  if (this->getOp() == Operator::and_op) {
    return _gen.GenLAndExp(*this, _code);
  } else if (this->getOp() == Operator::or_op) {
    return _gen.GenLOrExp(*this, _code);
  } else {
    return _gen.GenBinaryExp(*this, _code);
  }
}

std::string IfAST::GenerateIR(IRGenerator &_gen, std::string &_code) {
  _gen.GenIfElse(*this, _code);
  return {};
}

std::string WhileAST::GenerateIR(IRGenerator &_gen, std::string &_code) {
  _gen.GenWhile(*this, _code);
  return {};
}

std::string NumAST::GenerateIR(IRGenerator &_gen, std::string &_code) {
  return _gen.GenNumber(*this, _code);
}

std::string IdAST::GenerateIR(IRGenerator &_gen, std::string &_code) {
  return this->getName();
}

std::string ProcessedIdAST::GenerateIR(IRGenerator &_gen, std::string &_code) {
  return _gen.GenId(*this, _code);
}

std::string UnaryAST::GenerateIR(IRGenerator &_gen, std::string &_code) {
  return _gen.GenUnaryExp(*this, _code);
}

std::string ControlAST::GenerateIR(IRGenerator &_gen, std::string &_code) {
  _gen.GenControl(*this, _code);
  return {};
}

std::string AssignAST::GenerateIR(IRGenerator &_gen, std::string &_code) {
  return _gen.GenAssign(*this, _code);
}

std::string StmtAST::GenerateIR(IRGenerator &_gen, std::string &_code) {
  _gen.GenStmt(*this, _code);
  return {};
}

std::string LValAST::GenerateIR(IRGenerator &_gen, std::string &_code) {
  return _gen.GenLVal(*this, _code);
}

std::string FuncCallAST::GenerateIR(IRGenerator &_gen, std::string &_code) {
  return _gen.GenFuncCall(*this, _code);
}

std::string VarDeclAST::GenerateIR(IRGenerator &_gen, std::string &_code) {
  _gen.GenVarDecl(*this, _code);
  return {};
}

std::string VarDefAST::GenerateIR(IRGenerator &_gen, std::string &_code) {
  return _gen.GenVarDef(*this, _code);
}

std::string InitValAST::GenerateIR(IRGenerator &_gen, std::string &_code) {
  return _gen.GenInitVal(*this, _code);
}

std::string EmptyAST::GenerateIR(IRGenerator &_gen, std::string &_code) {
  return {};
}

std::string CompUnitAST::GenerateIR(IRGenerator &_gen, std::string &_code) {
  _gen.GenCompUnit(*this, _code);
  return {};
}