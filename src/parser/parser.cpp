
/**
 * @file parser.cpp
 * @brief parser
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

#include "parser.h"
#include "log.h"

Parser::Parser(Lexer &_lex) : lexer(_lex) { return; }

Parser::~Parser() { return; }

// 获取下一个 token
void Parser::next() {
  token = lexer.lexing();
  return;
}

// 匹配指定 token_base_t
bool Parser::match_token(tag_t _tag) {
  if (token->tag == _tag) {
    return true;
  } else {
    return false;
  }
}

// 进行解析，返回解析结果(AST)
ASTPtr Parser::parsing() {
  this->next(); // 读入第一个token

  ASTPtr prog = program();
  return prog;
}

// 程序由代码片段组成，代码片段由声明与定义组成
ASTPtr Parser::program() {
  ASTPtrList nodes;
  while (is_done() == false) {
    if (match_token(tag_t::KW_CONST)) {
      ASTPtr variable_decl = var_decl();
      if (!variable_decl) {
        error->display_err();
        exit(1);
      }
      nodes.push_back(std::move(variable_decl));
    } else if (match_token(tag_t::KW_VOID)) {
      ASTPtr func = function_def();
      if (!func) {
        error->display_err();
        exit(2);
      }
      nodes.push_back(std::move(func));
    } else if (match_token(tag_t::KW_INT)) { // var or func
      next();                              // int
      if (!match_token(tag_t::ID)) {
        error->display_err();
        exit(3);
      }
      token_identifier_t *token_casted = (token_identifier_t *)token;
      std::string name = token_casted->name;
      next(); // id

      // function def
      if (match_token(tag_t::LPAREN)) {
        next(); // (
        ASTPtrList args;
        if (!match_token(tag_t::RPAREN)) {
          while (true) {
            // TODO: only support int
            if ((!match_token(tag_t::KW_INT))) {
              error->display_err();
              exit(996);
            }
            next(); // type
            if (!match_token(tag_t::ID)) {
              error->display_err();
              exit(998);
            }
            // arg name
            token_identifier_t *token_casted = (token_identifier_t *)token;
            std::string arg_name = token_casted->name;
            next();                         // id
            if (match_token(tag_t::LBRACKET)) // [
            {
              ASTPtrList dim;
              dim.push_back(std::make_unique<NumAST>(0));
              next(); // [
              if (!match_token(tag_t::RBRACKET)) {
                exit(997);
              }
              next(); // ]
              while (match_token(tag_t::LBRACKET)) {
                next(); // [
                ASTPtr _dim = binary_add();
                if (!_dim) {
                  error->display_err();
                  exit(995);
                }
                dim.push_back(std::move(_dim));
                if (!match_token(tag_t::RBRACKET)) {
                  error->display_err();
                  exit(994);
                }
                next(); // ]
              }
              args.push_back(std::make_unique<IdAST>(arg_name, VarType::array_t,
                                                     false, std::move(dim)));
            } else {
              args.push_back(
                  std::make_unique<IdAST>(arg_name, VarType::var_t, false));
            }
            if (!match_token(tag_t::COMMA))
              break;
            next(); // ,
          }
          if (!match_token(tag_t::RPAREN)) {
            error->display_err();
            exit(993);
          }
        }
        next(); // )
        ASTPtr body = block();
        ASTPtr func = std::make_unique<FuncDefAST>(
            Type::int_t, name, std::move(args), std::move(body));
        nodes.push_back(std::move(func));
      } else { // var def
        ASTPtrList varDefs;
        // first , because id is consumed
        ASTPtrList dims;
        while (match_token(tag_t::LBRACKET)) {
          next(); // [
          ASTPtr exp = binary_add();
          if (!exp) {
            exit(453);
          }
          dims.push_back(std::move(exp));
          if (!match_token(tag_t::RBRACKET)) {
            exit(454);
          }
          next(); // ]
        }
        ASTPtr var;
        ASTPtr varDef;
        if (dims.empty())
          var = std::make_unique<IdAST>(name, VarType::var_t, false);
        else
          var = std::make_unique<IdAST>(name, VarType::array_t, false,
                                        std::move(dims));
        if (match_token(tag_t::ASSIGN)) {
          next(); // =
          ASTPtr init = init_val();
          if (!init) {
            exit(456);
          }
          varDef = std::make_unique<VarDefAST>(false, std::move(var),
                                               std::move(init));
        } else {
          varDef = std::make_unique<VarDefAST>(false, std::move(var));
        }
        varDefs.push_back(std::move(varDef));

        while (match_token(tag_t::COMMA)) {
          next(); // ,
          varDef = var_def(false);
          if (!varDef) {
            exit(133);
          }
          varDefs.push_back(std::move(varDef));
        }
        if (!match_token(tag_t::SEMICON)) {
          exit(134);
        }
        ASTPtr decl = std::make_unique<VarDeclAST>(false, std::move(varDefs));
        nodes.push_back(std::move(decl));
        next(); // ;
      }
    } else {
      error->display_err();
      exit(233);
    }
  }
  return std::make_unique<CompUnitAST>(std::move(nodes));
}

// 二元表达式
ASTPtr Parser::binary(const std::function<ASTPtr()> &_parser,
                      std::initializer_list<Operator> _ops) {
  auto lhs = _parser();
  if (!lhs) {
    SPDLOG_LOGGER_ERROR(SCLOG, "error 100");
    exit(100);
  }
  while (std::find(_ops.begin(), _ops.end(), tag_to_op(token->tag)) !=
         _ops.end()) {
    Operator op = tag_to_op(token->tag);
    next();
    auto rhs = _parser();
    if (!rhs) {
      SPDLOG_LOGGER_ERROR(SCLOG, "error 101");
      exit(101);
    }
    lhs = std::make_unique<BinaryAST>(op, std::move(lhs), std::move(rhs));
  }
  return lhs;
}

ASTPtr Parser::binary_relation() {
  return binary(
      [this] { return binary_add(); },
      {Operator::gt_op, Operator::ge_op, Operator::le_op, Operator::le_op});
}

ASTPtr Parser::binary_eq() {
  return binary([this] { return binary_relation(); },
                {Operator::equ_op, Operator::nequ_op});
}

ASTPtr Parser::binary_add() {
  return binary([this] { return binary_mul(); },
                {Operator::add_op, Operator::sub_op});
}

ASTPtr Parser::binary_mul() {
  return binary([this] { return unary(); },
                {Operator::mul_op, Operator::div_op, Operator::mod_op});
}

ASTPtr Parser::binary_and() {
  return binary([this] { return binary_eq(); }, {Operator::and_op});
}

ASTPtr Parser::binary_or() {
  return binary([this] { return binary_and(); }, {Operator::or_op});
}

// 一元表达式
ASTPtr Parser::unary() {
  if (match_token(tag_t::LPAREN)) {
    // (  EXP  )
    next(); // 消耗左括号
    ASTPtr exp = binary_add();
    if (token->tag != tag_t::RPAREN) {
      SPDLOG_LOGGER_ERROR(SCLOG, "error 102");
      exit(102);
    }
    next(); // 消耗右括号
    return exp;
  } else if (match_token(tag_t::NUM)) {
    // NUM
    token_num_t *token_casted = (token_num_t *)token;
    ASTPtr num = std::make_unique<NumAST>(token_casted->num_val);
    next();
    return num;
  } else if (match_token(tag_t::ADD)) {
    // + EXP
    next();
    ASTPtr exp = unary();
    if (!exp) {
      exit(103);
    }
    return std::make_unique<UnaryAST>(std::move(exp), Operator::add_op);
  } else if (match_token(tag_t::SUB)) {
    // - EXP
    next();
    ASTPtr exp = unary();
    if (!exp) {
      exit(104);
    }
    return std::make_unique<UnaryAST>(std::move(exp), Operator::sub_op);
  } else if (match_token(tag_t::NOT)) {
    // ! EXP
    next();
    ASTPtr exp = unary();
    if (!exp) {
      exit(105);
    }
    return std::make_unique<UnaryAST>(std::move(exp), Operator::not_op);
  } else if (match_token(tag_t::ID)) {
    token_identifier_t *token_casted = (token_identifier_t *)token;
    std::string id_name = token_casted->name;
    next();
    // Function call: token_identifier_t (params)
    if (match_token(tag_t::LPAREN)) {
      next();
      // id(): no params
      if (match_token(tag_t::RPAREN)) {
        ASTPtr function_call = std::make_unique<FuncCallAST>(id_name);
        next(); // 消耗右括号
        return function_call;
      } else {
        ASTPtrList params;
        while (true) {
          ASTPtr param = binary_add();
          if (!param) {
            exit(106);
          }
          params.push_back(std::move(param));
          // id(a,b,c)
          if (match_token(tag_t::COMMA) == false)
            break;
          next(); // ,
        }
        if (match_token(tag_t::RPAREN) == false) {
          exit(107);
        }
        next(); // )
        return std::make_unique<FuncCallAST>(id_name, std::move(params));
      }
    } else if (match_token(tag_t::LBRACKET)) { // LVal: array (id[exp])
      ASTPtrList position;
      while (match_token(tag_t::LBRACKET)) {
        next(); // [
        ASTPtr sub_position = binary_add();
        position.push_back(std::move(sub_position));
        if (match_token(tag_t::RBRACKET) == false) {
          exit(108);
        }
        next();
      }
      return std::make_unique<LValAST>(id_name, array_t, std::move(position));
    } else { // LVal: var (id)
      return std::make_unique<LValAST>(id_name, var_t);
    }
  }
  SPDLOG_LOGGER_ERROR(SCLOG, "error 55");
  exit(55);
}

ASTPtr Parser::statement() {
  if (match_token(tag_t::SEMICON)) {
    next(); // ;
    return std::make_unique<StmtAST>(std::make_unique<EmptyAST>());
  } else if (match_token(tag_t::LBRACE)) {
    ASTPtr body = block();
    if (!body) {
      exit(106);
    }
    return std::make_unique<StmtAST>(std::move(body));
  } else if (match_token(tag_t::KW_WHILE)) {
    ASTPtr stmt = while_loop();
    if (!stmt) {
      exit(107);
    }
    return std::make_unique<StmtAST>(std::move(stmt));
  } else if (match_token(tag_t::KW_IF)) {
    ASTPtr stmt = if_else();
    if (!stmt) {
      exit(108);
    }
    return std::make_unique<StmtAST>(std::move(stmt));
  } else if (match_token(tag_t::KW_BREAK) || match_token(tag_t::KW_CONTINUE) ||
             match_token(tag_t::KW_RETURN)) {
    tag_t temp = token->tag;
    next();
    ASTPtr stmt;
    if (token->tag == tag_t::SEMICON) { // break; return; continue;
      Control command;
      switch (temp) {
      case tag_t::KW_BREAK: {
        command = Control::break_c;
        break;
      }
      case tag_t::KW_CONTINUE: {
        command = Control::continue_c;
        break;
      }
      case tag_t::KW_RETURN: {
        command = Control::return_c;
        break;
      }
      default:
        break;
      }
      stmt = std::make_unique<ControlAST>(command);
    } else { // return exp;
      ASTPtr return_exp = binary_add();
      if (!return_exp) {
        exit(109);
      }
      if (!match_token(tag_t::SEMICON)) {
        exit(110);
      }
      stmt = std::make_unique<ControlAST>(Control::return_c,
                                          std::move(return_exp));
    }
    next(); // ;
    return std::make_unique<StmtAST>(std::move(stmt));
  } else {
    ASTPtr exp = binary_add();
    if (!exp) {
      exit(111);
    }
    if (dynamic_cast<LValAST *>(exp.get())) {
      // LVal = exp;
      if (match_token(tag_t::ASSIGN)) {
        next(); // =
        ASTPtr rhs = binary_add();
        if (!rhs) {
          exit(112);
        }
        ASTPtr stmt =
            std::make_unique<AssignAST>(std::move(exp), std::move(rhs));
        if (!match_token(tag_t::SEMICON)) {
          exit(113);
        }
        next(); // ;
        return std::make_unique<StmtAST>(std::move(stmt));
      } else if (match_token(tag_t::SEMICON)) {
        // exp;
        next(); // ;
        return std::make_unique<StmtAST>(std::move(exp));
      } else {
        exit(114);
      }
    } else {
      // exp;
      if (!match_token(tag_t::SEMICON)) {
        exit(115);
      }
      next(); // ;
      return std::make_unique<StmtAST>(std::move(exp));
    }
  }
  exit(56);
}

ASTPtr Parser::if_else() {
  next(); // if () then else
  if (!match_token(tag_t::LPAREN)) {
    exit(116);
  }
  next(); // (
  ASTPtr condition = binary_or();
  if (!condition) {
    exit(117);
  }
  if (!match_token(tag_t::RPAREN)) {
    exit(118);
  }
  next(); // )
  ASTPtr thenStatement = statement();
  if (!thenStatement) {
    exit(118);
  }
  if (match_token(tag_t::KW_ELSE)) {
    next(); // else
    ASTPtr elseStatement = statement();
    if (!elseStatement) {
      exit(119);
    }
    return std::make_unique<IfAST>(std::move(condition),
                                   std::move(thenStatement),
                                   std::move(elseStatement));
  } else {
    return std::make_unique<IfAST>(std::move(condition),
                                   std::move(thenStatement));
  }
  exit(57);
}

ASTPtr Parser::while_loop() {
  next(); // while () stmt
  if (!match_token(tag_t::LPAREN)) {
    exit(116);
  }
  next(); // (
  ASTPtr condition = binary_or();
  if (!condition) {
    exit(117);
  }
  if (!match_token(tag_t::RPAREN)) {
    exit(118);
  }
  next(); // )
  ASTPtr stmt = statement();
  if (!stmt) {
    exit(119);
  }
  return std::make_unique<WhileAST>(std::move(condition), std::move(stmt));
}

ASTPtr Parser::init_val() {
  if (match_token(tag_t::LBRACE)) {
    next();
    if (match_token(tag_t::RBRACE)) {
      next();
      return std::make_unique<InitValAST>(VarType::array_t, ASTPtrList{});
    } else {
      ASTPtrList inits;
      while (true) {
        ASTPtr init = init_val();
        if (!init) {
          SPDLOG_LOGGER_ERROR(SCLOG, "error 999");
          exit(999);
        }
        inits.push_back(std::move(init));
        if (!match_token(tag_t::COMMA))
          break;
        next(); // ,
      }
      if (!match_token(RBRACE)) {
        exit(998);
      }
      next(); // }
      return std::make_unique<InitValAST>(VarType::array_t, std::move(inits));
    }
  } else {
    ASTPtr exp = binary_add();
    if (!exp) {
      SPDLOG_LOGGER_ERROR(SCLOG, "error 1000");
      exit(1000);
    }
    ASTPtrList expList;
    expList.push_back(std::move(exp));
    return std::make_unique<InitValAST>(VarType::var_t, std::move(expList));
  }
}

ASTPtr Parser::var_decl() {
  bool isConst = false;
  if (match_token(tag_t::KW_CONST)) {
    isConst = true;
    next();
  }

  // TODO: only support int here
  if (!match_token(tag_t::KW_INT)) {
    SPDLOG_LOGGER_ERROR(SCLOG, "Only Support Type 'int'.");
    exit(450);
  }
  next();

  ASTPtrList vars;
  ASTPtr varDef = var_def(isConst);
  if (!varDef) {
    exit(451);
  }
  vars.push_back(std::move(varDef));

  while (match_token(tag_t::COMMA)) {
    next(); // ,
    ASTPtr varDef = var_def(isConst);
    if (!varDef) {
      exit(451);
    }
    vars.push_back(std::move(varDef));
  }

  if (!match_token(tag_t::SEMICON)) {
    exit(452);
  }
  next();
  return std::make_unique<VarDeclAST>(isConst, std::move(vars));
}

ASTPtr Parser::var_def(bool _isConst) {
  if (!match_token(tag_t::ID)) {
    exit(452);
  }
  token_identifier_t *token_casted = (token_identifier_t *)token;
  std::string id_name = token_casted->name;
  ASTPtrList dims;
  next(); // id
  while (match_token(tag_t::LBRACKET)) {
    next(); // [
    ASTPtr exp = binary_add();
    if (!exp) {
      exit(453);
    }
    dims.push_back(std::move(exp));
    if (!match_token(tag_t::RBRACKET)) {
      exit(454);
    }
    next(); // ]
  }
  ASTPtr var;
  if (dims.empty())
    var = std::make_unique<IdAST>(id_name, VarType::var_t, _isConst);
  else
    var = std::make_unique<IdAST>(id_name, VarType::array_t, _isConst,
                                  std::move(dims));
  if (match_token(tag_t::ASSIGN)) {
    next(); // =
    ASTPtr init = init_val();
    if (!init) {
      exit(456);
    }
    return std::make_unique<VarDefAST>(_isConst, std::move(var),
                                       std::move(init));
  } else {
    if (_isConst) {
      exit(457);
    }
    return std::make_unique<VarDefAST>(_isConst, std::move(var));
  }
}

ASTPtr Parser::block() {
  next(); // {
  if (match_token(tag_t::RBRACE)) {
    next(); // }
    return std::make_unique<BlockAST>(ASTPtrList{});
  } else {
    ASTPtrList stmts;
    while (!match_token(tag_t::RBRACE)) {
      if (match_token(tag_t::KW_CONST) || match_token(tag_t::KW_INT)) {
        ASTPtr var = var_decl();
        if (!var) {
          exit(460);
        }
        stmts.push_back(std::move(var));
      } else {
        ASTPtr stmt = statement();
        if (!stmt) {
          exit(461);
        }
        stmts.push_back(std::move(stmt));
      }
    }
    next(); // }
    return std::make_unique<BlockAST>(std::move(stmts));
  }
}

ASTPtr Parser::function_def() {
  // function type
  Type type;
  if (match_token(tag_t::KW_INT))
    type = Type::int_t;
  if (match_token(tag_t::KW_CHAR))
    type = Type::char_t;
  if (match_token(tag_t::KW_VOID))
    type = Type::void_t;
  next(); // type
  if (!match_token(tag_t::ID)) {
    exit(999);
  }
  // function name
  token_identifier_t *token_casted = (token_identifier_t *)token;
  std::string id_name = token_casted->name;
  next(); // id
  if (!match_token(tag_t::LPAREN)) {
    exit(998);
  }
  next(); // (
  ASTPtrList args;
  if (!match_token(tag_t::RPAREN)) {
    while (true) {
      // TODO: only support int
      if ((!match_token(tag_t::KW_INT))) {
        exit(996);
      }
      next(); // type
      if (!match_token(tag_t::ID)) {
        exit(998);
      }
      // arg name
      token_identifier_t *token_casted = (token_identifier_t *)token;
      std::string arg_name = token_casted->name;
      next();                         // id
      if (match_token(tag_t::LBRACKET)) // [
      {
        ASTPtrList dim;
        dim.push_back(std::make_unique<NumAST>(0));
        next(); // [
        if (!match_token(tag_t::RBRACKET)) {
          exit(997);
        }
        next(); // ]
        while (match_token(tag_t::LBRACKET)) {
          next(); // [
          ASTPtr _dim = binary_add();
          if (!_dim) {
            exit(995);
          }
          dim.push_back(std::move(_dim));
          if (!match_token(tag_t::RBRACKET)) {
            exit(994);
          }
          next(); // ]
        }
        args.push_back(std::make_unique<IdAST>(arg_name, VarType::array_t,
                                               false, std::move(dim)));
      } else {
        args.push_back(
            std::make_unique<IdAST>(arg_name, VarType::var_t, false));
      }
      if (!match_token(tag_t::COMMA))
        break;
      next(); // ,
    }
    if (!match_token(tag_t::RPAREN)) {
      exit(993);
    }
  }
  next(); // )
  ASTPtr body = block();
  return std::make_unique<FuncDefAST>(type, id_name, std::move(args),
                                      std::move(body));
}

bool Parser::is_done() const { return lexer.is_done(); }