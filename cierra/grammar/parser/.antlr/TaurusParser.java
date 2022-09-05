// Generated from /home/misaka/code/cierra/grammar/parser/Taurus.g4 by ANTLR 4.9.2

#![allow(unused_parens)]

import org.antlr.v4.runtime.atn.*;
import org.antlr.v4.runtime.dfa.DFA;
import org.antlr.v4.runtime.*;
import org.antlr.v4.runtime.misc.*;
import org.antlr.v4.runtime.tree.*;
import java.util.List;
import java.util.Iterator;
import java.util.ArrayList;

@SuppressWarnings({"all", "warnings", "unchecked", "unused", "cast"})
public class TaurusParser extends Parser {
	static { RuntimeMetaData.checkVersion("4.9.2", RuntimeMetaData.VERSION); }

	protected static final DFA[] _decisionToDFA;
	protected static final PredictionContextCache _sharedContextCache =
		new PredictionContextCache();
	public static final int
		VOID=1, STRUCT=2, LPAR=3, RPAR=4, LBRACE=5, RBRACE=6, COMMA=7, SEMICOLON=8, 
		LBRACKET=9, RBRACKET=10, PERIOD=11, INT=12, FLOAT=13, BOOL=14, IF=15, 
		ELSE=16, BREAK=17, CONTINUE=18, RETURN=19, WHILE=20, DO=21, FOR=22, ASSIGN=23, 
		EQ=24, NE=25, LE=26, LT=27, GE=28, GT=29, ADD=30, MINUS=31, MUL=32, DIV=33, 
		NOT=34, MOD=35, AND=36, OR=37, EXPR_TRUE=38, EXPR_FALSE=39, ANNO_TRUE=40, 
		ANNO_FALSE=41, RESULT=42, LENGTH=43, OLD=44, WITH=45, IMPLY=46, EQUIV=47, 
		XOR=48, FORALL=49, EXISTS=50, BOOLEAN=51, INTEGER=52, REAL=53, REQUIRES=54, 
		DECREASES=55, ENSURES=56, ASSERT=57, LOOP=58, INVARIANT=59, VARIANT=60, 
		PREDICATE=61, VALID=62, APOSTROPHE=63, INT_CONSTANT=64, FLOAT_CONSTANT=65, 
		IDENT=66, COMMENT=67, LINE_COMMENT=68, ANNOT_START=69, ANNOT_END=70, LINE_ANNOT_START=71, 
		AT=72, LINEEND=73, WS=74;
	public static final int
		RULE_main = 0, RULE_def = 1, RULE_declStmt = 2, RULE_funcDef = 3, RULE_structDef = 4, 
		RULE_localVar = 5, RULE_paraVar = 6, RULE_retVar = 7, RULE_retTy = 8, 
		RULE_atomicType = 9, RULE_logicParaVar = 10, RULE_logicAtomicType = 11, 
		RULE_stmt = 12, RULE_forInit = 13, RULE_forIter = 14, RULE_assign = 15, 
		RULE_decl = 16, RULE_expr = 17, RULE_logicConstant = 18, RULE_arithTerm = 19, 
		RULE_term = 20, RULE_pred = 21, RULE_arithOp = 22, RULE_addOp = 23, RULE_mulOp = 24, 
		RULE_cmpOp = 25, RULE_eqOp = 26, RULE_ordOp = 27, RULE_unaryOp = 28, RULE_quantifier = 29, 
		RULE_binder = 30, RULE_funcContract = 31, RULE_requiresClause = 32, RULE_decreasesClause = 33, 
		RULE_ensuresClause = 34, RULE_assertion = 35, RULE_loopAnnot = 36, RULE_predDefs = 37, 
		RULE_predDef = 38, RULE_constant = 39;
	private static String[] makeRuleNames() {
		return new String[] {
			"main", "def", "declStmt", "funcDef", "structDef", "localVar", "paraVar", 
			"retVar", "retTy", "atomicType", "logicParaVar", "logicAtomicType", "stmt", 
			"forInit", "forIter", "assign", "decl", "expr", "logicConstant", "arithTerm", 
			"term", "pred", "arithOp", "addOp", "mulOp", "cmpOp", "eqOp", "ordOp", 
			"unaryOp", "quantifier", "binder", "funcContract", "requiresClause", 
			"decreasesClause", "ensuresClause", "assertion", "loopAnnot", "predDefs", 
			"predDef", "constant"
		};
	}
	public static final String[] ruleNames = makeRuleNames();

	private static String[] makeLiteralNames() {
		return new String[] {
			null, "'void'", "'struct'", "'('", "')'", "'{'", "'}'", "','", "';'", 
			"'['", "']'", "'.'", "'int'", "'float'", "'bool'", "'if'", "'else'", 
			"'break'", "'continue'", "'return'", "'while'", "'do'", "'for'", "'='", 
			"'=='", "'!='", "'<='", "'<'", "'>='", "'>'", "'+'", "'-'", "'*'", "'/'", 
			"'!'", "'%'", "'&&'", "'||'", "'true'", "'false'", "'\\true'", "'\\false'", 
			"'\\result'", "'\\length'", "'\\old'", "'\\with'", "'==>'", "'<==>'", 
			"'^^'", "'\\forall'", "'\\exists'", "'boolean'", "'integer'", "'real'", 
			"'requires'", "'decreases'", "'ensures'", "'assert'", "'loop'", "'invariant'", 
			"'variant'", "'predicate'", "'\\valid'", "'..'", null, null, null, null, 
			null, "'/*@'", "'*/'", "'//@'", "'@'"
		};
	}
	private static final String[] _LITERAL_NAMES = makeLiteralNames();
	private static String[] makeSymbolicNames() {
		return new String[] {
			null, "VOID", "STRUCT", "LPAR", "RPAR", "LBRACE", "RBRACE", "COMMA", 
			"SEMICOLON", "LBRACKET", "RBRACKET", "PERIOD", "INT", "FLOAT", "BOOL", 
			"IF", "ELSE", "BREAK", "CONTINUE", "RETURN", "WHILE", "DO", "FOR", "ASSIGN", 
			"EQ", "NE", "LE", "LT", "GE", "GT", "ADD", "MINUS", "MUL", "DIV", "NOT", 
			"MOD", "AND", "OR", "EXPR_TRUE", "EXPR_FALSE", "ANNO_TRUE", "ANNO_FALSE", 
			"RESULT", "LENGTH", "OLD", "WITH", "IMPLY", "EQUIV", "XOR", "FORALL", 
			"EXISTS", "BOOLEAN", "INTEGER", "REAL", "REQUIRES", "DECREASES", "ENSURES", 
			"ASSERT", "LOOP", "INVARIANT", "VARIANT", "PREDICATE", "VALID", "APOSTROPHE", 
			"INT_CONSTANT", "FLOAT_CONSTANT", "IDENT", "COMMENT", "LINE_COMMENT", 
			"ANNOT_START", "ANNOT_END", "LINE_ANNOT_START", "AT", "LINEEND", "WS"
		};
	}
	private static final String[] _SYMBOLIC_NAMES = makeSymbolicNames();
	public static final Vocabulary VOCABULARY = new VocabularyImpl(_LITERAL_NAMES, _SYMBOLIC_NAMES);

	/**
	 * @deprecated Use {@link #VOCABULARY} instead.
	 */
	@Deprecated
	public static final String[] tokenNames;
	static {
		tokenNames = new String[_SYMBOLIC_NAMES.length];
		for (int i = 0; i < tokenNames.length; i++) {
			tokenNames[i] = VOCABULARY.getLiteralName(i);
			if (tokenNames[i] == null) {
				tokenNames[i] = VOCABULARY.getSymbolicName(i);
			}

			if (tokenNames[i] == null) {
				tokenNames[i] = "<INVALID>";
			}
		}
	}

	@Override
	@Deprecated
	public String[] getTokenNames() {
		return tokenNames;
	}

	@Override

	public Vocabulary getVocabulary() {
		return VOCABULARY;
	}

	@Override
	public String getGrammarFileName() { return "Taurus.g4"; }

	@Override
	public String[] getRuleNames() { return ruleNames; }

	@Override
	public String getSerializedATN() { return _serializedATN; }

	@Override
	public ATN getATN() { return _ATN; }

	public TaurusParser(TokenStream input) {
		super(input);
		_interp = new ParserATNSimulator(this,_ATN,_decisionToDFA,_sharedContextCache);
	}

	public static class MainContext extends ParserRuleContext {
		public DefContext defs;
		public TerminalNode EOF() { return getToken(TaurusParser.EOF, 0); }
		public List<DefContext> def() {
			return getRuleContexts(DefContext.class);
		}
		public DefContext def(int i) {
			return getRuleContext(DefContext.class,i);
		}
		public MainContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_main; }
	}

	public final MainContext main() throws RecognitionException {
		MainContext _localctx = new MainContext(_ctx, getState());
		enterRule(_localctx, 0, RULE_main);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(83);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while (_la==STRUCT || _la==ANNOT_START || _la==LINE_ANNOT_START) {
				{
				{
				setState(80);
				((MainContext)_localctx).defs = def();
				}
				}
				setState(85);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(86);
			match(EOF);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	public static class DefContext extends ParserRuleContext {
		public DefContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_def; }
	 
		public DefContext() { }
		public void copyFrom(DefContext ctx) {
			super.copyFrom(ctx);
		}
	}
	public static class FunctionContext extends DefContext {
		public FuncDefContext funcDef() {
			return getRuleContext(FuncDefContext.class,0);
		}
		public FunctionContext(DefContext ctx) { copyFrom(ctx); }
	}
	public static class StructureContext extends DefContext {
		public StructDefContext structDef() {
			return getRuleContext(StructDefContext.class,0);
		}
		public StructureContext(DefContext ctx) { copyFrom(ctx); }
	}
	public static class PredicateContext extends DefContext {
		public PredDefsContext predDefs() {
			return getRuleContext(PredDefsContext.class,0);
		}
		public PredicateContext(DefContext ctx) { copyFrom(ctx); }
	}

	public final DefContext def() throws RecognitionException {
		DefContext _localctx = new DefContext(_ctx, getState());
		enterRule(_localctx, 2, RULE_def);
		try {
			setState(91);
			_errHandler.sync(this);
			switch ( getInterpreter().adaptivePredict(_input,1,_ctx) ) {
			case 1:
				_localctx = new FunctionContext(_localctx);
				enterOuterAlt(_localctx, 1);
				{
				setState(88);
				funcDef();
				}
				break;
			case 2:
				_localctx = new StructureContext(_localctx);
				enterOuterAlt(_localctx, 2);
				{
				setState(89);
				structDef();
				}
				break;
			case 3:
				_localctx = new PredicateContext(_localctx);
				enterOuterAlt(_localctx, 3);
				{
				setState(90);
				predDefs();
				}
				break;
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	public static class DeclStmtContext extends ParserRuleContext {
		public DeclStmtContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_declStmt; }
	 
		public DeclStmtContext() { }
		public void copyFrom(DeclStmtContext ctx) {
			super.copyFrom(ctx);
		}
	}
	public static class StatementContext extends DeclStmtContext {
		public StmtContext stmt() {
			return getRuleContext(StmtContext.class,0);
		}
		public StatementContext(DeclStmtContext ctx) { copyFrom(ctx); }
	}
	public static class DeclarationContext extends DeclStmtContext {
		public DeclContext decl() {
			return getRuleContext(DeclContext.class,0);
		}
		public DeclarationContext(DeclStmtContext ctx) { copyFrom(ctx); }
	}

	public final DeclStmtContext declStmt() throws RecognitionException {
		DeclStmtContext _localctx = new DeclStmtContext(_ctx, getState());
		enterRule(_localctx, 4, RULE_declStmt);
		try {
			setState(95);
			_errHandler.sync(this);
			switch (_input.LA(1)) {
			case STRUCT:
			case INT:
			case FLOAT:
				_localctx = new DeclarationContext(_localctx);
				enterOuterAlt(_localctx, 1);
				{
				setState(93);
				decl();
				}
				break;
			case LPAR:
			case LBRACE:
			case SEMICOLON:
			case IF:
			case BREAK:
			case CONTINUE:
			case RETURN:
			case MINUS:
			case NOT:
			case EXPR_TRUE:
			case EXPR_FALSE:
			case INT_CONSTANT:
			case FLOAT_CONSTANT:
			case IDENT:
			case ANNOT_START:
			case LINE_ANNOT_START:
				_localctx = new StatementContext(_localctx);
				enterOuterAlt(_localctx, 2);
				{
				setState(94);
				stmt();
				}
				break;
			default:
				throw new NoViableAltException(this);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	public static class FuncDefContext extends ParserRuleContext {
		public FuncContractContext funcContract() {
			return getRuleContext(FuncContractContext.class,0);
		}
		public RetVarContext retVar() {
			return getRuleContext(RetVarContext.class,0);
		}
		public TerminalNode LPAR() { return getToken(TaurusParser.LPAR, 0); }
		public TerminalNode RPAR() { return getToken(TaurusParser.RPAR, 0); }
		public TerminalNode LBRACE() { return getToken(TaurusParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(TaurusParser.RBRACE, 0); }
		public List<ParaVarContext> paraVar() {
			return getRuleContexts(ParaVarContext.class);
		}
		public ParaVarContext paraVar(int i) {
			return getRuleContext(ParaVarContext.class,i);
		}
		public List<DeclStmtContext> declStmt() {
			return getRuleContexts(DeclStmtContext.class);
		}
		public DeclStmtContext declStmt(int i) {
			return getRuleContext(DeclStmtContext.class,i);
		}
		public List<TerminalNode> COMMA() { return getTokens(TaurusParser.COMMA); }
		public TerminalNode COMMA(int i) {
			return getToken(TaurusParser.COMMA, i);
		}
		public FuncDefContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_funcDef; }
	}

	public final FuncDefContext funcDef() throws RecognitionException {
		FuncDefContext _localctx = new FuncDefContext(_ctx, getState());
		enterRule(_localctx, 6, RULE_funcDef);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(97);
			funcContract();
			setState(98);
			retVar();
			setState(99);
			match(LPAR);
			setState(108);
			_errHandler.sync(this);
			_la = _input.LA(1);
			if ((((_la) & ~0x3f) == 0 && ((1L << _la) & ((1L << STRUCT) | (1L << INT) | (1L << FLOAT))) != 0)) {
				{
				setState(100);
				paraVar();
				setState(105);
				_errHandler.sync(this);
				_la = _input.LA(1);
				while (_la==COMMA) {
					{
					{
					setState(101);
					match(COMMA);
					setState(102);
					paraVar();
					}
					}
					setState(107);
					_errHandler.sync(this);
					_la = _input.LA(1);
				}
				}
			}

			setState(110);
			match(RPAR);
			setState(111);
			match(LBRACE);
			setState(115);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while ((((_la) & ~0x3f) == 0 && ((1L << _la) & ((1L << STRUCT) | (1L << LPAR) | (1L << LBRACE) | (1L << SEMICOLON) | (1L << INT) | (1L << FLOAT) | (1L << IF) | (1L << BREAK) | (1L << CONTINUE) | (1L << RETURN) | (1L << MINUS) | (1L << NOT) | (1L << EXPR_TRUE) | (1L << EXPR_FALSE))) != 0) || ((((_la - 64)) & ~0x3f) == 0 && ((1L << (_la - 64)) & ((1L << (INT_CONSTANT - 64)) | (1L << (FLOAT_CONSTANT - 64)) | (1L << (IDENT - 64)) | (1L << (ANNOT_START - 64)) | (1L << (LINE_ANNOT_START - 64)))) != 0)) {
				{
				{
				setState(112);
				declStmt();
				}
				}
				setState(117);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(118);
			match(RBRACE);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	public static class StructDefContext extends ParserRuleContext {
		public TerminalNode STRUCT() { return getToken(TaurusParser.STRUCT, 0); }
		public TerminalNode IDENT() { return getToken(TaurusParser.IDENT, 0); }
		public TerminalNode LBRACE() { return getToken(TaurusParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(TaurusParser.RBRACE, 0); }
		public List<TerminalNode> SEMICOLON() { return getTokens(TaurusParser.SEMICOLON); }
		public TerminalNode SEMICOLON(int i) {
			return getToken(TaurusParser.SEMICOLON, i);
		}
		public List<ParaVarContext> paraVar() {
			return getRuleContexts(ParaVarContext.class);
		}
		public ParaVarContext paraVar(int i) {
			return getRuleContext(ParaVarContext.class,i);
		}
		public StructDefContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_structDef; }
	}

	public final StructDefContext structDef() throws RecognitionException {
		StructDefContext _localctx = new StructDefContext(_ctx, getState());
		enterRule(_localctx, 8, RULE_structDef);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(120);
			match(STRUCT);
			setState(121);
			match(IDENT);
			setState(122);
			match(LBRACE);
			setState(128);
			_errHandler.sync(this);
			_la = _input.LA(1);
			while ((((_la) & ~0x3f) == 0 && ((1L << _la) & ((1L << STRUCT) | (1L << INT) | (1L << FLOAT))) != 0)) {
				{
				{
				setState(123);
				paraVar();
				setState(124);
				match(SEMICOLON);
				}
				}
				setState(130);
				_errHandler.sync(this);
				_la = _input.LA(1);
			}
			setState(131);
			match(RBRACE);
			setState(132);
			match(SEMICOLON);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	public static class LocalVarContext extends ParserRuleContext {
		public LocalVarContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_localVar; }
	 
		public LocalVarContext() { }
		public void copyFrom(LocalVarContext ctx) {
			super.copyFrom(ctx);
		}
	}
	public static class AtomLocalVarContext extends LocalVarContext {
		public AtomicTypeContext atomicType() {
			return getRuleContext(AtomicTypeContext.class,0);
		}
		public TerminalNode IDENT() { return getToken(TaurusParser.IDENT, 0); }
		public AtomLocalVarContext(LocalVarContext ctx) { copyFrom(ctx); }
	}
	public static class StructLocalVarContext extends LocalVarContext {
		public TerminalNode STRUCT() { return getToken(TaurusParser.STRUCT, 0); }
		public List<TerminalNode> IDENT() { return getTokens(TaurusParser.IDENT); }
		public TerminalNode IDENT(int i) {
			return getToken(TaurusParser.IDENT, i);
		}
		public StructLocalVarContext(LocalVarContext ctx) { copyFrom(ctx); }
	}
	public static class ArrLocalVarContext extends LocalVarContext {
		public AtomicTypeContext atomicType() {
			return getRuleContext(AtomicTypeContext.class,0);
		}
		public TerminalNode IDENT() { return getToken(TaurusParser.IDENT, 0); }
		public TerminalNode LBRACKET() { return getToken(TaurusParser.LBRACKET, 0); }
		public TerminalNode INT_CONSTANT() { return getToken(TaurusParser.INT_CONSTANT, 0); }
		public TerminalNode RBRACKET() { return getToken(TaurusParser.RBRACKET, 0); }
		public ArrLocalVarContext(LocalVarContext ctx) { copyFrom(ctx); }
	}

	public final LocalVarContext localVar() throws RecognitionException {
		LocalVarContext _localctx = new LocalVarContext(_ctx, getState());
		enterRule(_localctx, 10, RULE_localVar);
		try {
			setState(146);
			_errHandler.sync(this);
			switch ( getInterpreter().adaptivePredict(_input,7,_ctx) ) {
			case 1:
				_localctx = new AtomLocalVarContext(_localctx);
				enterOuterAlt(_localctx, 1);
				{
				setState(134);
				atomicType();
				setState(135);
				match(IDENT);
				}
				break;
			case 2:
				_localctx = new StructLocalVarContext(_localctx);
				enterOuterAlt(_localctx, 2);
				{
				setState(137);
				match(STRUCT);
				setState(138);
				match(IDENT);
				setState(139);
				match(IDENT);
				}
				break;
			case 3:
				_localctx = new ArrLocalVarContext(_localctx);
				enterOuterAlt(_localctx, 3);
				{
				setState(140);
				atomicType();
				setState(141);
				match(IDENT);
				setState(142);
				match(LBRACKET);
				setState(143);
				match(INT_CONSTANT);
				setState(144);
				match(RBRACKET);
				}
				break;
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	public static class ParaVarContext extends ParserRuleContext {
		public ParaVarContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_paraVar; }
	 
		public ParaVarContext() { }
		public void copyFrom(ParaVarContext ctx) {
			super.copyFrom(ctx);
		}
	}
	public static class AtomParaVarContext extends ParaVarContext {
		public AtomicTypeContext atomicType() {
			return getRuleContext(AtomicTypeContext.class,0);
		}
		public TerminalNode IDENT() { return getToken(TaurusParser.IDENT, 0); }
		public AtomParaVarContext(ParaVarContext ctx) { copyFrom(ctx); }
	}
	public static class ArrParaVarContext extends ParaVarContext {
		public AtomicTypeContext atomicType() {
			return getRuleContext(AtomicTypeContext.class,0);
		}
		public TerminalNode IDENT() { return getToken(TaurusParser.IDENT, 0); }
		public TerminalNode LBRACKET() { return getToken(TaurusParser.LBRACKET, 0); }
		public TerminalNode RBRACKET() { return getToken(TaurusParser.RBRACKET, 0); }
		public ArrParaVarContext(ParaVarContext ctx) { copyFrom(ctx); }
	}
	public static class StructParaVarContext extends ParaVarContext {
		public TerminalNode STRUCT() { return getToken(TaurusParser.STRUCT, 0); }
		public List<TerminalNode> IDENT() { return getTokens(TaurusParser.IDENT); }
		public TerminalNode IDENT(int i) {
			return getToken(TaurusParser.IDENT, i);
		}
		public StructParaVarContext(ParaVarContext ctx) { copyFrom(ctx); }
	}

	public final ParaVarContext paraVar() throws RecognitionException {
		ParaVarContext _localctx = new ParaVarContext(_ctx, getState());
		enterRule(_localctx, 12, RULE_paraVar);
		try {
			setState(159);
			_errHandler.sync(this);
			switch ( getInterpreter().adaptivePredict(_input,8,_ctx) ) {
			case 1:
				_localctx = new AtomParaVarContext(_localctx);
				enterOuterAlt(_localctx, 1);
				{
				setState(148);
				atomicType();
				setState(149);
				match(IDENT);
				}
				break;
			case 2:
				_localctx = new StructParaVarContext(_localctx);
				enterOuterAlt(_localctx, 2);
				{
				setState(151);
				match(STRUCT);
				setState(152);
				match(IDENT);
				setState(153);
				match(IDENT);
				}
				break;
			case 3:
				_localctx = new ArrParaVarContext(_localctx);
				enterOuterAlt(_localctx, 3);
				{
				setState(154);
				atomicType();
				setState(155);
				match(IDENT);
				setState(156);
				match(LBRACKET);
				setState(157);
				match(RBRACKET);
				}
				break;
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	public static class RetVarContext extends ParserRuleContext {
		public RetTyContext retTy() {
			return getRuleContext(RetTyContext.class,0);
		}
		public TerminalNode IDENT() { return getToken(TaurusParser.IDENT, 0); }
		public RetVarContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_retVar; }
	}

	public final RetVarContext retVar() throws RecognitionException {
		RetVarContext _localctx = new RetVarContext(_ctx, getState());
		enterRule(_localctx, 14, RULE_retVar);
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(161);
			retTy();
			setState(162);
			match(IDENT);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	public static class RetTyContext extends ParserRuleContext {
		public RetTyContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_retTy; }
	 
		public RetTyContext() { }
		public void copyFrom(RetTyContext ctx) {
			super.copyFrom(ctx);
		}
	}
	public static class VoidRetTyContext extends RetTyContext {
		public TerminalNode VOID() { return getToken(TaurusParser.VOID, 0); }
		public VoidRetTyContext(RetTyContext ctx) { copyFrom(ctx); }
	}
	public static class AtomRetTyContext extends RetTyContext {
		public AtomicTypeContext atomicType() {
			return getRuleContext(AtomicTypeContext.class,0);
		}
		public AtomRetTyContext(RetTyContext ctx) { copyFrom(ctx); }
	}
	public static class ArrRetTyContext extends RetTyContext {
		public AtomicTypeContext atomicType() {
			return getRuleContext(AtomicTypeContext.class,0);
		}
		public TerminalNode LBRACKET() { return getToken(TaurusParser.LBRACKET, 0); }
		public TerminalNode RBRACKET() { return getToken(TaurusParser.RBRACKET, 0); }
		public ArrRetTyContext(RetTyContext ctx) { copyFrom(ctx); }
	}
	public static class StructRetTyContext extends RetTyContext {
		public TerminalNode STRUCT() { return getToken(TaurusParser.STRUCT, 0); }
		public TerminalNode IDENT() { return getToken(TaurusParser.IDENT, 0); }
		public StructRetTyContext(RetTyContext ctx) { copyFrom(ctx); }
	}

	public final RetTyContext retTy() throws RecognitionException {
		RetTyContext _localctx = new RetTyContext(_ctx, getState());
		enterRule(_localctx, 16, RULE_retTy);
		try {
			setState(172);
			_errHandler.sync(this);
			switch ( getInterpreter().adaptivePredict(_input,9,_ctx) ) {
			case 1:
				_localctx = new AtomRetTyContext(_localctx);
				enterOuterAlt(_localctx, 1);
				{
				setState(164);
				atomicType();
				}
				break;
			case 2:
				_localctx = new StructRetTyContext(_localctx);
				enterOuterAlt(_localctx, 2);
				{
				setState(165);
				match(STRUCT);
				setState(166);
				match(IDENT);
				}
				break;
			case 3:
				_localctx = new ArrRetTyContext(_localctx);
				enterOuterAlt(_localctx, 3);
				{
				setState(167);
				atomicType();
				setState(168);
				match(LBRACKET);
				setState(169);
				match(RBRACKET);
				}
				break;
			case 4:
				_localctx = new VoidRetTyContext(_localctx);
				enterOuterAlt(_localctx, 4);
				{
				setState(171);
				match(VOID);
				}
				break;
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	public static class AtomicTypeContext extends ParserRuleContext {
		public AtomicTypeContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_atomicType; }
	 
		public AtomicTypeContext() { }
		public void copyFrom(AtomicTypeContext ctx) {
			super.copyFrom(ctx);
		}
	}
	public static class AtomFloatContext extends AtomicTypeContext {
		public TerminalNode FLOAT() { return getToken(TaurusParser.FLOAT, 0); }
		public AtomFloatContext(AtomicTypeContext ctx) { copyFrom(ctx); }
	}
	public static class AtomIntContext extends AtomicTypeContext {
		public TerminalNode INT() { return getToken(TaurusParser.INT, 0); }
		public AtomIntContext(AtomicTypeContext ctx) { copyFrom(ctx); }
	}

	public final AtomicTypeContext atomicType() throws RecognitionException {
		AtomicTypeContext _localctx = new AtomicTypeContext(_ctx, getState());
		enterRule(_localctx, 18, RULE_atomicType);
		try {
			setState(176);
			_errHandler.sync(this);
			switch (_input.LA(1)) {
			case INT:
				_localctx = new AtomIntContext(_localctx);
				enterOuterAlt(_localctx, 1);
				{
				setState(174);
				match(INT);
				}
				break;
			case FLOAT:
				_localctx = new AtomFloatContext(_localctx);
				enterOuterAlt(_localctx, 2);
				{
				setState(175);
				match(FLOAT);
				}
				break;
			default:
				throw new NoViableAltException(this);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	public static class LogicParaVarContext extends ParserRuleContext {
		public LogicParaVarContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_logicParaVar; }
	 
		public LogicParaVarContext() { }
		public void copyFrom(LogicParaVarContext ctx) {
			super.copyFrom(ctx);
		}
	}
	public static class ArrLogicParaVarContext extends LogicParaVarContext {
		public LogicAtomicTypeContext logicAtomicType() {
			return getRuleContext(LogicAtomicTypeContext.class,0);
		}
		public TerminalNode IDENT() { return getToken(TaurusParser.IDENT, 0); }
		public TerminalNode LBRACKET() { return getToken(TaurusParser.LBRACKET, 0); }
		public TerminalNode RBRACKET() { return getToken(TaurusParser.RBRACKET, 0); }
		public ArrLogicParaVarContext(LogicParaVarContext ctx) { copyFrom(ctx); }
	}
	public static class StructLogicParaVarContext extends LogicParaVarContext {
		public TerminalNode STRUCT() { return getToken(TaurusParser.STRUCT, 0); }
		public List<TerminalNode> IDENT() { return getTokens(TaurusParser.IDENT); }
		public TerminalNode IDENT(int i) {
			return getToken(TaurusParser.IDENT, i);
		}
		public StructLogicParaVarContext(LogicParaVarContext ctx) { copyFrom(ctx); }
	}
	public static class AtomLogicParaVarContext extends LogicParaVarContext {
		public LogicAtomicTypeContext logicAtomicType() {
			return getRuleContext(LogicAtomicTypeContext.class,0);
		}
		public TerminalNode IDENT() { return getToken(TaurusParser.IDENT, 0); }
		public AtomLogicParaVarContext(LogicParaVarContext ctx) { copyFrom(ctx); }
	}

	public final LogicParaVarContext logicParaVar() throws RecognitionException {
		LogicParaVarContext _localctx = new LogicParaVarContext(_ctx, getState());
		enterRule(_localctx, 20, RULE_logicParaVar);
		try {
			setState(189);
			_errHandler.sync(this);
			switch ( getInterpreter().adaptivePredict(_input,11,_ctx) ) {
			case 1:
				_localctx = new AtomLogicParaVarContext(_localctx);
				enterOuterAlt(_localctx, 1);
				{
				setState(178);
				logicAtomicType();
				setState(179);
				match(IDENT);
				}
				break;
			case 2:
				_localctx = new StructLogicParaVarContext(_localctx);
				enterOuterAlt(_localctx, 2);
				{
				setState(181);
				match(STRUCT);
				setState(182);
				match(IDENT);
				setState(183);
				match(IDENT);
				}
				break;
			case 3:
				_localctx = new ArrLogicParaVarContext(_localctx);
				enterOuterAlt(_localctx, 3);
				{
				setState(184);
				logicAtomicType();
				setState(185);
				match(IDENT);
				setState(186);
				match(LBRACKET);
				setState(187);
				match(RBRACKET);
				}
				break;
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	public static class LogicAtomicTypeContext extends ParserRuleContext {
		public LogicAtomicTypeContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_logicAtomicType; }
	 
		public LogicAtomicTypeContext() { }
		public void copyFrom(LogicAtomicTypeContext ctx) {
			super.copyFrom(ctx);
		}
	}
	public static class LogicAtomBoolContext extends LogicAtomicTypeContext {
		public TerminalNode BOOLEAN() { return getToken(TaurusParser.BOOLEAN, 0); }
		public LogicAtomBoolContext(LogicAtomicTypeContext ctx) { copyFrom(ctx); }
	}
	public static class LogicAtomRealContext extends LogicAtomicTypeContext {
		public TerminalNode REAL() { return getToken(TaurusParser.REAL, 0); }
		public LogicAtomRealContext(LogicAtomicTypeContext ctx) { copyFrom(ctx); }
	}
	public static class LogicAtomIntContext extends LogicAtomicTypeContext {
		public TerminalNode INTEGER() { return getToken(TaurusParser.INTEGER, 0); }
		public LogicAtomIntContext(LogicAtomicTypeContext ctx) { copyFrom(ctx); }
	}

	public final LogicAtomicTypeContext logicAtomicType() throws RecognitionException {
		LogicAtomicTypeContext _localctx = new LogicAtomicTypeContext(_ctx, getState());
		enterRule(_localctx, 22, RULE_logicAtomicType);
		try {
			setState(194);
			_errHandler.sync(this);
			switch (_input.LA(1)) {
			case INTEGER:
				_localctx = new LogicAtomIntContext(_localctx);
				enterOuterAlt(_localctx, 1);
				{
				setState(191);
				match(INTEGER);
				}
				break;
			case REAL:
				_localctx = new LogicAtomRealContext(_localctx);
				enterOuterAlt(_localctx, 2);
				{
				setState(192);
				match(REAL);
				}
				break;
			case BOOLEAN:
				_localctx = new LogicAtomBoolContext(_localctx);
				enterOuterAlt(_localctx, 3);
				{
				setState(193);
				match(BOOLEAN);
				}
				break;
			default:
				throw new NoViableAltException(this);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	public static class StmtContext extends ParserRuleContext {
		public StmtContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_stmt; }
	 
		public StmtContext() { }
		public void copyFrom(StmtContext ctx) {
			super.copyFrom(ctx);
		}
	}
	public static class IfStmtContext extends StmtContext {
		public TerminalNode IF() { return getToken(TaurusParser.IF, 0); }
		public TerminalNode LPAR() { return getToken(TaurusParser.LPAR, 0); }
		public ExprContext expr() {
			return getRuleContext(ExprContext.class,0);
		}
		public TerminalNode RPAR() { return getToken(TaurusParser.RPAR, 0); }
		public List<StmtContext> stmt() {
			return getRuleContexts(StmtContext.class);
		}
		public StmtContext stmt(int i) {
			return getRuleContext(StmtContext.class,i);
		}
		public TerminalNode ELSE() { return getToken(TaurusParser.ELSE, 0); }
		public IfStmtContext(StmtContext ctx) { copyFrom(ctx); }
	}
	public static class ExprStmtContext extends StmtContext {
		public ExprContext expr() {
			return getRuleContext(ExprContext.class,0);
		}
		public TerminalNode SEMICOLON() { return getToken(TaurusParser.SEMICOLON, 0); }
		public ExprStmtContext(StmtContext ctx) { copyFrom(ctx); }
	}
	public static class WhileStmtContext extends StmtContext {
		public LoopAnnotContext loopAnnot() {
			return getRuleContext(LoopAnnotContext.class,0);
		}
		public TerminalNode WHILE() { return getToken(TaurusParser.WHILE, 0); }
		public TerminalNode LPAR() { return getToken(TaurusParser.LPAR, 0); }
		public ExprContext expr() {
			return getRuleContext(ExprContext.class,0);
		}
		public TerminalNode RPAR() { return getToken(TaurusParser.RPAR, 0); }
		public StmtContext stmt() {
			return getRuleContext(StmtContext.class,0);
		}
		public WhileStmtContext(StmtContext ctx) { copyFrom(ctx); }
	}
	public static class AssignStmtContext extends StmtContext {
		public AssignContext assign() {
			return getRuleContext(AssignContext.class,0);
		}
		public TerminalNode SEMICOLON() { return getToken(TaurusParser.SEMICOLON, 0); }
		public AssignStmtContext(StmtContext ctx) { copyFrom(ctx); }
	}
	public static class BreakStmtContext extends StmtContext {
		public TerminalNode BREAK() { return getToken(TaurusParser.BREAK, 0); }
		public TerminalNode SEMICOLON() { return getToken(TaurusParser.SEMICOLON, 0); }
		public BreakStmtContext(StmtContext ctx) { copyFrom(ctx); }
	}
	public static class BlockStmtContext extends StmtContext {
		public TerminalNode LBRACE() { return getToken(TaurusParser.LBRACE, 0); }
		public TerminalNode RBRACE() { return getToken(TaurusParser.RBRACE, 0); }
		public List<DeclStmtContext> declStmt() {
			return getRuleContexts(DeclStmtContext.class);
		}
		public DeclStmtContext declStmt(int i) {
			return getRuleContext(DeclStmtContext.class,i);
		}
		public BlockStmtContext(StmtContext ctx) { copyFrom(ctx); }
	}
	public static class EmptyStmtContext extends StmtContext {
		public TerminalNode SEMICOLON() { return getToken(TaurusParser.SEMICOLON, 0); }
		public EmptyStmtContext(StmtContext ctx) { copyFrom(ctx); }
	}
	public static class ContStmtContext extends StmtContext {
		public TerminalNode CONTINUE() { return getToken(TaurusParser.CONTINUE, 0); }
		public TerminalNode SEMICOLON() { return getToken(TaurusParser.SEMICOLON, 0); }
		public ContStmtContext(StmtContext ctx) { copyFrom(ctx); }
	}
	public static class AssertStmtContext extends StmtContext {
		public AssertionContext assertion() {
			return getRuleContext(AssertionContext.class,0);
		}
		public AssertStmtContext(StmtContext ctx) { copyFrom(ctx); }
	}
	public static class ForStmtContext extends StmtContext {
		public LoopAnnotContext loopAnnot() {
			return getRuleContext(LoopAnnotContext.class,0);
		}
		public TerminalNode FOR() { return getToken(TaurusParser.FOR, 0); }
		public TerminalNode LPAR() { return getToken(TaurusParser.LPAR, 0); }
		public List<TerminalNode> SEMICOLON() { return getTokens(TaurusParser.SEMICOLON); }
		public TerminalNode SEMICOLON(int i) {
			return getToken(TaurusParser.SEMICOLON, i);
		}
		public TerminalNode RPAR() { return getToken(TaurusParser.RPAR, 0); }
		public StmtContext stmt() {
			return getRuleContext(StmtContext.class,0);
		}
		public ForInitContext forInit() {
			return getRuleContext(ForInitContext.class,0);
		}
		public ExprContext expr() {
			return getRuleContext(ExprContext.class,0);
		}
		public ForIterContext forIter() {
			return getRuleContext(ForIterContext.class,0);
		}
		public ForStmtContext(StmtContext ctx) { copyFrom(ctx); }
	}
	public static class ReturnStmtContext extends StmtContext {
		public TerminalNode RETURN() { return getToken(TaurusParser.RETURN, 0); }
		public TerminalNode SEMICOLON() { return getToken(TaurusParser.SEMICOLON, 0); }
		public ExprContext expr() {
			return getRuleContext(ExprContext.class,0);
		}
		public ReturnStmtContext(StmtContext ctx) { copyFrom(ctx); }
	}
	public static class DoStmtContext extends StmtContext {
		public LoopAnnotContext loopAnnot() {
			return getRuleContext(LoopAnnotContext.class,0);
		}
		public TerminalNode DO() { return getToken(TaurusParser.DO, 0); }
		public StmtContext stmt() {
			return getRuleContext(StmtContext.class,0);
		}
		public TerminalNode WHILE() { return getToken(TaurusParser.WHILE, 0); }
		public TerminalNode LPAR() { return getToken(TaurusParser.LPAR, 0); }
		public ExprContext expr() {
			return getRuleContext(ExprContext.class,0);
		}
		public TerminalNode RPAR() { return getToken(TaurusParser.RPAR, 0); }
		public DoStmtContext(StmtContext ctx) { copyFrom(ctx); }
	}

	public final StmtContext stmt() throws RecognitionException {
		StmtContext _localctx = new StmtContext(_ctx, getState());
		enterRule(_localctx, 24, RULE_stmt);
		int _la;
		try {
			setState(262);
			_errHandler.sync(this);
			switch ( getInterpreter().adaptivePredict(_input,19,_ctx) ) {
			case 1:
				_localctx = new EmptyStmtContext(_localctx);
				enterOuterAlt(_localctx, 1);
				{
				setState(196);
				match(SEMICOLON);
				}
				break;
			case 2:
				_localctx = new ExprStmtContext(_localctx);
				enterOuterAlt(_localctx, 2);
				{
				setState(197);
				expr(0);
				setState(198);
				match(SEMICOLON);
				}
				break;
			case 3:
				_localctx = new AssignStmtContext(_localctx);
				enterOuterAlt(_localctx, 3);
				{
				setState(200);
				assign();
				setState(201);
				match(SEMICOLON);
				}
				break;
			case 4:
				_localctx = new IfStmtContext(_localctx);
				enterOuterAlt(_localctx, 4);
				{
				setState(203);
				match(IF);
				setState(204);
				match(LPAR);
				setState(205);
				expr(0);
				setState(206);
				match(RPAR);
				setState(207);
				stmt();
				setState(210);
				_errHandler.sync(this);
				switch ( getInterpreter().adaptivePredict(_input,13,_ctx) ) {
				case 1:
					{
					setState(208);
					match(ELSE);
					setState(209);
					stmt();
					}
					break;
				}
				}
				break;
			case 5:
				_localctx = new WhileStmtContext(_localctx);
				enterOuterAlt(_localctx, 5);
				{
				setState(212);
				loopAnnot();
				setState(213);
				match(WHILE);
				setState(214);
				match(LPAR);
				setState(215);
				expr(0);
				setState(216);
				match(RPAR);
				setState(217);
				stmt();
				}
				break;
			case 6:
				_localctx = new DoStmtContext(_localctx);
				enterOuterAlt(_localctx, 6);
				{
				setState(219);
				loopAnnot();
				setState(220);
				match(DO);
				setState(221);
				stmt();
				setState(222);
				match(WHILE);
				setState(223);
				match(LPAR);
				setState(224);
				expr(0);
				setState(225);
				match(RPAR);
				}
				break;
			case 7:
				_localctx = new ForStmtContext(_localctx);
				enterOuterAlt(_localctx, 7);
				{
				setState(227);
				loopAnnot();
				setState(228);
				match(FOR);
				setState(229);
				match(LPAR);
				setState(231);
				_errHandler.sync(this);
				_la = _input.LA(1);
				if ((((_la) & ~0x3f) == 0 && ((1L << _la) & ((1L << STRUCT) | (1L << INT) | (1L << FLOAT))) != 0) || _la==IDENT) {
					{
					setState(230);
					forInit();
					}
				}

				setState(233);
				match(SEMICOLON);
				setState(235);
				_errHandler.sync(this);
				_la = _input.LA(1);
				if (((((_la - 3)) & ~0x3f) == 0 && ((1L << (_la - 3)) & ((1L << (LPAR - 3)) | (1L << (MINUS - 3)) | (1L << (NOT - 3)) | (1L << (EXPR_TRUE - 3)) | (1L << (EXPR_FALSE - 3)) | (1L << (INT_CONSTANT - 3)) | (1L << (FLOAT_CONSTANT - 3)) | (1L << (IDENT - 3)))) != 0)) {
					{
					setState(234);
					expr(0);
					}
				}

				setState(237);
				match(SEMICOLON);
				setState(239);
				_errHandler.sync(this);
				_la = _input.LA(1);
				if (((((_la - 3)) & ~0x3f) == 0 && ((1L << (_la - 3)) & ((1L << (LPAR - 3)) | (1L << (MINUS - 3)) | (1L << (NOT - 3)) | (1L << (EXPR_TRUE - 3)) | (1L << (EXPR_FALSE - 3)) | (1L << (INT_CONSTANT - 3)) | (1L << (FLOAT_CONSTANT - 3)) | (1L << (IDENT - 3)))) != 0)) {
					{
					setState(238);
					forIter();
					}
				}

				setState(241);
				match(RPAR);
				setState(242);
				stmt();
				}
				break;
			case 8:
				_localctx = new BreakStmtContext(_localctx);
				enterOuterAlt(_localctx, 8);
				{
				setState(244);
				match(BREAK);
				setState(245);
				match(SEMICOLON);
				}
				break;
			case 9:
				_localctx = new ContStmtContext(_localctx);
				enterOuterAlt(_localctx, 9);
				{
				setState(246);
				match(CONTINUE);
				setState(247);
				match(SEMICOLON);
				}
				break;
			case 10:
				_localctx = new ReturnStmtContext(_localctx);
				enterOuterAlt(_localctx, 10);
				{
				setState(248);
				match(RETURN);
				setState(250);
				_errHandler.sync(this);
				_la = _input.LA(1);
				if (((((_la - 3)) & ~0x3f) == 0 && ((1L << (_la - 3)) & ((1L << (LPAR - 3)) | (1L << (MINUS - 3)) | (1L << (NOT - 3)) | (1L << (EXPR_TRUE - 3)) | (1L << (EXPR_FALSE - 3)) | (1L << (INT_CONSTANT - 3)) | (1L << (FLOAT_CONSTANT - 3)) | (1L << (IDENT - 3)))) != 0)) {
					{
					setState(249);
					expr(0);
					}
				}

				setState(252);
				match(SEMICOLON);
				}
				break;
			case 11:
				_localctx = new AssertStmtContext(_localctx);
				enterOuterAlt(_localctx, 11);
				{
				setState(253);
				assertion();
				}
				break;
			case 12:
				_localctx = new BlockStmtContext(_localctx);
				enterOuterAlt(_localctx, 12);
				{
				setState(254);
				match(LBRACE);
				setState(258);
				_errHandler.sync(this);
				_la = _input.LA(1);
				while ((((_la) & ~0x3f) == 0 && ((1L << _la) & ((1L << STRUCT) | (1L << LPAR) | (1L << LBRACE) | (1L << SEMICOLON) | (1L << INT) | (1L << FLOAT) | (1L << IF) | (1L << BREAK) | (1L << CONTINUE) | (1L << RETURN) | (1L << MINUS) | (1L << NOT) | (1L << EXPR_TRUE) | (1L << EXPR_FALSE))) != 0) || ((((_la - 64)) & ~0x3f) == 0 && ((1L << (_la - 64)) & ((1L << (INT_CONSTANT - 64)) | (1L << (FLOAT_CONSTANT - 64)) | (1L << (IDENT - 64)) | (1L << (ANNOT_START - 64)) | (1L << (LINE_ANNOT_START - 64)))) != 0)) {
					{
					{
					setState(255);
					declStmt();
					}
					}
					setState(260);
					_errHandler.sync(this);
					_la = _input.LA(1);
				}
				setState(261);
				match(RBRACE);
				}
				break;
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	public static class ForInitContext extends ParserRuleContext {
		public ForInitContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_forInit; }
	 
		public ForInitContext() { }
		public void copyFrom(ForInitContext ctx) {
			super.copyFrom(ctx);
		}
	}
	public static class ForInitLocalVarContext extends ForInitContext {
		public LocalVarContext localVar() {
			return getRuleContext(LocalVarContext.class,0);
		}
		public TerminalNode ASSIGN() { return getToken(TaurusParser.ASSIGN, 0); }
		public ExprContext expr() {
			return getRuleContext(ExprContext.class,0);
		}
		public ForInitLocalVarContext(ForInitContext ctx) { copyFrom(ctx); }
	}
	public static class ForInitAssignContext extends ForInitContext {
		public AssignContext assign() {
			return getRuleContext(AssignContext.class,0);
		}
		public ForInitAssignContext(ForInitContext ctx) { copyFrom(ctx); }
	}

	public final ForInitContext forInit() throws RecognitionException {
		ForInitContext _localctx = new ForInitContext(_ctx, getState());
		enterRule(_localctx, 26, RULE_forInit);
		int _la;
		try {
			setState(270);
			_errHandler.sync(this);
			switch (_input.LA(1)) {
			case STRUCT:
			case INT:
			case FLOAT:
				_localctx = new ForInitLocalVarContext(_localctx);
				enterOuterAlt(_localctx, 1);
				{
				setState(264);
				localVar();
				setState(267);
				_errHandler.sync(this);
				_la = _input.LA(1);
				if (_la==ASSIGN) {
					{
					setState(265);
					match(ASSIGN);
					setState(266);
					expr(0);
					}
				}

				}
				break;
			case IDENT:
				_localctx = new ForInitAssignContext(_localctx);
				enterOuterAlt(_localctx, 2);
				{
				setState(269);
				assign();
				}
				break;
			default:
				throw new NoViableAltException(this);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	public static class ForIterContext extends ParserRuleContext {
		public ForIterContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_forIter; }
	 
		public ForIterContext() { }
		public void copyFrom(ForIterContext ctx) {
			super.copyFrom(ctx);
		}
	}
	public static class ForIterExprContext extends ForIterContext {
		public ExprContext expr() {
			return getRuleContext(ExprContext.class,0);
		}
		public ForIterExprContext(ForIterContext ctx) { copyFrom(ctx); }
	}
	public static class ForIterAssignContext extends ForIterContext {
		public AssignContext assign() {
			return getRuleContext(AssignContext.class,0);
		}
		public ForIterAssignContext(ForIterContext ctx) { copyFrom(ctx); }
	}

	public final ForIterContext forIter() throws RecognitionException {
		ForIterContext _localctx = new ForIterContext(_ctx, getState());
		enterRule(_localctx, 28, RULE_forIter);
		try {
			setState(274);
			_errHandler.sync(this);
			switch ( getInterpreter().adaptivePredict(_input,22,_ctx) ) {
			case 1:
				_localctx = new ForIterAssignContext(_localctx);
				enterOuterAlt(_localctx, 1);
				{
				setState(272);
				assign();
				}
				break;
			case 2:
				_localctx = new ForIterExprContext(_localctx);
				enterOuterAlt(_localctx, 2);
				{
				setState(273);
				expr(0);
				}
				break;
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	public static class AssignContext extends ParserRuleContext {
		public AssignContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_assign; }
	 
		public AssignContext() { }
		public void copyFrom(AssignContext ctx) {
			super.copyFrom(ctx);
		}
	}
	public static class MemAssignContext extends AssignContext {
		public List<TerminalNode> IDENT() { return getTokens(TaurusParser.IDENT); }
		public TerminalNode IDENT(int i) {
			return getToken(TaurusParser.IDENT, i);
		}
		public TerminalNode PERIOD() { return getToken(TaurusParser.PERIOD, 0); }
		public TerminalNode ASSIGN() { return getToken(TaurusParser.ASSIGN, 0); }
		public ExprContext expr() {
			return getRuleContext(ExprContext.class,0);
		}
		public MemAssignContext(AssignContext ctx) { copyFrom(ctx); }
	}
	public static class VarAssignContext extends AssignContext {
		public TerminalNode IDENT() { return getToken(TaurusParser.IDENT, 0); }
		public TerminalNode ASSIGN() { return getToken(TaurusParser.ASSIGN, 0); }
		public ExprContext expr() {
			return getRuleContext(ExprContext.class,0);
		}
		public VarAssignContext(AssignContext ctx) { copyFrom(ctx); }
	}
	public static class SubAssignContext extends AssignContext {
		public TerminalNode IDENT() { return getToken(TaurusParser.IDENT, 0); }
		public TerminalNode LBRACKET() { return getToken(TaurusParser.LBRACKET, 0); }
		public List<ExprContext> expr() {
			return getRuleContexts(ExprContext.class);
		}
		public ExprContext expr(int i) {
			return getRuleContext(ExprContext.class,i);
		}
		public TerminalNode RBRACKET() { return getToken(TaurusParser.RBRACKET, 0); }
		public TerminalNode ASSIGN() { return getToken(TaurusParser.ASSIGN, 0); }
		public SubAssignContext(AssignContext ctx) { copyFrom(ctx); }
	}

	public final AssignContext assign() throws RecognitionException {
		AssignContext _localctx = new AssignContext(_ctx, getState());
		enterRule(_localctx, 30, RULE_assign);
		try {
			setState(291);
			_errHandler.sync(this);
			switch ( getInterpreter().adaptivePredict(_input,23,_ctx) ) {
			case 1:
				_localctx = new VarAssignContext(_localctx);
				enterOuterAlt(_localctx, 1);
				{
				setState(276);
				match(IDENT);
				setState(277);
				match(ASSIGN);
				setState(278);
				expr(0);
				}
				break;
			case 2:
				_localctx = new SubAssignContext(_localctx);
				enterOuterAlt(_localctx, 2);
				{
				setState(279);
				match(IDENT);
				setState(280);
				match(LBRACKET);
				setState(281);
				expr(0);
				setState(282);
				match(RBRACKET);
				setState(283);
				match(ASSIGN);
				setState(284);
				expr(0);
				}
				break;
			case 3:
				_localctx = new MemAssignContext(_localctx);
				enterOuterAlt(_localctx, 3);
				{
				setState(286);
				match(IDENT);
				setState(287);
				match(PERIOD);
				setState(288);
				match(IDENT);
				setState(289);
				match(ASSIGN);
				setState(290);
				expr(0);
				}
				break;
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	public static class DeclContext extends ParserRuleContext {
		public LocalVarContext localVar() {
			return getRuleContext(LocalVarContext.class,0);
		}
		public TerminalNode SEMICOLON() { return getToken(TaurusParser.SEMICOLON, 0); }
		public TerminalNode ASSIGN() { return getToken(TaurusParser.ASSIGN, 0); }
		public ExprContext expr() {
			return getRuleContext(ExprContext.class,0);
		}
		public DeclContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_decl; }
	}

	public final DeclContext decl() throws RecognitionException {
		DeclContext _localctx = new DeclContext(_ctx, getState());
		enterRule(_localctx, 32, RULE_decl);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(293);
			localVar();
			setState(296);
			_errHandler.sync(this);
			_la = _input.LA(1);
			if (_la==ASSIGN) {
				{
				setState(294);
				match(ASSIGN);
				setState(295);
				expr(0);
				}
			}

			setState(298);
			match(SEMICOLON);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	public static class ExprContext extends ParserRuleContext {
		public ExprContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_expr; }
	 
		public ExprContext() { }
		public void copyFrom(ExprContext ctx) {
			super.copyFrom(ctx);
		}
	}
	public static class MulExprContext extends ExprContext {
		public List<ExprContext> expr() {
			return getRuleContexts(ExprContext.class);
		}
		public ExprContext expr(int i) {
			return getRuleContext(ExprContext.class,i);
		}
		public MulOpContext mulOp() {
			return getRuleContext(MulOpContext.class,0);
		}
		public MulExprContext(ExprContext ctx) { copyFrom(ctx); }
	}
	public static class AndExprContext extends ExprContext {
		public List<ExprContext> expr() {
			return getRuleContexts(ExprContext.class);
		}
		public ExprContext expr(int i) {
			return getRuleContext(ExprContext.class,i);
		}
		public TerminalNode AND() { return getToken(TaurusParser.AND, 0); }
		public AndExprContext(ExprContext ctx) { copyFrom(ctx); }
	}
	public static class OrdExprContext extends ExprContext {
		public List<ExprContext> expr() {
			return getRuleContexts(ExprContext.class);
		}
		public ExprContext expr(int i) {
			return getRuleContext(ExprContext.class,i);
		}
		public OrdOpContext ordOp() {
			return getRuleContext(OrdOpContext.class,0);
		}
		public OrdExprContext(ExprContext ctx) { copyFrom(ctx); }
	}
	public static class IdentExprContext extends ExprContext {
		public TerminalNode IDENT() { return getToken(TaurusParser.IDENT, 0); }
		public IdentExprContext(ExprContext ctx) { copyFrom(ctx); }
	}
	public static class ConstExprContext extends ExprContext {
		public ConstantContext constant() {
			return getRuleContext(ConstantContext.class,0);
		}
		public ConstExprContext(ExprContext ctx) { copyFrom(ctx); }
	}
	public static class AddExprContext extends ExprContext {
		public List<ExprContext> expr() {
			return getRuleContexts(ExprContext.class);
		}
		public ExprContext expr(int i) {
			return getRuleContext(ExprContext.class,i);
		}
		public AddOpContext addOp() {
			return getRuleContext(AddOpContext.class,0);
		}
		public AddExprContext(ExprContext ctx) { copyFrom(ctx); }
	}
	public static class ArrAccessExprContext extends ExprContext {
		public List<ExprContext> expr() {
			return getRuleContexts(ExprContext.class);
		}
		public ExprContext expr(int i) {
			return getRuleContext(ExprContext.class,i);
		}
		public TerminalNode LBRACKET() { return getToken(TaurusParser.LBRACKET, 0); }
		public TerminalNode RBRACKET() { return getToken(TaurusParser.RBRACKET, 0); }
		public ArrAccessExprContext(ExprContext ctx) { copyFrom(ctx); }
	}
	public static class UnaryExprContext extends ExprContext {
		public UnaryOpContext unaryOp() {
			return getRuleContext(UnaryOpContext.class,0);
		}
		public ExprContext expr() {
			return getRuleContext(ExprContext.class,0);
		}
		public UnaryExprContext(ExprContext ctx) { copyFrom(ctx); }
	}
	public static class OrExprContext extends ExprContext {
		public List<ExprContext> expr() {
			return getRuleContexts(ExprContext.class);
		}
		public ExprContext expr(int i) {
			return getRuleContext(ExprContext.class,i);
		}
		public TerminalNode OR() { return getToken(TaurusParser.OR, 0); }
		public OrExprContext(ExprContext ctx) { copyFrom(ctx); }
	}
	public static class MemExprContext extends ExprContext {
		public ExprContext expr() {
			return getRuleContext(ExprContext.class,0);
		}
		public TerminalNode PERIOD() { return getToken(TaurusParser.PERIOD, 0); }
		public TerminalNode IDENT() { return getToken(TaurusParser.IDENT, 0); }
		public MemExprContext(ExprContext ctx) { copyFrom(ctx); }
	}
	public static class ParExprContext extends ExprContext {
		public TerminalNode LPAR() { return getToken(TaurusParser.LPAR, 0); }
		public ExprContext expr() {
			return getRuleContext(ExprContext.class,0);
		}
		public TerminalNode RPAR() { return getToken(TaurusParser.RPAR, 0); }
		public ParExprContext(ExprContext ctx) { copyFrom(ctx); }
	}
	public static class EqExprContext extends ExprContext {
		public List<ExprContext> expr() {
			return getRuleContexts(ExprContext.class);
		}
		public ExprContext expr(int i) {
			return getRuleContext(ExprContext.class,i);
		}
		public EqOpContext eqOp() {
			return getRuleContext(EqOpContext.class,0);
		}
		public EqExprContext(ExprContext ctx) { copyFrom(ctx); }
	}
	public static class CallExprContext extends ExprContext {
		public TerminalNode IDENT() { return getToken(TaurusParser.IDENT, 0); }
		public TerminalNode LPAR() { return getToken(TaurusParser.LPAR, 0); }
		public TerminalNode RPAR() { return getToken(TaurusParser.RPAR, 0); }
		public List<ExprContext> expr() {
			return getRuleContexts(ExprContext.class);
		}
		public ExprContext expr(int i) {
			return getRuleContext(ExprContext.class,i);
		}
		public List<TerminalNode> COMMA() { return getTokens(TaurusParser.COMMA); }
		public TerminalNode COMMA(int i) {
			return getToken(TaurusParser.COMMA, i);
		}
		public CallExprContext(ExprContext ctx) { copyFrom(ctx); }
	}

	public final ExprContext expr() throws RecognitionException {
		return expr(0);
	}

	private ExprContext expr(int _p) throws RecognitionException {
		ParserRuleContext _parentctx = _ctx;
		int _parentState = getState();
		ExprContext _localctx = new ExprContext(_ctx, _parentState);
		ExprContext _prevctx = _localctx;
		int _startState = 34;
		enterRecursionRule(_localctx, 34, RULE_expr, _p);
		int _la;
		try {
			int _alt;
			enterOuterAlt(_localctx, 1);
			{
			setState(323);
			_errHandler.sync(this);
			switch ( getInterpreter().adaptivePredict(_input,27,_ctx) ) {
			case 1:
				{
				_localctx = new IdentExprContext(_localctx);
				_ctx = _localctx;
				_prevctx = _localctx;

				setState(301);
				match(IDENT);
				}
				break;
			case 2:
				{
				_localctx = new ConstExprContext(_localctx);
				_ctx = _localctx;
				_prevctx = _localctx;
				setState(302);
				constant();
				}
				break;
			case 3:
				{
				_localctx = new CallExprContext(_localctx);
				_ctx = _localctx;
				_prevctx = _localctx;
				setState(303);
				match(IDENT);
				setState(304);
				match(LPAR);
				setState(313);
				_errHandler.sync(this);
				_la = _input.LA(1);
				if (((((_la - 3)) & ~0x3f) == 0 && ((1L << (_la - 3)) & ((1L << (LPAR - 3)) | (1L << (MINUS - 3)) | (1L << (NOT - 3)) | (1L << (EXPR_TRUE - 3)) | (1L << (EXPR_FALSE - 3)) | (1L << (INT_CONSTANT - 3)) | (1L << (FLOAT_CONSTANT - 3)) | (1L << (IDENT - 3)))) != 0)) {
					{
					setState(305);
					expr(0);
					setState(310);
					_errHandler.sync(this);
					_la = _input.LA(1);
					while (_la==COMMA) {
						{
						{
						setState(306);
						match(COMMA);
						setState(307);
						expr(0);
						}
						}
						setState(312);
						_errHandler.sync(this);
						_la = _input.LA(1);
					}
					}
				}

				setState(315);
				match(RPAR);
				}
				break;
			case 4:
				{
				_localctx = new ParExprContext(_localctx);
				_ctx = _localctx;
				_prevctx = _localctx;
				setState(316);
				match(LPAR);
				setState(317);
				expr(0);
				setState(318);
				match(RPAR);
				}
				break;
			case 5:
				{
				_localctx = new UnaryExprContext(_localctx);
				_ctx = _localctx;
				_prevctx = _localctx;
				setState(320);
				unaryOp();
				setState(321);
				expr(7);
				}
				break;
			}
			_ctx.stop = _input.LT(-1);
			setState(357);
			_errHandler.sync(this);
			_alt = getInterpreter().adaptivePredict(_input,29,_ctx);
			while ( _alt!=2 && _alt!=org.antlr.v4.runtime.atn.ATN.INVALID_ALT_NUMBER ) {
				if ( _alt==1 ) {
					if ( _parseListeners!=null ) triggerExitRuleEvent();
					_prevctx = _localctx;
					{
					setState(355);
					_errHandler.sync(this);
					switch ( getInterpreter().adaptivePredict(_input,28,_ctx) ) {
					case 1:
						{
						_localctx = new MulExprContext(new ExprContext(_parentctx, _parentState));
						pushNewRecursionContext(_localctx, _startState, RULE_expr);
						setState(325);
						if (!(precpred(_ctx, 6))) throw new FailedPredicateException(this, "precpred(_ctx, 6)");
						setState(326);
						mulOp();
						setState(327);
						expr(7);
						}
						break;
					case 2:
						{
						_localctx = new AddExprContext(new ExprContext(_parentctx, _parentState));
						pushNewRecursionContext(_localctx, _startState, RULE_expr);
						setState(329);
						if (!(precpred(_ctx, 5))) throw new FailedPredicateException(this, "precpred(_ctx, 5)");
						setState(330);
						addOp();
						setState(331);
						expr(6);
						}
						break;
					case 3:
						{
						_localctx = new OrdExprContext(new ExprContext(_parentctx, _parentState));
						pushNewRecursionContext(_localctx, _startState, RULE_expr);
						setState(333);
						if (!(precpred(_ctx, 4))) throw new FailedPredicateException(this, "precpred(_ctx, 4)");
						setState(334);
						ordOp();
						setState(335);
						expr(5);
						}
						break;
					case 4:
						{
						_localctx = new EqExprContext(new ExprContext(_parentctx, _parentState));
						pushNewRecursionContext(_localctx, _startState, RULE_expr);
						setState(337);
						if (!(precpred(_ctx, 3))) throw new FailedPredicateException(this, "precpred(_ctx, 3)");
						setState(338);
						eqOp();
						setState(339);
						expr(4);
						}
						break;
					case 5:
						{
						_localctx = new AndExprContext(new ExprContext(_parentctx, _parentState));
						pushNewRecursionContext(_localctx, _startState, RULE_expr);
						setState(341);
						if (!(precpred(_ctx, 2))) throw new FailedPredicateException(this, "precpred(_ctx, 2)");
						setState(342);
						match(AND);
						setState(343);
						expr(3);
						}
						break;
					case 6:
						{
						_localctx = new OrExprContext(new ExprContext(_parentctx, _parentState));
						pushNewRecursionContext(_localctx, _startState, RULE_expr);
						setState(344);
						if (!(precpred(_ctx, 1))) throw new FailedPredicateException(this, "precpred(_ctx, 1)");
						setState(345);
						match(OR);
						setState(346);
						expr(2);
						}
						break;
					case 7:
						{
						_localctx = new ArrAccessExprContext(new ExprContext(_parentctx, _parentState));
						pushNewRecursionContext(_localctx, _startState, RULE_expr);
						setState(347);
						if (!(precpred(_ctx, 9))) throw new FailedPredicateException(this, "precpred(_ctx, 9)");
						setState(348);
						match(LBRACKET);
						setState(349);
						expr(0);
						setState(350);
						match(RBRACKET);
						}
						break;
					case 8:
						{
						_localctx = new MemExprContext(new ExprContext(_parentctx, _parentState));
						pushNewRecursionContext(_localctx, _startState, RULE_expr);
						setState(352);
						if (!(precpred(_ctx, 8))) throw new FailedPredicateException(this, "precpred(_ctx, 8)");
						setState(353);
						match(PERIOD);
						setState(354);
						match(IDENT);
						}
						break;
					}
					} 
				}
				setState(359);
				_errHandler.sync(this);
				_alt = getInterpreter().adaptivePredict(_input,29,_ctx);
			}
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			unrollRecursionContexts(_parentctx);
		}
		return _localctx;
	}

	public static class LogicConstantContext extends ParserRuleContext {
		public LogicConstantContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_logicConstant; }
	 
		public LogicConstantContext() { }
		public void copyFrom(LogicConstantContext ctx) {
			super.copyFrom(ctx);
		}
	}
	public static class LogicTrueConstContext extends LogicConstantContext {
		public TerminalNode ANNO_TRUE() { return getToken(TaurusParser.ANNO_TRUE, 0); }
		public LogicTrueConstContext(LogicConstantContext ctx) { copyFrom(ctx); }
	}
	public static class LogicFalseConstContext extends LogicConstantContext {
		public TerminalNode ANNO_FALSE() { return getToken(TaurusParser.ANNO_FALSE, 0); }
		public LogicFalseConstContext(LogicConstantContext ctx) { copyFrom(ctx); }
	}
	public static class LogicIntConstContext extends LogicConstantContext {
		public TerminalNode INT_CONSTANT() { return getToken(TaurusParser.INT_CONSTANT, 0); }
		public LogicIntConstContext(LogicConstantContext ctx) { copyFrom(ctx); }
	}
	public static class LogicFloatConstContext extends LogicConstantContext {
		public TerminalNode FLOAT_CONSTANT() { return getToken(TaurusParser.FLOAT_CONSTANT, 0); }
		public LogicFloatConstContext(LogicConstantContext ctx) { copyFrom(ctx); }
	}

	public final LogicConstantContext logicConstant() throws RecognitionException {
		LogicConstantContext _localctx = new LogicConstantContext(_ctx, getState());
		enterRule(_localctx, 36, RULE_logicConstant);
		try {
			setState(364);
			_errHandler.sync(this);
			switch (_input.LA(1)) {
			case INT_CONSTANT:
				_localctx = new LogicIntConstContext(_localctx);
				enterOuterAlt(_localctx, 1);
				{
				setState(360);
				match(INT_CONSTANT);
				}
				break;
			case FLOAT_CONSTANT:
				_localctx = new LogicFloatConstContext(_localctx);
				enterOuterAlt(_localctx, 2);
				{
				setState(361);
				match(FLOAT_CONSTANT);
				}
				break;
			case ANNO_TRUE:
				_localctx = new LogicTrueConstContext(_localctx);
				enterOuterAlt(_localctx, 3);
				{
				setState(362);
				match(ANNO_TRUE);
				}
				break;
			case ANNO_FALSE:
				_localctx = new LogicFalseConstContext(_localctx);
				enterOuterAlt(_localctx, 4);
				{
				setState(363);
				match(ANNO_FALSE);
				}
				break;
			default:
				throw new NoViableAltException(this);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	public static class ArithTermContext extends ParserRuleContext {
		public ArithTermContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_arithTerm; }
	 
		public ArithTermContext() { }
		public void copyFrom(ArithTermContext ctx) {
			super.copyFrom(ctx);
		}
	}
	public static class ParArithTermContext extends ArithTermContext {
		public TerminalNode LPAR() { return getToken(TaurusParser.LPAR, 0); }
		public ArithTermContext arithTerm() {
			return getRuleContext(ArithTermContext.class,0);
		}
		public TerminalNode RPAR() { return getToken(TaurusParser.RPAR, 0); }
		public ParArithTermContext(ArithTermContext ctx) { copyFrom(ctx); }
	}
	public static class MulTermContext extends ArithTermContext {
		public List<ArithTermContext> arithTerm() {
			return getRuleContexts(ArithTermContext.class);
		}
		public ArithTermContext arithTerm(int i) {
			return getRuleContext(ArithTermContext.class,i);
		}
		public MulOpContext mulOp() {
			return getRuleContext(MulOpContext.class,0);
		}
		public MulTermContext(ArithTermContext ctx) { copyFrom(ctx); }
	}
	public static class MemTermContext extends ArithTermContext {
		public ArithTermContext arithTerm() {
			return getRuleContext(ArithTermContext.class,0);
		}
		public TerminalNode PERIOD() { return getToken(TaurusParser.PERIOD, 0); }
		public TerminalNode IDENT() { return getToken(TaurusParser.IDENT, 0); }
		public MemTermContext(ArithTermContext ctx) { copyFrom(ctx); }
	}
	public static class IdentTermContext extends ArithTermContext {
		public TerminalNode IDENT() { return getToken(TaurusParser.IDENT, 0); }
		public IdentTermContext(ArithTermContext ctx) { copyFrom(ctx); }
	}
	public static class UnaryTermContext extends ArithTermContext {
		public UnaryOpContext unaryOp() {
			return getRuleContext(UnaryOpContext.class,0);
		}
		public ArithTermContext arithTerm() {
			return getRuleContext(ArithTermContext.class,0);
		}
		public UnaryTermContext(ArithTermContext ctx) { copyFrom(ctx); }
	}
	public static class ArrAccessTermContext extends ArithTermContext {
		public List<ArithTermContext> arithTerm() {
			return getRuleContexts(ArithTermContext.class);
		}
		public ArithTermContext arithTerm(int i) {
			return getRuleContext(ArithTermContext.class,i);
		}
		public TerminalNode LBRACKET() { return getToken(TaurusParser.LBRACKET, 0); }
		public TerminalNode RBRACKET() { return getToken(TaurusParser.RBRACKET, 0); }
		public ArrAccessTermContext(ArithTermContext ctx) { copyFrom(ctx); }
	}
	public static class ResTermContext extends ArithTermContext {
		public TerminalNode RESULT() { return getToken(TaurusParser.RESULT, 0); }
		public ResTermContext(ArithTermContext ctx) { copyFrom(ctx); }
	}
	public static class ArrUpdTermContext extends ArithTermContext {
		public TerminalNode LBRACE() { return getToken(TaurusParser.LBRACE, 0); }
		public List<ArithTermContext> arithTerm() {
			return getRuleContexts(ArithTermContext.class);
		}
		public ArithTermContext arithTerm(int i) {
			return getRuleContext(ArithTermContext.class,i);
		}
		public TerminalNode WITH() { return getToken(TaurusParser.WITH, 0); }
		public TerminalNode LBRACKET() { return getToken(TaurusParser.LBRACKET, 0); }
		public TerminalNode RBRACKET() { return getToken(TaurusParser.RBRACKET, 0); }
		public TerminalNode ASSIGN() { return getToken(TaurusParser.ASSIGN, 0); }
		public TerminalNode RBRACE() { return getToken(TaurusParser.RBRACE, 0); }
		public ArrUpdTermContext(ArithTermContext ctx) { copyFrom(ctx); }
	}
	public static class ConstTermContext extends ArithTermContext {
		public LogicConstantContext logicConstant() {
			return getRuleContext(LogicConstantContext.class,0);
		}
		public ConstTermContext(ArithTermContext ctx) { copyFrom(ctx); }
	}
	public static class AddTermContext extends ArithTermContext {
		public List<ArithTermContext> arithTerm() {
			return getRuleContexts(ArithTermContext.class);
		}
		public ArithTermContext arithTerm(int i) {
			return getRuleContext(ArithTermContext.class,i);
		}
		public AddOpContext addOp() {
			return getRuleContext(AddOpContext.class,0);
		}
		public AddTermContext(ArithTermContext ctx) { copyFrom(ctx); }
	}

	public final ArithTermContext arithTerm() throws RecognitionException {
		return arithTerm(0);
	}

	private ArithTermContext arithTerm(int _p) throws RecognitionException {
		ParserRuleContext _parentctx = _ctx;
		int _parentState = getState();
		ArithTermContext _localctx = new ArithTermContext(_ctx, _parentState);
		ArithTermContext _prevctx = _localctx;
		int _startState = 38;
		enterRecursionRule(_localctx, 38, RULE_arithTerm, _p);
		try {
			int _alt;
			enterOuterAlt(_localctx, 1);
			{
			setState(387);
			_errHandler.sync(this);
			switch (_input.LA(1)) {
			case IDENT:
				{
				_localctx = new IdentTermContext(_localctx);
				_ctx = _localctx;
				_prevctx = _localctx;

				setState(367);
				match(IDENT);
				}
				break;
			case RESULT:
				{
				_localctx = new ResTermContext(_localctx);
				_ctx = _localctx;
				_prevctx = _localctx;
				setState(368);
				match(RESULT);
				}
				break;
			case ANNO_TRUE:
			case ANNO_FALSE:
			case INT_CONSTANT:
			case FLOAT_CONSTANT:
				{
				_localctx = new ConstTermContext(_localctx);
				_ctx = _localctx;
				_prevctx = _localctx;
				setState(369);
				logicConstant();
				}
				break;
			case LBRACE:
				{
				_localctx = new ArrUpdTermContext(_localctx);
				_ctx = _localctx;
				_prevctx = _localctx;
				setState(370);
				match(LBRACE);
				setState(371);
				arithTerm(0);
				setState(372);
				match(WITH);
				setState(373);
				match(LBRACKET);
				setState(374);
				arithTerm(0);
				setState(375);
				match(RBRACKET);
				setState(376);
				match(ASSIGN);
				setState(377);
				arithTerm(0);
				setState(378);
				match(RBRACE);
				}
				break;
			case LPAR:
				{
				_localctx = new ParArithTermContext(_localctx);
				_ctx = _localctx;
				_prevctx = _localctx;
				setState(380);
				match(LPAR);
				setState(381);
				arithTerm(0);
				setState(382);
				match(RPAR);
				}
				break;
			case MINUS:
			case NOT:
				{
				_localctx = new UnaryTermContext(_localctx);
				_ctx = _localctx;
				_prevctx = _localctx;
				setState(384);
				unaryOp();
				setState(385);
				arithTerm(3);
				}
				break;
			default:
				throw new NoViableAltException(this);
			}
			_ctx.stop = _input.LT(-1);
			setState(407);
			_errHandler.sync(this);
			_alt = getInterpreter().adaptivePredict(_input,33,_ctx);
			while ( _alt!=2 && _alt!=org.antlr.v4.runtime.atn.ATN.INVALID_ALT_NUMBER ) {
				if ( _alt==1 ) {
					if ( _parseListeners!=null ) triggerExitRuleEvent();
					_prevctx = _localctx;
					{
					setState(405);
					_errHandler.sync(this);
					switch ( getInterpreter().adaptivePredict(_input,32,_ctx) ) {
					case 1:
						{
						_localctx = new MulTermContext(new ArithTermContext(_parentctx, _parentState));
						pushNewRecursionContext(_localctx, _startState, RULE_arithTerm);
						setState(389);
						if (!(precpred(_ctx, 2))) throw new FailedPredicateException(this, "precpred(_ctx, 2)");
						setState(390);
						mulOp();
						setState(391);
						arithTerm(3);
						}
						break;
					case 2:
						{
						_localctx = new AddTermContext(new ArithTermContext(_parentctx, _parentState));
						pushNewRecursionContext(_localctx, _startState, RULE_arithTerm);
						setState(393);
						if (!(precpred(_ctx, 1))) throw new FailedPredicateException(this, "precpred(_ctx, 1)");
						setState(394);
						addOp();
						setState(395);
						arithTerm(2);
						}
						break;
					case 3:
						{
						_localctx = new ArrAccessTermContext(new ArithTermContext(_parentctx, _parentState));
						pushNewRecursionContext(_localctx, _startState, RULE_arithTerm);
						setState(397);
						if (!(precpred(_ctx, 5))) throw new FailedPredicateException(this, "precpred(_ctx, 5)");
						setState(398);
						match(LBRACKET);
						setState(399);
						arithTerm(0);
						setState(400);
						match(RBRACKET);
						}
						break;
					case 4:
						{
						_localctx = new MemTermContext(new ArithTermContext(_parentctx, _parentState));
						pushNewRecursionContext(_localctx, _startState, RULE_arithTerm);
						setState(402);
						if (!(precpred(_ctx, 4))) throw new FailedPredicateException(this, "precpred(_ctx, 4)");
						setState(403);
						match(PERIOD);
						setState(404);
						match(IDENT);
						}
						break;
					}
					} 
				}
				setState(409);
				_errHandler.sync(this);
				_alt = getInterpreter().adaptivePredict(_input,33,_ctx);
			}
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			unrollRecursionContexts(_parentctx);
		}
		return _localctx;
	}

	public static class TermContext extends ParserRuleContext {
		public TermContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_term; }
	 
		public TermContext() { }
		public void copyFrom(TermContext ctx) {
			super.copyFrom(ctx);
		}
	}
	public static class OrdTermContext extends TermContext {
		public List<TermContext> term() {
			return getRuleContexts(TermContext.class);
		}
		public TermContext term(int i) {
			return getRuleContext(TermContext.class,i);
		}
		public OrdOpContext ordOp() {
			return getRuleContext(OrdOpContext.class,0);
		}
		public OrdTermContext(TermContext ctx) { copyFrom(ctx); }
	}
	public static class AndTermContext extends TermContext {
		public List<TermContext> term() {
			return getRuleContexts(TermContext.class);
		}
		public TermContext term(int i) {
			return getRuleContext(TermContext.class,i);
		}
		public TerminalNode AND() { return getToken(TaurusParser.AND, 0); }
		public AndTermContext(TermContext ctx) { copyFrom(ctx); }
	}
	public static class OrTermContext extends TermContext {
		public List<TermContext> term() {
			return getRuleContexts(TermContext.class);
		}
		public TermContext term(int i) {
			return getRuleContext(TermContext.class,i);
		}
		public TerminalNode OR() { return getToken(TaurusParser.OR, 0); }
		public OrTermContext(TermContext ctx) { copyFrom(ctx); }
	}
	public static class EqTermContext extends TermContext {
		public List<TermContext> term() {
			return getRuleContexts(TermContext.class);
		}
		public TermContext term(int i) {
			return getRuleContext(TermContext.class,i);
		}
		public EqOpContext eqOp() {
			return getRuleContext(EqOpContext.class,0);
		}
		public EqTermContext(TermContext ctx) { copyFrom(ctx); }
	}
	public static class AriTermContext extends TermContext {
		public ArithTermContext arithTerm() {
			return getRuleContext(ArithTermContext.class,0);
		}
		public AriTermContext(TermContext ctx) { copyFrom(ctx); }
	}
	public static class ParTermContext extends TermContext {
		public TerminalNode LPAR() { return getToken(TaurusParser.LPAR, 0); }
		public TermContext term() {
			return getRuleContext(TermContext.class,0);
		}
		public TerminalNode RPAR() { return getToken(TaurusParser.RPAR, 0); }
		public ParTermContext(TermContext ctx) { copyFrom(ctx); }
	}

	public final TermContext term() throws RecognitionException {
		return term(0);
	}

	private TermContext term(int _p) throws RecognitionException {
		ParserRuleContext _parentctx = _ctx;
		int _parentState = getState();
		TermContext _localctx = new TermContext(_ctx, _parentState);
		TermContext _prevctx = _localctx;
		int _startState = 40;
		enterRecursionRule(_localctx, 40, RULE_term, _p);
		try {
			int _alt;
			enterOuterAlt(_localctx, 1);
			{
			setState(416);
			_errHandler.sync(this);
			switch ( getInterpreter().adaptivePredict(_input,34,_ctx) ) {
			case 1:
				{
				_localctx = new AriTermContext(_localctx);
				_ctx = _localctx;
				_prevctx = _localctx;

				setState(411);
				arithTerm(0);
				}
				break;
			case 2:
				{
				_localctx = new ParTermContext(_localctx);
				_ctx = _localctx;
				_prevctx = _localctx;
				setState(412);
				match(LPAR);
				setState(413);
				term(0);
				setState(414);
				match(RPAR);
				}
				break;
			}
			_ctx.stop = _input.LT(-1);
			setState(434);
			_errHandler.sync(this);
			_alt = getInterpreter().adaptivePredict(_input,36,_ctx);
			while ( _alt!=2 && _alt!=org.antlr.v4.runtime.atn.ATN.INVALID_ALT_NUMBER ) {
				if ( _alt==1 ) {
					if ( _parseListeners!=null ) triggerExitRuleEvent();
					_prevctx = _localctx;
					{
					setState(432);
					_errHandler.sync(this);
					switch ( getInterpreter().adaptivePredict(_input,35,_ctx) ) {
					case 1:
						{
						_localctx = new OrdTermContext(new TermContext(_parentctx, _parentState));
						pushNewRecursionContext(_localctx, _startState, RULE_term);
						setState(418);
						if (!(precpred(_ctx, 4))) throw new FailedPredicateException(this, "precpred(_ctx, 4)");
						setState(419);
						ordOp();
						setState(420);
						term(5);
						}
						break;
					case 2:
						{
						_localctx = new EqTermContext(new TermContext(_parentctx, _parentState));
						pushNewRecursionContext(_localctx, _startState, RULE_term);
						setState(422);
						if (!(precpred(_ctx, 3))) throw new FailedPredicateException(this, "precpred(_ctx, 3)");
						setState(423);
						eqOp();
						setState(424);
						term(4);
						}
						break;
					case 3:
						{
						_localctx = new AndTermContext(new TermContext(_parentctx, _parentState));
						pushNewRecursionContext(_localctx, _startState, RULE_term);
						setState(426);
						if (!(precpred(_ctx, 2))) throw new FailedPredicateException(this, "precpred(_ctx, 2)");
						setState(427);
						match(AND);
						setState(428);
						term(3);
						}
						break;
					case 4:
						{
						_localctx = new OrTermContext(new TermContext(_parentctx, _parentState));
						pushNewRecursionContext(_localctx, _startState, RULE_term);
						setState(429);
						if (!(precpred(_ctx, 1))) throw new FailedPredicateException(this, "precpred(_ctx, 1)");
						setState(430);
						match(OR);
						setState(431);
						term(2);
						}
						break;
					}
					} 
				}
				setState(436);
				_errHandler.sync(this);
				_alt = getInterpreter().adaptivePredict(_input,36,_ctx);
			}
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			unrollRecursionContexts(_parentctx);
		}
		return _localctx;
	}

	public static class PredContext extends ParserRuleContext {
		public PredContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_pred; }
	 
		public PredContext() { }
		public void copyFrom(PredContext ctx) {
			super.copyFrom(ctx);
		}
	}
	public static class DisPredContext extends PredContext {
		public List<PredContext> pred() {
			return getRuleContexts(PredContext.class);
		}
		public PredContext pred(int i) {
			return getRuleContext(PredContext.class,i);
		}
		public TerminalNode OR() { return getToken(TaurusParser.OR, 0); }
		public DisPredContext(PredContext ctx) { copyFrom(ctx); }
	}
	public static class LengthPredContext extends PredContext {
		public TerminalNode VALID() { return getToken(TaurusParser.VALID, 0); }
		public List<TerminalNode> LPAR() { return getTokens(TaurusParser.LPAR); }
		public TerminalNode LPAR(int i) {
			return getToken(TaurusParser.LPAR, i);
		}
		public TerminalNode IDENT() { return getToken(TaurusParser.IDENT, 0); }
		public TerminalNode ADD() { return getToken(TaurusParser.ADD, 0); }
		public TerminalNode INT_CONSTANT() { return getToken(TaurusParser.INT_CONSTANT, 0); }
		public TerminalNode APOSTROPHE() { return getToken(TaurusParser.APOSTROPHE, 0); }
		public ArithTermContext arithTerm() {
			return getRuleContext(ArithTermContext.class,0);
		}
		public List<TerminalNode> RPAR() { return getTokens(TaurusParser.RPAR); }
		public TerminalNode RPAR(int i) {
			return getToken(TaurusParser.RPAR, i);
		}
		public LengthPredContext(PredContext ctx) { copyFrom(ctx); }
	}
	public static class QuantiPredContext extends PredContext {
		public QuantifierContext quantifier() {
			return getRuleContext(QuantifierContext.class,0);
		}
		public List<BinderContext> binder() {
			return getRuleContexts(BinderContext.class);
		}
		public BinderContext binder(int i) {
			return getRuleContext(BinderContext.class,i);
		}
		public TerminalNode SEMICOLON() { return getToken(TaurusParser.SEMICOLON, 0); }
		public PredContext pred() {
			return getRuleContext(PredContext.class,0);
		}
		public List<TerminalNode> COMMA() { return getTokens(TaurusParser.COMMA); }
		public TerminalNode COMMA(int i) {
			return getToken(TaurusParser.COMMA, i);
		}
		public QuantiPredContext(PredContext ctx) { copyFrom(ctx); }
	}
	public static class XorPredContext extends PredContext {
		public List<PredContext> pred() {
			return getRuleContexts(PredContext.class);
		}
		public PredContext pred(int i) {
			return getRuleContext(PredContext.class,i);
		}
		public TerminalNode XOR() { return getToken(TaurusParser.XOR, 0); }
		public XorPredContext(PredContext ctx) { copyFrom(ctx); }
	}
	public static class ParPredContext extends PredContext {
		public TerminalNode LPAR() { return getToken(TaurusParser.LPAR, 0); }
		public PredContext pred() {
			return getRuleContext(PredContext.class,0);
		}
		public TerminalNode RPAR() { return getToken(TaurusParser.RPAR, 0); }
		public ParPredContext(PredContext ctx) { copyFrom(ctx); }
	}
	public static class TruePredContext extends PredContext {
		public TerminalNode ANNO_TRUE() { return getToken(TaurusParser.ANNO_TRUE, 0); }
		public TruePredContext(PredContext ctx) { copyFrom(ctx); }
	}
	public static class NotPredContext extends PredContext {
		public TerminalNode NOT() { return getToken(TaurusParser.NOT, 0); }
		public PredContext pred() {
			return getRuleContext(PredContext.class,0);
		}
		public NotPredContext(PredContext ctx) { copyFrom(ctx); }
	}
	public static class CallPredContext extends PredContext {
		public TerminalNode IDENT() { return getToken(TaurusParser.IDENT, 0); }
		public TerminalNode LPAR() { return getToken(TaurusParser.LPAR, 0); }
		public List<TermContext> term() {
			return getRuleContexts(TermContext.class);
		}
		public TermContext term(int i) {
			return getRuleContext(TermContext.class,i);
		}
		public TerminalNode RPAR() { return getToken(TaurusParser.RPAR, 0); }
		public List<TerminalNode> COMMA() { return getTokens(TaurusParser.COMMA); }
		public TerminalNode COMMA(int i) {
			return getToken(TaurusParser.COMMA, i);
		}
		public CallPredContext(PredContext ctx) { copyFrom(ctx); }
	}
	public static class IffPredContext extends PredContext {
		public List<PredContext> pred() {
			return getRuleContexts(PredContext.class);
		}
		public PredContext pred(int i) {
			return getRuleContext(PredContext.class,i);
		}
		public TerminalNode EQUIV() { return getToken(TaurusParser.EQUIV, 0); }
		public IffPredContext(PredContext ctx) { copyFrom(ctx); }
	}
	public static class CmpPredContext extends PredContext {
		public List<ArithTermContext> arithTerm() {
			return getRuleContexts(ArithTermContext.class);
		}
		public ArithTermContext arithTerm(int i) {
			return getRuleContext(ArithTermContext.class,i);
		}
		public List<CmpOpContext> cmpOp() {
			return getRuleContexts(CmpOpContext.class);
		}
		public CmpOpContext cmpOp(int i) {
			return getRuleContext(CmpOpContext.class,i);
		}
		public CmpPredContext(PredContext ctx) { copyFrom(ctx); }
	}
	public static class FalsePredContext extends PredContext {
		public TerminalNode ANNO_FALSE() { return getToken(TaurusParser.ANNO_FALSE, 0); }
		public FalsePredContext(PredContext ctx) { copyFrom(ctx); }
	}
	public static class ImpPredContext extends PredContext {
		public List<PredContext> pred() {
			return getRuleContexts(PredContext.class);
		}
		public PredContext pred(int i) {
			return getRuleContext(PredContext.class,i);
		}
		public TerminalNode IMPLY() { return getToken(TaurusParser.IMPLY, 0); }
		public ImpPredContext(PredContext ctx) { copyFrom(ctx); }
	}
	public static class ConPredContext extends PredContext {
		public List<PredContext> pred() {
			return getRuleContexts(PredContext.class);
		}
		public PredContext pred(int i) {
			return getRuleContext(PredContext.class,i);
		}
		public TerminalNode AND() { return getToken(TaurusParser.AND, 0); }
		public ConPredContext(PredContext ctx) { copyFrom(ctx); }
	}

	public final PredContext pred() throws RecognitionException {
		return pred(0);
	}

	private PredContext pred(int _p) throws RecognitionException {
		ParserRuleContext _parentctx = _ctx;
		int _parentState = getState();
		PredContext _localctx = new PredContext(_ctx, _parentState);
		PredContext _prevctx = _localctx;
		int _startState = 42;
		enterRecursionRule(_localctx, 42, RULE_pred, _p);
		int _la;
		try {
			int _alt;
			enterOuterAlt(_localctx, 1);
			{
			setState(491);
			_errHandler.sync(this);
			switch ( getInterpreter().adaptivePredict(_input,41,_ctx) ) {
			case 1:
				{
				_localctx = new TruePredContext(_localctx);
				_ctx = _localctx;
				_prevctx = _localctx;

				setState(438);
				match(ANNO_TRUE);
				}
				break;
			case 2:
				{
				_localctx = new FalsePredContext(_localctx);
				_ctx = _localctx;
				_prevctx = _localctx;
				setState(439);
				match(ANNO_FALSE);
				}
				break;
			case 3:
				{
				_localctx = new CmpPredContext(_localctx);
				_ctx = _localctx;
				_prevctx = _localctx;
				setState(440);
				arithTerm(0);
				setState(444); 
				_errHandler.sync(this);
				_alt = 1;
				do {
					switch (_alt) {
					case 1:
						{
						{
						setState(441);
						cmpOp();
						setState(442);
						arithTerm(0);
						}
						}
						break;
					default:
						throw new NoViableAltException(this);
					}
					setState(446); 
					_errHandler.sync(this);
					_alt = getInterpreter().adaptivePredict(_input,37,_ctx);
				} while ( _alt!=2 && _alt!=org.antlr.v4.runtime.atn.ATN.INVALID_ALT_NUMBER );
				}
				break;
			case 4:
				{
				_localctx = new CallPredContext(_localctx);
				_ctx = _localctx;
				_prevctx = _localctx;
				setState(448);
				match(IDENT);
				setState(460);
				_errHandler.sync(this);
				switch ( getInterpreter().adaptivePredict(_input,39,_ctx) ) {
				case 1:
					{
					setState(449);
					match(LPAR);
					setState(450);
					term(0);
					setState(455);
					_errHandler.sync(this);
					_la = _input.LA(1);
					while (_la==COMMA) {
						{
						{
						setState(451);
						match(COMMA);
						setState(452);
						term(0);
						}
						}
						setState(457);
						_errHandler.sync(this);
						_la = _input.LA(1);
					}
					setState(458);
					match(RPAR);
					}
					break;
				}
				}
				break;
			case 5:
				{
				_localctx = new ParPredContext(_localctx);
				_ctx = _localctx;
				_prevctx = _localctx;
				setState(462);
				match(LPAR);
				setState(463);
				pred(0);
				setState(464);
				match(RPAR);
				}
				break;
			case 6:
				{
				_localctx = new NotPredContext(_localctx);
				_ctx = _localctx;
				_prevctx = _localctx;
				setState(466);
				match(NOT);
				setState(467);
				pred(4);
				}
				break;
			case 7:
				{
				_localctx = new LengthPredContext(_localctx);
				_ctx = _localctx;
				_prevctx = _localctx;
				setState(468);
				match(VALID);
				setState(469);
				match(LPAR);
				setState(470);
				match(IDENT);
				setState(471);
				match(ADD);
				setState(472);
				match(LPAR);
				setState(473);
				match(INT_CONSTANT);
				setState(474);
				match(APOSTROPHE);
				setState(475);
				arithTerm(0);
				setState(476);
				match(RPAR);
				setState(477);
				match(RPAR);
				}
				break;
			case 8:
				{
				_localctx = new QuantiPredContext(_localctx);
				_ctx = _localctx;
				_prevctx = _localctx;
				setState(479);
				quantifier();
				setState(480);
				binder();
				setState(485);
				_errHandler.sync(this);
				_la = _input.LA(1);
				while (_la==COMMA) {
					{
					{
					setState(481);
					match(COMMA);
					setState(482);
					binder();
					}
					}
					setState(487);
					_errHandler.sync(this);
					_la = _input.LA(1);
				}
				setState(488);
				match(SEMICOLON);
				setState(489);
				pred(1);
				}
				break;
			}
			_ctx.stop = _input.LT(-1);
			setState(510);
			_errHandler.sync(this);
			_alt = getInterpreter().adaptivePredict(_input,43,_ctx);
			while ( _alt!=2 && _alt!=org.antlr.v4.runtime.atn.ATN.INVALID_ALT_NUMBER ) {
				if ( _alt==1 ) {
					if ( _parseListeners!=null ) triggerExitRuleEvent();
					_prevctx = _localctx;
					{
					setState(508);
					_errHandler.sync(this);
					switch ( getInterpreter().adaptivePredict(_input,42,_ctx) ) {
					case 1:
						{
						_localctx = new ConPredContext(new PredContext(_parentctx, _parentState));
						pushNewRecursionContext(_localctx, _startState, RULE_pred);
						setState(493);
						if (!(precpred(_ctx, 8))) throw new FailedPredicateException(this, "precpred(_ctx, 8)");
						setState(494);
						match(AND);
						setState(495);
						pred(9);
						}
						break;
					case 2:
						{
						_localctx = new DisPredContext(new PredContext(_parentctx, _parentState));
						pushNewRecursionContext(_localctx, _startState, RULE_pred);
						setState(496);
						if (!(precpred(_ctx, 7))) throw new FailedPredicateException(this, "precpred(_ctx, 7)");
						setState(497);
						match(OR);
						setState(498);
						pred(8);
						}
						break;
					case 3:
						{
						_localctx = new ImpPredContext(new PredContext(_parentctx, _parentState));
						pushNewRecursionContext(_localctx, _startState, RULE_pred);
						setState(499);
						if (!(precpred(_ctx, 6))) throw new FailedPredicateException(this, "precpred(_ctx, 6)");
						setState(500);
						match(IMPLY);
						setState(501);
						pred(7);
						}
						break;
					case 4:
						{
						_localctx = new IffPredContext(new PredContext(_parentctx, _parentState));
						pushNewRecursionContext(_localctx, _startState, RULE_pred);
						setState(502);
						if (!(precpred(_ctx, 5))) throw new FailedPredicateException(this, "precpred(_ctx, 5)");
						setState(503);
						match(EQUIV);
						setState(504);
						pred(6);
						}
						break;
					case 5:
						{
						_localctx = new XorPredContext(new PredContext(_parentctx, _parentState));
						pushNewRecursionContext(_localctx, _startState, RULE_pred);
						setState(505);
						if (!(precpred(_ctx, 3))) throw new FailedPredicateException(this, "precpred(_ctx, 3)");
						setState(506);
						match(XOR);
						setState(507);
						pred(4);
						}
						break;
					}
					} 
				}
				setState(512);
				_errHandler.sync(this);
				_alt = getInterpreter().adaptivePredict(_input,43,_ctx);
			}
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			unrollRecursionContexts(_parentctx);
		}
		return _localctx;
	}

	public static class ArithOpContext extends ParserRuleContext {
		public ArithOpContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_arithOp; }
	 
		public ArithOpContext() { }
		public void copyFrom(ArithOpContext ctx) {
			super.copyFrom(ctx);
		}
	}
	public static class MulArithOpContext extends ArithOpContext {
		public MulOpContext mulOp() {
			return getRuleContext(MulOpContext.class,0);
		}
		public MulArithOpContext(ArithOpContext ctx) { copyFrom(ctx); }
	}
	public static class AddArithOpContext extends ArithOpContext {
		public AddOpContext addOp() {
			return getRuleContext(AddOpContext.class,0);
		}
		public AddArithOpContext(ArithOpContext ctx) { copyFrom(ctx); }
	}

	public final ArithOpContext arithOp() throws RecognitionException {
		ArithOpContext _localctx = new ArithOpContext(_ctx, getState());
		enterRule(_localctx, 44, RULE_arithOp);
		try {
			setState(515);
			_errHandler.sync(this);
			switch (_input.LA(1)) {
			case MUL:
			case DIV:
			case MOD:
				_localctx = new MulArithOpContext(_localctx);
				enterOuterAlt(_localctx, 1);
				{
				setState(513);
				mulOp();
				}
				break;
			case ADD:
			case MINUS:
				_localctx = new AddArithOpContext(_localctx);
				enterOuterAlt(_localctx, 2);
				{
				setState(514);
				addOp();
				}
				break;
			default:
				throw new NoViableAltException(this);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	public static class AddOpContext extends ParserRuleContext {
		public AddOpContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_addOp; }
	 
		public AddOpContext() { }
		public void copyFrom(AddOpContext ctx) {
			super.copyFrom(ctx);
		}
	}
	public static class PlusContext extends AddOpContext {
		public TerminalNode ADD() { return getToken(TaurusParser.ADD, 0); }
		public PlusContext(AddOpContext ctx) { copyFrom(ctx); }
	}
	public static class MinusContext extends AddOpContext {
		public TerminalNode MINUS() { return getToken(TaurusParser.MINUS, 0); }
		public MinusContext(AddOpContext ctx) { copyFrom(ctx); }
	}

	public final AddOpContext addOp() throws RecognitionException {
		AddOpContext _localctx = new AddOpContext(_ctx, getState());
		enterRule(_localctx, 46, RULE_addOp);
		try {
			setState(519);
			_errHandler.sync(this);
			switch (_input.LA(1)) {
			case ADD:
				_localctx = new PlusContext(_localctx);
				enterOuterAlt(_localctx, 1);
				{
				setState(517);
				match(ADD);
				}
				break;
			case MINUS:
				_localctx = new MinusContext(_localctx);
				enterOuterAlt(_localctx, 2);
				{
				setState(518);
				match(MINUS);
				}
				break;
			default:
				throw new NoViableAltException(this);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	public static class MulOpContext extends ParserRuleContext {
		public MulOpContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_mulOp; }
	 
		public MulOpContext() { }
		public void copyFrom(MulOpContext ctx) {
			super.copyFrom(ctx);
		}
	}
	public static class DivContext extends MulOpContext {
		public TerminalNode DIV() { return getToken(TaurusParser.DIV, 0); }
		public DivContext(MulOpContext ctx) { copyFrom(ctx); }
	}
	public static class ModContext extends MulOpContext {
		public TerminalNode MOD() { return getToken(TaurusParser.MOD, 0); }
		public ModContext(MulOpContext ctx) { copyFrom(ctx); }
	}
	public static class MulContext extends MulOpContext {
		public TerminalNode MUL() { return getToken(TaurusParser.MUL, 0); }
		public MulContext(MulOpContext ctx) { copyFrom(ctx); }
	}

	public final MulOpContext mulOp() throws RecognitionException {
		MulOpContext _localctx = new MulOpContext(_ctx, getState());
		enterRule(_localctx, 48, RULE_mulOp);
		try {
			setState(524);
			_errHandler.sync(this);
			switch (_input.LA(1)) {
			case MUL:
				_localctx = new MulContext(_localctx);
				enterOuterAlt(_localctx, 1);
				{
				setState(521);
				match(MUL);
				}
				break;
			case DIV:
				_localctx = new DivContext(_localctx);
				enterOuterAlt(_localctx, 2);
				{
				setState(522);
				match(DIV);
				}
				break;
			case MOD:
				_localctx = new ModContext(_localctx);
				enterOuterAlt(_localctx, 3);
				{
				setState(523);
				match(MOD);
				}
				break;
			default:
				throw new NoViableAltException(this);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	public static class CmpOpContext extends ParserRuleContext {
		public CmpOpContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_cmpOp; }
	 
		public CmpOpContext() { }
		public void copyFrom(CmpOpContext ctx) {
			super.copyFrom(ctx);
		}
	}
	public static class OrdCmpOpContext extends CmpOpContext {
		public OrdOpContext ordOp() {
			return getRuleContext(OrdOpContext.class,0);
		}
		public OrdCmpOpContext(CmpOpContext ctx) { copyFrom(ctx); }
	}
	public static class EqCmpOpContext extends CmpOpContext {
		public EqOpContext eqOp() {
			return getRuleContext(EqOpContext.class,0);
		}
		public EqCmpOpContext(CmpOpContext ctx) { copyFrom(ctx); }
	}

	public final CmpOpContext cmpOp() throws RecognitionException {
		CmpOpContext _localctx = new CmpOpContext(_ctx, getState());
		enterRule(_localctx, 50, RULE_cmpOp);
		try {
			setState(528);
			_errHandler.sync(this);
			switch (_input.LA(1)) {
			case LE:
			case LT:
			case GE:
			case GT:
				_localctx = new OrdCmpOpContext(_localctx);
				enterOuterAlt(_localctx, 1);
				{
				setState(526);
				ordOp();
				}
				break;
			case EQ:
			case NE:
				_localctx = new EqCmpOpContext(_localctx);
				enterOuterAlt(_localctx, 2);
				{
				setState(527);
				eqOp();
				}
				break;
			default:
				throw new NoViableAltException(this);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	public static class EqOpContext extends ParserRuleContext {
		public EqOpContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_eqOp; }
	 
		public EqOpContext() { }
		public void copyFrom(EqOpContext ctx) {
			super.copyFrom(ctx);
		}
	}
	public static class NeqContext extends EqOpContext {
		public TerminalNode NE() { return getToken(TaurusParser.NE, 0); }
		public NeqContext(EqOpContext ctx) { copyFrom(ctx); }
	}
	public static class EqContext extends EqOpContext {
		public TerminalNode EQ() { return getToken(TaurusParser.EQ, 0); }
		public EqContext(EqOpContext ctx) { copyFrom(ctx); }
	}

	public final EqOpContext eqOp() throws RecognitionException {
		EqOpContext _localctx = new EqOpContext(_ctx, getState());
		enterRule(_localctx, 52, RULE_eqOp);
		try {
			setState(532);
			_errHandler.sync(this);
			switch (_input.LA(1)) {
			case EQ:
				_localctx = new EqContext(_localctx);
				enterOuterAlt(_localctx, 1);
				{
				setState(530);
				match(EQ);
				}
				break;
			case NE:
				_localctx = new NeqContext(_localctx);
				enterOuterAlt(_localctx, 2);
				{
				setState(531);
				match(NE);
				}
				break;
			default:
				throw new NoViableAltException(this);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	public static class OrdOpContext extends ParserRuleContext {
		public OrdOpContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_ordOp; }
	 
		public OrdOpContext() { }
		public void copyFrom(OrdOpContext ctx) {
			super.copyFrom(ctx);
		}
	}
	public static class LtContext extends OrdOpContext {
		public TerminalNode LT() { return getToken(TaurusParser.LT, 0); }
		public LtContext(OrdOpContext ctx) { copyFrom(ctx); }
	}
	public static class LeContext extends OrdOpContext {
		public TerminalNode LE() { return getToken(TaurusParser.LE, 0); }
		public LeContext(OrdOpContext ctx) { copyFrom(ctx); }
	}
	public static class GtContext extends OrdOpContext {
		public TerminalNode GT() { return getToken(TaurusParser.GT, 0); }
		public GtContext(OrdOpContext ctx) { copyFrom(ctx); }
	}
	public static class GeContext extends OrdOpContext {
		public TerminalNode GE() { return getToken(TaurusParser.GE, 0); }
		public GeContext(OrdOpContext ctx) { copyFrom(ctx); }
	}

	public final OrdOpContext ordOp() throws RecognitionException {
		OrdOpContext _localctx = new OrdOpContext(_ctx, getState());
		enterRule(_localctx, 54, RULE_ordOp);
		try {
			setState(538);
			_errHandler.sync(this);
			switch (_input.LA(1)) {
			case LT:
				_localctx = new LtContext(_localctx);
				enterOuterAlt(_localctx, 1);
				{
				setState(534);
				match(LT);
				}
				break;
			case LE:
				_localctx = new LeContext(_localctx);
				enterOuterAlt(_localctx, 2);
				{
				setState(535);
				match(LE);
				}
				break;
			case GT:
				_localctx = new GtContext(_localctx);
				enterOuterAlt(_localctx, 3);
				{
				setState(536);
				match(GT);
				}
				break;
			case GE:
				_localctx = new GeContext(_localctx);
				enterOuterAlt(_localctx, 4);
				{
				setState(537);
				match(GE);
				}
				break;
			default:
				throw new NoViableAltException(this);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	public static class UnaryOpContext extends ParserRuleContext {
		public UnaryOpContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_unaryOp; }
	 
		public UnaryOpContext() { }
		public void copyFrom(UnaryOpContext ctx) {
			super.copyFrom(ctx);
		}
	}
	public static class NegContext extends UnaryOpContext {
		public TerminalNode MINUS() { return getToken(TaurusParser.MINUS, 0); }
		public NegContext(UnaryOpContext ctx) { copyFrom(ctx); }
	}
	public static class NotContext extends UnaryOpContext {
		public TerminalNode NOT() { return getToken(TaurusParser.NOT, 0); }
		public NotContext(UnaryOpContext ctx) { copyFrom(ctx); }
	}

	public final UnaryOpContext unaryOp() throws RecognitionException {
		UnaryOpContext _localctx = new UnaryOpContext(_ctx, getState());
		enterRule(_localctx, 56, RULE_unaryOp);
		try {
			setState(542);
			_errHandler.sync(this);
			switch (_input.LA(1)) {
			case MINUS:
				_localctx = new NegContext(_localctx);
				enterOuterAlt(_localctx, 1);
				{
				setState(540);
				match(MINUS);
				}
				break;
			case NOT:
				_localctx = new NotContext(_localctx);
				enterOuterAlt(_localctx, 2);
				{
				setState(541);
				match(NOT);
				}
				break;
			default:
				throw new NoViableAltException(this);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	public static class QuantifierContext extends ParserRuleContext {
		public QuantifierContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_quantifier; }
	 
		public QuantifierContext() { }
		public void copyFrom(QuantifierContext ctx) {
			super.copyFrom(ctx);
		}
	}
	public static class UniversalContext extends QuantifierContext {
		public TerminalNode FORALL() { return getToken(TaurusParser.FORALL, 0); }
		public UniversalContext(QuantifierContext ctx) { copyFrom(ctx); }
	}
	public static class ExistentialContext extends QuantifierContext {
		public TerminalNode EXISTS() { return getToken(TaurusParser.EXISTS, 0); }
		public ExistentialContext(QuantifierContext ctx) { copyFrom(ctx); }
	}

	public final QuantifierContext quantifier() throws RecognitionException {
		QuantifierContext _localctx = new QuantifierContext(_ctx, getState());
		enterRule(_localctx, 58, RULE_quantifier);
		try {
			setState(546);
			_errHandler.sync(this);
			switch (_input.LA(1)) {
			case FORALL:
				_localctx = new UniversalContext(_localctx);
				enterOuterAlt(_localctx, 1);
				{
				setState(544);
				match(FORALL);
				}
				break;
			case EXISTS:
				_localctx = new ExistentialContext(_localctx);
				enterOuterAlt(_localctx, 2);
				{
				setState(545);
				match(EXISTS);
				}
				break;
			default:
				throw new NoViableAltException(this);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	public static class BinderContext extends ParserRuleContext {
		public LogicAtomicTypeContext logicAtomicType() {
			return getRuleContext(LogicAtomicTypeContext.class,0);
		}
		public List<TerminalNode> IDENT() { return getTokens(TaurusParser.IDENT); }
		public TerminalNode IDENT(int i) {
			return getToken(TaurusParser.IDENT, i);
		}
		public List<TerminalNode> COMMA() { return getTokens(TaurusParser.COMMA); }
		public TerminalNode COMMA(int i) {
			return getToken(TaurusParser.COMMA, i);
		}
		public BinderContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_binder; }
	}

	public final BinderContext binder() throws RecognitionException {
		BinderContext _localctx = new BinderContext(_ctx, getState());
		enterRule(_localctx, 60, RULE_binder);
		try {
			int _alt;
			enterOuterAlt(_localctx, 1);
			{
			setState(548);
			logicAtomicType();
			setState(549);
			match(IDENT);
			setState(554);
			_errHandler.sync(this);
			_alt = getInterpreter().adaptivePredict(_input,52,_ctx);
			while ( _alt!=2 && _alt!=org.antlr.v4.runtime.atn.ATN.INVALID_ALT_NUMBER ) {
				if ( _alt==1 ) {
					{
					{
					setState(550);
					match(COMMA);
					setState(551);
					match(IDENT);
					}
					} 
				}
				setState(556);
				_errHandler.sync(this);
				_alt = getInterpreter().adaptivePredict(_input,52,_ctx);
			}
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	public static class FuncContractContext extends ParserRuleContext {
		public TerminalNode ANNOT_START() { return getToken(TaurusParser.ANNOT_START, 0); }
		public TerminalNode ANNOT_END() { return getToken(TaurusParser.ANNOT_END, 0); }
		public List<RequiresClauseContext> requiresClause() {
			return getRuleContexts(RequiresClauseContext.class);
		}
		public RequiresClauseContext requiresClause(int i) {
			return getRuleContext(RequiresClauseContext.class,i);
		}
		public DecreasesClauseContext decreasesClause() {
			return getRuleContext(DecreasesClauseContext.class,0);
		}
		public List<EnsuresClauseContext> ensuresClause() {
			return getRuleContexts(EnsuresClauseContext.class);
		}
		public EnsuresClauseContext ensuresClause(int i) {
			return getRuleContext(EnsuresClauseContext.class,i);
		}
		public TerminalNode LINE_ANNOT_START() { return getToken(TaurusParser.LINE_ANNOT_START, 0); }
		public TerminalNode LINEEND() { return getToken(TaurusParser.LINEEND, 0); }
		public FuncContractContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_funcContract; }
	}

	public final FuncContractContext funcContract() throws RecognitionException {
		FuncContractContext _localctx = new FuncContractContext(_ctx, getState());
		enterRule(_localctx, 62, RULE_funcContract);
		int _la;
		try {
			setState(591);
			_errHandler.sync(this);
			switch (_input.LA(1)) {
			case ANNOT_START:
				enterOuterAlt(_localctx, 1);
				{
				setState(557);
				match(ANNOT_START);
				setState(561);
				_errHandler.sync(this);
				_la = _input.LA(1);
				while (_la==REQUIRES) {
					{
					{
					setState(558);
					requiresClause();
					}
					}
					setState(563);
					_errHandler.sync(this);
					_la = _input.LA(1);
				}
				setState(565);
				_errHandler.sync(this);
				_la = _input.LA(1);
				if (_la==DECREASES) {
					{
					setState(564);
					decreasesClause();
					}
				}

				setState(570);
				_errHandler.sync(this);
				_la = _input.LA(1);
				while (_la==ENSURES) {
					{
					{
					setState(567);
					ensuresClause();
					}
					}
					setState(572);
					_errHandler.sync(this);
					_la = _input.LA(1);
				}
				setState(573);
				match(ANNOT_END);
				}
				break;
			case LINE_ANNOT_START:
				enterOuterAlt(_localctx, 2);
				{
				setState(574);
				match(LINE_ANNOT_START);
				setState(578);
				_errHandler.sync(this);
				_la = _input.LA(1);
				while (_la==REQUIRES) {
					{
					{
					setState(575);
					requiresClause();
					}
					}
					setState(580);
					_errHandler.sync(this);
					_la = _input.LA(1);
				}
				setState(582);
				_errHandler.sync(this);
				_la = _input.LA(1);
				if (_la==DECREASES) {
					{
					setState(581);
					decreasesClause();
					}
				}

				setState(587);
				_errHandler.sync(this);
				_la = _input.LA(1);
				while (_la==ENSURES) {
					{
					{
					setState(584);
					ensuresClause();
					}
					}
					setState(589);
					_errHandler.sync(this);
					_la = _input.LA(1);
				}
				setState(590);
				match(LINEEND);
				}
				break;
			default:
				throw new NoViableAltException(this);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	public static class RequiresClauseContext extends ParserRuleContext {
		public TerminalNode REQUIRES() { return getToken(TaurusParser.REQUIRES, 0); }
		public PredContext pred() {
			return getRuleContext(PredContext.class,0);
		}
		public TerminalNode SEMICOLON() { return getToken(TaurusParser.SEMICOLON, 0); }
		public RequiresClauseContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_requiresClause; }
	}

	public final RequiresClauseContext requiresClause() throws RecognitionException {
		RequiresClauseContext _localctx = new RequiresClauseContext(_ctx, getState());
		enterRule(_localctx, 64, RULE_requiresClause);
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(593);
			match(REQUIRES);
			setState(594);
			pred(0);
			setState(595);
			match(SEMICOLON);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	public static class DecreasesClauseContext extends ParserRuleContext {
		public TerminalNode DECREASES() { return getToken(TaurusParser.DECREASES, 0); }
		public TerminalNode SEMICOLON() { return getToken(TaurusParser.SEMICOLON, 0); }
		public List<ArithTermContext> arithTerm() {
			return getRuleContexts(ArithTermContext.class);
		}
		public ArithTermContext arithTerm(int i) {
			return getRuleContext(ArithTermContext.class,i);
		}
		public TerminalNode LPAR() { return getToken(TaurusParser.LPAR, 0); }
		public TerminalNode RPAR() { return getToken(TaurusParser.RPAR, 0); }
		public List<TerminalNode> COMMA() { return getTokens(TaurusParser.COMMA); }
		public TerminalNode COMMA(int i) {
			return getToken(TaurusParser.COMMA, i);
		}
		public DecreasesClauseContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_decreasesClause; }
	}

	public final DecreasesClauseContext decreasesClause() throws RecognitionException {
		DecreasesClauseContext _localctx = new DecreasesClauseContext(_ctx, getState());
		enterRule(_localctx, 66, RULE_decreasesClause);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(597);
			match(DECREASES);
			setState(609);
			_errHandler.sync(this);
			switch ( getInterpreter().adaptivePredict(_input,61,_ctx) ) {
			case 1:
				{
				setState(598);
				arithTerm(0);
				}
				break;
			case 2:
				{
				setState(599);
				match(LPAR);
				setState(600);
				arithTerm(0);
				setState(603); 
				_errHandler.sync(this);
				_la = _input.LA(1);
				do {
					{
					{
					setState(601);
					match(COMMA);
					setState(602);
					arithTerm(0);
					}
					}
					setState(605); 
					_errHandler.sync(this);
					_la = _input.LA(1);
				} while ( _la==COMMA );
				setState(607);
				match(RPAR);
				}
				break;
			}
			setState(611);
			match(SEMICOLON);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	public static class EnsuresClauseContext extends ParserRuleContext {
		public TerminalNode ENSURES() { return getToken(TaurusParser.ENSURES, 0); }
		public PredContext pred() {
			return getRuleContext(PredContext.class,0);
		}
		public TerminalNode SEMICOLON() { return getToken(TaurusParser.SEMICOLON, 0); }
		public EnsuresClauseContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_ensuresClause; }
	}

	public final EnsuresClauseContext ensuresClause() throws RecognitionException {
		EnsuresClauseContext _localctx = new EnsuresClauseContext(_ctx, getState());
		enterRule(_localctx, 68, RULE_ensuresClause);
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(613);
			match(ENSURES);
			setState(614);
			pred(0);
			setState(615);
			match(SEMICOLON);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	public static class AssertionContext extends ParserRuleContext {
		public TerminalNode ANNOT_START() { return getToken(TaurusParser.ANNOT_START, 0); }
		public TerminalNode ASSERT() { return getToken(TaurusParser.ASSERT, 0); }
		public PredContext pred() {
			return getRuleContext(PredContext.class,0);
		}
		public TerminalNode SEMICOLON() { return getToken(TaurusParser.SEMICOLON, 0); }
		public TerminalNode ANNOT_END() { return getToken(TaurusParser.ANNOT_END, 0); }
		public TerminalNode LINE_ANNOT_START() { return getToken(TaurusParser.LINE_ANNOT_START, 0); }
		public TerminalNode LINEEND() { return getToken(TaurusParser.LINEEND, 0); }
		public AssertionContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_assertion; }
	}

	public final AssertionContext assertion() throws RecognitionException {
		AssertionContext _localctx = new AssertionContext(_ctx, getState());
		enterRule(_localctx, 70, RULE_assertion);
		try {
			setState(629);
			_errHandler.sync(this);
			switch (_input.LA(1)) {
			case ANNOT_START:
				enterOuterAlt(_localctx, 1);
				{
				setState(617);
				match(ANNOT_START);
				setState(618);
				match(ASSERT);
				setState(619);
				pred(0);
				setState(620);
				match(SEMICOLON);
				setState(621);
				match(ANNOT_END);
				}
				break;
			case LINE_ANNOT_START:
				enterOuterAlt(_localctx, 2);
				{
				setState(623);
				match(LINE_ANNOT_START);
				setState(624);
				match(ASSERT);
				setState(625);
				pred(0);
				setState(626);
				match(SEMICOLON);
				setState(627);
				match(LINEEND);
				}
				break;
			default:
				throw new NoViableAltException(this);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	public static class LoopAnnotContext extends ParserRuleContext {
		public TerminalNode ANNOT_START() { return getToken(TaurusParser.ANNOT_START, 0); }
		public TerminalNode ANNOT_END() { return getToken(TaurusParser.ANNOT_END, 0); }
		public List<TerminalNode> LOOP() { return getTokens(TaurusParser.LOOP); }
		public TerminalNode LOOP(int i) {
			return getToken(TaurusParser.LOOP, i);
		}
		public List<TerminalNode> INVARIANT() { return getTokens(TaurusParser.INVARIANT); }
		public TerminalNode INVARIANT(int i) {
			return getToken(TaurusParser.INVARIANT, i);
		}
		public List<PredContext> pred() {
			return getRuleContexts(PredContext.class);
		}
		public PredContext pred(int i) {
			return getRuleContext(PredContext.class,i);
		}
		public List<TerminalNode> SEMICOLON() { return getTokens(TaurusParser.SEMICOLON); }
		public TerminalNode SEMICOLON(int i) {
			return getToken(TaurusParser.SEMICOLON, i);
		}
		public TerminalNode VARIANT() { return getToken(TaurusParser.VARIANT, 0); }
		public List<ArithTermContext> arithTerm() {
			return getRuleContexts(ArithTermContext.class);
		}
		public ArithTermContext arithTerm(int i) {
			return getRuleContext(ArithTermContext.class,i);
		}
		public TerminalNode LPAR() { return getToken(TaurusParser.LPAR, 0); }
		public TerminalNode RPAR() { return getToken(TaurusParser.RPAR, 0); }
		public List<TerminalNode> COMMA() { return getTokens(TaurusParser.COMMA); }
		public TerminalNode COMMA(int i) {
			return getToken(TaurusParser.COMMA, i);
		}
		public TerminalNode LINE_ANNOT_START() { return getToken(TaurusParser.LINE_ANNOT_START, 0); }
		public TerminalNode LINEEND() { return getToken(TaurusParser.LINEEND, 0); }
		public LoopAnnotContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_loopAnnot; }
	}

	public final LoopAnnotContext loopAnnot() throws RecognitionException {
		LoopAnnotContext _localctx = new LoopAnnotContext(_ctx, getState());
		enterRule(_localctx, 72, RULE_loopAnnot);
		int _la;
		try {
			int _alt;
			setState(693);
			_errHandler.sync(this);
			switch (_input.LA(1)) {
			case ANNOT_START:
				enterOuterAlt(_localctx, 1);
				{
				setState(631);
				match(ANNOT_START);
				setState(639);
				_errHandler.sync(this);
				_alt = getInterpreter().adaptivePredict(_input,63,_ctx);
				while ( _alt!=2 && _alt!=org.antlr.v4.runtime.atn.ATN.INVALID_ALT_NUMBER ) {
					if ( _alt==1 ) {
						{
						{
						setState(632);
						match(LOOP);
						setState(633);
						match(INVARIANT);
						setState(634);
						pred(0);
						setState(635);
						match(SEMICOLON);
						}
						} 
					}
					setState(641);
					_errHandler.sync(this);
					_alt = getInterpreter().adaptivePredict(_input,63,_ctx);
				}
				setState(659);
				_errHandler.sync(this);
				_la = _input.LA(1);
				if (_la==LOOP) {
					{
					setState(642);
					match(LOOP);
					setState(643);
					match(VARIANT);
					setState(655);
					_errHandler.sync(this);
					switch ( getInterpreter().adaptivePredict(_input,65,_ctx) ) {
					case 1:
						{
						setState(644);
						arithTerm(0);
						}
						break;
					case 2:
						{
						setState(645);
						match(LPAR);
						setState(646);
						arithTerm(0);
						setState(649); 
						_errHandler.sync(this);
						_la = _input.LA(1);
						do {
							{
							{
							setState(647);
							match(COMMA);
							setState(648);
							arithTerm(0);
							}
							}
							setState(651); 
							_errHandler.sync(this);
							_la = _input.LA(1);
						} while ( _la==COMMA );
						setState(653);
						match(RPAR);
						}
						break;
					}
					setState(657);
					match(SEMICOLON);
					}
				}

				setState(661);
				match(ANNOT_END);
				}
				break;
			case LINE_ANNOT_START:
				enterOuterAlt(_localctx, 2);
				{
				setState(662);
				match(LINE_ANNOT_START);
				setState(670);
				_errHandler.sync(this);
				_alt = getInterpreter().adaptivePredict(_input,67,_ctx);
				while ( _alt!=2 && _alt!=org.antlr.v4.runtime.atn.ATN.INVALID_ALT_NUMBER ) {
					if ( _alt==1 ) {
						{
						{
						setState(663);
						match(LOOP);
						setState(664);
						match(INVARIANT);
						setState(665);
						pred(0);
						setState(666);
						match(SEMICOLON);
						}
						} 
					}
					setState(672);
					_errHandler.sync(this);
					_alt = getInterpreter().adaptivePredict(_input,67,_ctx);
				}
				setState(690);
				_errHandler.sync(this);
				_la = _input.LA(1);
				if (_la==LOOP) {
					{
					setState(673);
					match(LOOP);
					setState(674);
					match(VARIANT);
					setState(686);
					_errHandler.sync(this);
					switch ( getInterpreter().adaptivePredict(_input,69,_ctx) ) {
					case 1:
						{
						setState(675);
						arithTerm(0);
						}
						break;
					case 2:
						{
						setState(676);
						match(LPAR);
						setState(677);
						arithTerm(0);
						setState(680); 
						_errHandler.sync(this);
						_la = _input.LA(1);
						do {
							{
							{
							setState(678);
							match(COMMA);
							setState(679);
							arithTerm(0);
							}
							}
							setState(682); 
							_errHandler.sync(this);
							_la = _input.LA(1);
						} while ( _la==COMMA );
						setState(684);
						match(RPAR);
						}
						break;
					}
					setState(688);
					match(SEMICOLON);
					}
				}

				setState(692);
				match(LINEEND);
				}
				break;
			default:
				throw new NoViableAltException(this);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	public static class PredDefsContext extends ParserRuleContext {
		public TerminalNode ANNOT_START() { return getToken(TaurusParser.ANNOT_START, 0); }
		public TerminalNode ANNOT_END() { return getToken(TaurusParser.ANNOT_END, 0); }
		public List<PredDefContext> predDef() {
			return getRuleContexts(PredDefContext.class);
		}
		public PredDefContext predDef(int i) {
			return getRuleContext(PredDefContext.class,i);
		}
		public TerminalNode LINE_ANNOT_START() { return getToken(TaurusParser.LINE_ANNOT_START, 0); }
		public TerminalNode LINEEND() { return getToken(TaurusParser.LINEEND, 0); }
		public PredDefsContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_predDefs; }
	}

	public final PredDefsContext predDefs() throws RecognitionException {
		PredDefsContext _localctx = new PredDefsContext(_ctx, getState());
		enterRule(_localctx, 74, RULE_predDefs);
		int _la;
		try {
			setState(711);
			_errHandler.sync(this);
			switch (_input.LA(1)) {
			case ANNOT_START:
				enterOuterAlt(_localctx, 1);
				{
				setState(695);
				match(ANNOT_START);
				setState(699);
				_errHandler.sync(this);
				_la = _input.LA(1);
				while (_la==PREDICATE) {
					{
					{
					setState(696);
					predDef();
					}
					}
					setState(701);
					_errHandler.sync(this);
					_la = _input.LA(1);
				}
				setState(702);
				match(ANNOT_END);
				}
				break;
			case LINE_ANNOT_START:
				enterOuterAlt(_localctx, 2);
				{
				setState(703);
				match(LINE_ANNOT_START);
				setState(707);
				_errHandler.sync(this);
				_la = _input.LA(1);
				while (_la==PREDICATE) {
					{
					{
					setState(704);
					predDef();
					}
					}
					setState(709);
					_errHandler.sync(this);
					_la = _input.LA(1);
				}
				setState(710);
				match(LINEEND);
				}
				break;
			default:
				throw new NoViableAltException(this);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	public static class PredDefContext extends ParserRuleContext {
		public TerminalNode PREDICATE() { return getToken(TaurusParser.PREDICATE, 0); }
		public TerminalNode IDENT() { return getToken(TaurusParser.IDENT, 0); }
		public TerminalNode ASSIGN() { return getToken(TaurusParser.ASSIGN, 0); }
		public PredContext pred() {
			return getRuleContext(PredContext.class,0);
		}
		public TerminalNode SEMICOLON() { return getToken(TaurusParser.SEMICOLON, 0); }
		public TerminalNode LPAR() { return getToken(TaurusParser.LPAR, 0); }
		public List<LogicParaVarContext> logicParaVar() {
			return getRuleContexts(LogicParaVarContext.class);
		}
		public LogicParaVarContext logicParaVar(int i) {
			return getRuleContext(LogicParaVarContext.class,i);
		}
		public TerminalNode RPAR() { return getToken(TaurusParser.RPAR, 0); }
		public List<TerminalNode> COMMA() { return getTokens(TaurusParser.COMMA); }
		public TerminalNode COMMA(int i) {
			return getToken(TaurusParser.COMMA, i);
		}
		public PredDefContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_predDef; }
	}

	public final PredDefContext predDef() throws RecognitionException {
		PredDefContext _localctx = new PredDefContext(_ctx, getState());
		enterRule(_localctx, 76, RULE_predDef);
		int _la;
		try {
			enterOuterAlt(_localctx, 1);
			{
			setState(713);
			match(PREDICATE);
			setState(714);
			match(IDENT);
			setState(726);
			_errHandler.sync(this);
			_la = _input.LA(1);
			if (_la==LPAR) {
				{
				setState(715);
				match(LPAR);
				setState(716);
				logicParaVar();
				setState(721);
				_errHandler.sync(this);
				_la = _input.LA(1);
				while (_la==COMMA) {
					{
					{
					setState(717);
					match(COMMA);
					setState(718);
					logicParaVar();
					}
					}
					setState(723);
					_errHandler.sync(this);
					_la = _input.LA(1);
				}
				setState(724);
				match(RPAR);
				}
			}

			setState(728);
			match(ASSIGN);
			setState(729);
			pred(0);
			setState(730);
			match(SEMICOLON);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	public static class ConstantContext extends ParserRuleContext {
		public ConstantContext(ParserRuleContext parent, int invokingState) {
			super(parent, invokingState);
		}
		@Override public int getRuleIndex() { return RULE_constant; }
	 
		public ConstantContext() { }
		public void copyFrom(ConstantContext ctx) {
			super.copyFrom(ctx);
		}
	}
	public static class TrueConstContext extends ConstantContext {
		public TerminalNode EXPR_TRUE() { return getToken(TaurusParser.EXPR_TRUE, 0); }
		public TrueConstContext(ConstantContext ctx) { copyFrom(ctx); }
	}
	public static class FalseConstContext extends ConstantContext {
		public TerminalNode EXPR_FALSE() { return getToken(TaurusParser.EXPR_FALSE, 0); }
		public FalseConstContext(ConstantContext ctx) { copyFrom(ctx); }
	}
	public static class IntConstContext extends ConstantContext {
		public TerminalNode INT_CONSTANT() { return getToken(TaurusParser.INT_CONSTANT, 0); }
		public IntConstContext(ConstantContext ctx) { copyFrom(ctx); }
	}
	public static class FloatConstContext extends ConstantContext {
		public TerminalNode FLOAT_CONSTANT() { return getToken(TaurusParser.FLOAT_CONSTANT, 0); }
		public FloatConstContext(ConstantContext ctx) { copyFrom(ctx); }
	}

	public final ConstantContext constant() throws RecognitionException {
		ConstantContext _localctx = new ConstantContext(_ctx, getState());
		enterRule(_localctx, 78, RULE_constant);
		try {
			setState(736);
			_errHandler.sync(this);
			switch (_input.LA(1)) {
			case INT_CONSTANT:
				_localctx = new IntConstContext(_localctx);
				enterOuterAlt(_localctx, 1);
				{
				setState(732);
				match(INT_CONSTANT);
				}
				break;
			case FLOAT_CONSTANT:
				_localctx = new FloatConstContext(_localctx);
				enterOuterAlt(_localctx, 2);
				{
				setState(733);
				match(FLOAT_CONSTANT);
				}
				break;
			case EXPR_TRUE:
				_localctx = new TrueConstContext(_localctx);
				enterOuterAlt(_localctx, 3);
				{
				setState(734);
				match(EXPR_TRUE);
				}
				break;
			case EXPR_FALSE:
				_localctx = new FalseConstContext(_localctx);
				enterOuterAlt(_localctx, 4);
				{
				setState(735);
				match(EXPR_FALSE);
				}
				break;
			default:
				throw new NoViableAltException(this);
			}
		}
		catch (RecognitionException re) {
			_localctx.exception = re;
			_errHandler.reportError(this, re);
			_errHandler.recover(this, re);
		}
		finally {
			exitRule();
		}
		return _localctx;
	}

	public boolean sempred(RuleContext _localctx, int ruleIndex, int predIndex) {
		switch (ruleIndex) {
		case 17:
			return expr_sempred((ExprContext)_localctx, predIndex);
		case 19:
			return arithTerm_sempred((ArithTermContext)_localctx, predIndex);
		case 20:
			return term_sempred((TermContext)_localctx, predIndex);
		case 21:
			return pred_sempred((PredContext)_localctx, predIndex);
		}
		return true;
	}
	private boolean expr_sempred(ExprContext _localctx, int predIndex) {
		switch (predIndex) {
		case 0:
			return precpred(_ctx, 6);
		case 1:
			return precpred(_ctx, 5);
		case 2:
			return precpred(_ctx, 4);
		case 3:
			return precpred(_ctx, 3);
		case 4:
			return precpred(_ctx, 2);
		case 5:
			return precpred(_ctx, 1);
		case 6:
			return precpred(_ctx, 9);
		case 7:
			return precpred(_ctx, 8);
		}
		return true;
	}
	private boolean arithTerm_sempred(ArithTermContext _localctx, int predIndex) {
		switch (predIndex) {
		case 8:
			return precpred(_ctx, 2);
		case 9:
			return precpred(_ctx, 1);
		case 10:
			return precpred(_ctx, 5);
		case 11:
			return precpred(_ctx, 4);
		}
		return true;
	}
	private boolean term_sempred(TermContext _localctx, int predIndex) {
		switch (predIndex) {
		case 12:
			return precpred(_ctx, 4);
		case 13:
			return precpred(_ctx, 3);
		case 14:
			return precpred(_ctx, 2);
		case 15:
			return precpred(_ctx, 1);
		}
		return true;
	}
	private boolean pred_sempred(PredContext _localctx, int predIndex) {
		switch (predIndex) {
		case 16:
			return precpred(_ctx, 8);
		case 17:
			return precpred(_ctx, 7);
		case 18:
			return precpred(_ctx, 6);
		case 19:
			return precpred(_ctx, 5);
		case 20:
			return precpred(_ctx, 3);
		}
		return true;
	}

	public static final String _serializedATN =
		"\3\u608b\ua72a\u8133\ub9ed\u417c\u3be7\u7786\u5964\3L\u02e5\4\2\t\2\4"+
		"\3\t\3\4\4\t\4\4\5\t\5\4\6\t\6\4\7\t\7\4\b\t\b\4\t\t\t\4\n\t\n\4\13\t"+
		"\13\4\f\t\f\4\r\t\r\4\16\t\16\4\17\t\17\4\20\t\20\4\21\t\21\4\22\t\22"+
		"\4\23\t\23\4\24\t\24\4\25\t\25\4\26\t\26\4\27\t\27\4\30\t\30\4\31\t\31"+
		"\4\32\t\32\4\33\t\33\4\34\t\34\4\35\t\35\4\36\t\36\4\37\t\37\4 \t \4!"+
		"\t!\4\"\t\"\4#\t#\4$\t$\4%\t%\4&\t&\4\'\t\'\4(\t(\4)\t)\3\2\7\2T\n\2\f"+
		"\2\16\2W\13\2\3\2\3\2\3\3\3\3\3\3\5\3^\n\3\3\4\3\4\5\4b\n\4\3\5\3\5\3"+
		"\5\3\5\3\5\3\5\7\5j\n\5\f\5\16\5m\13\5\5\5o\n\5\3\5\3\5\3\5\7\5t\n\5\f"+
		"\5\16\5w\13\5\3\5\3\5\3\6\3\6\3\6\3\6\3\6\3\6\7\6\u0081\n\6\f\6\16\6\u0084"+
		"\13\6\3\6\3\6\3\6\3\7\3\7\3\7\3\7\3\7\3\7\3\7\3\7\3\7\3\7\3\7\3\7\5\7"+
		"\u0095\n\7\3\b\3\b\3\b\3\b\3\b\3\b\3\b\3\b\3\b\3\b\3\b\5\b\u00a2\n\b\3"+
		"\t\3\t\3\t\3\n\3\n\3\n\3\n\3\n\3\n\3\n\3\n\5\n\u00af\n\n\3\13\3\13\5\13"+
		"\u00b3\n\13\3\f\3\f\3\f\3\f\3\f\3\f\3\f\3\f\3\f\3\f\3\f\5\f\u00c0\n\f"+
		"\3\r\3\r\3\r\5\r\u00c5\n\r\3\16\3\16\3\16\3\16\3\16\3\16\3\16\3\16\3\16"+
		"\3\16\3\16\3\16\3\16\3\16\5\16\u00d5\n\16\3\16\3\16\3\16\3\16\3\16\3\16"+
		"\3\16\3\16\3\16\3\16\3\16\3\16\3\16\3\16\3\16\3\16\3\16\3\16\3\16\5\16"+
		"\u00ea\n\16\3\16\3\16\5\16\u00ee\n\16\3\16\3\16\5\16\u00f2\n\16\3\16\3"+
		"\16\3\16\3\16\3\16\3\16\3\16\3\16\3\16\5\16\u00fd\n\16\3\16\3\16\3\16"+
		"\3\16\7\16\u0103\n\16\f\16\16\16\u0106\13\16\3\16\5\16\u0109\n\16\3\17"+
		"\3\17\3\17\5\17\u010e\n\17\3\17\5\17\u0111\n\17\3\20\3\20\5\20\u0115\n"+
		"\20\3\21\3\21\3\21\3\21\3\21\3\21\3\21\3\21\3\21\3\21\3\21\3\21\3\21\3"+
		"\21\3\21\5\21\u0126\n\21\3\22\3\22\3\22\5\22\u012b\n\22\3\22\3\22\3\23"+
		"\3\23\3\23\3\23\3\23\3\23\3\23\3\23\7\23\u0137\n\23\f\23\16\23\u013a\13"+
		"\23\5\23\u013c\n\23\3\23\3\23\3\23\3\23\3\23\3\23\3\23\3\23\5\23\u0146"+
		"\n\23\3\23\3\23\3\23\3\23\3\23\3\23\3\23\3\23\3\23\3\23\3\23\3\23\3\23"+
		"\3\23\3\23\3\23\3\23\3\23\3\23\3\23\3\23\3\23\3\23\3\23\3\23\3\23\3\23"+
		"\3\23\3\23\3\23\7\23\u0166\n\23\f\23\16\23\u0169\13\23\3\24\3\24\3\24"+
		"\3\24\5\24\u016f\n\24\3\25\3\25\3\25\3\25\3\25\3\25\3\25\3\25\3\25\3\25"+
		"\3\25\3\25\3\25\3\25\3\25\3\25\3\25\3\25\3\25\3\25\3\25\5\25\u0186\n\25"+
		"\3\25\3\25\3\25\3\25\3\25\3\25\3\25\3\25\3\25\3\25\3\25\3\25\3\25\3\25"+
		"\3\25\3\25\7\25\u0198\n\25\f\25\16\25\u019b\13\25\3\26\3\26\3\26\3\26"+
		"\3\26\3\26\5\26\u01a3\n\26\3\26\3\26\3\26\3\26\3\26\3\26\3\26\3\26\3\26"+
		"\3\26\3\26\3\26\3\26\3\26\7\26\u01b3\n\26\f\26\16\26\u01b6\13\26\3\27"+
		"\3\27\3\27\3\27\3\27\3\27\3\27\6\27\u01bf\n\27\r\27\16\27\u01c0\3\27\3"+
		"\27\3\27\3\27\3\27\7\27\u01c8\n\27\f\27\16\27\u01cb\13\27\3\27\3\27\5"+
		"\27\u01cf\n\27\3\27\3\27\3\27\3\27\3\27\3\27\3\27\3\27\3\27\3\27\3\27"+
		"\3\27\3\27\3\27\3\27\3\27\3\27\3\27\3\27\3\27\3\27\7\27\u01e6\n\27\f\27"+
		"\16\27\u01e9\13\27\3\27\3\27\3\27\5\27\u01ee\n\27\3\27\3\27\3\27\3\27"+
		"\3\27\3\27\3\27\3\27\3\27\3\27\3\27\3\27\3\27\3\27\3\27\7\27\u01ff\n\27"+
		"\f\27\16\27\u0202\13\27\3\30\3\30\5\30\u0206\n\30\3\31\3\31\5\31\u020a"+
		"\n\31\3\32\3\32\3\32\5\32\u020f\n\32\3\33\3\33\5\33\u0213\n\33\3\34\3"+
		"\34\5\34\u0217\n\34\3\35\3\35\3\35\3\35\5\35\u021d\n\35\3\36\3\36\5\36"+
		"\u0221\n\36\3\37\3\37\5\37\u0225\n\37\3 \3 \3 \3 \7 \u022b\n \f \16 \u022e"+
		"\13 \3!\3!\7!\u0232\n!\f!\16!\u0235\13!\3!\5!\u0238\n!\3!\7!\u023b\n!"+
		"\f!\16!\u023e\13!\3!\3!\3!\7!\u0243\n!\f!\16!\u0246\13!\3!\5!\u0249\n"+
		"!\3!\7!\u024c\n!\f!\16!\u024f\13!\3!\5!\u0252\n!\3\"\3\"\3\"\3\"\3#\3"+
		"#\3#\3#\3#\3#\6#\u025e\n#\r#\16#\u025f\3#\3#\5#\u0264\n#\3#\3#\3$\3$\3"+
		"$\3$\3%\3%\3%\3%\3%\3%\3%\3%\3%\3%\3%\3%\5%\u0278\n%\3&\3&\3&\3&\3&\3"+
		"&\7&\u0280\n&\f&\16&\u0283\13&\3&\3&\3&\3&\3&\3&\3&\6&\u028c\n&\r&\16"+
		"&\u028d\3&\3&\5&\u0292\n&\3&\3&\5&\u0296\n&\3&\3&\3&\3&\3&\3&\3&\7&\u029f"+
		"\n&\f&\16&\u02a2\13&\3&\3&\3&\3&\3&\3&\3&\6&\u02ab\n&\r&\16&\u02ac\3&"+
		"\3&\5&\u02b1\n&\3&\3&\5&\u02b5\n&\3&\5&\u02b8\n&\3\'\3\'\7\'\u02bc\n\'"+
		"\f\'\16\'\u02bf\13\'\3\'\3\'\3\'\7\'\u02c4\n\'\f\'\16\'\u02c7\13\'\3\'"+
		"\5\'\u02ca\n\'\3(\3(\3(\3(\3(\3(\7(\u02d2\n(\f(\16(\u02d5\13(\3(\3(\5"+
		"(\u02d9\n(\3(\3(\3(\3(\3)\3)\3)\3)\5)\u02e3\n)\3)\2\6$(*,*\2\4\6\b\n\f"+
		"\16\20\22\24\26\30\32\34\36 \"$&(*,.\60\62\64\668:<>@BDFHJLNP\2\2\2\u033d"+
		"\2U\3\2\2\2\4]\3\2\2\2\6a\3\2\2\2\bc\3\2\2\2\nz\3\2\2\2\f\u0094\3\2\2"+
		"\2\16\u00a1\3\2\2\2\20\u00a3\3\2\2\2\22\u00ae\3\2\2\2\24\u00b2\3\2\2\2"+
		"\26\u00bf\3\2\2\2\30\u00c4\3\2\2\2\32\u0108\3\2\2\2\34\u0110\3\2\2\2\36"+
		"\u0114\3\2\2\2 \u0125\3\2\2\2\"\u0127\3\2\2\2$\u0145\3\2\2\2&\u016e\3"+
		"\2\2\2(\u0185\3\2\2\2*\u01a2\3\2\2\2,\u01ed\3\2\2\2.\u0205\3\2\2\2\60"+
		"\u0209\3\2\2\2\62\u020e\3\2\2\2\64\u0212\3\2\2\2\66\u0216\3\2\2\28\u021c"+
		"\3\2\2\2:\u0220\3\2\2\2<\u0224\3\2\2\2>\u0226\3\2\2\2@\u0251\3\2\2\2B"+
		"\u0253\3\2\2\2D\u0257\3\2\2\2F\u0267\3\2\2\2H\u0277\3\2\2\2J\u02b7\3\2"+
		"\2\2L\u02c9\3\2\2\2N\u02cb\3\2\2\2P\u02e2\3\2\2\2RT\5\4\3\2SR\3\2\2\2"+
		"TW\3\2\2\2US\3\2\2\2UV\3\2\2\2VX\3\2\2\2WU\3\2\2\2XY\7\2\2\3Y\3\3\2\2"+
		"\2Z^\5\b\5\2[^\5\n\6\2\\^\5L\'\2]Z\3\2\2\2][\3\2\2\2]\\\3\2\2\2^\5\3\2"+
		"\2\2_b\5\"\22\2`b\5\32\16\2a_\3\2\2\2a`\3\2\2\2b\7\3\2\2\2cd\5@!\2de\5"+
		"\20\t\2en\7\5\2\2fk\5\16\b\2gh\7\t\2\2hj\5\16\b\2ig\3\2\2\2jm\3\2\2\2"+
		"ki\3\2\2\2kl\3\2\2\2lo\3\2\2\2mk\3\2\2\2nf\3\2\2\2no\3\2\2\2op\3\2\2\2"+
		"pq\7\6\2\2qu\7\7\2\2rt\5\6\4\2sr\3\2\2\2tw\3\2\2\2us\3\2\2\2uv\3\2\2\2"+
		"vx\3\2\2\2wu\3\2\2\2xy\7\b\2\2y\t\3\2\2\2z{\7\4\2\2{|\7D\2\2|\u0082\7"+
		"\7\2\2}~\5\16\b\2~\177\7\n\2\2\177\u0081\3\2\2\2\u0080}\3\2\2\2\u0081"+
		"\u0084\3\2\2\2\u0082\u0080\3\2\2\2\u0082\u0083\3\2\2\2\u0083\u0085\3\2"+
		"\2\2\u0084\u0082\3\2\2\2\u0085\u0086\7\b\2\2\u0086\u0087\7\n\2\2\u0087"+
		"\13\3\2\2\2\u0088\u0089\5\24\13\2\u0089\u008a\7D\2\2\u008a\u0095\3\2\2"+
		"\2\u008b\u008c\7\4\2\2\u008c\u008d\7D\2\2\u008d\u0095\7D\2\2\u008e\u008f"+
		"\5\24\13\2\u008f\u0090\7D\2\2\u0090\u0091\7\13\2\2\u0091\u0092\7B\2\2"+
		"\u0092\u0093\7\f\2\2\u0093\u0095\3\2\2\2\u0094\u0088\3\2\2\2\u0094\u008b"+
		"\3\2\2\2\u0094\u008e\3\2\2\2\u0095\r\3\2\2\2\u0096\u0097\5\24\13\2\u0097"+
		"\u0098\7D\2\2\u0098\u00a2\3\2\2\2\u0099\u009a\7\4\2\2\u009a\u009b\7D\2"+
		"\2\u009b\u00a2\7D\2\2\u009c\u009d\5\24\13\2\u009d\u009e\7D\2\2\u009e\u009f"+
		"\7\13\2\2\u009f\u00a0\7\f\2\2\u00a0\u00a2\3\2\2\2\u00a1\u0096\3\2\2\2"+
		"\u00a1\u0099\3\2\2\2\u00a1\u009c\3\2\2\2\u00a2\17\3\2\2\2\u00a3\u00a4"+
		"\5\22\n\2\u00a4\u00a5\7D\2\2\u00a5\21\3\2\2\2\u00a6\u00af\5\24\13\2\u00a7"+
		"\u00a8\7\4\2\2\u00a8\u00af\7D\2\2\u00a9\u00aa\5\24\13\2\u00aa\u00ab\7"+
		"\13\2\2\u00ab\u00ac\7\f\2\2\u00ac\u00af\3\2\2\2\u00ad\u00af\7\3\2\2\u00ae"+
		"\u00a6\3\2\2\2\u00ae\u00a7\3\2\2\2\u00ae\u00a9\3\2\2\2\u00ae\u00ad\3\2"+
		"\2\2\u00af\23\3\2\2\2\u00b0\u00b3\7\16\2\2\u00b1\u00b3\7\17\2\2\u00b2"+
		"\u00b0\3\2\2\2\u00b2\u00b1\3\2\2\2\u00b3\25\3\2\2\2\u00b4\u00b5\5\30\r"+
		"\2\u00b5\u00b6\7D\2\2\u00b6\u00c0\3\2\2\2\u00b7\u00b8\7\4\2\2\u00b8\u00b9"+
		"\7D\2\2\u00b9\u00c0\7D\2\2\u00ba\u00bb\5\30\r\2\u00bb\u00bc\7D\2\2\u00bc"+
		"\u00bd\7\13\2\2\u00bd\u00be\7\f\2\2\u00be\u00c0\3\2\2\2\u00bf\u00b4\3"+
		"\2\2\2\u00bf\u00b7\3\2\2\2\u00bf\u00ba\3\2\2\2\u00c0\27\3\2\2\2\u00c1"+
		"\u00c5\7\66\2\2\u00c2\u00c5\7\67\2\2\u00c3\u00c5\7\65\2\2\u00c4\u00c1"+
		"\3\2\2\2\u00c4\u00c2\3\2\2\2\u00c4\u00c3\3\2\2\2\u00c5\31\3\2\2\2\u00c6"+
		"\u0109\7\n\2\2\u00c7\u00c8\5$\23\2\u00c8\u00c9\7\n\2\2\u00c9\u0109\3\2"+
		"\2\2\u00ca\u00cb\5 \21\2\u00cb\u00cc\7\n\2\2\u00cc\u0109\3\2\2\2\u00cd"+
		"\u00ce\7\21\2\2\u00ce\u00cf\7\5\2\2\u00cf\u00d0\5$\23\2\u00d0\u00d1\7"+
		"\6\2\2\u00d1\u00d4\5\32\16\2\u00d2\u00d3\7\22\2\2\u00d3\u00d5\5\32\16"+
		"\2\u00d4\u00d2\3\2\2\2\u00d4\u00d5\3\2\2\2\u00d5\u0109\3\2\2\2\u00d6\u00d7"+
		"\5J&\2\u00d7\u00d8\7\26\2\2\u00d8\u00d9\7\5\2\2\u00d9\u00da\5$\23\2\u00da"+
		"\u00db\7\6\2\2\u00db\u00dc\5\32\16\2\u00dc\u0109\3\2\2\2\u00dd\u00de\5"+
		"J&\2\u00de\u00df\7\27\2\2\u00df\u00e0\5\32\16\2\u00e0\u00e1\7\26\2\2\u00e1"+
		"\u00e2\7\5\2\2\u00e2\u00e3\5$\23\2\u00e3\u00e4\7\6\2\2\u00e4\u0109\3\2"+
		"\2\2\u00e5\u00e6\5J&\2\u00e6\u00e7\7\30\2\2\u00e7\u00e9\7\5\2\2\u00e8"+
		"\u00ea\5\34\17\2\u00e9\u00e8\3\2\2\2\u00e9\u00ea\3\2\2\2\u00ea\u00eb\3"+
		"\2\2\2\u00eb\u00ed\7\n\2\2\u00ec\u00ee\5$\23\2\u00ed\u00ec\3\2\2\2\u00ed"+
		"\u00ee\3\2\2\2\u00ee\u00ef\3\2\2\2\u00ef\u00f1\7\n\2\2\u00f0\u00f2\5\36"+
		"\20\2\u00f1\u00f0\3\2\2\2\u00f1\u00f2\3\2\2\2\u00f2\u00f3\3\2\2\2\u00f3"+
		"\u00f4\7\6\2\2\u00f4\u00f5\5\32\16\2\u00f5\u0109\3\2\2\2\u00f6\u00f7\7"+
		"\23\2\2\u00f7\u0109\7\n\2\2\u00f8\u00f9\7\24\2\2\u00f9\u0109\7\n\2\2\u00fa"+
		"\u00fc\7\25\2\2\u00fb\u00fd\5$\23\2\u00fc\u00fb\3\2\2\2\u00fc\u00fd\3"+
		"\2\2\2\u00fd\u00fe\3\2\2\2\u00fe\u0109\7\n\2\2\u00ff\u0109\5H%\2\u0100"+
		"\u0104\7\7\2\2\u0101\u0103\5\6\4\2\u0102\u0101\3\2\2\2\u0103\u0106\3\2"+
		"\2\2\u0104\u0102\3\2\2\2\u0104\u0105\3\2\2\2\u0105\u0107\3\2\2\2\u0106"+
		"\u0104\3\2\2\2\u0107\u0109\7\b\2\2\u0108\u00c6\3\2\2\2\u0108\u00c7\3\2"+
		"\2\2\u0108\u00ca\3\2\2\2\u0108\u00cd\3\2\2\2\u0108\u00d6\3\2\2\2\u0108"+
		"\u00dd\3\2\2\2\u0108\u00e5\3\2\2\2\u0108\u00f6\3\2\2\2\u0108\u00f8\3\2"+
		"\2\2\u0108\u00fa\3\2\2\2\u0108\u00ff\3\2\2\2\u0108\u0100\3\2\2\2\u0109"+
		"\33\3\2\2\2\u010a\u010d\5\f\7\2\u010b\u010c\7\31\2\2\u010c\u010e\5$\23"+
		"\2\u010d\u010b\3\2\2\2\u010d\u010e\3\2\2\2\u010e\u0111\3\2\2\2\u010f\u0111"+
		"\5 \21\2\u0110\u010a\3\2\2\2\u0110\u010f\3\2\2\2\u0111\35\3\2\2\2\u0112"+
		"\u0115\5 \21\2\u0113\u0115\5$\23\2\u0114\u0112\3\2\2\2\u0114\u0113\3\2"+
		"\2\2\u0115\37\3\2\2\2\u0116\u0117\7D\2\2\u0117\u0118\7\31\2\2\u0118\u0126"+
		"\5$\23\2\u0119\u011a\7D\2\2\u011a\u011b\7\13\2\2\u011b\u011c\5$\23\2\u011c"+
		"\u011d\7\f\2\2\u011d\u011e\7\31\2\2\u011e\u011f\5$\23\2\u011f\u0126\3"+
		"\2\2\2\u0120\u0121\7D\2\2\u0121\u0122\7\r\2\2\u0122\u0123\7D\2\2\u0123"+
		"\u0124\7\31\2\2\u0124\u0126\5$\23\2\u0125\u0116\3\2\2\2\u0125\u0119\3"+
		"\2\2\2\u0125\u0120\3\2\2\2\u0126!\3\2\2\2\u0127\u012a\5\f\7\2\u0128\u0129"+
		"\7\31\2\2\u0129\u012b\5$\23\2\u012a\u0128\3\2\2\2\u012a\u012b\3\2\2\2"+
		"\u012b\u012c\3\2\2\2\u012c\u012d\7\n\2\2\u012d#\3\2\2\2\u012e\u012f\b"+
		"\23\1\2\u012f\u0146\7D\2\2\u0130\u0146\5P)\2\u0131\u0132\7D\2\2\u0132"+
		"\u013b\7\5\2\2\u0133\u0138\5$\23\2\u0134\u0135\7\t\2\2\u0135\u0137\5$"+
		"\23\2\u0136\u0134\3\2\2\2\u0137\u013a\3\2\2\2\u0138\u0136\3\2\2\2\u0138"+
		"\u0139\3\2\2\2\u0139\u013c\3\2\2\2\u013a\u0138\3\2\2\2\u013b\u0133\3\2"+
		"\2\2\u013b\u013c\3\2\2\2\u013c\u013d\3\2\2\2\u013d\u0146\7\6\2\2\u013e"+
		"\u013f\7\5\2\2\u013f\u0140\5$\23\2\u0140\u0141\7\6\2\2\u0141\u0146\3\2"+
		"\2\2\u0142\u0143\5:\36\2\u0143\u0144\5$\23\t\u0144\u0146\3\2\2\2\u0145"+
		"\u012e\3\2\2\2\u0145\u0130\3\2\2\2\u0145\u0131\3\2\2\2\u0145\u013e\3\2"+
		"\2\2\u0145\u0142\3\2\2\2\u0146\u0167\3\2\2\2\u0147\u0148\f\b\2\2\u0148"+
		"\u0149\5\62\32\2\u0149\u014a\5$\23\t\u014a\u0166\3\2\2\2\u014b\u014c\f"+
		"\7\2\2\u014c\u014d\5\60\31\2\u014d\u014e\5$\23\b\u014e\u0166\3\2\2\2\u014f"+
		"\u0150\f\6\2\2\u0150\u0151\58\35\2\u0151\u0152\5$\23\7\u0152\u0166\3\2"+
		"\2\2\u0153\u0154\f\5\2\2\u0154\u0155\5\66\34\2\u0155\u0156\5$\23\6\u0156"+
		"\u0166\3\2\2\2\u0157\u0158\f\4\2\2\u0158\u0159\7&\2\2\u0159\u0166\5$\23"+
		"\5\u015a\u015b\f\3\2\2\u015b\u015c\7\'\2\2\u015c\u0166\5$\23\4\u015d\u015e"+
		"\f\13\2\2\u015e\u015f\7\13\2\2\u015f\u0160\5$\23\2\u0160\u0161\7\f\2\2"+
		"\u0161\u0166\3\2\2\2\u0162\u0163\f\n\2\2\u0163\u0164\7\r\2\2\u0164\u0166"+
		"\7D\2\2\u0165\u0147\3\2\2\2\u0165\u014b\3\2\2\2\u0165\u014f\3\2\2\2\u0165"+
		"\u0153\3\2\2\2\u0165\u0157\3\2\2\2\u0165\u015a\3\2\2\2\u0165\u015d\3\2"+
		"\2\2\u0165\u0162\3\2\2\2\u0166\u0169\3\2\2\2\u0167\u0165\3\2\2\2\u0167"+
		"\u0168\3\2\2\2\u0168%\3\2\2\2\u0169\u0167\3\2\2\2\u016a\u016f\7B\2\2\u016b"+
		"\u016f\7C\2\2\u016c\u016f\7*\2\2\u016d\u016f\7+\2\2\u016e\u016a\3\2\2"+
		"\2\u016e\u016b\3\2\2\2\u016e\u016c\3\2\2\2\u016e\u016d\3\2\2\2\u016f\'"+
		"\3\2\2\2\u0170\u0171\b\25\1\2\u0171\u0186\7D\2\2\u0172\u0186\7,\2\2\u0173"+
		"\u0186\5&\24\2\u0174\u0175\7\7\2\2\u0175\u0176\5(\25\2\u0176\u0177\7/"+
		"\2\2\u0177\u0178\7\13\2\2\u0178\u0179\5(\25\2\u0179\u017a\7\f\2\2\u017a"+
		"\u017b\7\31\2\2\u017b\u017c\5(\25\2\u017c\u017d\7\b\2\2\u017d\u0186\3"+
		"\2\2\2\u017e\u017f\7\5\2\2\u017f\u0180\5(\25\2\u0180\u0181\7\6\2\2\u0181"+
		"\u0186\3\2\2\2\u0182\u0183\5:\36\2\u0183\u0184\5(\25\5\u0184\u0186\3\2"+
		"\2\2\u0185\u0170\3\2\2\2\u0185\u0172\3\2\2\2\u0185\u0173\3\2\2\2\u0185"+
		"\u0174\3\2\2\2\u0185\u017e\3\2\2\2\u0185\u0182\3\2\2\2\u0186\u0199\3\2"+
		"\2\2\u0187\u0188\f\4\2\2\u0188\u0189\5\62\32\2\u0189\u018a\5(\25\5\u018a"+
		"\u0198\3\2\2\2\u018b\u018c\f\3\2\2\u018c\u018d\5\60\31\2\u018d\u018e\5"+
		"(\25\4\u018e\u0198\3\2\2\2\u018f\u0190\f\7\2\2\u0190\u0191\7\13\2\2\u0191"+
		"\u0192\5(\25\2\u0192\u0193\7\f\2\2\u0193\u0198\3\2\2\2\u0194\u0195\f\6"+
		"\2\2\u0195\u0196\7\r\2\2\u0196\u0198\7D\2\2\u0197\u0187\3\2\2\2\u0197"+
		"\u018b\3\2\2\2\u0197\u018f\3\2\2\2\u0197\u0194\3\2\2\2\u0198\u019b\3\2"+
		"\2\2\u0199\u0197\3\2\2\2\u0199\u019a\3\2\2\2\u019a)\3\2\2\2\u019b\u0199"+
		"\3\2\2\2\u019c\u019d\b\26\1\2\u019d\u01a3\5(\25\2\u019e\u019f\7\5\2\2"+
		"\u019f\u01a0\5*\26\2\u01a0\u01a1\7\6\2\2\u01a1\u01a3\3\2\2\2\u01a2\u019c"+
		"\3\2\2\2\u01a2\u019e\3\2\2\2\u01a3\u01b4\3\2\2\2\u01a4\u01a5\f\6\2\2\u01a5"+
		"\u01a6\58\35\2\u01a6\u01a7\5*\26\7\u01a7\u01b3\3\2\2\2\u01a8\u01a9\f\5"+
		"\2\2\u01a9\u01aa\5\66\34\2\u01aa\u01ab\5*\26\6\u01ab\u01b3\3\2\2\2\u01ac"+
		"\u01ad\f\4\2\2\u01ad\u01ae\7&\2\2\u01ae\u01b3\5*\26\5\u01af\u01b0\f\3"+
		"\2\2\u01b0\u01b1\7\'\2\2\u01b1\u01b3\5*\26\4\u01b2\u01a4\3\2\2\2\u01b2"+
		"\u01a8\3\2\2\2\u01b2\u01ac\3\2\2\2\u01b2\u01af\3\2\2\2\u01b3\u01b6\3\2"+
		"\2\2\u01b4\u01b2\3\2\2\2\u01b4\u01b5\3\2\2\2\u01b5+\3\2\2\2\u01b6\u01b4"+
		"\3\2\2\2\u01b7\u01b8\b\27\1\2\u01b8\u01ee\7*\2\2\u01b9\u01ee\7+\2\2\u01ba"+
		"\u01be\5(\25\2\u01bb\u01bc\5\64\33\2\u01bc\u01bd\5(\25\2\u01bd\u01bf\3"+
		"\2\2\2\u01be\u01bb\3\2\2\2\u01bf\u01c0\3\2\2\2\u01c0\u01be\3\2\2\2\u01c0"+
		"\u01c1\3\2\2\2\u01c1\u01ee\3\2\2\2\u01c2\u01ce\7D\2\2\u01c3\u01c4\7\5"+
		"\2\2\u01c4\u01c9\5*\26\2\u01c5\u01c6\7\t\2\2\u01c6\u01c8\5*\26\2\u01c7"+
		"\u01c5\3\2\2\2\u01c8\u01cb\3\2\2\2\u01c9\u01c7\3\2\2\2\u01c9\u01ca\3\2"+
		"\2\2\u01ca\u01cc\3\2\2\2\u01cb\u01c9\3\2\2\2\u01cc\u01cd\7\6\2\2\u01cd"+
		"\u01cf\3\2\2\2\u01ce\u01c3\3\2\2\2\u01ce\u01cf\3\2\2\2\u01cf\u01ee\3\2"+
		"\2\2\u01d0\u01d1\7\5\2\2\u01d1\u01d2\5,\27\2\u01d2\u01d3\7\6\2\2\u01d3"+
		"\u01ee\3\2\2\2\u01d4\u01d5\7$\2\2\u01d5\u01ee\5,\27\6\u01d6\u01d7\7@\2"+
		"\2\u01d7\u01d8\7\5\2\2\u01d8\u01d9\7D\2\2\u01d9\u01da\7 \2\2\u01da\u01db"+
		"\7\5\2\2\u01db\u01dc\7B\2\2\u01dc\u01dd\7A\2\2\u01dd\u01de\5(\25\2\u01de"+
		"\u01df\7\6\2\2\u01df\u01e0\7\6\2\2\u01e0\u01ee\3\2\2\2\u01e1\u01e2\5<"+
		"\37\2\u01e2\u01e7\5> \2\u01e3\u01e4\7\t\2\2\u01e4\u01e6\5> \2\u01e5\u01e3"+
		"\3\2\2\2\u01e6\u01e9\3\2\2\2\u01e7\u01e5\3\2\2\2\u01e7\u01e8\3\2\2\2\u01e8"+
		"\u01ea\3\2\2\2\u01e9\u01e7\3\2\2\2\u01ea\u01eb\7\n\2\2\u01eb\u01ec\5,"+
		"\27\3\u01ec\u01ee\3\2\2\2\u01ed\u01b7\3\2\2\2\u01ed\u01b9\3\2\2\2\u01ed"+
		"\u01ba\3\2\2\2\u01ed\u01c2\3\2\2\2\u01ed\u01d0\3\2\2\2\u01ed\u01d4\3\2"+
		"\2\2\u01ed\u01d6\3\2\2\2\u01ed\u01e1\3\2\2\2\u01ee\u0200\3\2\2\2\u01ef"+
		"\u01f0\f\n\2\2\u01f0\u01f1\7&\2\2\u01f1\u01ff\5,\27\13\u01f2\u01f3\f\t"+
		"\2\2\u01f3\u01f4\7\'\2\2\u01f4\u01ff\5,\27\n\u01f5\u01f6\f\b\2\2\u01f6"+
		"\u01f7\7\60\2\2\u01f7\u01ff\5,\27\t\u01f8\u01f9\f\7\2\2\u01f9\u01fa\7"+
		"\61\2\2\u01fa\u01ff\5,\27\b\u01fb\u01fc\f\5\2\2\u01fc\u01fd\7\62\2\2\u01fd"+
		"\u01ff\5,\27\6\u01fe\u01ef\3\2\2\2\u01fe\u01f2\3\2\2\2\u01fe\u01f5\3\2"+
		"\2\2\u01fe\u01f8\3\2\2\2\u01fe\u01fb\3\2\2\2\u01ff\u0202\3\2\2\2\u0200"+
		"\u01fe\3\2\2\2\u0200\u0201\3\2\2\2\u0201-\3\2\2\2\u0202\u0200\3\2\2\2"+
		"\u0203\u0206\5\62\32\2\u0204\u0206\5\60\31\2\u0205\u0203\3\2\2\2\u0205"+
		"\u0204\3\2\2\2\u0206/\3\2\2\2\u0207\u020a\7 \2\2\u0208\u020a\7!\2\2\u0209"+
		"\u0207\3\2\2\2\u0209\u0208\3\2\2\2\u020a\61\3\2\2\2\u020b\u020f\7\"\2"+
		"\2\u020c\u020f\7#\2\2\u020d\u020f\7%\2\2\u020e\u020b\3\2\2\2\u020e\u020c"+
		"\3\2\2\2\u020e\u020d\3\2\2\2\u020f\63\3\2\2\2\u0210\u0213\58\35\2\u0211"+
		"\u0213\5\66\34\2\u0212\u0210\3\2\2\2\u0212\u0211\3\2\2\2\u0213\65\3\2"+
		"\2\2\u0214\u0217\7\32\2\2\u0215\u0217\7\33\2\2\u0216\u0214\3\2\2\2\u0216"+
		"\u0215\3\2\2\2\u0217\67\3\2\2\2\u0218\u021d\7\35\2\2\u0219\u021d\7\34"+
		"\2\2\u021a\u021d\7\37\2\2\u021b\u021d\7\36\2\2\u021c\u0218\3\2\2\2\u021c"+
		"\u0219\3\2\2\2\u021c\u021a\3\2\2\2\u021c\u021b\3\2\2\2\u021d9\3\2\2\2"+
		"\u021e\u0221\7!\2\2\u021f\u0221\7$\2\2\u0220\u021e\3\2\2\2\u0220\u021f"+
		"\3\2\2\2\u0221;\3\2\2\2\u0222\u0225\7\63\2\2\u0223\u0225\7\64\2\2\u0224"+
		"\u0222\3\2\2\2\u0224\u0223\3\2\2\2\u0225=\3\2\2\2\u0226\u0227\5\30\r\2"+
		"\u0227\u022c\7D\2\2\u0228\u0229\7\t\2\2\u0229\u022b\7D\2\2\u022a\u0228"+
		"\3\2\2\2\u022b\u022e\3\2\2\2\u022c\u022a\3\2\2\2\u022c\u022d\3\2\2\2\u022d"+
		"?\3\2\2\2\u022e\u022c\3\2\2\2\u022f\u0233\7G\2\2\u0230\u0232\5B\"\2\u0231"+
		"\u0230\3\2\2\2\u0232\u0235\3\2\2\2\u0233\u0231\3\2\2\2\u0233\u0234\3\2"+
		"\2\2\u0234\u0237\3\2\2\2\u0235\u0233\3\2\2\2\u0236\u0238\5D#\2\u0237\u0236"+
		"\3\2\2\2\u0237\u0238\3\2\2\2\u0238\u023c\3\2\2\2\u0239\u023b\5F$\2\u023a"+
		"\u0239\3\2\2\2\u023b\u023e\3\2\2\2\u023c\u023a\3\2\2\2\u023c\u023d\3\2"+
		"\2\2\u023d\u023f\3\2\2\2\u023e\u023c\3\2\2\2\u023f\u0252\7H\2\2\u0240"+
		"\u0244\7I\2\2\u0241\u0243\5B\"\2\u0242\u0241\3\2\2\2\u0243\u0246\3\2\2"+
		"\2\u0244\u0242\3\2\2\2\u0244\u0245\3\2\2\2\u0245\u0248\3\2\2\2\u0246\u0244"+
		"\3\2\2\2\u0247\u0249\5D#\2\u0248\u0247\3\2\2\2\u0248\u0249\3\2\2\2\u0249"+
		"\u024d\3\2\2\2\u024a\u024c\5F$\2\u024b\u024a\3\2\2\2\u024c\u024f\3\2\2"+
		"\2\u024d\u024b\3\2\2\2\u024d\u024e\3\2\2\2\u024e\u0250\3\2\2\2\u024f\u024d"+
		"\3\2\2\2\u0250\u0252\7K\2\2\u0251\u022f\3\2\2\2\u0251\u0240\3\2\2\2\u0252"+
		"A\3\2\2\2\u0253\u0254\78\2\2\u0254\u0255\5,\27\2\u0255\u0256\7\n\2\2\u0256"+
		"C\3\2\2\2\u0257\u0263\79\2\2\u0258\u0264\5(\25\2\u0259\u025a\7\5\2\2\u025a"+
		"\u025d\5(\25\2\u025b\u025c\7\t\2\2\u025c\u025e\5(\25\2\u025d\u025b\3\2"+
		"\2\2\u025e\u025f\3\2\2\2\u025f\u025d\3\2\2\2\u025f\u0260\3\2\2\2\u0260"+
		"\u0261\3\2\2\2\u0261\u0262\7\6\2\2\u0262\u0264\3\2\2\2\u0263\u0258\3\2"+
		"\2\2\u0263\u0259\3\2\2\2\u0264\u0265\3\2\2\2\u0265\u0266\7\n\2\2\u0266"+
		"E\3\2\2\2\u0267\u0268\7:\2\2\u0268\u0269\5,\27\2\u0269\u026a\7\n\2\2\u026a"+
		"G\3\2\2\2\u026b\u026c\7G\2\2\u026c\u026d\7;\2\2\u026d\u026e\5,\27\2\u026e"+
		"\u026f\7\n\2\2\u026f\u0270\7H\2\2\u0270\u0278\3\2\2\2\u0271\u0272\7I\2"+
		"\2\u0272\u0273\7;\2\2\u0273\u0274\5,\27\2\u0274\u0275\7\n\2\2\u0275\u0276"+
		"\7K\2\2\u0276\u0278\3\2\2\2\u0277\u026b\3\2\2\2\u0277\u0271\3\2\2\2\u0278"+
		"I\3\2\2\2\u0279\u0281\7G\2\2\u027a\u027b\7<\2\2\u027b\u027c\7=\2\2\u027c"+
		"\u027d\5,\27\2\u027d\u027e\7\n\2\2\u027e\u0280\3\2\2\2\u027f\u027a\3\2"+
		"\2\2\u0280\u0283\3\2\2\2\u0281\u027f\3\2\2\2\u0281\u0282\3\2\2\2\u0282"+
		"\u0295\3\2\2\2\u0283\u0281\3\2\2\2\u0284\u0285\7<\2\2\u0285\u0291\7>\2"+
		"\2\u0286\u0292\5(\25\2\u0287\u0288\7\5\2\2\u0288\u028b\5(\25\2\u0289\u028a"+
		"\7\t\2\2\u028a\u028c\5(\25\2\u028b\u0289\3\2\2\2\u028c\u028d\3\2\2\2\u028d"+
		"\u028b\3\2\2\2\u028d\u028e\3\2\2\2\u028e\u028f\3\2\2\2\u028f\u0290\7\6"+
		"\2\2\u0290\u0292\3\2\2\2\u0291\u0286\3\2\2\2\u0291\u0287\3\2\2\2\u0292"+
		"\u0293\3\2\2\2\u0293\u0294\7\n\2\2\u0294\u0296\3\2\2\2\u0295\u0284\3\2"+
		"\2\2\u0295\u0296\3\2\2\2\u0296\u0297\3\2\2\2\u0297\u02b8\7H\2\2\u0298"+
		"\u02a0\7I\2\2\u0299\u029a\7<\2\2\u029a\u029b\7=\2\2\u029b\u029c\5,\27"+
		"\2\u029c\u029d\7\n\2\2\u029d\u029f\3\2\2\2\u029e\u0299\3\2\2\2\u029f\u02a2"+
		"\3\2\2\2\u02a0\u029e\3\2\2\2\u02a0\u02a1\3\2\2\2\u02a1\u02b4\3\2\2\2\u02a2"+
		"\u02a0\3\2\2\2\u02a3\u02a4\7<\2\2\u02a4\u02b0\7>\2\2\u02a5\u02b1\5(\25"+
		"\2\u02a6\u02a7\7\5\2\2\u02a7\u02aa\5(\25\2\u02a8\u02a9\7\t\2\2\u02a9\u02ab"+
		"\5(\25\2\u02aa\u02a8\3\2\2\2\u02ab\u02ac\3\2\2\2\u02ac\u02aa\3\2\2\2\u02ac"+
		"\u02ad\3\2\2\2\u02ad\u02ae\3\2\2\2\u02ae\u02af\7\6\2\2\u02af\u02b1\3\2"+
		"\2\2\u02b0\u02a5\3\2\2\2\u02b0\u02a6\3\2\2\2\u02b1\u02b2\3\2\2\2\u02b2"+
		"\u02b3\7\n\2\2\u02b3\u02b5\3\2\2\2\u02b4\u02a3\3\2\2\2\u02b4\u02b5\3\2"+
		"\2\2\u02b5\u02b6\3\2\2\2\u02b6\u02b8\7K\2\2\u02b7\u0279\3\2\2\2\u02b7"+
		"\u0298\3\2\2\2\u02b8K\3\2\2\2\u02b9\u02bd\7G\2\2\u02ba\u02bc\5N(\2\u02bb"+
		"\u02ba\3\2\2\2\u02bc\u02bf\3\2\2\2\u02bd\u02bb\3\2\2\2\u02bd\u02be\3\2"+
		"\2\2\u02be\u02c0\3\2\2\2\u02bf\u02bd\3\2\2\2\u02c0\u02ca\7H\2\2\u02c1"+
		"\u02c5\7I\2\2\u02c2\u02c4\5N(\2\u02c3\u02c2\3\2\2\2\u02c4\u02c7\3\2\2"+
		"\2\u02c5\u02c3\3\2\2\2\u02c5\u02c6\3\2\2\2\u02c6\u02c8\3\2\2\2\u02c7\u02c5"+
		"\3\2\2\2\u02c8\u02ca\7K\2\2\u02c9\u02b9\3\2\2\2\u02c9\u02c1\3\2\2\2\u02ca"+
		"M\3\2\2\2\u02cb\u02cc\7?\2\2\u02cc\u02d8\7D\2\2\u02cd\u02ce\7\5\2\2\u02ce"+
		"\u02d3\5\26\f\2\u02cf\u02d0\7\t\2\2\u02d0\u02d2\5\26\f\2\u02d1\u02cf\3"+
		"\2\2\2\u02d2\u02d5\3\2\2\2\u02d3\u02d1\3\2\2\2\u02d3\u02d4\3\2\2\2\u02d4"+
		"\u02d6\3\2\2\2\u02d5\u02d3\3\2\2\2\u02d6\u02d7\7\6\2\2\u02d7\u02d9\3\2"+
		"\2\2\u02d8\u02cd\3\2\2\2\u02d8\u02d9\3\2\2\2\u02d9\u02da\3\2\2\2\u02da"+
		"\u02db\7\31\2\2\u02db\u02dc\5,\27\2\u02dc\u02dd\7\n\2\2\u02ddO\3\2\2\2"+
		"\u02de\u02e3\7B\2\2\u02df\u02e3\7C\2\2\u02e0\u02e3\7(\2\2\u02e1\u02e3"+
		"\7)\2\2\u02e2\u02de\3\2\2\2\u02e2\u02df\3\2\2\2\u02e2\u02e0\3\2\2\2\u02e2"+
		"\u02e1\3\2\2\2\u02e3Q\3\2\2\2PU]aknu\u0082\u0094\u00a1\u00ae\u00b2\u00bf"+
		"\u00c4\u00d4\u00e9\u00ed\u00f1\u00fc\u0104\u0108\u010d\u0110\u0114\u0125"+
		"\u012a\u0138\u013b\u0145\u0165\u0167\u016e\u0185\u0197\u0199\u01a2\u01b2"+
		"\u01b4\u01c0\u01c9\u01ce\u01e7\u01ed\u01fe\u0200\u0205\u0209\u020e\u0212"+
		"\u0216\u021c\u0220\u0224\u022c\u0233\u0237\u023c\u0244\u0248\u024d\u0251"+
		"\u025f\u0263\u0277\u0281\u028d\u0291\u0295\u02a0\u02ac\u02b0\u02b4\u02b7"+
		"\u02bd\u02c5\u02c9\u02d3\u02d8\u02e2";
	public static final ATN _ATN =
		new ATNDeserializer().deserialize(_serializedATN.toCharArray());
	static {
		_decisionToDFA = new DFA[_ATN.getNumberOfDecisions()];
		for (int i = 0; i < _ATN.getNumberOfDecisions(); i++) {
			_decisionToDFA[i] = new DFA(_ATN.getDecisionState(i), i);
		}
	}
}