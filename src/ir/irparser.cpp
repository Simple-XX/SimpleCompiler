
/**
 * @file irparser.cpp
 * @brief ir parser
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

#include <iostream>
#include <memory>
#include <string>
#include <utility>

#include "ir.h"
#include "irast.h"
#include "irparser.h"

void IRParser::NextIRToken() { current = lexer.NextIRToken(); }

/*
 * Declaration ::= "var" [NUM] SYMBOL_IR;
 */
IRPtr IRParser::ParseDecl() {
  if (current != IRToken::VARDECL_IR) {
    exit(201);
  }
  NextIRToken();
  std::string name;
  if (current == IRToken::NUMBER_IR) {
    int size = lexer.getVal();
    NextIRToken();
    name = lexer.getName();
    NextIRToken();
    return std::make_unique<DeclIR>(VarType::array_t, size, name,
                                    lexer.getLineno() - 1);
  } else {
    name = lexer.getName();
    NextIRToken();
    return std::make_unique<DeclIR>(VarType::var_t, 0, name,
                                    lexer.getLineno() - 1);
  }
}

/*
    * Initialization  ::= SYMBOL_IR "=" NUM
                | SYMBOL_IR "[" NUM "]" "=" NUM;
    */
IRPtr IRParser::ParseInit() {
  if (current != IRToken::SYMBOL_IR) {
    exit(202);
  }
  std::string name = lexer.getName();
  NextIRToken();
  VarType type = VarType::var_t;
  int pos = -1;
  if (current == IRToken::LSB_IR) {
    type = VarType::array_t;
    NextIRToken();
    if (current != IRToken::NUMBER_IR) {
      exit(203);
    }
    pos = lexer.getVal();
    NextIRToken();
    if (current != IRToken::RSB_IR) {
      exit(204);
    }
    NextIRToken(); // consume RSB_IR
  }
  if (current != IRToken::ASSIGN_IR) {
    exit(205);
  }
  NextIRToken(); // consume assign
  if (current != IRToken::NUMBER_IR) {
    exit(206);
  }
  int value = lexer.getVal();
  NextIRToken(); // consume value

  return std::make_unique<InitIR>(type, name, pos, value,
                                  lexer.getLineno() - 1);
}

/*
 * Program ::= {Declaration | Initialization | FunctionDef};
 */
IRPtr IRParser::ParseProgram() {
  NextIRToken();
  IRPtrList nodes;
  while (current != IRToken::END_IR) {
    if (current == IRToken::VARDECL_IR) {
      nodes.push_back(ParseDecl());
    } else if (current == IRToken::SYMBOL_IR) {
      std::string name = lexer.getName();
      if (name[0] == 'f' && name[1] == '_') {
        nodes.push_back(ParseFuncDef());
      } else {
        nodes.push_back(ParseInit());
      }
    } else {
      exit(207);
    }
  }
  return std::make_unique<ProgramIR>(std::move(nodes), lexer.getLineno());
}

/*
 * FunctionDef     ::= FunctionHeader Statements FunctionEnd;
 * FunctionHeader  ::= FUNCTION "[" NUM "]";
 * Statements      ::= {Statement};
 * FunctionEnd     ::= "end" FUNCTION;
 */
IRPtr IRParser::ParseFuncDef() {
  if (current != IRToken::SYMBOL_IR || lexer.getName()[0] != 'f' ||
      lexer.getName()[1] != '_') {
    exit(208);
  }
  std::string funcName = lexer.getName();
  NextIRToken();
  if (current != IRToken::LSB_IR) {
    exit(209);
  }
  NextIRToken();
  if (current != IRToken::NUMBER_IR) {
    exit(210);
  }
  int param = lexer.getVal();
  NextIRToken();
  if (current != IRToken::RSB_IR) {
    exit(211);
  }
  NextIRToken();
  IRPtr body = ParseStatements();
  if (current != IRToken::FUNCEND_IR) {
    exit(lexer.getName()[0]);
  }
  NextIRToken();
  if (current != IRToken::SYMBOL_IR || lexer.getName() != funcName) {
    exit(213);
  }
  NextIRToken();

  return std::make_unique<FuncDefIR>(funcName, param, std::move(body),
                                     lexer.getLineno() - 1);
}

/*
 * Statements ::= {Statement};
 */
IRPtr IRParser::ParseStatements() {
  IRPtrList stmts;
  while (true) {
    if (current == IRToken::VARDECL_IR) {
      stmts.push_back(ParseDecl());
    } else if (current == IRToken::SYMBOL_IR) {
      if (lexer.getName()[0] == 'l') {
        stmts.push_back(ParseLabel());
      } else {
        stmts.push_back(ParseAssign());
      }
    } else if (current == IRToken::IF_IR) {
      stmts.push_back(ParseCondGoto());
    } else if (current == IRToken::GOTO_IR) {
      stmts.push_back(ParseGoto());
    } else if (current == IRToken::PARAM_IR) {
      stmts.push_back(ParseParams());
    } else if (current == IRToken::CALL_IR) {
      stmts.push_back(ParseFuncCall());
    } else if (current == IRToken::RETURN_IR) {
      stmts.push_back(ParseReturn());
    } else {
      break;
    }
  }
  return std::make_unique<StatementsIR>(std::move(stmts),
                                        lexer.getLineno() - 1);
}

/*
 * SYMBOL_IR "=" RightValue BinOp RightValue
 * SYMBOL_IR "=" OP_IR RightValue
 * LVal "=" RightValue
 * SYMBOL_IR "=" SYMBOL_IR "[" RightValue "]"
 * SYMBOL_IR "=" "call" FUNCTION
 */

IRPtr IRParser::ParseAssign() {
  if (current != IRToken::SYMBOL_IR) {
    exit(214);
  }
  IRPtr lhs = ParseLVal();
  if (current != IRToken::ASSIGN_IR) {
    exit(215);
  }
  NextIRToken();
  IRPtr rhs;
  if (current == IRToken::OP_IR) {
    Operator op = lexer.getOp();
    NextIRToken();
    IRPtr rValue;
    if (current == IRToken::SYMBOL_IR) {
      if (dynamic_cast<LValIR *>(lhs.get())->getType() == VarType::array_t) {
        exit(216);
      }
      rValue = std::make_unique<RightValIR>(current, lexer.getName(),
                                            lexer.getLineno() - 1);
    } else if (current == IRToken::NUMBER_IR) {
      rValue = std::make_unique<RightValIR>(current, lexer.getVal(),
                                            lexer.getLineno() - 1);
    } else if (current == IRToken::OP_IR) {
      Operator op2 = lexer.getOp();
      NextIRToken();
      if (current != IRToken::NUMBER_IR) {
        exit(241);
      }
      switch (op2) {
      case Operator::add_op: {
        rValue = std::make_unique<RightValIR>(current, lexer.getVal(),
                                              lexer.getLineno());
        break;
      }
      case Operator::sub_op: {
        rValue = std::make_unique<RightValIR>(current, -lexer.getVal(),
                                              lexer.getLineno());
        break;
      }
      case Operator::not_op: {
        rValue = std::make_unique<RightValIR>(current, !lexer.getVal(),
                                              lexer.getLineno());
        break;
      }
      default: {
        exit(242);
      }
      }
    } else {
      exit(217);
    }
    NextIRToken();
    rhs = std::make_unique<UnaryExpIR>(std::move(rValue), op,
                                       lexer.getLineno() - 1);
    return std::make_unique<AssignIR>(std::move(lhs), std::move(rhs),
                                      lexer.getLineno() - 1);
  } else if (current == IRToken::NUMBER_IR) {
    IRPtr rValue_1 = std::make_unique<RightValIR>(current, lexer.getVal(),
                                                  lexer.getLineno());
    NextIRToken();
    if (current == IRToken::OP_IR || current == IRToken::LOGICOP_IR) {
      if (dynamic_cast<LValIR *>(lhs.get())->getType() == VarType::array_t) {
        exit(218);
      }
      Operator op = lexer.getOp();
      NextIRToken();
      IRPtr rValue_2;
      if (current == IRToken::SYMBOL_IR) {
        rValue_2 = std::make_unique<RightValIR>(current, lexer.getName(),
                                                lexer.getLineno());
      } else if (current == IRToken::NUMBER_IR) {
        rValue_2 = std::make_unique<RightValIR>(current, lexer.getVal(),
                                                lexer.getLineno());
      } else if (current == IRToken::OP_IR) {
        Operator op2 = lexer.getOp();
        NextIRToken();
        if (current != IRToken::NUMBER_IR) {
          exit(241);
        }
        switch (op2) {
        case Operator::add_op: {
          rValue_2 = std::make_unique<RightValIR>(current, lexer.getVal(),
                                                  lexer.getLineno());
          break;
        }
        case Operator::sub_op: {
          rValue_2 = std::make_unique<RightValIR>(current, -lexer.getVal(),
                                                  lexer.getLineno());
          break;
        }
        case Operator::not_op: {
          rValue_2 = std::make_unique<RightValIR>(current, !lexer.getVal(),
                                                  lexer.getLineno());
          break;
        }
        default: {
          exit(242);
        }
        }
      } else {
        exit(219);
      }
      NextIRToken();
      rhs = std::make_unique<BinaryExpIR>(
          std::move(rValue_1), std::move(rValue_2), op, lexer.getLineno() - 1);
      return std::make_unique<AssignIR>(std::move(lhs), std::move(rhs),
                                        lexer.getLineno() - 1);
    } else {
      rhs = std::move(rValue_1);
      return std::make_unique<AssignIR>(std::move(lhs), std::move(rhs),
                                        lexer.getLineno() - 1);
    }
  } else if (current == IRToken::SYMBOL_IR) {
    std::string name = lexer.getName();
    NextIRToken();
    if (current == IRToken::LSB_IR) {
      if (dynamic_cast<LValIR *>(lhs.get())->getType() == VarType::array_t) {
        exit(220);
      }
      IRPtr rValue;
      NextIRToken();
      if (current == IRToken::SYMBOL_IR) {
        rValue = std::make_unique<RightValIR>(current, lexer.getName(),
                                              lexer.getLineno());
      } else if (current == IRToken::NUMBER_IR) {
        rValue = std::make_unique<RightValIR>(current, lexer.getVal(),
                                              lexer.getLineno());
      } else if (current == IRToken::OP_IR) {
        Operator op2 = lexer.getOp();
        NextIRToken();
        if (current != IRToken::NUMBER_IR) {
          exit(241);
        }
        switch (op2) {
        case Operator::add_op: {
          rValue = std::make_unique<RightValIR>(current, lexer.getVal(),
                                                lexer.getLineno());
          break;
        }
        case Operator::sub_op: {
          rValue = std::make_unique<RightValIR>(current, -lexer.getVal(),
                                                lexer.getLineno());
          break;
        }
        case Operator::not_op: {
          rValue = std::make_unique<RightValIR>(current, !lexer.getVal(),
                                                lexer.getLineno());
          break;
        }
        default: {
          exit(242);
        }
        }
      } else {
        exit(221);
      }
      NextIRToken();
      if (current != IRToken::RSB_IR) {
        exit(222);
      }
      NextIRToken();
      rhs = std::make_unique<LValIR>(VarType::array_t, name,
                                     lexer.getLineno() - 1, std::move(rValue));
      return std::make_unique<AssignIR>(std::move(lhs), std::move(rhs),
                                        lexer.getLineno() - 1);
    } else {
      IRPtr rValue_1 = std::make_unique<RightValIR>(IRToken::SYMBOL_IR, name,
                                                    lexer.getLineno());
      if (current == IRToken::OP_IR || current == IRToken::LOGICOP_IR) {
        if (dynamic_cast<LValIR *>(lhs.get())->getType() == VarType::array_t) {
          exit(223);
        }
        Operator op = lexer.getOp();
        NextIRToken();
        IRPtr rValue_2;
        if (current == IRToken::SYMBOL_IR) {
          rValue_2 = std::make_unique<RightValIR>(current, lexer.getName(),
                                                  lexer.getLineno());
        } else if (current == IRToken::NUMBER_IR) {
          rValue_2 = std::make_unique<RightValIR>(current, lexer.getVal(),
                                                  lexer.getLineno());
        } else if (current == IRToken::OP_IR) {
          Operator op2 = lexer.getOp();
          NextIRToken();
          if (current != IRToken::NUMBER_IR) {
            exit(241);
          }
          switch (op2) {
          case Operator::add_op: {
            rValue_2 = std::make_unique<RightValIR>(current, lexer.getVal(),
                                                    lexer.getLineno());
            break;
          }
          case Operator::sub_op: {
            rValue_2 = std::make_unique<RightValIR>(current, -lexer.getVal(),
                                                    lexer.getLineno());
            break;
          }
          case Operator::not_op: {
            rValue_2 = std::make_unique<RightValIR>(current, !lexer.getVal(),
                                                    lexer.getLineno());
            break;
          }
          default: {
            exit(242);
          }
          }
        } else {
          exit(224);
        }
        NextIRToken();
        rhs = std::make_unique<BinaryExpIR>(std::move(rValue_1),
                                            std::move(rValue_2), op,
                                            lexer.getLineno() - 1);
        return std::make_unique<AssignIR>(std::move(lhs), std::move(rhs),
                                          lexer.getLineno() - 1);
      } else {
        rhs = std::move(rValue_1);
        return std::make_unique<AssignIR>(std::move(lhs), std::move(rhs),
                                          lexer.getLineno() - 1);
      }
    }
  } else if (current == IRToken::CALL_IR) {
    if (dynamic_cast<LValIR *>(lhs.get())->getType() == VarType::array_t) {
      exit(225);
    }
    rhs = ParseFuncCall();
    return std::make_unique<AssignIR>(std::move(lhs), std::move(rhs),
                                      lexer.getLineno() - 1);
  }
}

/*
 * LVal := SYMBOL_IR "[" RightValue "]" | SYMBOL_IR
 */
IRPtr IRParser::ParseLVal() {
  if (current != IRToken::SYMBOL_IR) {
    exit(226);
  }
  std::string name = lexer.getName();
  NextIRToken();
  if (current == IRToken::LSB_IR) {
    IRPtr rValue;
    NextIRToken();
    if (current == IRToken::SYMBOL_IR) {
      rValue = std::make_unique<RightValIR>(current, lexer.getName(),
                                            lexer.getLineno());
    } else if (current == IRToken::NUMBER_IR) {
      rValue = std::make_unique<RightValIR>(current, lexer.getVal(),
                                            lexer.getLineno());
    } else if (current == IRToken::OP_IR) {
      Operator op2 = lexer.getOp();
      NextIRToken();
      if (current != IRToken::NUMBER_IR) {
        exit(241);
      }
      switch (op2) {
      case Operator::add_op: {
        rValue = std::make_unique<RightValIR>(current, lexer.getVal(),
                                              lexer.getLineno());
        break;
      }
      case Operator::sub_op: {
        rValue = std::make_unique<RightValIR>(current, -lexer.getVal(),
                                              lexer.getLineno());
        break;
      }
      case Operator::not_op: {
        rValue = std::make_unique<RightValIR>(current, !lexer.getVal(),
                                              lexer.getLineno());
        break;
      }
      default: {
        exit(242);
      }
      }
    } else {
      exit(227);
    }
    NextIRToken();
    if (current != IRToken::RSB_IR) {
      exit(228);
    }
    NextIRToken();
    return std::make_unique<LValIR>(VarType::array_t, name, lexer.getLineno(),
                                    std::move(rValue));
  } else {
    return std::make_unique<LValIR>(VarType::var_t, name, lexer.getLineno());
  }
}

/*
 * "call" FUNCTION
 */
IRPtr IRParser::ParseFuncCall() {
  if (current != IRToken::CALL_IR) {
    exit(229);
  }
  NextIRToken();
  if (current != IRToken::SYMBOL_IR) {
    exit(230);
  }
  std::string funcName = lexer.getName();
  NextIRToken();
  return std::make_unique<FuncCallIR>(funcName, lexer.getLineno() - 1);
}

/*
 * "return" RightValue
 * "return"
 */
IRPtr IRParser::ParseReturn() {
  if (current != IRToken::RETURN_IR) {
    exit(231);
  }
  NextIRToken();
  IRPtr exp;
  if (current == IRToken::SYMBOL_IR) {
    exp = std::make_unique<RightValIR>(current, lexer.getName(),
                                       lexer.getLineno());
    NextIRToken();
  } else if (current == IRToken::NUMBER_IR) {
    exp = std::make_unique<RightValIR>(current, lexer.getVal(),
                                       lexer.getLineno());
    NextIRToken();
  } else {
    exp = nullptr;
  }
  return std::make_unique<ReturnIR>(std::move(exp), lexer.getLineno() - 1);
}

/*
 * param RightValue
 */
IRPtr IRParser::ParseParams() {
  IRPtrList params;
  while (current == IRToken::PARAM_IR) {
    NextIRToken();
    if (current == IRToken::SYMBOL_IR) {
      params.push_back(std::make_unique<RightValIR>(current, lexer.getName(),
                                                    lexer.getLineno()));
    } else if (current == IRToken::NUMBER_IR) {
      params.push_back(std::make_unique<RightValIR>(current, lexer.getVal(),
                                                    lexer.getLineno()));
    } else if (current == IRToken::OP_IR) {
      Operator op2 = lexer.getOp();
      NextIRToken();
      if (current != IRToken::NUMBER_IR) {
        exit(241);
      }
      switch (op2) {
      case Operator::add_op: {
        params.push_back(std::make_unique<RightValIR>(current, lexer.getVal(),
                                                      lexer.getLineno()));
        break;
      }
      case Operator::sub_op: {
        params.push_back(std::make_unique<RightValIR>(current, -lexer.getVal(),
                                                      lexer.getLineno()));
        break;
      }
      case Operator::not_op: {
        params.push_back(std::make_unique<RightValIR>(current, !lexer.getVal(),
                                                      lexer.getLineno()));
        break;
      }
      default: {
        exit(242);
      }
      }
    }
    NextIRToken();
  }
  return std::make_unique<ParamListIR>(std::move(params),
                                       lexer.getLineno() - 1);
}

IRPtr IRParser::ParseGoto() {
  if (current != IRToken::GOTO_IR) {
    exit(232);
  }
  NextIRToken();
  if (current != IRToken::SYMBOL_IR || lexer.getName()[0] != 'l') {
    exit(233);
  }
  int num = stoi(lexer.getName().substr(1, lexer.getName().length() - 1));
  NextIRToken();
  return std::make_unique<GotoIR>(num, lexer.getLineno() - 1);
}

IRPtr IRParser::ParseCondGoto() {
  if (current != IRToken::IF_IR) {
    exit(234);
  }
  NextIRToken();
  IRPtr r1, r2;
  if (current == IRToken::SYMBOL_IR) {
    r1 = std::make_unique<RightValIR>(current, lexer.getName(),
                                      lexer.getLineno());
  } else if (current == IRToken::NUMBER_IR) {
    r1 = std::make_unique<RightValIR>(current, lexer.getVal(),
                                      lexer.getLineno());
  } else if (current == IRToken::OP_IR) {
    Operator op2 = lexer.getOp();
    NextIRToken();
    if (current != IRToken::NUMBER_IR) {
      exit(241);
    }
    switch (op2) {
    case Operator::add_op: {
      r1 = std::make_unique<RightValIR>(current, lexer.getVal(),
                                        lexer.getLineno());
      break;
    }
    case Operator::sub_op: {
      r1 = std::make_unique<RightValIR>(current, -lexer.getVal(),
                                        lexer.getLineno());
      break;
    }
    case Operator::not_op: {
      r1 = std::make_unique<RightValIR>(current, !lexer.getVal(),
                                        lexer.getLineno());
      break;
    }
    default: {
      exit(242);
    }
    }
  } else {
    exit(235);
  }
  NextIRToken();
  if (current != IRToken::LOGICOP_IR) {
    exit(236);
  }
  Operator op = lexer.getOp();
  NextIRToken();
  if (current == IRToken::SYMBOL_IR) {
    r2 = std::make_unique<RightValIR>(current, lexer.getName(),
                                      lexer.getLineno());
  } else if (current == IRToken::NUMBER_IR) {
    r2 = std::make_unique<RightValIR>(current, lexer.getVal(),
                                      lexer.getLineno());
  } else if (current == IRToken::OP_IR) {
    Operator op2 = lexer.getOp();
    NextIRToken();
    if (current != IRToken::NUMBER_IR) {
      exit(243);
    }
    switch (op2) {
    case Operator::add_op: {
      r1 = std::make_unique<RightValIR>(current, lexer.getVal(),
                                        lexer.getLineno());
      break;
    }
    case Operator::sub_op: {
      r1 = std::make_unique<RightValIR>(current, -lexer.getVal(),
                                        lexer.getLineno());
      break;
    }
    case Operator::not_op: {
      r1 = std::make_unique<RightValIR>(current, !lexer.getVal(),
                                        lexer.getLineno());
      break;
    }
    default: {
      exit(244);
    }
    }
  } else {
    exit(237);
  }

  IRPtr cond = std::make_unique<BinaryExpIR>(std::move(r1), std::move(r2), op,
                                             lexer.getLineno());
  NextIRToken();
  if (current != IRToken::GOTO_IR) {
    exit(238);
  }
  NextIRToken();
  if (current != IRToken::SYMBOL_IR || lexer.getName()[0] != 'l') {
    exit(239);
  }
  int num = stoi(lexer.getName().substr(1, lexer.getName().length() - 1));
  NextIRToken();

  return std::make_unique<CondGotoIR>(std::move(cond), num,
                                      lexer.getLineno() - 1);
}

IRPtr IRParser::ParseLabel() {
  if (current != IRToken::SYMBOL_IR || lexer.getName()[0] != 'l') {
    exit(239);
  }
  int num = stoi(lexer.getName().substr(1, lexer.getName().length() - 1));
  IRPtr label = std::make_unique<Label>(num, lexer.getLineno());
  NextIRToken();
  if (current != IRToken::COLON_IR) {
    exit(240);
  }
  NextIRToken();
  return label;
}
