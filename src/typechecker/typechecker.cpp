
/**
 * @file typechecker.cpp
 * @brief type checker
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

#include "typechecker.h"

bool TypeCheck::FillInValue(int *_memory, InitValAST *_init,
                            std::vector<int> &_dim, size_t _i) {
  int idx = 0;
  int i_idx = 0;
  int elem = 1;
  for (int j = _i + 1; j < (int)_dim.size(); j++) {
    elem *= _dim[j];
  }
  if (_init) {
    for (const auto &initval : _init->getValues()) {
      if (dynamic_cast<NumAST *>(initval.get())) {
        idx++;
        if (idx == elem) {
          idx = 0;
          i_idx++;
        }
        *_memory = dynamic_cast<NumAST *>(initval.get())->getVal();
        _memory++;
      } else if (dynamic_cast<InitValAST *>(initval.get())) {
        if (dynamic_cast<InitValAST *>(initval.get())->getType() ==
            VarType::var_t) {
          if (!dynamic_cast<NumAST *>(dynamic_cast<InitValAST *>(initval.get())
                                          ->getValues()[0]
                                          .get())) {
            return false;
          }
          idx++;
          if (idx == elem) {
            idx = 0;
            i_idx++;
          }
          *_memory =
              dynamic_cast<NumAST *>(dynamic_cast<InitValAST *>(initval.get())
                                         ->getValues()[0]
                                         .get())
                  ->getVal();
          _memory++;
        } else {
          if (idx != 0) {
            return false;
          }
          i_idx++;
          if (_i == _dim.size() - 1) {
            return false;
          }
          if (!FillInValue(_memory, dynamic_cast<InitValAST *>(initval.get()),
                           _dim, _i + 1))
            return false;
        }
      } else {
        return false;
      }
    }
    if (i_idx > _dim[_i] || (i_idx == _dim[_i] && idx > 0)) {
      return false;
    }
    for (; i_idx < _dim[_i]; i_idx++) {
      if (_i == _dim.size() - 1) {
        *_memory = 0;
        _memory++;
      } else {
        if (idx > 0) {
          return false;
        }
        if (!FillInValue(_memory, nullptr, _dim, _i + 1))
          return false;
      }
    }
  } else {
    if (_i == _dim.size() - 1) {
      for (int j = 0; j < _dim[_i]; j++) {
        *_memory = 0;
        _memory++;
      }
    } else {
      for (int j = 0; j < _dim[_i]; j++) {
        if (!FillInValue(_memory, nullptr, _dim, _i + 1))
          return false;
      }
    }
  }
  return true;
}

std::unique_ptr<VarDeclAST> TypeCheck::EvalVarDecl(VarDeclAST &_varDecl) {
  if (!(_varDecl.isConst())) {
    ASTPtrList list;
    for (const auto &def : _varDecl.getVarDefs()) {
      auto varDef = def->Eval(*this);
      if (!varDef) {
        return nullptr;
      }
      list.push_back(move(varDef));
    }
    return std::make_unique<VarDeclAST>(_varDecl.isConst(), move(list));
  } else {
    ASTPtrList list;
    for (const auto &def : _varDecl.getVarDefs()) {
      auto nDef = def->Eval(*this);
      if (dynamic_cast<ProcessedIdAST *>(
              dynamic_cast<VarDefAST *>(nDef.get())->getVar().get())
              ->getType() == VarType::array_t) {
        list.push_back(move(nDef));
      }
    }
    return std::make_unique<VarDeclAST>(_varDecl.isConst(), move(list));
  }
}

std::unique_ptr<ProcessedIdAST> TypeCheck::EvalId(IdAST &_id) {
  std::vector<int> ndim;
  for (const auto &exp : _id.getDim()) {
    if (dynamic_cast<NumAST *>(exp.get())) {
      ndim.push_back(dynamic_cast<NumAST *>(exp.get())->getVal());
    } else if (dynamic_cast<BinaryAST *>(exp.get())) {
      auto result = dynamic_cast<BinaryAST *>(exp.get())->Eval(*this);
      if (!dynamic_cast<NumAST *>(result.get())) {
        return nullptr;
      }
      ndim.push_back(dynamic_cast<NumAST *>(result.get())->getVal());
    } else if (dynamic_cast<UnaryAST *>(exp.get())) {
      auto result = dynamic_cast<UnaryAST *>(exp.get())->Eval(*this);
      if (!dynamic_cast<NumAST *>(result.get())) {
        return nullptr;
      }
      ndim.push_back(dynamic_cast<NumAST *>(result.get())->getVal());
    } else if (dynamic_cast<LValAST *>(exp.get())) {
      auto result = dynamic_cast<LValAST *>(exp.get())->Eval(*this);
      if (!dynamic_cast<NumAST *>(result.get())) {
        return nullptr;
      }
      ndim.push_back(dynamic_cast<NumAST *>(result.get())->getVal());
    }
  }
  return std::make_unique<ProcessedIdAST>(_id.getName(), _id.getType(),
                                          _id.isConst(), move(ndim));
}

std::unique_ptr<VarDefAST> TypeCheck::EvalVarDef(VarDefAST &_varDef) {
  if (_varDef.isConst()) {
    if (!_varDef.getInitVal()) {
      return nullptr;
    }
    auto id = _varDef.getVar()->Eval(*this);
    std::string name = dynamic_cast<ProcessedIdAST *>(id.get())->getName();
    VarType type = dynamic_cast<ProcessedIdAST *>(id.get())->getType();
    std::vector<int> ndim = dynamic_cast<ProcessedIdAST *>(id.get())->getDim();
    auto initVal = _varDef.getInitVal()->Eval(*this);
    if (!initVal) {
      return nullptr;
    }
    if (type == VarType::array_t) {
      int size = 1;
      for (auto x : ndim) {
        size *= x;
      }
      int *arrayVal = (int *)malloc(size * sizeof(int));
      int *tmp = arrayVal;
      if (!FillInValue(tmp, dynamic_cast<InitValAST *>(initVal.get()), ndim,
                       0)) {
        return nullptr;
      }
      if (BlockVars[currentBlock].find(name) != BlockVars[currentBlock].end()) {
        return nullptr;
      }
      BlockVars[currentBlock][name] =
          Var(name, VarType::array_t, _varDef.isConst(), ndim);
    } else {
      if (BlockVars[currentBlock].find(name) != BlockVars[currentBlock].end()) {
        return nullptr;
      }
      if (!dynamic_cast<NumAST *>(dynamic_cast<InitValAST *>(initVal.get())
                                      ->getValues()[0]
                                      .get())) {
        return nullptr;
      }
      BlockVars[currentBlock][name] = Var(
          name, VarType::var_t, _varDef.isConst(), std::vector<int>{},
          dynamic_cast<NumAST *>(
              dynamic_cast<InitValAST *>(initVal.get())->getValues()[0].get())
              ->getVal());
    }

    dynamic_cast<InitValAST *>(initVal.get())->setDim(ndim);
    return std::make_unique<VarDefAST>(_varDef.isConst(), move(id),
                                       move(initVal));
  } else {
    auto id = _varDef.getVar()->Eval(*this);
    std::string name = dynamic_cast<ProcessedIdAST *>(id.get())->getName();
    VarType type = dynamic_cast<ProcessedIdAST *>(id.get())->getType();
    std::vector<int> ndim = dynamic_cast<ProcessedIdAST *>(id.get())->getDim();
    if (type == VarType::array_t) {
      int size = 1;
      for (auto x : ndim) {
        size *= x;
      }
      if (BlockVars[currentBlock].find(name) != BlockVars[currentBlock].end()) {
        return nullptr;
      }
      BlockVars[currentBlock][name] =
          Var(name, VarType::array_t, _varDef.isConst(), ndim);
      if (_varDef.getInitVal()) {
        auto initVal = _varDef.getInitVal()->Eval(*this);
        if (!initVal) {
          return nullptr;
        }
        if (currentBlock == 0) {
          int *arrayVal = (int *)malloc(size * sizeof(int));
          int *tmp = arrayVal;
          if (!FillInValue(tmp, dynamic_cast<InitValAST *>(initVal.get()), ndim,
                           0)) {
            return nullptr;
          }
        }
        dynamic_cast<InitValAST *>(initVal.get())->setDim(ndim);
        return std::make_unique<VarDefAST>(_varDef.isConst(), move(id),
                                           move(initVal));
      } else {
        return std::make_unique<VarDefAST>(_varDef.isConst(), move(id));
      }
    } else {
      if (BlockVars[currentBlock].find(name) != BlockVars[currentBlock].end()) {
        return nullptr;
      }
      if (_varDef.getInitVal()) {
        auto initVal = _varDef.getInitVal()->Eval(*this);
        if (!initVal) {
          return nullptr;
        }
        if (currentBlock == 0) {
          if (!dynamic_cast<NumAST *>(dynamic_cast<InitValAST *>(initVal.get())
                                          ->getValues()[0]
                                          .get())) {
            return nullptr;
          }
          BlockVars[currentBlock][name] = Var(
              name, VarType::var_t, _varDef.isConst(), std::vector<int>{},
              dynamic_cast<NumAST *>(dynamic_cast<InitValAST *>(initVal.get())
                                         ->getValues()[0]
                                         .get())
                  ->getVal());
        } else {
          BlockVars[currentBlock][name] =
              Var(name, VarType::var_t, _varDef.isConst(), std::vector<int>{});
        }
        return std::make_unique<VarDefAST>(_varDef.isConst(), move(id),
                                           move(initVal));
      } else {
        if (currentBlock == 0) {
          BlockVars[currentBlock][name] = Var(
              name, VarType::var_t, _varDef.isConst(), std::vector<int>{}, 0);
          ASTPtrList retlist;
          retlist.push_back(std::make_unique<NumAST>(0));
          return std::make_unique<VarDefAST>(
              _varDef.isConst(), move(id),
              std::make_unique<InitValAST>(VarType::var_t, move(retlist)));
        } else {
          BlockVars[currentBlock][name] =
              Var(name, VarType::var_t, _varDef.isConst(), std::vector<int>{});
          return std::make_unique<VarDefAST>(_varDef.isConst(), move(id));
        }
      }
    }
  }
}

std::unique_ptr<FuncCallAST> TypeCheck::EvalFuncCall(FuncCallAST &_func) {
  if (FuncTable.find(_func.getName()) == FuncTable.end()) {
    return nullptr;
  } else {
    if (_func.getArgs().size() != FuncTable[_func.getName()].argTable.size()) {
      return nullptr;
    }
    ASTPtrList newArgs;
    for (size_t i = 0; i < _func.getArgs().size(); i++) {
      auto arg = _func.getArgs()[i]->Eval(*this);
      if (!arg) {
        return nullptr;
      }
      if (FuncTable[_func.getName()].argTable[i].type == VarType::array_t) {
        if (dynamic_cast<NumAST *>(arg.get())) {
          return nullptr;
        }
        if (dynamic_cast<BinaryAST *>(arg.get())) {
          return nullptr;
        }
        if (dynamic_cast<FuncCallAST *>(arg.get())) {
          return nullptr;
        }
        if (dynamic_cast<UnaryAST *>(arg.get())) {
          return nullptr;
        }
        if (dynamic_cast<LValAST *>(arg.get())) {
          int tmpCurrentBlock = currentBlock;
          std::map<std::string, Var>::iterator iter;
          while (tmpCurrentBlock != -1) {
            iter = BlockVars[tmpCurrentBlock].find(
                dynamic_cast<LValAST *>(arg.get())->getName());
            if (iter != BlockVars[tmpCurrentBlock].end()) {
              break;
            }
            tmpCurrentBlock = parentBlock[tmpCurrentBlock];
          }
          if (tmpCurrentBlock == -1) {
            return nullptr;
          }
          if (iter->second.type == VarType::var_t) {
            return nullptr;
          }
          if (iter->second.dims.size() ==
              dynamic_cast<LValAST *>(arg.get())->getPosition().size()) {
            return nullptr;
          }
          for (size_t j =
                   dynamic_cast<LValAST *>(arg.get())->getPosition().size() + 1;
               j < iter->second.dims.size(); j++) {
            if (iter->second.dims[j] !=
                FuncTable[_func.getName()]
                    .argTable[i]
                    .dims[j - dynamic_cast<LValAST *>(arg.get())
                                  ->getPosition()
                                  .size()]) {
              return nullptr;
            }
          }
        }
      } else {
        if (dynamic_cast<LValAST *>(arg.get())) {
          int tmpCurrentBlock = currentBlock;
          std::map<std::string, Var>::iterator iter;
          while (tmpCurrentBlock != -1) {
            iter = BlockVars[tmpCurrentBlock].find(
                dynamic_cast<LValAST *>(arg.get())->getName());
            if (iter != BlockVars[tmpCurrentBlock].end()) {
              break;
            }
            tmpCurrentBlock = parentBlock[tmpCurrentBlock];
          }
          if (tmpCurrentBlock == -1) {
            return nullptr;
          }
          if (iter->second.dims.size() !=
              dynamic_cast<LValAST *>(arg.get())->getPosition().size()) {
            return nullptr;
          }
        }
      }
      newArgs.push_back(move(arg));
    }
    return std::make_unique<FuncCallAST>(_func.getName(), move(newArgs));
  }
}

std::unique_ptr<BlockAST> TypeCheck::EvalBlock(BlockAST &_block) {
  ASTPtrList stmts;
  parentBlock.push_back(currentBlock);
  currentBlock = parentBlock.size() - 1;
  for (const auto &stmt : _block.getStmts()) {
    auto nStmt = stmt->Eval(*this);
    if (!nStmt) {
      return nullptr;
    }
    stmts.push_back(move(nStmt));
  }
  currentBlock = parentBlock[currentBlock];
  return std::make_unique<BlockAST>(move(stmts));
}

std::unique_ptr<IfAST> TypeCheck::EvalIfElse(IfAST &_stmt) {
  auto cond = _stmt.getCond()->Eval(*this);
  if (!cond) {
    return nullptr;
  }
  auto then = _stmt.getThenStmt()->Eval(*this);
  if (!then) {
    return nullptr;
  }
  if (_stmt.getElseStmt()) {
    auto els = _stmt.getElseStmt()->Eval(*this);
    if (!els) {
      return nullptr;
    }
    return std::make_unique<IfAST>(move(cond), move(then), move(els));
  }
  return std::make_unique<IfAST>(move(cond), move(then));
}

std::unique_ptr<WhileAST> TypeCheck::EvalWhile(WhileAST &_stmt) {
  auto cond = _stmt.getCond()->Eval(*this);
  if (!cond) {
    return nullptr;
  }
  auto body = _stmt.getStmt()->Eval(*this);
  if (!body) {
    return nullptr;
  }
  return std::make_unique<WhileAST>(move(cond), move(body));
}

std::unique_ptr<ControlAST> TypeCheck::EvalControl(ControlAST &_stmt) {
  if (_stmt.getControl() == Control::return_c) {
    if (FuncTable[currentFunc].funcType == Type::void_t) {
      if (_stmt.getReturnExp() != nullptr) {
        return nullptr;
      }
    } else {
      if (_stmt.getReturnExp() == nullptr) {
      } else {
        auto exp = _stmt.getReturnExp()->Eval(*this);
        if (!exp) {
          return nullptr;
        }
        return std::make_unique<ControlAST>(_stmt.getControl(), move(exp));
      }
    }
  }
  return std::make_unique<ControlAST>(_stmt.getControl());
}

std::unique_ptr<AssignAST> TypeCheck::EvalAssign(AssignAST &_assign) {
  auto lhs = _assign.getLeft()->Eval(*this);
  if (!lhs) {
    return nullptr;
  }
  if (!dynamic_cast<LValAST *>(lhs.get())) {
    return nullptr;
  }
  int tmpCurrentBlock = currentBlock;
  std::map<std::string, Var>::iterator iter;
  while (tmpCurrentBlock != -1) {
    iter = BlockVars[tmpCurrentBlock].find(
        dynamic_cast<LValAST *>(lhs.get())->getName());
    if (iter != BlockVars[tmpCurrentBlock].end()) {
      break;
    }
    tmpCurrentBlock = parentBlock[tmpCurrentBlock];
  }
  if (tmpCurrentBlock == -1) {
    return nullptr;
  }
  if (iter->second.type == VarType::array_t &&
      iter->second.dims.size() !=
          dynamic_cast<LValAST *>(lhs.get())->getPosition().size()) {
    return nullptr;
  }
  if (iter->second.isConst) {
    return nullptr;
  }
  auto rhs = _assign.getRight()->Eval(*this);
  if (!rhs) {
    return nullptr;
  }
  if (dynamic_cast<LValAST *>(rhs.get())) {
    int tmpCurrentBlock = currentBlock;
    std::map<std::string, Var>::iterator iter;
    while (tmpCurrentBlock != -1) {
      iter = BlockVars[tmpCurrentBlock].find(
          dynamic_cast<LValAST *>(rhs.get())->getName());
      if (iter != BlockVars[tmpCurrentBlock].end()) {
        break;
      }
      tmpCurrentBlock = parentBlock[tmpCurrentBlock];
    }
    if (tmpCurrentBlock == -1) {
      return nullptr;
    }
    if (iter->second.type == VarType::array_t &&
        iter->second.dims.size() !=
            dynamic_cast<LValAST *>(rhs.get())->getPosition().size()) {
      return nullptr;
    }
  }
  return std::make_unique<AssignAST>(move(lhs), move(rhs));
}

ASTPtr TypeCheck::EvalLVal(LValAST &_lval) {
  if (_lval.getType() == VarType::array_t) {
    ASTPtrList pos;
    for (const auto &exp : _lval.getPosition()) {
      auto val = exp->Eval(*this);
      if (!val) {
        return nullptr;
      }
      pos.push_back(move(val));
    }
    const std::string &name = _lval.getName();
    int tmpCurrentBlock = currentBlock;
    std::map<std::string, Var>::iterator iter;
    while (tmpCurrentBlock != -1) {
      iter = BlockVars[tmpCurrentBlock].find(name);
      if (iter != BlockVars[tmpCurrentBlock].end()) {
        break;
      }
      tmpCurrentBlock = parentBlock[tmpCurrentBlock];
    }
    if (tmpCurrentBlock == -1) {
      return nullptr;
    }
    return std::make_unique<LValAST>(_lval.getName(), _lval.getType(),
                                     move(pos));

  } else {
    const std::string &name = _lval.getName();
    int tmpCurrentBlock = currentBlock;
    std::map<std::string, Var>::iterator iter;
    while (tmpCurrentBlock != -1) {
      iter = BlockVars[tmpCurrentBlock].find(name);
      if (iter != BlockVars[tmpCurrentBlock].end()) {
        break;
      }
      tmpCurrentBlock = parentBlock[tmpCurrentBlock];
    }
    if (tmpCurrentBlock == -1) {
      return nullptr;
    }
    if (iter->second.isConst) {
      return std::make_unique<NumAST>(iter->second.val);
    } else {
      return std::make_unique<LValAST>(_lval.getName(), _lval.getType());
    }
  }
}

std::unique_ptr<InitValAST> TypeCheck::EvalInitVal(InitValAST &_init) {
  ASTPtrList newInitVals;
  for (const auto &val : _init.getValues()) {
    auto newVal = val->Eval(*this);
    if (!newVal) {
      return nullptr;
    }
    newInitVals.push_back(move(newVal));
  }
  return std::make_unique<InitValAST>(_init.getType(), move(newInitVals));
}

ASTPtr TypeCheck::EvalAddExp(BinaryAST &_exp) {
  auto lval = _exp.getLHS()->Eval(*this);
  if (!lval) {
    return nullptr;
  }
  auto rval = _exp.getRHS()->Eval(*this);
  if (!rval) {
    return nullptr;
  }
  if (!dynamic_cast<NumAST *>(lval.get()) ||
      !dynamic_cast<NumAST *>(rval.get())) {
    return std::make_unique<BinaryAST>(_exp.getOp(), move(lval), move(rval));
  }
  switch (_exp.getOp()) {
  case Operator::add_op: {
    return std::make_unique<NumAST>(
        dynamic_cast<NumAST *>(lval.get())->getVal() +
        dynamic_cast<NumAST *>(rval.get())->getVal());
  }
  case Operator::sub_op: {
    return std::make_unique<NumAST>(
        dynamic_cast<NumAST *>(lval.get())->getVal() -
        dynamic_cast<NumAST *>(rval.get())->getVal());
  }
  default:
    return nullptr;
  }
}

ASTPtr TypeCheck::EvalMulExp(BinaryAST &_exp) {
  auto lval = _exp.getLHS()->Eval(*this);
  if (!lval) {
    return nullptr;
  }
  auto rval = _exp.getRHS()->Eval(*this);
  if (!rval) {
    return nullptr;
  }
  if (!dynamic_cast<NumAST *>(lval.get()) ||
      !dynamic_cast<NumAST *>(rval.get())) {
    return std::make_unique<BinaryAST>(_exp.getOp(), move(lval), move(rval));
  }
  switch (_exp.getOp()) {
  case Operator::mul_op: {
    return std::make_unique<NumAST>(
        dynamic_cast<NumAST *>(lval.get())->getVal() *
        dynamic_cast<NumAST *>(rval.get())->getVal());
  }
  case Operator::div_op: {
    if (dynamic_cast<NumAST *>(rval.get())->getVal() == 0) {
      return nullptr;
    }
    return std::make_unique<NumAST>(
        dynamic_cast<NumAST *>(lval.get())->getVal() /
        dynamic_cast<NumAST *>(rval.get())->getVal());
  }
  case Operator::mod_op: {
    if (dynamic_cast<NumAST *>(rval.get())->getVal() == 0) {
      return nullptr;
    }
    return std::make_unique<NumAST>(
        dynamic_cast<NumAST *>(lval.get())->getVal() %
        dynamic_cast<NumAST *>(rval.get())->getVal());
  }
  default:
    return nullptr;
  }
}

ASTPtr TypeCheck::EvalRelExp(BinaryAST &_exp) {
  auto lhs = _exp.getLHS()->Eval(*this);
  if (!lhs) {
    return nullptr;
  }
  auto rhs = _exp.getRHS()->Eval(*this);
  if (!rhs) {
    return nullptr;
  }
  if (!dynamic_cast<NumAST *>(lhs.get()) ||
      !dynamic_cast<NumAST *>(rhs.get())) {
    return std::make_unique<BinaryAST>(_exp.getOp(), move(lhs), move(rhs));
  }
  switch (_exp.getOp()) {
  case Operator::lt_op:
    return std::make_unique<NumAST>(
        dynamic_cast<NumAST *>(lhs.get())->getVal() <
        dynamic_cast<NumAST *>(rhs.get())->getVal());
  case Operator::gt_op:
    return std::make_unique<NumAST>(
        dynamic_cast<NumAST *>(lhs.get())->getVal() >
        dynamic_cast<NumAST *>(rhs.get())->getVal());
  case Operator::le_op:
    return std::make_unique<NumAST>(
        dynamic_cast<NumAST *>(lhs.get())->getVal() <=
        dynamic_cast<NumAST *>(rhs.get())->getVal());
  case Operator::ge_op:
    return std::make_unique<NumAST>(
        dynamic_cast<NumAST *>(lhs.get())->getVal() >=
        dynamic_cast<NumAST *>(rhs.get())->getVal());
  default:
    return nullptr;
  }
}

ASTPtr TypeCheck::EvalLAndExp(BinaryAST &_exp) {
  auto lhs = _exp.getLHS()->Eval(*this);
  if (dynamic_cast<NumAST *>(lhs.get()) &&
      dynamic_cast<NumAST *>(lhs.get())->getVal() == 0) {
    return std::make_unique<NumAST>(0);
  }
  auto rhs = _exp.getRHS()->Eval(*this);
  if (!rhs) {
    return nullptr;
  }
  if (dynamic_cast<NumAST *>(rhs.get()) &&
      dynamic_cast<NumAST *>(lhs.get())->getVal()) {
    return std::make_unique<NumAST>(1);
  } else if (dynamic_cast<NumAST *>(rhs.get())) {
    return std::make_unique<NumAST>(0);
  } else {
    return std::make_unique<BinaryAST>(Operator::and_op, move(lhs), move(rhs));
  }
}

ASTPtr TypeCheck::EvalLOrExp(BinaryAST &_exp) {
  auto lhs = _exp.getLHS()->Eval(*this);
  if (dynamic_cast<NumAST *>(lhs.get()) &&
      dynamic_cast<NumAST *>(lhs.get())->getVal()) {
    return std::make_unique<NumAST>(1);
  }
  auto rhs = _exp.getRHS()->Eval(*this);
  if (!rhs) {
    return nullptr;
  }
  if (dynamic_cast<NumAST *>(rhs.get()) &&
      dynamic_cast<NumAST *>(rhs.get())->getVal()) {
    return std::make_unique<NumAST>(1);
  } else if (dynamic_cast<NumAST *>(rhs.get())) {
    return std::make_unique<NumAST>(0);
  } else {
    return std::make_unique<BinaryAST>(Operator::or_op, move(lhs), move(rhs));
  }
}

ASTPtr TypeCheck::EvalEqExp(BinaryAST &_exp) {
  auto lhs = _exp.getLHS()->Eval(*this);
  if (!lhs) {
    return nullptr;
  }
  auto rhs = _exp.getRHS()->Eval(*this);
  if (!rhs) {
    return nullptr;
  }
  if (!dynamic_cast<NumAST *>(lhs.get()) ||
      !dynamic_cast<NumAST *>(rhs.get())) {
    return std::make_unique<BinaryAST>(_exp.getOp(), move(lhs), move(rhs));
  }
  switch (_exp.getOp()) {
  case Operator::equ_op:
    return std::make_unique<NumAST>(
        dynamic_cast<NumAST *>(lhs.get())->getVal() ==
        dynamic_cast<NumAST *>(rhs.get())->getVal());
  case Operator::nequ_op:
    return std::make_unique<NumAST>(
        dynamic_cast<NumAST *>(lhs.get())->getVal() !=
        dynamic_cast<NumAST *>(rhs.get())->getVal());
  default:
    return nullptr;
  }
}

ASTPtr TypeCheck::EvalUnaryExp(UnaryAST &_exp) {
  if (dynamic_cast<NumAST *>(_exp.getNode().get())) {
    if (_exp.getOp() == Operator::ERROR) {
      return std::make_unique<NumAST>(
          dynamic_cast<NumAST *>(_exp.getNode().get())->getVal());
    } else {
      switch (_exp.getOp()) {
      case Operator::not_op:
        return std::make_unique<NumAST>(
            !dynamic_cast<NumAST *>(_exp.getNode().get())->getVal());
      case Operator::sub_op:
        return std::make_unique<NumAST>(
            -dynamic_cast<NumAST *>(_exp.getNode().get())->getVal());
      case Operator::add_op:
        return std::make_unique<NumAST>(
            dynamic_cast<NumAST *>(_exp.getNode().get())->getVal());
      default:
        return nullptr;
      }
    }
  } else {
    auto lval = _exp.getNode()->Eval(*this);
    if (!lval) {
      return nullptr;
    }
    if (dynamic_cast<NumAST *>(lval.get())) {
      int value = dynamic_cast<NumAST *>(lval.get())->getVal();
      switch (_exp.getOp()) {
      case Operator::not_op:
        return std::make_unique<NumAST>(!value);
      case Operator::sub_op:
        return std::make_unique<NumAST>(-value);
      case Operator::add_op:
        return std::make_unique<NumAST>(+value);
      case Operator::ERROR:
        return std::make_unique<NumAST>(value);
      default:
        return nullptr;
      }
    } else {
      return std::make_unique<UnaryAST>(move(lval), _exp.getOp());
    }
  }
}

std::unique_ptr<FuncDefAST> TypeCheck::EvalFuncDef(FuncDefAST &_funcDef) {
  currentFunc = _funcDef.getName();
  ASTPtrList newArgs;
  std::vector<Var> args;
  for (const auto &arg : _funcDef.getArgs()) {
    if (dynamic_cast<IdAST *>(arg.get())->getType() == VarType::array_t) {
      std::vector<int> dims;
      for (const auto &exp : dynamic_cast<IdAST *>(arg.get())->getDim()) {
        auto res = exp->Eval(*this);
        if (!res || !dynamic_cast<NumAST *>(res.get())) {
          return nullptr;
        }
        dims.push_back(dynamic_cast<NumAST *>(res.get())->getVal());
      }
      newArgs.push_back(std::make_unique<ProcessedIdAST>(
          dynamic_cast<IdAST *>(arg.get())->getName(), VarType::array_t, false,
          dims));
      args.emplace_back(dynamic_cast<IdAST *>(arg.get())->getName(),
                        VarType::array_t, false, dims);
    } else {
      newArgs.push_back(std::make_unique<ProcessedIdAST>(
          dynamic_cast<IdAST *>(arg.get())->getName(), VarType::var_t, false));
      args.emplace_back(dynamic_cast<IdAST *>(arg.get())->getName(),
                        VarType::var_t, false);
    }
    BlockVars[parentBlock.size()][dynamic_cast<IdAST *>(arg.get())->getName()] =
        args[args.size() - 1];
  }
  if (FuncTable.find(_funcDef.getName()) != FuncTable.end()) {
    return nullptr;
  }
  FuncTable[_funcDef.getName()] =
      Function(_funcDef.getName(), _funcDef.getType(), args);
  ASTPtr block = _funcDef.getBody()->Eval(*this);
  if (!block) {
    return nullptr;
  }
  currentFunc = "";
  return std::make_unique<FuncDefAST>(_funcDef.getType(), _funcDef.getName(),
                                      move(newArgs), move(block));
}

std::unique_ptr<CompUnitAST> TypeCheck::EvalCompUnit(CompUnitAST &_unit) {
  ASTPtrList newNodes;
  parentBlock.push_back(-1);
  for (const auto &node : _unit.getNodes()) {
    auto newNode = node->Eval(*this);
    if (!newNode) {
      return nullptr;
    }
    newNodes.push_back(move(newNode));
  }
  if (FuncTable.find("main") == FuncTable.end()) {
    return nullptr;
  }
  return std::make_unique<CompUnitAST>(move(newNodes));
}

std::unique_ptr<StmtAST> TypeCheck::EvalStmt(StmtAST &_stmt) {
  if (!_stmt.getStmt())
    return std::make_unique<StmtAST>(nullptr);
  else {
    auto nStmt = _stmt.getStmt()->Eval(*this);
    if (!nStmt) {
      return nullptr;
    }
    return std::make_unique<StmtAST>(move(nStmt));
  }
}

std::unique_ptr<NumAST> TypeCheck::EvalNumber(NumAST &_num) {
  return std::make_unique<NumAST>(_num.getVal());
}

std::unique_ptr<EmptyAST> TypeCheck::EvalEmpty() {
  return std::make_unique<EmptyAST>();
}

std::unique_ptr<ProcessedIdAST>
TypeCheck::EvalProcessedId(ProcessedIdAST &_id) {
  return std::make_unique<ProcessedIdAST>(_id);
}