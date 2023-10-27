
/**
 * @file lowirgen.cpp
 * @brief low ir gen
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

#include "lowirgen.h"

std::string LowIRGenerator::op2char(Operator _op) {
  std::string c;
  switch (_op) {
  case Operator::add_op:
    c = "+";
    break;
  case Operator::sub_op:
    c = "-";
    break;
  case Operator::mul_op:
    c = "*";
    break;
  case Operator::div_op:
    c = "/";
    break;
  case Operator::mod_op:
    c = "%";
    break;
  case Operator::orbit_op:
    c = "|";
    break;
  case Operator::andbit_op:
    c = "&";
    break;
  case Operator::eorbit_op:
    c = "^";
    break;
  case Operator::and_op:
    c = "&&";
    break;
  case Operator::or_op:
    c = "||";
    break;
  case Operator::not_op:
    c = "!";
    break;
  case Operator::gt_op:
    c = ">";
    break;
  case Operator::ge_op:
    c = ">=";
    break;

  case Operator::lt_op:
    c = "<";
    break;
  case Operator::le_op:
    c = "<=";
    break;
  case Operator::equ_op:
    c = "==";
    break;
  case Operator::nequ_op:
    c = "!=";
    break;
  case ERROR:
    c = "";
    break;
  }
  return c;
}

std::string LowIRGenerator::GenDecl(DeclIR &_decl, std::string &_code) {
  if (currentFunc.empty()) {
    // global _decl
    if (_decl.getType() == VarType::array_t) {
      globalVars[_decl.getName()] = Variable(v_num, VarType::array_t);
      _code += "v" + std::to_string(v_num++) + " = malloc " +
               std::to_string(_decl.getSize()) + "\n";
    } else {
      globalVars[_decl.getName()] = Variable(v_num, VarType::var_t);
      _code += "v" + std::to_string(v_num++) + " = 0\n";
    }
  } else {
    // local var
    if (_decl.getType() == VarType::array_t) {
      varStack[_decl.getName()] = StackVar(
          funcStack[currentFunc], funcStack[currentFunc] + _decl.getSize() / 4,
          currentFunc, VarType::array_t);
      funcStack[currentFunc] += _decl.getSize() / 4;
    } else {
      varStack[_decl.getName()] =
          StackVar(funcStack[currentFunc], funcStack[currentFunc], currentFunc,
                   VarType::var_t);
      funcStack[currentFunc]++;
    }
  }
  return {};
}

std::string LowIRGenerator::GenInit(InitIR &_init, std::string &_code) {
  if (currentFunc.empty()) {
    if (_init.getType() == VarType::var_t) {
      globalVars[_init.getName()] = Variable(v_num, VarType::var_t);
      _code += "v" + std::to_string(v_num++) + " = " +
               std::to_string(_init.getVal()) + "\n";
    } else {
      _code += "loadaddr v" +
               std::to_string(globalVars[_init.getName()].v_num) + " s" +
               std::to_string(reg_num) + " " + "\n";
      _code += "s" + std::to_string(reg_num) + "[" +
               std::to_string(_init.getPos()) +
               "] = " + std::to_string(_init.getVal()) + "\n";
      // reg_num--;
    }
  } else {
    _code += "\ts" + std::to_string(reg_num) + " = " +
             std::to_string(_init.getVal()) + "\n";
    _code += "\tstore s" + std::to_string(reg_num) + " " +
             std::to_string(
                 varStack[_init.getName()].pos_min +
                 (_init.getType() == VarType::array_t ? _init.getPos() : 0)) +
             "\n";
    // reg_num--;
  }
  return {};
}

std::string LowIRGenerator::GenFuncDef(FuncDefIR &_funcDef,
                                       std::string &_code) {
  currentFunc = _funcDef.getName();
  funcStack[currentFunc] = _funcDef.getParamNum();
  for (int i = 0; i < _funcDef.getParamNum(); i++) {
    varStack["p" + std::to_string(i)].pos_min =
        varStack["p" + std::to_string(i)].pos_max = i;
  }
  std::string funcHeader =
      currentFunc + " [" + std::to_string(_funcDef.getParamNum()) + "] [";
  std::string funcBody;
  std::string funcEnd = "end " + currentFunc + "\n";
  _funcDef.getBody()->Generate(*this, funcBody);
  funcHeader += std::to_string(funcStack[currentFunc]) + "]\n";
  reg_num = 1;
  currentFunc = "";
  _code += funcHeader;
  for (int i = 0; i < _funcDef.getParamNum(); i++) {
    _code += "\tstore a" + std::to_string(i) + " " + std::to_string(i) + "\n";
  }
  _code += funcBody + funcEnd;
  return {};
}

std::string LowIRGenerator::GenFuncCall(FuncCallIR &_funcCall,
                                        std::string &_code) {
  _code += "\tcall " + _funcCall.getName() + "\n";
  return {};
}

std::string LowIRGenerator::GenLVal(LValIR &_lval, std::string &_code) {
  if (_lval.getName()[0] == 'T') {
    if (globalVars.find(_lval.getName()) != globalVars.end()) {
      // global var
      _code += "\tloadaddr v" +
               std::to_string(globalVars[_lval.getName()].v_num) + " s" +
               std::to_string(reg_num) + "\n";
      if (_lval.getType() == VarType::var_t) {
        std::string ret = "s" + std::to_string(reg_num) + "[0]";
        reg_num++;
        return ret;
      } else {
        if (dynamic_cast<RightValIR *>(_lval.getPos().get())->getType() ==
            IRToken::NUMBER_IR) {
          std::string ret =
              "s" + std::to_string(reg_num) + "[" +
              std::to_string(
                  dynamic_cast<RightValIR *>(_lval.getPos().get())->getVal()) +
              "]";
          reg_num++;
          return ret;
        } else {
          _code += "\tload " +
                   std::to_string(
                       varStack[dynamic_cast<RightValIR *>(_lval.getPos().get())
                                    ->getName()]
                           .pos_min) +
                   " t0\n";
          _code += "\ts" + std::to_string(reg_num) + " = s" +
                   std::to_string(reg_num) + " + t0\n";
          std::string ret = "s" + std::to_string(reg_num) + "[0]";
          reg_num++;
          return ret;
        }
      }
    } else {
      if (_lval.getType() == VarType::array_t) {
        _code += "\tloadaddr " +
                 std::to_string(varStack[_lval.getName()].pos_min) + " s" +
                 std::to_string(reg_num) + "\n";
        if (dynamic_cast<RightValIR *>(_lval.getPos().get())->getType() ==
            IRToken::NUMBER_IR) {
          std::string ret =
              "s" + std::to_string(reg_num) + "[" +
              std::to_string(
                  dynamic_cast<RightValIR *>(_lval.getPos().get())->getVal()) +
              "]";
          reg_num++;
          return ret;
        } else {
          _code += "\tload " +
                   std::to_string(
                       varStack[dynamic_cast<RightValIR *>(_lval.getPos().get())
                                    ->getName()]
                           .pos_min) +
                   " t0\n";
          _code += "\ts" + std::to_string(reg_num) + " = s" +
                   std::to_string(reg_num) + " + t0\n";
          std::string ret = "s" + std::to_string(reg_num) + "[0]";
          reg_num++;
          return ret;
        }
      } else {
        std::string ret = "s" + std::to_string(reg_num);
        _code += "\tload " + std::to_string(varStack[_lval.getName()].pos_min) +
                 " s" + std::to_string(reg_num) + "\n";
        reg_num++;
        return ret;
      }
    }
  } else if (_lval.getName()[0] == 'p') {
    if (_lval.getType() == VarType::var_t) {
      std::string ret = _lval.getName();
      ret[0] = 'a';
      _code += "\tload " + std::to_string(varStack[_lval.getName()].pos_min) +
               " " + ret + "\n";
      return ret;
    } else {
      std::string ret = _lval.getName();
      ret[0] = 'a';
      _code += "\tload " + std::to_string(varStack[_lval.getName()].pos_min) +
               " " + ret + "\n";
      std::string pos = _lval.getPos()->Generate(*this, _code);
      _code +=
          "\ts" + std::to_string(reg_num) + " = " + ret + " + " + pos + "\n";
      ret = "s" + std::to_string(reg_num);
      reg_num++;
      ret = ret + "[0]";
      return ret;
    }
  } else {
    std::string ret = "s" + std::to_string(reg_num);
    _code += "\tload " + std::to_string(varStack[_lval.getName()].pos_min) +
             " s" + std::to_string(reg_num) + "\n";
    reg_num++;
    return ret;
  }
}

std::string LowIRGenerator::GenAssign(AssignIR &_assign, std::string &_code) {
  std::string lhs = _assign.getLHS()->Generate(*this, _code);
  std::string rhs = _assign.getRHS()->Generate(*this, _code);
  if (dynamic_cast<FuncCallIR *>(_assign.getRHS().get())) {
    _code += "\t" + lhs + " = a0\n";
    if (lhs.find("[") == std::string::npos) {
      _code +=
          "\tstore " + lhs + " " +
          std::to_string(varStack[dynamic_cast<LValIR *>(_assign.getLHS().get())
                                      ->getName()]
                             .pos_min) +
          "\n";
    }
    if (lhs[0] == 's') {
      // reg_num--;
      reg_num = 1;
    }
    return {};
  } else {
    if (!(rhs[0] == 'a' || rhs[0] == 's' || rhs[0] == 't')) {
      _code += "\ts" + std::to_string(reg_num) + " = " + rhs + "\n";
      rhs = "s" + std::to_string(reg_num);
      reg_num++;
    }
    if (dynamic_cast<RightValIR *>(_assign.getRHS().get()) && rhs[0] == 's') {
      // reg_num--;
    }
    if (dynamic_cast<LValIR *>(_assign.getRHS().get()) && rhs[0] == 's') {
      // reg_num--;
    }
    _code += "\t" + lhs + " = " + rhs + "\n";
    if (lhs.find("[") == std::string::npos) {
      _code +=
          "\tstore " + lhs + " " +
          std::to_string(varStack[dynamic_cast<LValIR *>(_assign.getLHS().get())
                                      ->getName()]
                             .pos_min) +
          "\n";
    }
    if (lhs[0] == 's') {
      // reg_num--;
      reg_num = 1;
    }
    return {};
  }
}

std::string LowIRGenerator::GenUnaryExp(UnaryExpIR &_unary,
                                        std::string &_code) {
  if (dynamic_cast<RightValIR *>(_unary.getExp().get())->getType() ==
      IRToken::SYMBOL_IR) {
    _code += "\tload " +
             std::to_string(
                 varStack[dynamic_cast<RightValIR *>(_unary.getExp().get())
                              ->getName()]
                     .pos_min) +
             " s" + std::to_string(reg_num) + "\n";
    if (_unary.getOp() == Operator::add_op) {
      return "s" + std::to_string(reg_num);
    }
    return op2char(_unary.getOp()) + "s" + std::to_string(reg_num);
  } else {
    if (_unary.getOp() == Operator::sub_op) {
      return std::to_string(
          -dynamic_cast<RightValIR *>(_unary.getExp().get())->getVal());
    } else if (_unary.getOp() == Operator::not_op) {
      return std::to_string(
          !dynamic_cast<RightValIR *>(_unary.getExp().get())->getVal());
    } else {
      return std::to_string(
          dynamic_cast<RightValIR *>(_unary.getExp().get())->getVal());
    }
  }
}

std::string LowIRGenerator::GenBinaryExp(BinaryExpIR &_binary,
                                         std::string &_code) {
  std::string lhs = _binary.getLHS()->Generate(*this, _code);
  std::string rhs = _binary.getRHS()->Generate(*this, _code);
  std::set<std::string> logicOp{">=", "<=", "==", "!=", ">", "<"};
  if (dynamic_cast<RightValIR *>(_binary.getRHS().get())->getType() ==
          IRToken::NUMBER_IR &&
      logicOp.find(op2char(_binary.getOp())) != logicOp.end()) {
    _code += "\ts" + std::to_string(reg_num) + " = " + rhs + "\n";
    rhs = "s" + std::to_string(reg_num);
    reg_num++;
  }
  if (dynamic_cast<RightValIR *>(_binary.getLHS().get())->getType() ==
          IRToken::NUMBER_IR &&
      logicOp.find(op2char(_binary.getOp())) != logicOp.end()) {
    _code += "\ts" + std::to_string(reg_num) + " = " + lhs + "\n";
    lhs = "s" + std::to_string(reg_num);
    reg_num++;
  }
  if (dynamic_cast<RightValIR *>(_binary.getLHS().get())->getType() ==
          IRToken::NUMBER_IR &&
      dynamic_cast<RightValIR *>(_binary.getRHS().get())->getType() ==
          IRToken::SYMBOL_IR) {
    _code += "\ts" + std::to_string(reg_num) + " = " + lhs + "\n";
    lhs = "s" + std::to_string(reg_num);
    reg_num++;
  }
  if (rhs[0] == 's') {
    // reg_num--;
  }
  if (lhs[0] == 's') {
    // reg_num--;
  }
  return lhs + " " + op2char(_binary.getOp()) + " " + rhs;
}

std::string LowIRGenerator::GenLabel(Label &_label, std::string &_code) {
  _code += "l" + std::to_string(_label.getNum()) + ":\n";
  return {};
}

std::string LowIRGenerator::GenCondGoto(CondGotoIR &_cond, std::string &_code) {
  std::string conds = _cond.getCond()->Generate(*this, _code);
  _code +=
      "\tif " + conds + " goto l" + std::to_string(_cond.getLabel()) + "\n";
  return {};
}

std::string LowIRGenerator::GenGoto(GotoIR &_gt, std::string &_code) {
  _code += "\tgoto l" + std::to_string(_gt.getLabel()) + "\n";
  return {};
}

std::string LowIRGenerator::GenRightVal(RightValIR &_rightval,
                                        std::string &_code) {
  if (_rightval.getType() == IRToken::NUMBER_IR) {
    return std::to_string(_rightval.getVal());
  } else {
    std::string ret;
    if (_rightval.getName()[0] == 'p') {
      ret = _rightval.getName();
      ret[0] = 'a';
      _code += "\tload " +
               std::to_string(varStack[_rightval.getName()].pos_min) + " " +
               ret + "\n";
      return ret;
    }
    if (globalVars.find(_rightval.getName()) == globalVars.end()) {
      if (varStack[_rightval.getName()].type == VarType::var_t) {
        _code += "\tload " +
                 std::to_string(varStack[_rightval.getName()].pos_min) + " s" +
                 std::to_string(reg_num) + "\n";
        ret = "s" + std::to_string(reg_num);
        reg_num++;
        return ret;
      } else {
        _code += "\tloadaddr " +
                 std::to_string(varStack[_rightval.getName()].pos_min) + " s" +
                 std::to_string(reg_num) + "\n";
        ret = "s" + std::to_string(reg_num);
        reg_num++;
        return ret;
      }
    } else {
      if (globalVars[_rightval.getName()].varType == VarType::var_t) {
        _code += "\tload v" +
                 std::to_string(globalVars[_rightval.getName()].v_num) + " s" +
                 std::to_string(reg_num) + "\n";
        ret = "s" + std::to_string(reg_num);
        reg_num++;
        return ret;
      } else {
        _code += "\tloadaddr v" +
                 std::to_string(globalVars[_rightval.getName()].v_num) + " s" +
                 std::to_string(reg_num) + "\n";
        ret = "s" + std::to_string(reg_num);
        reg_num++;
        return ret;
      }
    }
  }
}

std::string LowIRGenerator::GenReturn(ReturnIR &_ret, std::string &_code) {
  if (_ret.getReturnValue()) {
    std::string r = _ret.getReturnValue()->Generate(*this, _code);
    _code += "\ta0 = " + r + "\n";
  }
  _code += "\treturn\n";
  return {};
}

std::string LowIRGenerator::GenParamList(ParamListIR &_params,
                                         std::string &_code) {
  for (size_t i = 0; i < _params.getParams().size(); i++) {
    std::string param = _params.getParams()[i]->Generate(*this, _code);
    _code += "\ta" + std::to_string(i) + " = " + param + "\n";
  }
  return {};
}

std::string LowIRGenerator::GenStatements(StatementsIR &_stmts,
                                          std::string &_code) {
  for (const auto &stmt : _stmts.getStmts()) {
    stmt->Generate(*this, _code);
  }
  return {};
}

std::string LowIRGenerator::GenProgram(ProgramIR &_program,
                                       std::string &_code) {
  for (const auto &stmt : _program.getNodes()) {
    stmt->Generate(*this, _code);
  }
  return {};
}

std::string DeclIR::Generate(LowIRGenerator &_generator, std::string &_code) {
  return _generator.GenDecl(*this, _code);
}

std::string InitIR::Generate(LowIRGenerator &_generator, std::string &_code) {
  return _generator.GenInit(*this, _code);
}

std::string FuncDefIR::Generate(LowIRGenerator &generator, std::string &code) {
  return generator.GenFuncDef(*this, code);
}

std::string StatementsIR::Generate(LowIRGenerator &_generator,
                                   std::string &_code) {
  return _generator.GenStatements(*this, _code);
}

std::string BinaryExpIR::Generate(LowIRGenerator &_generator,
                                  std::string &_code) {
  return _generator.GenBinaryExp(*this, _code);
}

std::string UnaryExpIR::Generate(LowIRGenerator &_generator,
                                 std::string &_code) {
  return _generator.GenUnaryExp(*this, _code);
}

std::string AssignIR::Generate(LowIRGenerator &_generator, std::string &_code) {
  return _generator.GenAssign(*this, _code);
}

std::string CondGotoIR::Generate(LowIRGenerator &_generator,
                                 std::string &_code) {
  return _generator.GenCondGoto(*this, _code);
}

std::string LValIR::Generate(LowIRGenerator &_generator, std::string &_code) {
  return _generator.GenLVal(*this, _code);
}

std::string GotoIR::Generate(LowIRGenerator &_generator, std::string &_code) {
  return _generator.GenGoto(*this, _code);
}

std::string Label::Generate(LowIRGenerator &_generator, std::string &_code) {
  return _generator.GenLabel(*this, _code);
}

std::string ParamListIR::Generate(LowIRGenerator &_generator,
                                  std::string &_code) {
  return _generator.GenParamList(*this, _code);
}

std::string FuncCallIR::Generate(LowIRGenerator &_generator,
                                 std::string &_code) {
  return _generator.GenFuncCall(*this, _code);
}

std::string ReturnIR::Generate(LowIRGenerator &_generator, std::string &_code) {
  return _generator.GenReturn(*this, _code);
}

std::string RightValIR::Generate(LowIRGenerator &_generator,
                                 std::string &_code) {
  return _generator.GenRightVal(*this, _code);
}

std::string ProgramIR::Generate(LowIRGenerator &_generator,
                                std::string &_code) {
  return _generator.GenProgram(*this, _code);
}