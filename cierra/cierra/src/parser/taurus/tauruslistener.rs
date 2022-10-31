#![allow(nonstandard_style)]
// Generated from Taurus.g4 by ANTLR 4.8

#![allow(warnings, clippy::all)]

use antlr_rust::tree::ParseTreeListener;
use super::taurusparser::*;

pub trait TaurusListener<'input> : ParseTreeListener<'input,TaurusParserContextType>{
/**
 * Enter a parse tree produced by {@link TaurusParser#main}.
 * @param ctx the parse tree
 */
fn enter_main(&mut self, _ctx: &MainContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TaurusParser#main}.
 * @param ctx the parse tree
 */
fn exit_main(&mut self, _ctx: &MainContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code Function}
 * labeled alternative in {@link TaurusParser#def}.
 * @param ctx the parse tree
 */
fn enter_Function(&mut self, _ctx: &FunctionContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code Function}
 * labeled alternative in {@link TaurusParser#def}.
 * @param ctx the parse tree
 */
fn exit_Function(&mut self, _ctx: &FunctionContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code Structure}
 * labeled alternative in {@link TaurusParser#def}.
 * @param ctx the parse tree
 */
fn enter_Structure(&mut self, _ctx: &StructureContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code Structure}
 * labeled alternative in {@link TaurusParser#def}.
 * @param ctx the parse tree
 */
fn exit_Structure(&mut self, _ctx: &StructureContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code Predicate}
 * labeled alternative in {@link TaurusParser#def}.
 * @param ctx the parse tree
 */
fn enter_Predicate(&mut self, _ctx: &PredicateContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code Predicate}
 * labeled alternative in {@link TaurusParser#def}.
 * @param ctx the parse tree
 */
fn exit_Predicate(&mut self, _ctx: &PredicateContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code Declaration}
 * labeled alternative in {@link TaurusParser#declStmt}.
 * @param ctx the parse tree
 */
fn enter_Declaration(&mut self, _ctx: &DeclarationContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code Declaration}
 * labeled alternative in {@link TaurusParser#declStmt}.
 * @param ctx the parse tree
 */
fn exit_Declaration(&mut self, _ctx: &DeclarationContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code Statement}
 * labeled alternative in {@link TaurusParser#declStmt}.
 * @param ctx the parse tree
 */
fn enter_Statement(&mut self, _ctx: &StatementContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code Statement}
 * labeled alternative in {@link TaurusParser#declStmt}.
 * @param ctx the parse tree
 */
fn exit_Statement(&mut self, _ctx: &StatementContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TaurusParser#funcDef}.
 * @param ctx the parse tree
 */
fn enter_funcDef(&mut self, _ctx: &FuncDefContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TaurusParser#funcDef}.
 * @param ctx the parse tree
 */
fn exit_funcDef(&mut self, _ctx: &FuncDefContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TaurusParser#structDef}.
 * @param ctx the parse tree
 */
fn enter_structDef(&mut self, _ctx: &StructDefContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TaurusParser#structDef}.
 * @param ctx the parse tree
 */
fn exit_structDef(&mut self, _ctx: &StructDefContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code AtomLocalVar}
 * labeled alternative in {@link TaurusParser#localVar}.
 * @param ctx the parse tree
 */
fn enter_AtomLocalVar(&mut self, _ctx: &AtomLocalVarContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code AtomLocalVar}
 * labeled alternative in {@link TaurusParser#localVar}.
 * @param ctx the parse tree
 */
fn exit_AtomLocalVar(&mut self, _ctx: &AtomLocalVarContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code StructLocalVar}
 * labeled alternative in {@link TaurusParser#localVar}.
 * @param ctx the parse tree
 */
fn enter_StructLocalVar(&mut self, _ctx: &StructLocalVarContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code StructLocalVar}
 * labeled alternative in {@link TaurusParser#localVar}.
 * @param ctx the parse tree
 */
fn exit_StructLocalVar(&mut self, _ctx: &StructLocalVarContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code ArrLocalVar}
 * labeled alternative in {@link TaurusParser#localVar}.
 * @param ctx the parse tree
 */
fn enter_ArrLocalVar(&mut self, _ctx: &ArrLocalVarContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code ArrLocalVar}
 * labeled alternative in {@link TaurusParser#localVar}.
 * @param ctx the parse tree
 */
fn exit_ArrLocalVar(&mut self, _ctx: &ArrLocalVarContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code AtomParaVar}
 * labeled alternative in {@link TaurusParser#paraVar}.
 * @param ctx the parse tree
 */
fn enter_AtomParaVar(&mut self, _ctx: &AtomParaVarContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code AtomParaVar}
 * labeled alternative in {@link TaurusParser#paraVar}.
 * @param ctx the parse tree
 */
fn exit_AtomParaVar(&mut self, _ctx: &AtomParaVarContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code StructParaVar}
 * labeled alternative in {@link TaurusParser#paraVar}.
 * @param ctx the parse tree
 */
fn enter_StructParaVar(&mut self, _ctx: &StructParaVarContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code StructParaVar}
 * labeled alternative in {@link TaurusParser#paraVar}.
 * @param ctx the parse tree
 */
fn exit_StructParaVar(&mut self, _ctx: &StructParaVarContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code ArrParaVar}
 * labeled alternative in {@link TaurusParser#paraVar}.
 * @param ctx the parse tree
 */
fn enter_ArrParaVar(&mut self, _ctx: &ArrParaVarContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code ArrParaVar}
 * labeled alternative in {@link TaurusParser#paraVar}.
 * @param ctx the parse tree
 */
fn exit_ArrParaVar(&mut self, _ctx: &ArrParaVarContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TaurusParser#retVar}.
 * @param ctx the parse tree
 */
fn enter_retVar(&mut self, _ctx: &RetVarContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TaurusParser#retVar}.
 * @param ctx the parse tree
 */
fn exit_retVar(&mut self, _ctx: &RetVarContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code AtomRetTy}
 * labeled alternative in {@link TaurusParser#retTy}.
 * @param ctx the parse tree
 */
fn enter_AtomRetTy(&mut self, _ctx: &AtomRetTyContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code AtomRetTy}
 * labeled alternative in {@link TaurusParser#retTy}.
 * @param ctx the parse tree
 */
fn exit_AtomRetTy(&mut self, _ctx: &AtomRetTyContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code StructRetTy}
 * labeled alternative in {@link TaurusParser#retTy}.
 * @param ctx the parse tree
 */
fn enter_StructRetTy(&mut self, _ctx: &StructRetTyContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code StructRetTy}
 * labeled alternative in {@link TaurusParser#retTy}.
 * @param ctx the parse tree
 */
fn exit_StructRetTy(&mut self, _ctx: &StructRetTyContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code ArrRetTy}
 * labeled alternative in {@link TaurusParser#retTy}.
 * @param ctx the parse tree
 */
fn enter_ArrRetTy(&mut self, _ctx: &ArrRetTyContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code ArrRetTy}
 * labeled alternative in {@link TaurusParser#retTy}.
 * @param ctx the parse tree
 */
fn exit_ArrRetTy(&mut self, _ctx: &ArrRetTyContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code VoidRetTy}
 * labeled alternative in {@link TaurusParser#retTy}.
 * @param ctx the parse tree
 */
fn enter_VoidRetTy(&mut self, _ctx: &VoidRetTyContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code VoidRetTy}
 * labeled alternative in {@link TaurusParser#retTy}.
 * @param ctx the parse tree
 */
fn exit_VoidRetTy(&mut self, _ctx: &VoidRetTyContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code AtomInt}
 * labeled alternative in {@link TaurusParser#atomicType}.
 * @param ctx the parse tree
 */
fn enter_AtomInt(&mut self, _ctx: &AtomIntContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code AtomInt}
 * labeled alternative in {@link TaurusParser#atomicType}.
 * @param ctx the parse tree
 */
fn exit_AtomInt(&mut self, _ctx: &AtomIntContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code AtomFloat}
 * labeled alternative in {@link TaurusParser#atomicType}.
 * @param ctx the parse tree
 */
fn enter_AtomFloat(&mut self, _ctx: &AtomFloatContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code AtomFloat}
 * labeled alternative in {@link TaurusParser#atomicType}.
 * @param ctx the parse tree
 */
fn exit_AtomFloat(&mut self, _ctx: &AtomFloatContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code AtomLogicParaVar}
 * labeled alternative in {@link TaurusParser#logicParaVar}.
 * @param ctx the parse tree
 */
fn enter_AtomLogicParaVar(&mut self, _ctx: &AtomLogicParaVarContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code AtomLogicParaVar}
 * labeled alternative in {@link TaurusParser#logicParaVar}.
 * @param ctx the parse tree
 */
fn exit_AtomLogicParaVar(&mut self, _ctx: &AtomLogicParaVarContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code StructLogicParaVar}
 * labeled alternative in {@link TaurusParser#logicParaVar}.
 * @param ctx the parse tree
 */
fn enter_StructLogicParaVar(&mut self, _ctx: &StructLogicParaVarContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code StructLogicParaVar}
 * labeled alternative in {@link TaurusParser#logicParaVar}.
 * @param ctx the parse tree
 */
fn exit_StructLogicParaVar(&mut self, _ctx: &StructLogicParaVarContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code ArrLogicParaVar}
 * labeled alternative in {@link TaurusParser#logicParaVar}.
 * @param ctx the parse tree
 */
fn enter_ArrLogicParaVar(&mut self, _ctx: &ArrLogicParaVarContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code ArrLogicParaVar}
 * labeled alternative in {@link TaurusParser#logicParaVar}.
 * @param ctx the parse tree
 */
fn exit_ArrLogicParaVar(&mut self, _ctx: &ArrLogicParaVarContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code LogicAtomInt}
 * labeled alternative in {@link TaurusParser#logicAtomicType}.
 * @param ctx the parse tree
 */
fn enter_LogicAtomInt(&mut self, _ctx: &LogicAtomIntContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code LogicAtomInt}
 * labeled alternative in {@link TaurusParser#logicAtomicType}.
 * @param ctx the parse tree
 */
fn exit_LogicAtomInt(&mut self, _ctx: &LogicAtomIntContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code LogicAtomReal}
 * labeled alternative in {@link TaurusParser#logicAtomicType}.
 * @param ctx the parse tree
 */
fn enter_LogicAtomReal(&mut self, _ctx: &LogicAtomRealContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code LogicAtomReal}
 * labeled alternative in {@link TaurusParser#logicAtomicType}.
 * @param ctx the parse tree
 */
fn exit_LogicAtomReal(&mut self, _ctx: &LogicAtomRealContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code LogicAtomBool}
 * labeled alternative in {@link TaurusParser#logicAtomicType}.
 * @param ctx the parse tree
 */
fn enter_LogicAtomBool(&mut self, _ctx: &LogicAtomBoolContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code LogicAtomBool}
 * labeled alternative in {@link TaurusParser#logicAtomicType}.
 * @param ctx the parse tree
 */
fn exit_LogicAtomBool(&mut self, _ctx: &LogicAtomBoolContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code EmptyStmt}
 * labeled alternative in {@link TaurusParser#stmt}.
 * @param ctx the parse tree
 */
fn enter_EmptyStmt(&mut self, _ctx: &EmptyStmtContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code EmptyStmt}
 * labeled alternative in {@link TaurusParser#stmt}.
 * @param ctx the parse tree
 */
fn exit_EmptyStmt(&mut self, _ctx: &EmptyStmtContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code ExprStmt}
 * labeled alternative in {@link TaurusParser#stmt}.
 * @param ctx the parse tree
 */
fn enter_ExprStmt(&mut self, _ctx: &ExprStmtContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code ExprStmt}
 * labeled alternative in {@link TaurusParser#stmt}.
 * @param ctx the parse tree
 */
fn exit_ExprStmt(&mut self, _ctx: &ExprStmtContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code AssignStmt}
 * labeled alternative in {@link TaurusParser#stmt}.
 * @param ctx the parse tree
 */
fn enter_AssignStmt(&mut self, _ctx: &AssignStmtContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code AssignStmt}
 * labeled alternative in {@link TaurusParser#stmt}.
 * @param ctx the parse tree
 */
fn exit_AssignStmt(&mut self, _ctx: &AssignStmtContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code IfStmt}
 * labeled alternative in {@link TaurusParser#stmt}.
 * @param ctx the parse tree
 */
fn enter_IfStmt(&mut self, _ctx: &IfStmtContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code IfStmt}
 * labeled alternative in {@link TaurusParser#stmt}.
 * @param ctx the parse tree
 */
fn exit_IfStmt(&mut self, _ctx: &IfStmtContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code WhileStmt}
 * labeled alternative in {@link TaurusParser#stmt}.
 * @param ctx the parse tree
 */
fn enter_WhileStmt(&mut self, _ctx: &WhileStmtContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code WhileStmt}
 * labeled alternative in {@link TaurusParser#stmt}.
 * @param ctx the parse tree
 */
fn exit_WhileStmt(&mut self, _ctx: &WhileStmtContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code DoStmt}
 * labeled alternative in {@link TaurusParser#stmt}.
 * @param ctx the parse tree
 */
fn enter_DoStmt(&mut self, _ctx: &DoStmtContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code DoStmt}
 * labeled alternative in {@link TaurusParser#stmt}.
 * @param ctx the parse tree
 */
fn exit_DoStmt(&mut self, _ctx: &DoStmtContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code ForStmt}
 * labeled alternative in {@link TaurusParser#stmt}.
 * @param ctx the parse tree
 */
fn enter_ForStmt(&mut self, _ctx: &ForStmtContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code ForStmt}
 * labeled alternative in {@link TaurusParser#stmt}.
 * @param ctx the parse tree
 */
fn exit_ForStmt(&mut self, _ctx: &ForStmtContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code BreakStmt}
 * labeled alternative in {@link TaurusParser#stmt}.
 * @param ctx the parse tree
 */
fn enter_BreakStmt(&mut self, _ctx: &BreakStmtContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code BreakStmt}
 * labeled alternative in {@link TaurusParser#stmt}.
 * @param ctx the parse tree
 */
fn exit_BreakStmt(&mut self, _ctx: &BreakStmtContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code ContStmt}
 * labeled alternative in {@link TaurusParser#stmt}.
 * @param ctx the parse tree
 */
fn enter_ContStmt(&mut self, _ctx: &ContStmtContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code ContStmt}
 * labeled alternative in {@link TaurusParser#stmt}.
 * @param ctx the parse tree
 */
fn exit_ContStmt(&mut self, _ctx: &ContStmtContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code ReturnStmt}
 * labeled alternative in {@link TaurusParser#stmt}.
 * @param ctx the parse tree
 */
fn enter_ReturnStmt(&mut self, _ctx: &ReturnStmtContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code ReturnStmt}
 * labeled alternative in {@link TaurusParser#stmt}.
 * @param ctx the parse tree
 */
fn exit_ReturnStmt(&mut self, _ctx: &ReturnStmtContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code AssertStmt}
 * labeled alternative in {@link TaurusParser#stmt}.
 * @param ctx the parse tree
 */
fn enter_AssertStmt(&mut self, _ctx: &AssertStmtContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code AssertStmt}
 * labeled alternative in {@link TaurusParser#stmt}.
 * @param ctx the parse tree
 */
fn exit_AssertStmt(&mut self, _ctx: &AssertStmtContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code BlockStmt}
 * labeled alternative in {@link TaurusParser#stmt}.
 * @param ctx the parse tree
 */
fn enter_BlockStmt(&mut self, _ctx: &BlockStmtContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code BlockStmt}
 * labeled alternative in {@link TaurusParser#stmt}.
 * @param ctx the parse tree
 */
fn exit_BlockStmt(&mut self, _ctx: &BlockStmtContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code ForInitLocalVar}
 * labeled alternative in {@link TaurusParser#forInit}.
 * @param ctx the parse tree
 */
fn enter_ForInitLocalVar(&mut self, _ctx: &ForInitLocalVarContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code ForInitLocalVar}
 * labeled alternative in {@link TaurusParser#forInit}.
 * @param ctx the parse tree
 */
fn exit_ForInitLocalVar(&mut self, _ctx: &ForInitLocalVarContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code ForInitAssign}
 * labeled alternative in {@link TaurusParser#forInit}.
 * @param ctx the parse tree
 */
fn enter_ForInitAssign(&mut self, _ctx: &ForInitAssignContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code ForInitAssign}
 * labeled alternative in {@link TaurusParser#forInit}.
 * @param ctx the parse tree
 */
fn exit_ForInitAssign(&mut self, _ctx: &ForInitAssignContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code VarAssign}
 * labeled alternative in {@link TaurusParser#assign}.
 * @param ctx the parse tree
 */
fn enter_VarAssign(&mut self, _ctx: &VarAssignContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code VarAssign}
 * labeled alternative in {@link TaurusParser#assign}.
 * @param ctx the parse tree
 */
fn exit_VarAssign(&mut self, _ctx: &VarAssignContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code SubAssign}
 * labeled alternative in {@link TaurusParser#assign}.
 * @param ctx the parse tree
 */
fn enter_SubAssign(&mut self, _ctx: &SubAssignContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code SubAssign}
 * labeled alternative in {@link TaurusParser#assign}.
 * @param ctx the parse tree
 */
fn exit_SubAssign(&mut self, _ctx: &SubAssignContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code MemAssign}
 * labeled alternative in {@link TaurusParser#assign}.
 * @param ctx the parse tree
 */
fn enter_MemAssign(&mut self, _ctx: &MemAssignContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code MemAssign}
 * labeled alternative in {@link TaurusParser#assign}.
 * @param ctx the parse tree
 */
fn exit_MemAssign(&mut self, _ctx: &MemAssignContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TaurusParser#decl}.
 * @param ctx the parse tree
 */
fn enter_decl(&mut self, _ctx: &DeclContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TaurusParser#decl}.
 * @param ctx the parse tree
 */
fn exit_decl(&mut self, _ctx: &DeclContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code MulExpr}
 * labeled alternative in {@link TaurusParser#expr}.
 * @param ctx the parse tree
 */
fn enter_MulExpr(&mut self, _ctx: &MulExprContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code MulExpr}
 * labeled alternative in {@link TaurusParser#expr}.
 * @param ctx the parse tree
 */
fn exit_MulExpr(&mut self, _ctx: &MulExprContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code AndExpr}
 * labeled alternative in {@link TaurusParser#expr}.
 * @param ctx the parse tree
 */
fn enter_AndExpr(&mut self, _ctx: &AndExprContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code AndExpr}
 * labeled alternative in {@link TaurusParser#expr}.
 * @param ctx the parse tree
 */
fn exit_AndExpr(&mut self, _ctx: &AndExprContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code OrdExpr}
 * labeled alternative in {@link TaurusParser#expr}.
 * @param ctx the parse tree
 */
fn enter_OrdExpr(&mut self, _ctx: &OrdExprContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code OrdExpr}
 * labeled alternative in {@link TaurusParser#expr}.
 * @param ctx the parse tree
 */
fn exit_OrdExpr(&mut self, _ctx: &OrdExprContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code IdentExpr}
 * labeled alternative in {@link TaurusParser#expr}.
 * @param ctx the parse tree
 */
fn enter_IdentExpr(&mut self, _ctx: &IdentExprContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code IdentExpr}
 * labeled alternative in {@link TaurusParser#expr}.
 * @param ctx the parse tree
 */
fn exit_IdentExpr(&mut self, _ctx: &IdentExprContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code ConstExpr}
 * labeled alternative in {@link TaurusParser#expr}.
 * @param ctx the parse tree
 */
fn enter_ConstExpr(&mut self, _ctx: &ConstExprContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code ConstExpr}
 * labeled alternative in {@link TaurusParser#expr}.
 * @param ctx the parse tree
 */
fn exit_ConstExpr(&mut self, _ctx: &ConstExprContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code AddExpr}
 * labeled alternative in {@link TaurusParser#expr}.
 * @param ctx the parse tree
 */
fn enter_AddExpr(&mut self, _ctx: &AddExprContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code AddExpr}
 * labeled alternative in {@link TaurusParser#expr}.
 * @param ctx the parse tree
 */
fn exit_AddExpr(&mut self, _ctx: &AddExprContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code ArrAccessExpr}
 * labeled alternative in {@link TaurusParser#expr}.
 * @param ctx the parse tree
 */
fn enter_ArrAccessExpr(&mut self, _ctx: &ArrAccessExprContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code ArrAccessExpr}
 * labeled alternative in {@link TaurusParser#expr}.
 * @param ctx the parse tree
 */
fn exit_ArrAccessExpr(&mut self, _ctx: &ArrAccessExprContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code UnaryExpr}
 * labeled alternative in {@link TaurusParser#expr}.
 * @param ctx the parse tree
 */
fn enter_UnaryExpr(&mut self, _ctx: &UnaryExprContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code UnaryExpr}
 * labeled alternative in {@link TaurusParser#expr}.
 * @param ctx the parse tree
 */
fn exit_UnaryExpr(&mut self, _ctx: &UnaryExprContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code OrExpr}
 * labeled alternative in {@link TaurusParser#expr}.
 * @param ctx the parse tree
 */
fn enter_OrExpr(&mut self, _ctx: &OrExprContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code OrExpr}
 * labeled alternative in {@link TaurusParser#expr}.
 * @param ctx the parse tree
 */
fn exit_OrExpr(&mut self, _ctx: &OrExprContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code MemExpr}
 * labeled alternative in {@link TaurusParser#expr}.
 * @param ctx the parse tree
 */
fn enter_MemExpr(&mut self, _ctx: &MemExprContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code MemExpr}
 * labeled alternative in {@link TaurusParser#expr}.
 * @param ctx the parse tree
 */
fn exit_MemExpr(&mut self, _ctx: &MemExprContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code ParExpr}
 * labeled alternative in {@link TaurusParser#expr}.
 * @param ctx the parse tree
 */
fn enter_ParExpr(&mut self, _ctx: &ParExprContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code ParExpr}
 * labeled alternative in {@link TaurusParser#expr}.
 * @param ctx the parse tree
 */
fn exit_ParExpr(&mut self, _ctx: &ParExprContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code EqExpr}
 * labeled alternative in {@link TaurusParser#expr}.
 * @param ctx the parse tree
 */
fn enter_EqExpr(&mut self, _ctx: &EqExprContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code EqExpr}
 * labeled alternative in {@link TaurusParser#expr}.
 * @param ctx the parse tree
 */
fn exit_EqExpr(&mut self, _ctx: &EqExprContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code CallExpr}
 * labeled alternative in {@link TaurusParser#expr}.
 * @param ctx the parse tree
 */
fn enter_CallExpr(&mut self, _ctx: &CallExprContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code CallExpr}
 * labeled alternative in {@link TaurusParser#expr}.
 * @param ctx the parse tree
 */
fn exit_CallExpr(&mut self, _ctx: &CallExprContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code LogicIntConst}
 * labeled alternative in {@link TaurusParser#logicConstant}.
 * @param ctx the parse tree
 */
fn enter_LogicIntConst(&mut self, _ctx: &LogicIntConstContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code LogicIntConst}
 * labeled alternative in {@link TaurusParser#logicConstant}.
 * @param ctx the parse tree
 */
fn exit_LogicIntConst(&mut self, _ctx: &LogicIntConstContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code LogicFloatConst}
 * labeled alternative in {@link TaurusParser#logicConstant}.
 * @param ctx the parse tree
 */
fn enter_LogicFloatConst(&mut self, _ctx: &LogicFloatConstContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code LogicFloatConst}
 * labeled alternative in {@link TaurusParser#logicConstant}.
 * @param ctx the parse tree
 */
fn exit_LogicFloatConst(&mut self, _ctx: &LogicFloatConstContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code LogicTrueConst}
 * labeled alternative in {@link TaurusParser#logicConstant}.
 * @param ctx the parse tree
 */
fn enter_LogicTrueConst(&mut self, _ctx: &LogicTrueConstContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code LogicTrueConst}
 * labeled alternative in {@link TaurusParser#logicConstant}.
 * @param ctx the parse tree
 */
fn exit_LogicTrueConst(&mut self, _ctx: &LogicTrueConstContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code LogicFalseConst}
 * labeled alternative in {@link TaurusParser#logicConstant}.
 * @param ctx the parse tree
 */
fn enter_LogicFalseConst(&mut self, _ctx: &LogicFalseConstContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code LogicFalseConst}
 * labeled alternative in {@link TaurusParser#logicConstant}.
 * @param ctx the parse tree
 */
fn exit_LogicFalseConst(&mut self, _ctx: &LogicFalseConstContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code ParArithTerm}
 * labeled alternative in {@link TaurusParser#arithTerm}.
 * @param ctx the parse tree
 */
fn enter_ParArithTerm(&mut self, _ctx: &ParArithTermContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code ParArithTerm}
 * labeled alternative in {@link TaurusParser#arithTerm}.
 * @param ctx the parse tree
 */
fn exit_ParArithTerm(&mut self, _ctx: &ParArithTermContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code MulTerm}
 * labeled alternative in {@link TaurusParser#arithTerm}.
 * @param ctx the parse tree
 */
fn enter_MulTerm(&mut self, _ctx: &MulTermContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code MulTerm}
 * labeled alternative in {@link TaurusParser#arithTerm}.
 * @param ctx the parse tree
 */
fn exit_MulTerm(&mut self, _ctx: &MulTermContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code MemTerm}
 * labeled alternative in {@link TaurusParser#arithTerm}.
 * @param ctx the parse tree
 */
fn enter_MemTerm(&mut self, _ctx: &MemTermContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code MemTerm}
 * labeled alternative in {@link TaurusParser#arithTerm}.
 * @param ctx the parse tree
 */
fn exit_MemTerm(&mut self, _ctx: &MemTermContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code IdentTerm}
 * labeled alternative in {@link TaurusParser#arithTerm}.
 * @param ctx the parse tree
 */
fn enter_IdentTerm(&mut self, _ctx: &IdentTermContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code IdentTerm}
 * labeled alternative in {@link TaurusParser#arithTerm}.
 * @param ctx the parse tree
 */
fn exit_IdentTerm(&mut self, _ctx: &IdentTermContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code UnaryTerm}
 * labeled alternative in {@link TaurusParser#arithTerm}.
 * @param ctx the parse tree
 */
fn enter_UnaryTerm(&mut self, _ctx: &UnaryTermContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code UnaryTerm}
 * labeled alternative in {@link TaurusParser#arithTerm}.
 * @param ctx the parse tree
 */
fn exit_UnaryTerm(&mut self, _ctx: &UnaryTermContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code ArrAccessTerm}
 * labeled alternative in {@link TaurusParser#arithTerm}.
 * @param ctx the parse tree
 */
fn enter_ArrAccessTerm(&mut self, _ctx: &ArrAccessTermContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code ArrAccessTerm}
 * labeled alternative in {@link TaurusParser#arithTerm}.
 * @param ctx the parse tree
 */
fn exit_ArrAccessTerm(&mut self, _ctx: &ArrAccessTermContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code ResTerm}
 * labeled alternative in {@link TaurusParser#arithTerm}.
 * @param ctx the parse tree
 */
fn enter_ResTerm(&mut self, _ctx: &ResTermContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code ResTerm}
 * labeled alternative in {@link TaurusParser#arithTerm}.
 * @param ctx the parse tree
 */
fn exit_ResTerm(&mut self, _ctx: &ResTermContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code ArrUpdTerm}
 * labeled alternative in {@link TaurusParser#arithTerm}.
 * @param ctx the parse tree
 */
fn enter_ArrUpdTerm(&mut self, _ctx: &ArrUpdTermContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code ArrUpdTerm}
 * labeled alternative in {@link TaurusParser#arithTerm}.
 * @param ctx the parse tree
 */
fn exit_ArrUpdTerm(&mut self, _ctx: &ArrUpdTermContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code ConstTerm}
 * labeled alternative in {@link TaurusParser#arithTerm}.
 * @param ctx the parse tree
 */
fn enter_ConstTerm(&mut self, _ctx: &ConstTermContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code ConstTerm}
 * labeled alternative in {@link TaurusParser#arithTerm}.
 * @param ctx the parse tree
 */
fn exit_ConstTerm(&mut self, _ctx: &ConstTermContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code AddTerm}
 * labeled alternative in {@link TaurusParser#arithTerm}.
 * @param ctx the parse tree
 */
fn enter_AddTerm(&mut self, _ctx: &AddTermContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code AddTerm}
 * labeled alternative in {@link TaurusParser#arithTerm}.
 * @param ctx the parse tree
 */
fn exit_AddTerm(&mut self, _ctx: &AddTermContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code OrdTerm}
 * labeled alternative in {@link TaurusParser#term}.
 * @param ctx the parse tree
 */
fn enter_OrdTerm(&mut self, _ctx: &OrdTermContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code OrdTerm}
 * labeled alternative in {@link TaurusParser#term}.
 * @param ctx the parse tree
 */
fn exit_OrdTerm(&mut self, _ctx: &OrdTermContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code AndTerm}
 * labeled alternative in {@link TaurusParser#term}.
 * @param ctx the parse tree
 */
fn enter_AndTerm(&mut self, _ctx: &AndTermContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code AndTerm}
 * labeled alternative in {@link TaurusParser#term}.
 * @param ctx the parse tree
 */
fn exit_AndTerm(&mut self, _ctx: &AndTermContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code OrTerm}
 * labeled alternative in {@link TaurusParser#term}.
 * @param ctx the parse tree
 */
fn enter_OrTerm(&mut self, _ctx: &OrTermContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code OrTerm}
 * labeled alternative in {@link TaurusParser#term}.
 * @param ctx the parse tree
 */
fn exit_OrTerm(&mut self, _ctx: &OrTermContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code EqTerm}
 * labeled alternative in {@link TaurusParser#term}.
 * @param ctx the parse tree
 */
fn enter_EqTerm(&mut self, _ctx: &EqTermContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code EqTerm}
 * labeled alternative in {@link TaurusParser#term}.
 * @param ctx the parse tree
 */
fn exit_EqTerm(&mut self, _ctx: &EqTermContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code AriTerm}
 * labeled alternative in {@link TaurusParser#term}.
 * @param ctx the parse tree
 */
fn enter_AriTerm(&mut self, _ctx: &AriTermContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code AriTerm}
 * labeled alternative in {@link TaurusParser#term}.
 * @param ctx the parse tree
 */
fn exit_AriTerm(&mut self, _ctx: &AriTermContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code ParTerm}
 * labeled alternative in {@link TaurusParser#term}.
 * @param ctx the parse tree
 */
fn enter_ParTerm(&mut self, _ctx: &ParTermContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code ParTerm}
 * labeled alternative in {@link TaurusParser#term}.
 * @param ctx the parse tree
 */
fn exit_ParTerm(&mut self, _ctx: &ParTermContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code DisPred}
 * labeled alternative in {@link TaurusParser#pred}.
 * @param ctx the parse tree
 */
fn enter_DisPred(&mut self, _ctx: &DisPredContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code DisPred}
 * labeled alternative in {@link TaurusParser#pred}.
 * @param ctx the parse tree
 */
fn exit_DisPred(&mut self, _ctx: &DisPredContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code LengthPred}
 * labeled alternative in {@link TaurusParser#pred}.
 * @param ctx the parse tree
 */
fn enter_LengthPred(&mut self, _ctx: &LengthPredContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code LengthPred}
 * labeled alternative in {@link TaurusParser#pred}.
 * @param ctx the parse tree
 */
fn exit_LengthPred(&mut self, _ctx: &LengthPredContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code QuantiPred}
 * labeled alternative in {@link TaurusParser#pred}.
 * @param ctx the parse tree
 */
fn enter_QuantiPred(&mut self, _ctx: &QuantiPredContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code QuantiPred}
 * labeled alternative in {@link TaurusParser#pred}.
 * @param ctx the parse tree
 */
fn exit_QuantiPred(&mut self, _ctx: &QuantiPredContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code XorPred}
 * labeled alternative in {@link TaurusParser#pred}.
 * @param ctx the parse tree
 */
fn enter_XorPred(&mut self, _ctx: &XorPredContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code XorPred}
 * labeled alternative in {@link TaurusParser#pred}.
 * @param ctx the parse tree
 */
fn exit_XorPred(&mut self, _ctx: &XorPredContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code ParPred}
 * labeled alternative in {@link TaurusParser#pred}.
 * @param ctx the parse tree
 */
fn enter_ParPred(&mut self, _ctx: &ParPredContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code ParPred}
 * labeled alternative in {@link TaurusParser#pred}.
 * @param ctx the parse tree
 */
fn exit_ParPred(&mut self, _ctx: &ParPredContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code TruePred}
 * labeled alternative in {@link TaurusParser#pred}.
 * @param ctx the parse tree
 */
fn enter_TruePred(&mut self, _ctx: &TruePredContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code TruePred}
 * labeled alternative in {@link TaurusParser#pred}.
 * @param ctx the parse tree
 */
fn exit_TruePred(&mut self, _ctx: &TruePredContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code NotPred}
 * labeled alternative in {@link TaurusParser#pred}.
 * @param ctx the parse tree
 */
fn enter_NotPred(&mut self, _ctx: &NotPredContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code NotPred}
 * labeled alternative in {@link TaurusParser#pred}.
 * @param ctx the parse tree
 */
fn exit_NotPred(&mut self, _ctx: &NotPredContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code CallPred}
 * labeled alternative in {@link TaurusParser#pred}.
 * @param ctx the parse tree
 */
fn enter_CallPred(&mut self, _ctx: &CallPredContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code CallPred}
 * labeled alternative in {@link TaurusParser#pred}.
 * @param ctx the parse tree
 */
fn exit_CallPred(&mut self, _ctx: &CallPredContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code IffPred}
 * labeled alternative in {@link TaurusParser#pred}.
 * @param ctx the parse tree
 */
fn enter_IffPred(&mut self, _ctx: &IffPredContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code IffPred}
 * labeled alternative in {@link TaurusParser#pred}.
 * @param ctx the parse tree
 */
fn exit_IffPred(&mut self, _ctx: &IffPredContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code CmpPred}
 * labeled alternative in {@link TaurusParser#pred}.
 * @param ctx the parse tree
 */
fn enter_CmpPred(&mut self, _ctx: &CmpPredContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code CmpPred}
 * labeled alternative in {@link TaurusParser#pred}.
 * @param ctx the parse tree
 */
fn exit_CmpPred(&mut self, _ctx: &CmpPredContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code FalsePred}
 * labeled alternative in {@link TaurusParser#pred}.
 * @param ctx the parse tree
 */
fn enter_FalsePred(&mut self, _ctx: &FalsePredContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code FalsePred}
 * labeled alternative in {@link TaurusParser#pred}.
 * @param ctx the parse tree
 */
fn exit_FalsePred(&mut self, _ctx: &FalsePredContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code ImpPred}
 * labeled alternative in {@link TaurusParser#pred}.
 * @param ctx the parse tree
 */
fn enter_ImpPred(&mut self, _ctx: &ImpPredContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code ImpPred}
 * labeled alternative in {@link TaurusParser#pred}.
 * @param ctx the parse tree
 */
fn exit_ImpPred(&mut self, _ctx: &ImpPredContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code ConPred}
 * labeled alternative in {@link TaurusParser#pred}.
 * @param ctx the parse tree
 */
fn enter_ConPred(&mut self, _ctx: &ConPredContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code ConPred}
 * labeled alternative in {@link TaurusParser#pred}.
 * @param ctx the parse tree
 */
fn exit_ConPred(&mut self, _ctx: &ConPredContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code MulArithOp}
 * labeled alternative in {@link TaurusParser#arithOp}.
 * @param ctx the parse tree
 */
fn enter_MulArithOp(&mut self, _ctx: &MulArithOpContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code MulArithOp}
 * labeled alternative in {@link TaurusParser#arithOp}.
 * @param ctx the parse tree
 */
fn exit_MulArithOp(&mut self, _ctx: &MulArithOpContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code AddArithOp}
 * labeled alternative in {@link TaurusParser#arithOp}.
 * @param ctx the parse tree
 */
fn enter_AddArithOp(&mut self, _ctx: &AddArithOpContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code AddArithOp}
 * labeled alternative in {@link TaurusParser#arithOp}.
 * @param ctx the parse tree
 */
fn exit_AddArithOp(&mut self, _ctx: &AddArithOpContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code Plus}
 * labeled alternative in {@link TaurusParser#addOp}.
 * @param ctx the parse tree
 */
fn enter_Plus(&mut self, _ctx: &PlusContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code Plus}
 * labeled alternative in {@link TaurusParser#addOp}.
 * @param ctx the parse tree
 */
fn exit_Plus(&mut self, _ctx: &PlusContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code Minus}
 * labeled alternative in {@link TaurusParser#addOp}.
 * @param ctx the parse tree
 */
fn enter_Minus(&mut self, _ctx: &MinusContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code Minus}
 * labeled alternative in {@link TaurusParser#addOp}.
 * @param ctx the parse tree
 */
fn exit_Minus(&mut self, _ctx: &MinusContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code Mul}
 * labeled alternative in {@link TaurusParser#mulOp}.
 * @param ctx the parse tree
 */
fn enter_Mul(&mut self, _ctx: &MulContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code Mul}
 * labeled alternative in {@link TaurusParser#mulOp}.
 * @param ctx the parse tree
 */
fn exit_Mul(&mut self, _ctx: &MulContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code Div}
 * labeled alternative in {@link TaurusParser#mulOp}.
 * @param ctx the parse tree
 */
fn enter_Div(&mut self, _ctx: &DivContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code Div}
 * labeled alternative in {@link TaurusParser#mulOp}.
 * @param ctx the parse tree
 */
fn exit_Div(&mut self, _ctx: &DivContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code Mod}
 * labeled alternative in {@link TaurusParser#mulOp}.
 * @param ctx the parse tree
 */
fn enter_Mod(&mut self, _ctx: &ModContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code Mod}
 * labeled alternative in {@link TaurusParser#mulOp}.
 * @param ctx the parse tree
 */
fn exit_Mod(&mut self, _ctx: &ModContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code OrdCmpOp}
 * labeled alternative in {@link TaurusParser#cmpOp}.
 * @param ctx the parse tree
 */
fn enter_OrdCmpOp(&mut self, _ctx: &OrdCmpOpContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code OrdCmpOp}
 * labeled alternative in {@link TaurusParser#cmpOp}.
 * @param ctx the parse tree
 */
fn exit_OrdCmpOp(&mut self, _ctx: &OrdCmpOpContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code EqCmpOp}
 * labeled alternative in {@link TaurusParser#cmpOp}.
 * @param ctx the parse tree
 */
fn enter_EqCmpOp(&mut self, _ctx: &EqCmpOpContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code EqCmpOp}
 * labeled alternative in {@link TaurusParser#cmpOp}.
 * @param ctx the parse tree
 */
fn exit_EqCmpOp(&mut self, _ctx: &EqCmpOpContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code Eq}
 * labeled alternative in {@link TaurusParser#eqOp}.
 * @param ctx the parse tree
 */
fn enter_Eq(&mut self, _ctx: &EqContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code Eq}
 * labeled alternative in {@link TaurusParser#eqOp}.
 * @param ctx the parse tree
 */
fn exit_Eq(&mut self, _ctx: &EqContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code Neq}
 * labeled alternative in {@link TaurusParser#eqOp}.
 * @param ctx the parse tree
 */
fn enter_Neq(&mut self, _ctx: &NeqContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code Neq}
 * labeled alternative in {@link TaurusParser#eqOp}.
 * @param ctx the parse tree
 */
fn exit_Neq(&mut self, _ctx: &NeqContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code Lt}
 * labeled alternative in {@link TaurusParser#ordOp}.
 * @param ctx the parse tree
 */
fn enter_Lt(&mut self, _ctx: &LtContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code Lt}
 * labeled alternative in {@link TaurusParser#ordOp}.
 * @param ctx the parse tree
 */
fn exit_Lt(&mut self, _ctx: &LtContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code Le}
 * labeled alternative in {@link TaurusParser#ordOp}.
 * @param ctx the parse tree
 */
fn enter_Le(&mut self, _ctx: &LeContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code Le}
 * labeled alternative in {@link TaurusParser#ordOp}.
 * @param ctx the parse tree
 */
fn exit_Le(&mut self, _ctx: &LeContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code Gt}
 * labeled alternative in {@link TaurusParser#ordOp}.
 * @param ctx the parse tree
 */
fn enter_Gt(&mut self, _ctx: &GtContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code Gt}
 * labeled alternative in {@link TaurusParser#ordOp}.
 * @param ctx the parse tree
 */
fn exit_Gt(&mut self, _ctx: &GtContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code Ge}
 * labeled alternative in {@link TaurusParser#ordOp}.
 * @param ctx the parse tree
 */
fn enter_Ge(&mut self, _ctx: &GeContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code Ge}
 * labeled alternative in {@link TaurusParser#ordOp}.
 * @param ctx the parse tree
 */
fn exit_Ge(&mut self, _ctx: &GeContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code Neg}
 * labeled alternative in {@link TaurusParser#unaryOp}.
 * @param ctx the parse tree
 */
fn enter_Neg(&mut self, _ctx: &NegContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code Neg}
 * labeled alternative in {@link TaurusParser#unaryOp}.
 * @param ctx the parse tree
 */
fn exit_Neg(&mut self, _ctx: &NegContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code Not}
 * labeled alternative in {@link TaurusParser#unaryOp}.
 * @param ctx the parse tree
 */
fn enter_Not(&mut self, _ctx: &NotContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code Not}
 * labeled alternative in {@link TaurusParser#unaryOp}.
 * @param ctx the parse tree
 */
fn exit_Not(&mut self, _ctx: &NotContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code Universal}
 * labeled alternative in {@link TaurusParser#quantifier}.
 * @param ctx the parse tree
 */
fn enter_Universal(&mut self, _ctx: &UniversalContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code Universal}
 * labeled alternative in {@link TaurusParser#quantifier}.
 * @param ctx the parse tree
 */
fn exit_Universal(&mut self, _ctx: &UniversalContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code Existential}
 * labeled alternative in {@link TaurusParser#quantifier}.
 * @param ctx the parse tree
 */
fn enter_Existential(&mut self, _ctx: &ExistentialContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code Existential}
 * labeled alternative in {@link TaurusParser#quantifier}.
 * @param ctx the parse tree
 */
fn exit_Existential(&mut self, _ctx: &ExistentialContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TaurusParser#binder}.
 * @param ctx the parse tree
 */
fn enter_binder(&mut self, _ctx: &BinderContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TaurusParser#binder}.
 * @param ctx the parse tree
 */
fn exit_binder(&mut self, _ctx: &BinderContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TaurusParser#funcContract}.
 * @param ctx the parse tree
 */
fn enter_funcContract(&mut self, _ctx: &FuncContractContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TaurusParser#funcContract}.
 * @param ctx the parse tree
 */
fn exit_funcContract(&mut self, _ctx: &FuncContractContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TaurusParser#requiresClause}.
 * @param ctx the parse tree
 */
fn enter_requiresClause(&mut self, _ctx: &RequiresClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TaurusParser#requiresClause}.
 * @param ctx the parse tree
 */
fn exit_requiresClause(&mut self, _ctx: &RequiresClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TaurusParser#decreasesClause}.
 * @param ctx the parse tree
 */
fn enter_decreasesClause(&mut self, _ctx: &DecreasesClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TaurusParser#decreasesClause}.
 * @param ctx the parse tree
 */
fn exit_decreasesClause(&mut self, _ctx: &DecreasesClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TaurusParser#ensuresClause}.
 * @param ctx the parse tree
 */
fn enter_ensuresClause(&mut self, _ctx: &EnsuresClauseContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TaurusParser#ensuresClause}.
 * @param ctx the parse tree
 */
fn exit_ensuresClause(&mut self, _ctx: &EnsuresClauseContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TaurusParser#assertion}.
 * @param ctx the parse tree
 */
fn enter_assertion(&mut self, _ctx: &AssertionContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TaurusParser#assertion}.
 * @param ctx the parse tree
 */
fn exit_assertion(&mut self, _ctx: &AssertionContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TaurusParser#loopAnnot}.
 * @param ctx the parse tree
 */
fn enter_loopAnnot(&mut self, _ctx: &LoopAnnotContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TaurusParser#loopAnnot}.
 * @param ctx the parse tree
 */
fn exit_loopAnnot(&mut self, _ctx: &LoopAnnotContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TaurusParser#predDefs}.
 * @param ctx the parse tree
 */
fn enter_predDefs(&mut self, _ctx: &PredDefsContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TaurusParser#predDefs}.
 * @param ctx the parse tree
 */
fn exit_predDefs(&mut self, _ctx: &PredDefsContext<'input>) { }
/**
 * Enter a parse tree produced by {@link TaurusParser#predDef}.
 * @param ctx the parse tree
 */
fn enter_predDef(&mut self, _ctx: &PredDefContext<'input>) { }
/**
 * Exit a parse tree produced by {@link TaurusParser#predDef}.
 * @param ctx the parse tree
 */
fn exit_predDef(&mut self, _ctx: &PredDefContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code IntConst}
 * labeled alternative in {@link TaurusParser#constant}.
 * @param ctx the parse tree
 */
fn enter_IntConst(&mut self, _ctx: &IntConstContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code IntConst}
 * labeled alternative in {@link TaurusParser#constant}.
 * @param ctx the parse tree
 */
fn exit_IntConst(&mut self, _ctx: &IntConstContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code FloatConst}
 * labeled alternative in {@link TaurusParser#constant}.
 * @param ctx the parse tree
 */
fn enter_FloatConst(&mut self, _ctx: &FloatConstContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code FloatConst}
 * labeled alternative in {@link TaurusParser#constant}.
 * @param ctx the parse tree
 */
fn exit_FloatConst(&mut self, _ctx: &FloatConstContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code TrueConst}
 * labeled alternative in {@link TaurusParser#constant}.
 * @param ctx the parse tree
 */
fn enter_TrueConst(&mut self, _ctx: &TrueConstContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code TrueConst}
 * labeled alternative in {@link TaurusParser#constant}.
 * @param ctx the parse tree
 */
fn exit_TrueConst(&mut self, _ctx: &TrueConstContext<'input>) { }
/**
 * Enter a parse tree produced by the {@code FalseConst}
 * labeled alternative in {@link TaurusParser#constant}.
 * @param ctx the parse tree
 */
fn enter_FalseConst(&mut self, _ctx: &FalseConstContext<'input>) { }
/**
 * Exit a parse tree produced by the {@code FalseConst}
 * labeled alternative in {@link TaurusParser#constant}.
 * @param ctx the parse tree
 */
fn exit_FalseConst(&mut self, _ctx: &FalseConstContext<'input>) { }

}

antlr_rust::coerce_from!{ 'input : TaurusListener<'input> }


