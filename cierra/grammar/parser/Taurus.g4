grammar Taurus;

@header {
#![allow(unused_parens)]
}

@fields {
    in_annot: bool,
    in_line_annot: bool,
}

@init {
    in_annot: false,
    in_line_annot: false,
}

/* top level */
main: defs=def* EOF;

def: funcDef    # Function
    | structDef # Structure
    | predDefs  # Predicate
    ;

declStmt:
    decl    # Declaration
    | stmt  # Statement
    ;

funcDef:
	funcContract retVar LPAR (paraVar (COMMA paraVar)*)? RPAR LBRACE declStmt* RBRACE;

structDef: STRUCT IDENT LBRACE (paraVar SEMICOLON)* RBRACE SEMICOLON;

/* variable */
localVar:
	atomicType IDENT                                    # AtomLocalVar
	| STRUCT IDENT IDENT                                # StructLocalVar
	| atomicType IDENT LBRACKET INT_CONSTANT RBRACKET   # ArrLocalVar
	;

paraVar:
	atomicType IDENT                        # AtomParaVar
	| STRUCT IDENT IDENT                    # StructParaVar
	| atomicType IDENT LBRACKET RBRACKET    # ArrParaVar
	;

retVar: retTy IDENT;

retTy:
	atomicType                      # AtomRetTy
	| STRUCT IDENT                  # StructRetTy
	| atomicType LBRACKET RBRACKET  # ArrRetTy
	| VOID                          # VoidRetTy
	;

atomicType:
    INT           # AtomInt
    | FLOAT       # AtomFloat
    ;

logicParaVar:
	logicAtomicType IDENT                       # AtomLogicParaVar
	| STRUCT IDENT IDENT                        # StructLogicParaVar
	| logicAtomicType LBRACKET RBRACKET IDENT   # ArrLogicParaVar
	;

logicAtomicType:
    INTEGER       # LogicAtomInt
    | REAL        # LogicAtomReal
    | BOOLEAN     # LogicAtomBool
    ;

/* about statement */
stmt:
	SEMICOLON																    # EmptyStmt
	| expr SEMICOLON														    # ExprStmt
	| assign SEMICOLON													        # AssignStmt
	| IF LPAR expr RPAR stmt (ELSE stmt)?							            # IfStmt
	| loopAnnot WHILE LPAR expr RPAR stmt							            # WhileStmt
	| loopAnnot DO stmt WHILE LPAR expr RPAR						            # DoStmt
	| loopAnnot FOR LPAR forInit? SEMICOLON expr? SEMICOLON forIter? RPAR stmt	# ForStmt
	| BREAK SEMICOLON													        # BreakStmt
	| CONTINUE SEMICOLON												        # ContStmt
	| RETURN expr? SEMICOLON										        	# ReturnStmt
	| assertion														            # AssertStmt
	| LBRACE declStmt* RBRACE										            # BlockStmt
	;

forInit:
    localVar (ASSIGN expr)? # ForInitLocalVar
    | assign                # ForInitAssign
    ;

forIter:
    assign  # ForIterAssign
    | expr  # ForIterExpr
    ;

assign:
	IDENT ASSIGN expr					        # VarAssign
	| IDENT LBRACKET expr RBRACKET ASSIGN expr	# SubAssign
	| IDENT PERIOD IDENT ASSIGN expr		    # MemAssign;

decl: localVar (ASSIGN expr)? SEMICOLON;

/* about expression */
expr:
	IDENT									# IdentExpr
	| constant								# ConstExpr
	| IDENT LPAR (expr (COMMA expr)*)? RPAR # CallExpr
	| LPAR expr RPAR                        # ParExpr
	| expr LBRACKET expr RBRACKET           # ArrAccessExpr
	| expr PERIOD IDENT                     # MemExpr
	| unaryOp expr                          # UnaryExpr
	| expr mulOp expr                       # MulExpr
	| expr addOp expr                       # AddExpr
	| expr ordOp expr	                    # OrdExpr
	| expr eqOp expr	                    # EqExpr
	| expr AND expr                         # AndExpr
	| expr OR expr                          # OrExpr;

/* annotation */
logicConstant:
	INT_CONSTANT        # LogicIntConst
	| FLOAT_CONSTANT    # LogicFloatConst
	| ANNO_TRUE         # LogicTrueConst
	| ANNO_FALSE        # LogicFalseConst
	;

arithTerm:
	IDENT                                                                       # IdentTerm
	| RESULT                                                                    # ResTerm
	| logicConstant                                                             # ConstTerm
	| LBRACE arithTerm WITH LBRACKET arithTerm RBRACKET ASSIGN arithTerm RBRACE # ArrUpdTerm
	| LPAR arithTerm RPAR                                                       # ParArithTerm
	| arithTerm LBRACKET arithTerm RBRACKET                                     # ArrAccessTerm
	| arithTerm PERIOD IDENT                                                    # MemTerm
	| unaryOp arithTerm                                                         # UnaryTerm
	| arithTerm mulOp arithTerm                                                 # MulTerm
	| arithTerm addOp arithTerm                                                 # AddTerm
	;

term:
	arithTerm           # AriTerm
	| LPAR term RPAR    # ParTerm
	| term ordOp term   # OrdTerm
	| term eqOp term    # EqTerm
	| term AND term     # AndTerm
	| term OR term      # OrTerm;

pred:
	ANNO_TRUE                               # TruePred
	| ANNO_FALSE	                        # FalsePred
	| arithTerm ( cmpOp arithTerm )+	    # CmpPred
	| IDENT (LPAR term (COMMA term)* RPAR)?	# CallPred
	| LPAR pred RPAR						# ParPred
	| pred AND pred					        # ConPred
	| pred OR pred					        # DisPred
	| pred IMPLY pred					    # ImpPred
	| pred EQUIV pred					    # IffPred
	| NOT pred							    # NotPred
	| pred XOR pred					        # XorPred
	// 这里化简了 ACSL 中 location 和 tset 的概念 range 是个闭区间，其首尾的类型都只能是 integer
	| VALID LPAR IDENT ADD LPAR INT_CONSTANT APOSTROPHE arithTerm RPAR RPAR	# LengthPred
	| quantifier binder (COMMA binder)* SEMICOLON pred			        # QuantiPred;

arithOp:
    mulOp   # MulArithOp
    | addOp # AddArithOp
    ;

addOp:
    ADD     # Plus
    | MINUS # Minus
    ;

mulOp:
    MUL     # Mul
    | DIV   # Div
    | MOD   # Mod
    ;

cmpOp:
    ordOp   # OrdCmpOp
    | eqOp  # EqCmpOp
    ;

eqOp:
    EQ      # Eq
    | NE    # Neq
    ;

ordOp:
    LT    # Lt
    | LE  # Le
    | GT  # Gt
    | GE  # Ge
    ;

unaryOp:
    MINUS   # Neg
    | NOT   # Not
    ;

quantifier:
    FORALL      # Universal
    | EXISTS    # Existential
    ;

binder: logicAtomicType IDENT (COMMA IDENT)*;

funcContract:
	ANNOT_START requiresClause* decreasesClause? ensuresClause* ANNOT_END
	| LINE_ANNOT_START requiresClause* decreasesClause? ensuresClause* LINEEND;

requiresClause: REQUIRES pred SEMICOLON;

decreasesClause:
	DECREASES (arithTerm | LPAR arithTerm (COMMA arithTerm)+ RPAR) SEMICOLON;

ensuresClause: ENSURES pred SEMICOLON;

assertion:
	ANNOT_START ASSERT pred SEMICOLON ANNOT_END
	| LINE_ANNOT_START ASSERT pred SEMICOLON LINEEND;

loopAnnot:
	ANNOT_START (LOOP INVARIANT pred SEMICOLON)* (
		LOOP VARIANT (arithTerm | LPAR arithTerm (COMMA arithTerm)+ RPAR) SEMICOLON
	)? ANNOT_END
	| LINE_ANNOT_START (LOOP INVARIANT pred SEMICOLON)* (
		LOOP VARIANT (arithTerm | LPAR arithTerm (COMMA arithTerm)+ RPAR) SEMICOLON
	)? LINEEND;

predDefs:
	ANNOT_START predDef* ANNOT_END
	| LINE_ANNOT_START predDef* LINEEND;

predDef:
	PREDICATE IDENT (
		LPAR logicParaVar (COMMA logicParaVar)* RPAR
	)? ASSIGN pred SEMICOLON;

/* miscellaneous */
constant:
    INT_CONSTANT        # IntConst
    | FLOAT_CONSTANT    # FloatConst
    | EXPR_TRUE         # TrueConst
    | EXPR_FALSE        # FalseConst
    ;

/* --- literals --- */
VOID: 'void';
STRUCT: 'struct';

LPAR: '(';
RPAR: ')';
LBRACE: '{';
RBRACE: '}';
COMMA: ',';
SEMICOLON: ';';
LBRACKET: '[';
RBRACKET: ']';
PERIOD: '.';

INT: 'int';
FLOAT: 'float';
BOOL: 'bool';

IF: 'if';
ELSE: 'else';
BREAK: 'break';
CONTINUE: 'continue';
RETURN: 'return';
WHILE: 'while';
DO: 'do';
FOR: 'for';

ASSIGN: '=';

EQ: '==';
NE: '!=';
LE: '<=';
LT: '<';
GE: '>=';
GT: '>';
ADD: '+';
MINUS: '-';
MUL: '*';
DIV: '/';
NOT: '!';
MOD: '%';
AND: '&&';
OR: '||';
EXPR_TRUE: 'true';
EXPR_FALSE: 'false';

ANNO_TRUE: '\\true';
ANNO_FALSE: '\\false';
RESULT: '\\result';
LENGTH: '\\length';
OLD: '\\old';
WITH: '\\with';
IMPLY: '==>';
EQUIV: '<==>';
XOR: '^^';
FORALL: '\\forall';
EXISTS: '\\exists';
BOOLEAN: 'boolean';
INTEGER: 'integer';
REAL: 'real';
REQUIRES: 'requires';
DECREASES: 'decreases';
ENSURES: 'ensures';
ASSERT: 'assert';
LOOP: 'loop';
INVARIANT: 'invariant';
VARIANT: 'variant';
PREDICATE: 'predicate';
VALID: '\\valid';
APOSTROPHE: '..';

/* --- constants --- */
INT_CONSTANT: [0-9]+;
FLOAT_CONSTANT: [0-9]+ '.' [0-9]+;
IDENT: [a-zA-Z] [a-zA-Z0-9_]*;

/* --- comments --- */
COMMENT: '/*' ('*/' | ~('@') .*? '*/') -> skip;
LINE_COMMENT: '//' ([\r\n] | ~('@') ~[\r\n]*) -> skip;

/* --- annotationss --- */
ANNOT_START: '/*@' { recog.in_annot = true; };
ANNOT_END: '*/' { recog.in_annot = false; };
LINE_ANNOT_START: '//@' { recog.in_line_annot = true; };

/* --- '@' is skipped in annotation --- */
AT: '@' { if recog.in_annot || recog.in_line_annot { recog.skip(); } };

/* --- LINEEND cannot be skipped for line annotation --- */
LINEEND: [\r\n] {
    if recog.in_line_annot {
        recog.in_line_annot = false;
    } else {
        recog.skip();
    }
};

/* --- skip white spaces --- */
WS: [ \t\u000C] -> skip;