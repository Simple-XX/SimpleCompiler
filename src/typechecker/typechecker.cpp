
// This file is a part of Simple-XX/SimpleCompiler
// (https://github.com/Simple-XX/SimpleCompiler).
//
// typechecker.cpp for Simple-XX/SimpleCompiler.

#include "typechecker.h"

bool TypeCheck::FillInValue(int *memory, InitValAST *init, vector<int> &dim, size_t i) {
    int idx = 0;
    int i_idx = 0;
    int elem = 1;
    for (int j = i + 1; j < (int)dim.size(); j++) {
        elem *= dim[j];
    }
    if (init) {
        for (const auto &initval: init->getValues()) {
            if (dynamic_cast<NumAST *>(initval.get())) {
                idx++;
                if (idx == elem) {
                    idx = 0;
                    i_idx++;
                }
                *memory = dynamic_cast<NumAST *>(initval.get())->getVal();
                memory++;
            } else if (dynamic_cast<InitValAST *>(initval.get())) {
                if (dynamic_cast<InitValAST *>(initval.get())->getType() == VarType::var_t) {
                    if (!dynamic_cast<NumAST *>(dynamic_cast<InitValAST *>(initval.get())->getValues()[0].get())) {
                        return false;
                    }
                    idx++;
                    if (idx == elem) {
                        idx = 0;
                        i_idx++;
                    }
                    *memory = dynamic_cast<NumAST *>(dynamic_cast<InitValAST *>(initval.get())->getValues()[0].get())->getVal();
                    memory++;
                } else {
                    if (idx != 0) {
                        return false;
                    }
                    i_idx++;
                    if (i == dim.size() - 1) {
                        return false;
                    }
                    if (!FillInValue(memory, dynamic_cast<InitValAST *>(initval.get()), dim, i + 1))
                        return false;
                }
            } else {
                return false;
            }
        }
        if (i_idx > dim[i] || (i_idx == dim[i] && idx > 0)) {
            return false;
        }
        for (; i_idx < dim[i]; i_idx++) {
            if (i == dim.size() - 1) {
                *memory = 0;
                memory++;
            } else {
                if (idx > 0) {
                    return false;
                }
                if (!FillInValue(memory, nullptr, dim, i + 1))
                    return false;
            }
        }
    } else {
        if (i == dim.size() - 1) {
            for (int j = 0; j < dim[i]; j++) {
                *memory = 0;
                memory++;
            }
        } else {
            for (int j = 0; j < dim[i]; j++) {
                if (!FillInValue(memory, nullptr, dim, i + 1))
                    return false;
            }
        }
    }
    return true;
}

unique_ptr<VarDeclAST> TypeCheck::EvalVarDecl(VarDeclAST &varDecl) {
    if (!(varDecl.isConst())) {
        ASTPtrList list;
        for (const auto &def: varDecl.getVarDefs()) {
            auto varDef = def->Eval(*this);
            if (!varDef) {
                return nullptr;
            }
            list.push_back(move(varDef));
        }
        return make_unique<VarDeclAST>(varDecl.isConst(), move(list));
    } else {
        ASTPtrList list;
        for (const auto &def: varDecl.getVarDefs()) {
            auto nDef = def->Eval(*this);
            if (dynamic_cast<ProcessedIdAST *>(dynamic_cast<VarDefAST *>(nDef.get())->getVar().get())->getType() ==
                VarType::array_t) {
                list.push_back(move(nDef));
            }
        }
        return make_unique<VarDeclAST>(varDecl.isConst(), move(list));
    }
}

unique_ptr<ProcessedIdAST> TypeCheck::EvalId(IdAST &id) {
    vector<int> ndim;
    for (const auto &exp: id.getDim()) {
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
    return make_unique<ProcessedIdAST>(id.getName(), id.getType(), id.isConst(), move(ndim));
}

unique_ptr<VarDefAST> TypeCheck::EvalVarDef(VarDefAST &varDef) {
    if (varDef.isConst()) {
        if (!varDef.getInitVal()) {
            return nullptr;
        }
        auto id = varDef.getVar()->Eval(*this);
        string name = dynamic_cast<ProcessedIdAST *>(id.get())->getName();
        VarType type = dynamic_cast<ProcessedIdAST *>(id.get())->getType();
        vector<int> ndim = dynamic_cast<ProcessedIdAST *>(id.get())->getDim();
        auto initVal = varDef.getInitVal()->Eval(*this);
        if (!initVal) {
            return nullptr;
        }
        if (type == VarType::array_t) {
            int size = 1;
            for (auto x: ndim) {
                size *= x;
            }
            int *arrayVal = (int *) malloc(size * sizeof(int));
            int *tmp = arrayVal;
            if (!FillInValue(tmp, dynamic_cast<InitValAST *>(initVal.get()), ndim, 0)) {
                return nullptr;
            }
            if (BlockVars[currentBlock].find(name) != BlockVars[currentBlock].end()) {
                return nullptr;
            }
            BlockVars[currentBlock][name] = Var(name, VarType::array_t, varDef.isConst(), ndim);
        } else {
            if (BlockVars[currentBlock].find(name) != BlockVars[currentBlock].end()) {
                return nullptr;
            }
            if (!dynamic_cast<NumAST *>(dynamic_cast<InitValAST *>(initVal.get())->getValues()[0].get())) {
                return nullptr;
            }
            BlockVars[currentBlock][name] = Var(name, VarType::var_t, varDef.isConst(), vector<int>{},
                                                dynamic_cast<NumAST *>(dynamic_cast<InitValAST *>(initVal.get())->getValues()[0].get())->getVal());
        }

        dynamic_cast<InitValAST *>(initVal.get())->setDim(ndim);
        return make_unique<VarDefAST>(varDef.isConst(), move(id), move(initVal));
    } else {
        auto id = varDef.getVar()->Eval(*this);
        string name = dynamic_cast<ProcessedIdAST *>(id.get())->getName();
        VarType type = dynamic_cast<ProcessedIdAST *>(id.get())->getType();
        vector<int> ndim = dynamic_cast<ProcessedIdAST *>(id.get())->getDim();
        if (type == VarType::array_t) {
            int size = 1;
            for (auto x: ndim) {
                size *= x;
            }
            if (BlockVars[currentBlock].find(name) != BlockVars[currentBlock].end()) {
                return nullptr;
            }
            BlockVars[currentBlock][name] = Var(name, VarType::array_t, varDef.isConst(), ndim);
            if (varDef.getInitVal()) {
                auto initVal = varDef.getInitVal()->Eval(*this);
                if (!initVal) {
                    return nullptr;
                }
                if (currentBlock == 0) {
                    int *arrayVal = (int *) malloc(size * sizeof(int));
                    int *tmp = arrayVal;
                    if (!FillInValue(tmp, dynamic_cast<InitValAST *>(initVal.get()), ndim, 0)) {
                        return nullptr;
                    }
                }
                dynamic_cast<InitValAST *>(initVal.get())->setDim(ndim);
                return make_unique<VarDefAST>(varDef.isConst(), move(id), move(initVal));
            } else {
                return make_unique<VarDefAST>(varDef.isConst(), move(id));
            }
        } else {
            if (BlockVars[currentBlock].find(name) != BlockVars[currentBlock].end()) {
                return nullptr;
            }
            if (varDef.getInitVal()) {
                auto initVal = varDef.getInitVal()->Eval(*this);
                if (!initVal) {
                    return nullptr;
                }
                if (currentBlock == 0) {
                    if (!dynamic_cast<NumAST *>(dynamic_cast<InitValAST *>(initVal.get())->getValues()[0].get())) {
                        return nullptr;
                    }
                    BlockVars[currentBlock][name] = Var(name, VarType::var_t, varDef.isConst(), vector<int>{},
                                                        dynamic_cast<NumAST *>(dynamic_cast<InitValAST *>(initVal.get())->getValues()[0].get())->getVal());
                } else {
                    BlockVars[currentBlock][name] = Var(name, VarType::var_t, varDef.isConst(), vector<int>{});
                }
                return make_unique<VarDefAST>(varDef.isConst(), move(id), move(initVal));
            } else {
                if (currentBlock == 0) {
                    BlockVars[currentBlock][name] = Var(name, VarType::var_t, varDef.isConst(), vector<int>{},
                                                        0);
                    ASTPtrList retlist;
                    retlist.push_back(make_unique<NumAST>(0));
                    return make_unique<VarDefAST>(varDef.isConst(), move(id),
                                                        make_unique<InitValAST>(VarType::var_t,
                                                                                    move(retlist)));
                } else {
                    BlockVars[currentBlock][name] = Var(name, VarType::var_t, varDef.isConst(), vector<int>{});
                    return make_unique<VarDefAST>(varDef.isConst(), move(id));
                }
            }
        }
    }
}

unique_ptr<FuncCallAST> TypeCheck::EvalFuncCall(FuncCallAST &func) {
    if (FuncTable.find(func.getName()) == FuncTable.end()) {
        return nullptr;
    } else {
        if (func.getArgs().size() != FuncTable[func.getName()].argTable.size()) {
            return nullptr;
        }
        ASTPtrList newArgs;
        for (size_t i = 0; i < func.getArgs().size(); i++) {
            auto arg = func.getArgs()[i]->Eval(*this);
            if (!arg) {
                return nullptr;
            }
            if (FuncTable[func.getName()].argTable[i].type == VarType::array_t) {
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
                    map<string, Var>::iterator iter;
                    while (tmpCurrentBlock != -1) {
                        iter = BlockVars[tmpCurrentBlock].find(dynamic_cast<LValAST *>(arg.get())->getName());
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
                    if (iter->second.dims.size() == dynamic_cast<LValAST *>(arg.get())->getPosition().size()) {
                        return nullptr;
                    }
                    for (size_t j = dynamic_cast<LValAST *>(arg.get())->getPosition().size() + 1;
                            j < iter->second.dims.size(); j++) {
                        if (iter->second.dims[j] != FuncTable[func.getName()].argTable[i].dims[j -
                                                                                                dynamic_cast<LValAST *>(arg.get())->getPosition().size()]) {
                            return nullptr;
                        }
                    }
                }
            } else {
                if (dynamic_cast<LValAST *>(arg.get())) {
                    int tmpCurrentBlock = currentBlock;
                    map<string, Var>::iterator iter;
                    while (tmpCurrentBlock != -1) {
                        iter = BlockVars[tmpCurrentBlock].find(dynamic_cast<LValAST *>(arg.get())->getName());
                        if (iter != BlockVars[tmpCurrentBlock].end()) {
                            break;
                        }
                        tmpCurrentBlock = parentBlock[tmpCurrentBlock];
                    }
                    if (tmpCurrentBlock == -1) {
                        return nullptr;
                    }
                    if (iter->second.dims.size() != dynamic_cast<LValAST *>(arg.get())->getPosition().size()) {
                        return nullptr;
                    }
                }
            }
            newArgs.push_back(move(arg));
        }
        return make_unique<FuncCallAST>(func.getName(), move(newArgs));
    }
}

unique_ptr<BlockAST> TypeCheck::EvalBlock(BlockAST &block) {
    ASTPtrList stmts;
    parentBlock.push_back(currentBlock);
    currentBlock = parentBlock.size() - 1;
    for (const auto &stmt: block.getStmts()) {
        auto nStmt = stmt->Eval(*this);
        if (!nStmt) {
            return nullptr;
        }
        stmts.push_back(move(nStmt));

    }
    currentBlock = parentBlock[currentBlock];
    return make_unique<BlockAST>(move(stmts));
}

unique_ptr<IfAST> TypeCheck::EvalIfElse(IfAST &stmt) {
    auto cond = stmt.getCond()->Eval(*this);
    if (!cond) {
        return nullptr;
    }
    auto then = stmt.getThenStmt()->Eval(*this);
    if (!then) {
        return nullptr;
    }
    if (stmt.getElseStmt()) {
        auto els = stmt.getElseStmt()->Eval(*this);
        if (!els) {
            return nullptr;
        }
        return make_unique<IfAST>(move(cond), move(then), move(els));
    }
    return make_unique<IfAST>(move(cond), move(then));
}

unique_ptr<WhileAST> TypeCheck::EvalWhile(WhileAST &stmt) {
    auto cond = stmt.getCond()->Eval(*this);
    if (!cond) {
        return nullptr;
    }
    auto body = stmt.getStmt()->Eval(*this);
    if (!body) {
        return nullptr;
    }
    return make_unique<WhileAST>(move(cond), move(body));
}

unique_ptr<ControlAST> TypeCheck::EvalControl(ControlAST &stmt) {
    if (stmt.getControl() == Control::return_c) {
        if (FuncTable[currentFunc].funcType == Type::void_t) {
            if (stmt.getReturnExp() != nullptr) {
                return nullptr;
            }
        } else {
            if (stmt.getReturnExp() == nullptr) {
            } else {
                auto exp = stmt.getReturnExp()->Eval(*this);
                if (!exp) {
                    return nullptr;
                }
                return make_unique<ControlAST>(stmt.getControl(), move(exp));
            }
        }
    }
    return make_unique<ControlAST>(stmt.getControl());
}

unique_ptr<AssignAST> TypeCheck::EvalAssign(AssignAST &assign) {
    auto lhs = assign.getLeft()->Eval(*this);
    if (!lhs) {
        return nullptr;
    }
    if (!dynamic_cast<LValAST *>(lhs.get())) {
        return nullptr;
    }
    int tmpCurrentBlock = currentBlock;
    map<string, Var>::iterator iter;
    while (tmpCurrentBlock != -1) {
        iter = BlockVars[tmpCurrentBlock].find(dynamic_cast<LValAST *>(lhs.get())->getName());
        if (iter != BlockVars[tmpCurrentBlock].end()) {
            break;
        }
        tmpCurrentBlock = parentBlock[tmpCurrentBlock];
    }
    if (tmpCurrentBlock == -1) {
        return nullptr;
    }
    if (iter->second.type == VarType::array_t &&
        iter->second.dims.size() != dynamic_cast<LValAST *>(lhs.get())->getPosition().size()) {
        return nullptr;
    }
    if (iter->second.isConst) {
        return nullptr;
    }
    auto rhs = assign.getRight()->Eval(*this);
    if (!rhs) {
        return nullptr;
    }
    if (dynamic_cast<LValAST *>(rhs.get())) {
        int tmpCurrentBlock = currentBlock;
        map<string, Var>::iterator iter;
        while (tmpCurrentBlock != -1) {
            iter = BlockVars[tmpCurrentBlock].find(dynamic_cast<LValAST *>(rhs.get())->getName());
            if (iter != BlockVars[tmpCurrentBlock].end()) {
                break;
            }
            tmpCurrentBlock = parentBlock[tmpCurrentBlock];
        }
        if (tmpCurrentBlock == -1) {
            return nullptr;
        }
        if (iter->second.type == VarType::array_t &&
            iter->second.dims.size() != dynamic_cast<LValAST *>(rhs.get())->getPosition().size()) {
            return nullptr;
        }
    }
    return make_unique<AssignAST>(move(lhs), move(rhs));
}

ASTPtr TypeCheck::EvalLVal(LValAST &lval) {
    if (lval.getType() == VarType::array_t) {
        ASTPtrList pos;
        for (const auto &exp: lval.getPosition()) {
            auto val = exp->Eval(*this);
            if (!val) {
                return nullptr;
            }
            pos.push_back(move(val));
        }
        const string &name = lval.getName();
        int tmpCurrentBlock = currentBlock;
        map<string, Var>::iterator iter;
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
        return make_unique<LValAST>(lval.getName(), lval.getType(), move(pos));

    } else {
        const string &name = lval.getName();
        int tmpCurrentBlock = currentBlock;
        map<string, Var>::iterator iter;
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
            return make_unique<NumAST>(iter->second.val);
        } else {
            return make_unique<LValAST>(lval.getName(), lval.getType());
        }
    }
}

unique_ptr<InitValAST> TypeCheck::EvalInitVal(InitValAST &init) {
    ASTPtrList newInitVals;
    for (const auto &val: init.getValues()) {
        auto newVal = val->Eval(*this);
        if (!newVal) {
            return nullptr;
        }
        newInitVals.push_back(move(newVal));
    }
    return make_unique<InitValAST>(init.getType(), move(newInitVals));
}

ASTPtr TypeCheck::EvalAddExp(BinaryAST &exp) {
    auto lval = exp.getLHS()->Eval(*this);
    if (!lval) {
        return nullptr;
    }
    auto rval = exp.getRHS()->Eval(*this);
    if (!rval) {
        return nullptr;
    }
    if (!dynamic_cast<NumAST *>(lval.get()) || !dynamic_cast<NumAST *>(rval.get())) {
        return make_unique<BinaryAST>(exp.getOp(), move(lval), move(rval));
    }
    switch (exp.getOp()) {
        case Operator::add_op: {
            return make_unique<NumAST>(
                    dynamic_cast<NumAST *>(lval.get())->getVal()
                    + dynamic_cast<NumAST *>(rval.get())->getVal());
        }
        case Operator::sub_op: {
            return make_unique<NumAST>(
                    dynamic_cast<NumAST *>(lval.get())->getVal()
                    - dynamic_cast<NumAST *>(rval.get())->getVal());
        }
        default:
            return nullptr;
    }
}

ASTPtr TypeCheck::EvalMulExp(BinaryAST &exp) {
    auto lval = exp.getLHS()->Eval(*this);
    if (!lval) {
        return nullptr;
    }
    auto rval = exp.getRHS()->Eval(*this);
    if (!rval) {
        return nullptr;
    }
    if (!dynamic_cast<NumAST *>(lval.get()) || !dynamic_cast<NumAST *>(rval.get())) {
        return make_unique<BinaryAST>(exp.getOp(), move(lval), move(rval));
    }
    switch (exp.getOp()) {
        case Operator::mul_op: {
            return make_unique<NumAST>(
                    dynamic_cast<NumAST *>(lval.get())->getVal()
                    * dynamic_cast<NumAST *>(rval.get())->getVal());
        }
        case Operator::div_op: {
            if (dynamic_cast<NumAST *>(rval.get())->getVal() == 0) {
                return nullptr;
            }
            return make_unique<NumAST>(
                    dynamic_cast<NumAST *>(lval.get())->getVal()
                    / dynamic_cast<NumAST *>(rval.get())->getVal());
        }
        case Operator::mod_op: {
            if (dynamic_cast<NumAST *>(rval.get())->getVal() == 0) {
                return nullptr;
            }
            return make_unique<NumAST>(
                    dynamic_cast<NumAST *>(lval.get())->getVal()
                    % dynamic_cast<NumAST *>(rval.get())->getVal());
        }
        default:
            return nullptr;
    }
}

ASTPtr TypeCheck::EvalRelExp(BinaryAST &exp) {
    auto lhs = exp.getLHS()->Eval(*this);
    if (!lhs) {
        return nullptr;
    }
    auto rhs = exp.getRHS()->Eval(*this);
    if (!rhs) {
        return nullptr;
    }
    if (!dynamic_cast<NumAST *>(lhs.get()) || !dynamic_cast<NumAST *>(rhs.get())) {
        return make_unique<BinaryAST>(exp.getOp(), move(lhs), move(rhs));
    }
    switch (exp.getOp()) {
        case Operator::lt_op:
            return make_unique<NumAST>(dynamic_cast<NumAST *>(lhs.get())->getVal() <
                                                dynamic_cast<NumAST *>(rhs.get())->getVal());
        case Operator::gt_op:
            return make_unique<NumAST>(dynamic_cast<NumAST *>(lhs.get())->getVal() >
                                                dynamic_cast<NumAST *>(rhs.get())->getVal());
        case Operator::le_op:
            return make_unique<NumAST>(dynamic_cast<NumAST *>(lhs.get())->getVal() <=
                                                dynamic_cast<NumAST *>(rhs.get())->getVal());
        case Operator::ge_op:
            return make_unique<NumAST>(dynamic_cast<NumAST *>(lhs.get())->getVal() >=
                                                dynamic_cast<NumAST *>(rhs.get())->getVal());
        default:
            return nullptr;
    }
}

ASTPtr TypeCheck::EvalLAndExp(BinaryAST &exp) {
    auto lhs = exp.getLHS()->Eval(*this);
    if (dynamic_cast<NumAST *>(lhs.get()) && dynamic_cast<NumAST *>(lhs.get())->getVal() == 0) {
        return make_unique<NumAST>(0);
    }
    auto rhs = exp.getRHS()->Eval(*this);
    if (!rhs) {
        return nullptr;
    }
    if (dynamic_cast<NumAST *>(rhs.get()) && dynamic_cast<NumAST *>(lhs.get())->getVal()) {
        return make_unique<NumAST>(1);
    } else if (dynamic_cast<NumAST *>(rhs.get())) {
        return make_unique<NumAST>(0);
    } else {
        return make_unique<BinaryAST>(Operator::and_op, move(lhs), move(rhs));
    }
}

ASTPtr TypeCheck::EvalLOrExp(BinaryAST &exp) {
    auto lhs = exp.getLHS()->Eval(*this);
    if (dynamic_cast<NumAST *>(lhs.get()) && dynamic_cast<NumAST *>(lhs.get())->getVal()) {
        return make_unique<NumAST>(1);
    }
    auto rhs = exp.getRHS()->Eval(*this);
    if (!rhs) {
        return nullptr;
    }
    if (dynamic_cast<NumAST *>(rhs.get()) && dynamic_cast<NumAST *>(rhs.get())->getVal()) {
        return make_unique<NumAST>(1);
    } else if (dynamic_cast<NumAST *>(rhs.get())) {
        return make_unique<NumAST>(0);
    } else {
        return make_unique<BinaryAST>(Operator::or_op, move(lhs), move(rhs));
    }
}

ASTPtr TypeCheck::EvalEqExp(BinaryAST &exp) {
    auto lhs = exp.getLHS()->Eval(*this);
    if (!lhs) {
        return nullptr;
    }
    auto rhs = exp.getRHS()->Eval(*this);
    if (!rhs) {
        return nullptr;
    }
    if (!dynamic_cast<NumAST *>(lhs.get()) || !dynamic_cast<NumAST *>(rhs.get())) {
        return make_unique<BinaryAST>(exp.getOp(), move(lhs), move(rhs));
    }
    switch (exp.getOp()) {
        case Operator::equ_op:
            return make_unique<NumAST>(
                    dynamic_cast<NumAST *>(lhs.get())->getVal() ==
                    dynamic_cast<NumAST *>(rhs.get())->getVal());
        case Operator::nequ_op:
            return make_unique<NumAST>(
                    dynamic_cast<NumAST *>(lhs.get())->getVal() !=
                    dynamic_cast<NumAST *>(rhs.get())->getVal());
        default:
            return nullptr;
    }
}

ASTPtr TypeCheck::EvalUnaryExp(UnaryAST &exp) {
    if (dynamic_cast<NumAST *>(exp.getNode().get())) {
        if (exp.getOp() == Operator::ERROR) {
            return make_unique<NumAST>(dynamic_cast<NumAST *>(exp.getNode().get())->getVal());
        } else {
            switch (exp.getOp()) {
                case Operator::not_op:
                    return make_unique<NumAST>(!dynamic_cast<NumAST *>(exp.getNode().get())->getVal());
                case Operator::sub_op:
                    return make_unique<NumAST>(-dynamic_cast<NumAST *>(exp.getNode().get())->getVal());
                case Operator::add_op:
                    return make_unique<NumAST>(dynamic_cast<NumAST *>(exp.getNode().get())->getVal());
                default:
                    return nullptr;
            }
        }
    } else {
        auto lval = exp.getNode()->Eval(*this);
        if (!lval) {
            return nullptr;
        }
        if (dynamic_cast<NumAST *>(lval.get())) {
            int value = dynamic_cast<NumAST *>(lval.get())->getVal();
            switch (exp.getOp()) {
                case Operator::not_op:
                    return make_unique<NumAST>(!value);
                case Operator::sub_op:
                    return make_unique<NumAST>(-value);
                case Operator::add_op:
                    return make_unique<NumAST>(+value);
                case Operator::ERROR:
                    return make_unique<NumAST>(value);
                default:
                    return nullptr;
            }
        } else {
            return make_unique<UnaryAST>(move(lval), exp.getOp());
        }
    }
}

unique_ptr<FuncDefAST> TypeCheck::EvalFuncDef(FuncDefAST &funcDef) {
    currentFunc = funcDef.getName();
    ASTPtrList newArgs;
    vector<Var> args;
    for (const auto &arg: funcDef.getArgs()) {
        if (dynamic_cast<IdAST *>(arg.get())->getType() == VarType::array_t) {
            vector<int> dims;
            for (const auto &exp: dynamic_cast<IdAST *>(arg.get())->getDim()) {
                auto res = exp->Eval(*this);
                if (!res || !dynamic_cast<NumAST *>(res.get())) {
                    return nullptr;
                }
                dims.push_back(dynamic_cast<NumAST *>(res.get())->getVal());
            }
            newArgs.push_back(
                    make_unique<ProcessedIdAST>(dynamic_cast<IdAST *>(arg.get())->getName(),
                                                        VarType::array_t, false, dims));
            args.emplace_back(dynamic_cast<IdAST *>(arg.get())->getName(), VarType::array_t, false, dims);
        } else {
            newArgs.push_back(
                    make_unique<ProcessedIdAST>(dynamic_cast<IdAST *>(arg.get())->getName(), VarType::var_t,
                                                        false));
            args.emplace_back(dynamic_cast<IdAST *>(arg.get())->getName(), VarType::var_t, false);
        }
        BlockVars[parentBlock.size()][dynamic_cast<IdAST *>(arg.get())->getName()] = args[args.size() - 1];
    }
    if (FuncTable.find(funcDef.getName()) != FuncTable.end()) {
        return nullptr;
    }
    FuncTable[funcDef.getName()] = Function(funcDef.getName(), funcDef.getType(), args);
    ASTPtr block = funcDef.getBody()->Eval(*this);
    if (!block) {
        return nullptr;
    }
    currentFunc = "";
    return make_unique<FuncDefAST>(funcDef.getType(), funcDef.getName(), move(newArgs), move(block));
}

unique_ptr<CompUnitAST> TypeCheck::EvalCompUnit(CompUnitAST &unit) {
    ASTPtrList newNodes;
    parentBlock.push_back(-1);
    for (const auto &node: unit.getNodes()) {
        auto newNode = node->Eval(*this);
        if (!newNode) {
            return nullptr;
        }
        newNodes.push_back(move(newNode));
    }
    if (FuncTable.find("main") == FuncTable.end()) {
        return nullptr;
    }
    return make_unique<CompUnitAST>(move(newNodes));
}

unique_ptr<StmtAST> TypeCheck::EvalStmt(StmtAST &stmt) {
    if (!stmt.getStmt())
        return make_unique<StmtAST>(nullptr);
    else {
        auto nStmt = stmt.getStmt()->Eval(*this);
        if (!nStmt) {
            return nullptr;
        }
        return make_unique<StmtAST>(move(nStmt));
    }
}

unique_ptr<NumAST> TypeCheck::EvalNumber(NumAST &num) {
    return make_unique<NumAST>(num.getVal());
}

unique_ptr<EmptyAST> TypeCheck::EvalEmpty() {
    return make_unique<EmptyAST>();
}

unique_ptr<ProcessedIdAST> TypeCheck::EvalProcessedId(ProcessedIdAST &id) {
    return make_unique<ProcessedIdAST>(id);
}