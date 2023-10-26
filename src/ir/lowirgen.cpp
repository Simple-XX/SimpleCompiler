
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

std::string LowIRGenerator::op2char(Operator op) {
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
  case NOT:
    c = "!";
    break;
  }
  return c;
}

std::string LowIRGenerator::GenDecl(DeclIR &decl, std::string &code) {
  if (currentFunc.empty()) {
    // global decl
    if (decl.getType() == VarType::array_t) {
      globalVars[decl.getName()] = Variable(v_num, VarType::array_t);
      code += "v" + std::to_string(v_num++) + " = malloc " +
              std::to_string(decl.getSize()) + "\n";
    } else {
      globalVars[decl.getName()] = Variable(v_num, VarType::var_t);
      code += "v" + std::to_string(v_num++) + " = 0\n";
    }
  } else {
    // local var
    if (decl.getType() == VarType::array_t) {
      varStack[decl.getName()] = StackVar(
          funcStack[currentFunc], funcStack[currentFunc] + decl.getSize() / 4,
          currentFunc, VarType::array_t);
      funcStack[currentFunc] += decl.getSize() / 4;
    } else {
      varStack[decl.getName()] =
          StackVar(funcStack[currentFunc], funcStack[currentFunc], currentFunc,
                   VarType::var_t);
      funcStack[currentFunc]++;
    }
  }
  return {};
}

std::string LowIRGenerator::GenInit(InitIR &init, std::string &code) {
  if (currentFunc.empty()) {
    if (init.getType() == VarType::var_t) {
      globalVars[init.getName()] = Variable(v_num, VarType::var_t);
      code += "v" + std::to_string(v_num++) + " = " +
              std::to_string(init.getVal()) + "\n";
    } else {
      code += "loadaddr v" + std::to_string(globalVars[init.getName()].v_num) +
              " s" + std::to_string(reg_num) + " " + "\n";
      code += "s" + std::to_string(reg_num) + "[" +
              std::to_string(init.getPos()) +
              "] = " + std::to_string(init.getVal()) + "\n";
      // reg_num--;
    }
  } else {
    code += "\ts" + std::to_string(reg_num) + " = " +
            std::to_string(init.getVal()) + "\n";
    code += "\tstore s" + std::to_string(reg_num) + " " +
            std::to_string(
                varStack[init.getName()].pos_min +
                (init.getType() == VarType::array_t ? init.getPos() : 0)) +
            "\n";
    // reg_num--;
  }
  return {};
}

std::string LowIRGenerator::GenFuncDef(FuncDefIR &funcDef, std::string &code) {
  currentFunc = funcDef.getName();
  funcStack[currentFunc] = funcDef.getParamNum();
  for (int i = 0; i < funcDef.getParamNum(); i++) {
    varStack["p" + std::to_string(i)].pos_min =
        varStack["p" + std::to_string(i)].pos_max = i;
  }
  std::string funcHeader =
      currentFunc + " [" + std::to_string(funcDef.getParamNum()) + "] [";
  std::string funcBody;
  std::string funcEnd = "end " + currentFunc + "\n";
  funcDef.getBody()->Generate(*this, funcBody);
  funcHeader += std::to_string(funcStack[currentFunc]) + "]\n";
  reg_num = 1;
  currentFunc = "";
  code += funcHeader;
  for (int i = 0; i < funcDef.getParamNum(); i++) {
    code += "\tstore a" + std::to_string(i) + " " + std::to_string(i) + "\n";
  }
  code += funcBody + funcEnd;
  return {};
}

std::string LowIRGenerator::GenFuncCall(FuncCallIR &funcCall,
                                        std::string &code) {
  code += "\tcall " + funcCall.getName() + "\n";
  return {};
}

std::string LowIRGenerator::GenLVal(LValIR &lval, std::string &code) {
  if (lval.getName()[0] == 'T') {
    if (globalVars.find(lval.getName()) != globalVars.end()) {
      // global var
      code += "\tloadaddr v" +
              std::to_string(globalVars[lval.getName()].v_num) + " s" +
              std::to_string(reg_num) + "\n";
      if (lval.getType() == VarType::var_t) {
        std::string ret = "s" + std::to_string(reg_num) + "[0]";
        reg_num++;
        return ret;
      } else {
        if (dynamic_cast<RightValIR *>(lval.getPos().get())->getType() ==
            IRToken::NUMBER_IR) {
          std::string ret =
              "s" + std::to_string(reg_num) + "[" +
              std::to_string(
                  dynamic_cast<RightValIR *>(lval.getPos().get())->getVal()) +
              "]";
          reg_num++;
          return ret;
        } else {
          code += "\tload " +
                  std::to_string(
                      varStack[dynamic_cast<RightValIR *>(lval.getPos().get())
                                   ->getName()]
                          .pos_min) +
                  " t0\n";
          code += "\ts" + std::to_string(reg_num) + " = s" +
                  std::to_string(reg_num) + " + t0\n";
          std::string ret = "s" + std::to_string(reg_num) + "[0]";
          reg_num++;
          return ret;
        }
      }
    } else {
      if (lval.getType() == VarType::array_t) {
        code += "\tloadaddr " +
                std::to_string(varStack[lval.getName()].pos_min) + " s" +
                std::to_string(reg_num) + "\n";
        if (dynamic_cast<RightValIR *>(lval.getPos().get())->getType() ==
            IRToken::NUMBER_IR) {
          std::string ret =
              "s" + std::to_string(reg_num) + "[" +
              std::to_string(
                  dynamic_cast<RightValIR *>(lval.getPos().get())->getVal()) +
              "]";
          reg_num++;
          return ret;
        } else {
          code += "\tload " +
                  std::to_string(
                      varStack[dynamic_cast<RightValIR *>(lval.getPos().get())
                                   ->getName()]
                          .pos_min) +
                  " t0\n";
          code += "\ts" + std::to_string(reg_num) + " = s" +
                  std::to_string(reg_num) + " + t0\n";
          std::string ret = "s" + std::to_string(reg_num) + "[0]";
          reg_num++;
          return ret;
        }
      } else {
        std::string ret = "s" + std::to_string(reg_num);
        code += "\tload " + std::to_string(varStack[lval.getName()].pos_min) +
                " s" + std::to_string(reg_num) + "\n";
        reg_num++;
        return ret;
      }
    }
  } else if (lval.getName()[0] == 'p') {
    if (lval.getType() == VarType::var_t) {
      std::string ret = lval.getName();
      ret[0] = 'a';
      code += "\tload " + std::to_string(varStack[lval.getName()].pos_min) +
              " " + ret + "\n";
      return ret;
    } else {
      std::string ret = lval.getName();
      ret[0] = 'a';
      code += "\tload " + std::to_string(varStack[lval.getName()].pos_min) +
              " " + ret + "\n";
      std::string pos = lval.getPos()->Generate(*this, code);
      code +=
          "\ts" + std::to_string(reg_num) + " = " + ret + " + " + pos + "\n";
      ret = "s" + std::to_string(reg_num);
      reg_num++;
      ret = ret + "[0]";
      return ret;
    }
  } else {
    std::string ret = "s" + std::to_string(reg_num);
    code += "\tload " + std::to_string(varStack[lval.getName()].pos_min) +
            " s" + std::to_string(reg_num) + "\n";
    reg_num++;
    return ret;
  }
}

std::string LowIRGenerator::GenAssign(AssignIR &assign, std::string &code) {
  std::string lhs = assign.getLHS()->Generate(*this, code);
  std::string rhs = assign.getRHS()->Generate(*this, code);
  if (dynamic_cast<FuncCallIR *>(assign.getRHS().get())) {
    code += "\t" + lhs + " = a0\n";
    if (lhs.find("[") == std::string::npos) {
      code +=
          "\tstore " + lhs + " " +
          std::to_string(
              varStack[dynamic_cast<LValIR *>(assign.getLHS().get())->getName()]
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
      code += "\ts" + std::to_string(reg_num) + " = " + rhs + "\n";
      rhs = "s" + std::to_string(reg_num);
      reg_num++;
    }
    if (dynamic_cast<RightValIR *>(assign.getRHS().get()) && rhs[0] == 's') {
      // reg_num--;
    }
    if (dynamic_cast<LValIR *>(assign.getRHS().get()) && rhs[0] == 's') {
      // reg_num--;
    }
    code += "\t" + lhs + " = " + rhs + "\n";
    if (lhs.find("[") == std::string::npos) {
      code +=
          "\tstore " + lhs + " " +
          std::to_string(
              varStack[dynamic_cast<LValIR *>(assign.getLHS().get())->getName()]
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

std::string LowIRGenerator::GenUnaryExp(UnaryExpIR &unary, std::string &code) {
  if (dynamic_cast<RightValIR *>(unary.getExp().get())->getType() ==
      IRToken::SYMBOL_IR) {
    code +=
        "\tload " +
        std::to_string(varStack[dynamic_cast<RightValIR *>(unary.getExp().get())
                                    ->getName()]
                           .pos_min) +
        " s" + std::to_string(reg_num) + "\n";
    if (unary.getOp() == Operator::add_op) {
      return "s" + std::to_string(reg_num);
    }
    return op2char(unary.getOp()) + "s" + std::to_string(reg_num);
  } else {
    if (unary.getOp() == Operator::sub_op) {
      return std::to_string(
          -dynamic_cast<RightValIR *>(unary.getExp().get())->getVal());
    } else if (unary.getOp() == Operator::not_op) {
      return std::to_string(
          !dynamic_cast<RightValIR *>(unary.getExp().get())->getVal());
    } else {
      return std::to_string(
          dynamic_cast<RightValIR *>(unary.getExp().get())->getVal());
    }
  }
}

std::string LowIRGenerator::GenBinaryExp(BinaryExpIR &binary,
                                         std::string &code) {
  std::string lhs = binary.getLHS()->Generate(*this, code);
  std::string rhs = binary.getRHS()->Generate(*this, code);
  std::set<std::string> logicOp{">=", "<=", "==", "!=", ">", "<"};
  if (dynamic_cast<RightValIR *>(binary.getRHS().get())->getType() ==
          IRToken::NUMBER_IR &&
      logicOp.find(op2char(binary.getOp())) != logicOp.end()) {
    code += "\ts" + std::to_string(reg_num) + " = " + rhs + "\n";
    rhs = "s" + std::to_string(reg_num);
    reg_num++;
  }
  if (dynamic_cast<RightValIR *>(binary.getLHS().get())->getType() ==
          IRToken::NUMBER_IR &&
      logicOp.find(op2char(binary.getOp())) != logicOp.end()) {
    code += "\ts" + std::to_string(reg_num) + " = " + lhs + "\n";
    lhs = "s" + std::to_string(reg_num);
    reg_num++;
  }
  if (dynamic_cast<RightValIR *>(binary.getLHS().get())->getType() ==
          IRToken::NUMBER_IR &&
      dynamic_cast<RightValIR *>(binary.getRHS().get())->getType() ==
          IRToken::SYMBOL_IR) {
    code += "\ts" + std::to_string(reg_num) + " = " + lhs + "\n";
    lhs = "s" + std::to_string(reg_num);
    reg_num++;
  }
  if (rhs[0] == 's') {
    // reg_num--;
  }
  if (lhs[0] == 's') {
    // reg_num--;
  }
  return lhs + " " + op2char(binary.getOp()) + " " + rhs;
}

std::string LowIRGenerator::GenLabel(Label &label, std::string &code) {
  code += "l" + std::to_string(label.getNum()) + ":\n";
  return {};
}

std::string LowIRGenerator::GenCondGoto(CondGotoIR &cond, std::string &code) {
  std::string conds = cond.getCond()->Generate(*this, code);
  code += "\tif " + conds + " goto l" + std::to_string(cond.getLabel()) + "\n";
  return {};
}

std::string LowIRGenerator::GenGoto(GotoIR &gt, std::string &code) {
  code += "\tgoto l" + std::to_string(gt.getLabel()) + "\n";
  return {};
}

std::string LowIRGenerator::GenRightVal(RightValIR &rightval,
                                        std::string &code) {
  if (rightval.getType() == IRToken::NUMBER_IR) {
    return std::to_string(rightval.getVal());
  } else {
    std::string ret;
    if (rightval.getName()[0] == 'p') {
      ret = rightval.getName();
      ret[0] = 'a';
      code += "\tload " + std::to_string(varStack[rightval.getName()].pos_min) +
              " " + ret + "\n";
      return ret;
    }
    if (globalVars.find(rightval.getName()) == globalVars.end()) {
      if (varStack[rightval.getName()].type == VarType::var_t) {
        code += "\tload " +
                std::to_string(varStack[rightval.getName()].pos_min) + " s" +
                std::to_string(reg_num) + "\n";
        ret = "s" + std::to_string(reg_num);
        reg_num++;
        return ret;
      } else {
        code += "\tloadaddr " +
                std::to_string(varStack[rightval.getName()].pos_min) + " s" +
                std::to_string(reg_num) + "\n";
        ret = "s" + std::to_string(reg_num);
        reg_num++;
        return ret;
      }
    } else {
      if (globalVars[rightval.getName()].varType == VarType::var_t) {
        code += "\tload v" +
                std::to_string(globalVars[rightval.getName()].v_num) + " s" +
                std::to_string(reg_num) + "\n";
        ret = "s" + std::to_string(reg_num);
        reg_num++;
        return ret;
      } else {
        code += "\tloadaddr v" +
                std::to_string(globalVars[rightval.getName()].v_num) + " s" +
                std::to_string(reg_num) + "\n";
        ret = "s" + std::to_string(reg_num);
        reg_num++;
        return ret;
      }
    }
  }
}

std::string LowIRGenerator::GenReturn(ReturnIR &ret, std::string &code) {
  if (ret.getReturnValue()) {
    std::string r = ret.getReturnValue()->Generate(*this, code);
    code += "\ta0 = " + r + "\n";
  }
  code += "\treturn\n";
  return {};
}

std::string LowIRGenerator::GenParamList(ParamListIR &params,
                                         std::string &code) {
  for (int i = 0; i < params.getParams().size(); i++) {
    std::string param = params.getParams()[i]->Generate(*this, code);
    code += "\ta" + std::to_string(i) + " = " + param + "\n";
  }
  return {};
}

std::string LowIRGenerator::GenStatements(StatementsIR &stmts,
                                          std::string &code) {
  for (const auto &stmt : stmts.getStmts()) {
    stmt->Generate(*this, code);
  }
  return {};
}

std::string LowIRGenerator::GenProgram(ProgramIR &program, std::string &code) {
  for (const auto &stmt : program.getNodes()) {
    stmt->Generate(*this, code);
  }
  return {};
}

std::string DeclIR::Generate(LowIRGenerator &generator, std::string &code) {
  return generator.GenDecl(*this, code);
}

std::string InitIR::Generate(LowIRGenerator &generator, std::string &code) {
  return generator.GenInit(*this, code);
}

std::string FuncDefIR::Generate(LowIRGenerator &generator, std::string &code) {
  return generator.GenFuncDef(*this, code);
}

std::string StatementsIR::Generate(LowIRGenerator &generator,
                                   std::string &code) {
  return generator.GenStatements(*this, code);
}

std::string BinaryExpIR::Generate(LowIRGenerator &generator,
                                  std::string &code) {
  return generator.GenBinaryExp(*this, code);
}

std::string UnaryExpIR::Generate(LowIRGenerator &generator, std::string &code) {
  return generator.GenUnaryExp(*this, code);
}

std::string AssignIR::Generate(LowIRGenerator &generator, std::string &code) {
  return generator.GenAssign(*this, code);
}

std::string CondGotoIR::Generate(LowIRGenerator &generator, std::string &code) {
  return generator.GenCondGoto(*this, code);
}

std::string LValIR::Generate(LowIRGenerator &generator, std::string &code) {
  return generator.GenLVal(*this, code);
}

std::string GotoIR::Generate(LowIRGenerator &generator, std::string &code) {
  return generator.GenGoto(*this, code);
}

std::string Label::Generate(LowIRGenerator &generator, std::string &code) {
  return generator.GenLabel(*this, code);
}

std::string ParamListIR::Generate(LowIRGenerator &generator,
                                  std::string &code) {
  return generator.GenParamList(*this, code);
}

std::string FuncCallIR::Generate(LowIRGenerator &generator, std::string &code) {
  return generator.GenFuncCall(*this, code);
}

std::string ReturnIR::Generate(LowIRGenerator &generator, std::string &code) {
  return generator.GenReturn(*this, code);
}

std::string RightValIR::Generate(LowIRGenerator &generator, std::string &code) {
  return generator.GenRightVal(*this, code);
}

std::string ProgramIR::Generate(LowIRGenerator &generator, std::string &code) {
  return generator.GenProgram(*this, code);
}