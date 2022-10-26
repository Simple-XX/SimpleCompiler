// Generated from /Users/lightquantum/Projects/cierra/grammar/parser/Taurus.g4 by ANTLR 4.9.2

#![allow(unused_parens)]

import org.antlr.v4.runtime.Lexer;
import org.antlr.v4.runtime.CharStream;
import org.antlr.v4.runtime.Token;
import org.antlr.v4.runtime.TokenStream;
import org.antlr.v4.runtime.*;
import org.antlr.v4.runtime.atn.*;
import org.antlr.v4.runtime.dfa.DFA;
import org.antlr.v4.runtime.misc.*;

@SuppressWarnings({"all", "warnings", "unchecked", "unused", "cast"})
public class TaurusLexer extends Lexer {
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
	public static String[] channelNames = {
		"DEFAULT_TOKEN_CHANNEL", "HIDDEN"
	};

	public static String[] modeNames = {
		"DEFAULT_MODE"
	};

	private static String[] makeRuleNames() {
		return new String[] {
			"VOID", "STRUCT", "LPAR", "RPAR", "LBRACE", "RBRACE", "COMMA", "SEMICOLON", 
			"LBRACKET", "RBRACKET", "PERIOD", "INT", "FLOAT", "BOOL", "IF", "ELSE", 
			"BREAK", "CONTINUE", "RETURN", "WHILE", "DO", "FOR", "ASSIGN", "EQ", 
			"NE", "LE", "LT", "GE", "GT", "ADD", "MINUS", "MUL", "DIV", "NOT", "MOD", 
			"AND", "OR", "EXPR_TRUE", "EXPR_FALSE", "ANNO_TRUE", "ANNO_FALSE", "RESULT", 
			"LENGTH", "OLD", "WITH", "IMPLY", "EQUIV", "XOR", "FORALL", "EXISTS", 
			"BOOLEAN", "INTEGER", "REAL", "REQUIRES", "DECREASES", "ENSURES", "ASSERT", 
			"LOOP", "INVARIANT", "VARIANT", "PREDICATE", "VALID", "APOSTROPHE", "INT_CONSTANT", 
			"FLOAT_CONSTANT", "IDENT", "COMMENT", "LINE_COMMENT", "ANNOT_START", 
			"ANNOT_END", "LINE_ANNOT_START", "AT", "LINEEND", "WS"
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


	public TaurusLexer(CharStream input) {
		super(input);
		_interp = new LexerATNSimulator(this,_ATN,_decisionToDFA,_sharedContextCache);
	}

	@Override
	public String getGrammarFileName() { return "Taurus.g4"; }

	@Override
	public String[] getRuleNames() { return ruleNames; }

	@Override
	public String getSerializedATN() { return _serializedATN; }

	@Override
	public String[] getChannelNames() { return channelNames; }

	@Override
	public String[] getModeNames() { return modeNames; }

	@Override
	public ATN getATN() { return _ATN; }

	@Override
	public void action(RuleContext _localctx, int ruleIndex, int actionIndex) {
		switch (ruleIndex) {
		case 68:
			ANNOT_START_action((RuleContext)_localctx, actionIndex);
			break;
		case 69:
			ANNOT_END_action((RuleContext)_localctx, actionIndex);
			break;
		case 70:
			LINE_ANNOT_START_action((RuleContext)_localctx, actionIndex);
			break;
		case 71:
			AT_action((RuleContext)_localctx, actionIndex);
			break;
		case 72:
			LINEEND_action((RuleContext)_localctx, actionIndex);
			break;
		}
	}
	private void ANNOT_START_action(RuleContext _localctx, int actionIndex) {
		switch (actionIndex) {
		case 0:
			 recog.in_annot = true; 
			break;
		}
	}
	private void ANNOT_END_action(RuleContext _localctx, int actionIndex) {
		switch (actionIndex) {
		case 1:
			 recog.in_annot = false; 
			break;
		}
	}
	private void LINE_ANNOT_START_action(RuleContext _localctx, int actionIndex) {
		switch (actionIndex) {
		case 2:
			 recog.in_line_annot = true; 
			break;
		}
	}
	private void AT_action(RuleContext _localctx, int actionIndex) {
		switch (actionIndex) {
		case 3:
			 if recog.in_annot || recog.in_line_annot { recog.skip(); } 
			break;
		}
	}
	private void LINEEND_action(RuleContext _localctx, int actionIndex) {
		switch (actionIndex) {
		case 4:

			    if recog.in_line_annot {
			        recog.in_line_annot = false;
			    } else {
			        recog.skip();
			    }

			break;
		}
	}

	public static final String _serializedATN =
		"\3\u608b\ua72a\u8133\ub9ed\u417c\u3be7\u7786\u5964\2L\u0217\b\1\4\2\t"+
		"\2\4\3\t\3\4\4\t\4\4\5\t\5\4\6\t\6\4\7\t\7\4\b\t\b\4\t\t\t\4\n\t\n\4\13"+
		"\t\13\4\f\t\f\4\r\t\r\4\16\t\16\4\17\t\17\4\20\t\20\4\21\t\21\4\22\t\22"+
		"\4\23\t\23\4\24\t\24\4\25\t\25\4\26\t\26\4\27\t\27\4\30\t\30\4\31\t\31"+
		"\4\32\t\32\4\33\t\33\4\34\t\34\4\35\t\35\4\36\t\36\4\37\t\37\4 \t \4!"+
		"\t!\4\"\t\"\4#\t#\4$\t$\4%\t%\4&\t&\4\'\t\'\4(\t(\4)\t)\4*\t*\4+\t+\4"+
		",\t,\4-\t-\4.\t.\4/\t/\4\60\t\60\4\61\t\61\4\62\t\62\4\63\t\63\4\64\t"+
		"\64\4\65\t\65\4\66\t\66\4\67\t\67\48\t8\49\t9\4:\t:\4;\t;\4<\t<\4=\t="+
		"\4>\t>\4?\t?\4@\t@\4A\tA\4B\tB\4C\tC\4D\tD\4E\tE\4F\tF\4G\tG\4H\tH\4I"+
		"\tI\4J\tJ\4K\tK\3\2\3\2\3\2\3\2\3\2\3\3\3\3\3\3\3\3\3\3\3\3\3\3\3\4\3"+
		"\4\3\5\3\5\3\6\3\6\3\7\3\7\3\b\3\b\3\t\3\t\3\n\3\n\3\13\3\13\3\f\3\f\3"+
		"\r\3\r\3\r\3\r\3\16\3\16\3\16\3\16\3\16\3\16\3\17\3\17\3\17\3\17\3\17"+
		"\3\20\3\20\3\20\3\21\3\21\3\21\3\21\3\21\3\22\3\22\3\22\3\22\3\22\3\22"+
		"\3\23\3\23\3\23\3\23\3\23\3\23\3\23\3\23\3\23\3\24\3\24\3\24\3\24\3\24"+
		"\3\24\3\24\3\25\3\25\3\25\3\25\3\25\3\25\3\26\3\26\3\26\3\27\3\27\3\27"+
		"\3\27\3\30\3\30\3\31\3\31\3\31\3\32\3\32\3\32\3\33\3\33\3\33\3\34\3\34"+
		"\3\35\3\35\3\35\3\36\3\36\3\37\3\37\3 \3 \3!\3!\3\"\3\"\3#\3#\3$\3$\3"+
		"%\3%\3%\3&\3&\3&\3\'\3\'\3\'\3\'\3\'\3(\3(\3(\3(\3(\3(\3)\3)\3)\3)\3)"+
		"\3)\3*\3*\3*\3*\3*\3*\3*\3+\3+\3+\3+\3+\3+\3+\3+\3,\3,\3,\3,\3,\3,\3,"+
		"\3,\3-\3-\3-\3-\3-\3.\3.\3.\3.\3.\3.\3/\3/\3/\3/\3\60\3\60\3\60\3\60\3"+
		"\60\3\61\3\61\3\61\3\62\3\62\3\62\3\62\3\62\3\62\3\62\3\62\3\63\3\63\3"+
		"\63\3\63\3\63\3\63\3\63\3\63\3\64\3\64\3\64\3\64\3\64\3\64\3\64\3\64\3"+
		"\65\3\65\3\65\3\65\3\65\3\65\3\65\3\65\3\66\3\66\3\66\3\66\3\66\3\67\3"+
		"\67\3\67\3\67\3\67\3\67\3\67\3\67\3\67\38\38\38\38\38\38\38\38\38\38\3"+
		"9\39\39\39\39\39\39\39\3:\3:\3:\3:\3:\3:\3:\3;\3;\3;\3;\3;\3<\3<\3<\3"+
		"<\3<\3<\3<\3<\3<\3<\3=\3=\3=\3=\3=\3=\3=\3=\3>\3>\3>\3>\3>\3>\3>\3>\3"+
		">\3>\3?\3?\3?\3?\3?\3?\3?\3@\3@\3@\3A\6A\u01c6\nA\rA\16A\u01c7\3B\6B\u01cb"+
		"\nB\rB\16B\u01cc\3B\3B\6B\u01d1\nB\rB\16B\u01d2\3C\3C\7C\u01d7\nC\fC\16"+
		"C\u01da\13C\3D\3D\3D\3D\3D\3D\3D\7D\u01e3\nD\fD\16D\u01e6\13D\3D\3D\5"+
		"D\u01ea\nD\3D\3D\3E\3E\3E\3E\3E\3E\7E\u01f4\nE\fE\16E\u01f7\13E\5E\u01f9"+
		"\nE\3E\3E\3F\3F\3F\3F\3F\3F\3G\3G\3G\3G\3G\3H\3H\3H\3H\3H\3H\3I\3I\3I"+
		"\3J\3J\3J\3K\3K\3K\3K\3\u01e4\2L\3\3\5\4\7\5\t\6\13\7\r\b\17\t\21\n\23"+
		"\13\25\f\27\r\31\16\33\17\35\20\37\21!\22#\23%\24\'\25)\26+\27-\30/\31"+
		"\61\32\63\33\65\34\67\359\36;\37= ?!A\"C#E$G%I&K\'M(O)Q*S+U,W-Y.[/]\60"+
		"_\61a\62c\63e\64g\65i\66k\67m8o9q:s;u<w=y>{?}@\177A\u0081B\u0083C\u0085"+
		"D\u0087E\u0089F\u008bG\u008dH\u008fI\u0091J\u0093K\u0095L\3\2\b\3\2\62"+
		";\4\2C\\c|\6\2\62;C\\aac|\3\2BB\4\2\f\f\17\17\5\2\13\13\16\16\"\"\2\u021e"+
		"\2\3\3\2\2\2\2\5\3\2\2\2\2\7\3\2\2\2\2\t\3\2\2\2\2\13\3\2\2\2\2\r\3\2"+
		"\2\2\2\17\3\2\2\2\2\21\3\2\2\2\2\23\3\2\2\2\2\25\3\2\2\2\2\27\3\2\2\2"+
		"\2\31\3\2\2\2\2\33\3\2\2\2\2\35\3\2\2\2\2\37\3\2\2\2\2!\3\2\2\2\2#\3\2"+
		"\2\2\2%\3\2\2\2\2\'\3\2\2\2\2)\3\2\2\2\2+\3\2\2\2\2-\3\2\2\2\2/\3\2\2"+
		"\2\2\61\3\2\2\2\2\63\3\2\2\2\2\65\3\2\2\2\2\67\3\2\2\2\29\3\2\2\2\2;\3"+
		"\2\2\2\2=\3\2\2\2\2?\3\2\2\2\2A\3\2\2\2\2C\3\2\2\2\2E\3\2\2\2\2G\3\2\2"+
		"\2\2I\3\2\2\2\2K\3\2\2\2\2M\3\2\2\2\2O\3\2\2\2\2Q\3\2\2\2\2S\3\2\2\2\2"+
		"U\3\2\2\2\2W\3\2\2\2\2Y\3\2\2\2\2[\3\2\2\2\2]\3\2\2\2\2_\3\2\2\2\2a\3"+
		"\2\2\2\2c\3\2\2\2\2e\3\2\2\2\2g\3\2\2\2\2i\3\2\2\2\2k\3\2\2\2\2m\3\2\2"+
		"\2\2o\3\2\2\2\2q\3\2\2\2\2s\3\2\2\2\2u\3\2\2\2\2w\3\2\2\2\2y\3\2\2\2\2"+
		"{\3\2\2\2\2}\3\2\2\2\2\177\3\2\2\2\2\u0081\3\2\2\2\2\u0083\3\2\2\2\2\u0085"+
		"\3\2\2\2\2\u0087\3\2\2\2\2\u0089\3\2\2\2\2\u008b\3\2\2\2\2\u008d\3\2\2"+
		"\2\2\u008f\3\2\2\2\2\u0091\3\2\2\2\2\u0093\3\2\2\2\2\u0095\3\2\2\2\3\u0097"+
		"\3\2\2\2\5\u009c\3\2\2\2\7\u00a3\3\2\2\2\t\u00a5\3\2\2\2\13\u00a7\3\2"+
		"\2\2\r\u00a9\3\2\2\2\17\u00ab\3\2\2\2\21\u00ad\3\2\2\2\23\u00af\3\2\2"+
		"\2\25\u00b1\3\2\2\2\27\u00b3\3\2\2\2\31\u00b5\3\2\2\2\33\u00b9\3\2\2\2"+
		"\35\u00bf\3\2\2\2\37\u00c4\3\2\2\2!\u00c7\3\2\2\2#\u00cc\3\2\2\2%\u00d2"+
		"\3\2\2\2\'\u00db\3\2\2\2)\u00e2\3\2\2\2+\u00e8\3\2\2\2-\u00eb\3\2\2\2"+
		"/\u00ef\3\2\2\2\61\u00f1\3\2\2\2\63\u00f4\3\2\2\2\65\u00f7\3\2\2\2\67"+
		"\u00fa\3\2\2\29\u00fc\3\2\2\2;\u00ff\3\2\2\2=\u0101\3\2\2\2?\u0103\3\2"+
		"\2\2A\u0105\3\2\2\2C\u0107\3\2\2\2E\u0109\3\2\2\2G\u010b\3\2\2\2I\u010d"+
		"\3\2\2\2K\u0110\3\2\2\2M\u0113\3\2\2\2O\u0118\3\2\2\2Q\u011e\3\2\2\2S"+
		"\u0124\3\2\2\2U\u012b\3\2\2\2W\u0133\3\2\2\2Y\u013b\3\2\2\2[\u0140\3\2"+
		"\2\2]\u0146\3\2\2\2_\u014a\3\2\2\2a\u014f\3\2\2\2c\u0152\3\2\2\2e\u015a"+
		"\3\2\2\2g\u0162\3\2\2\2i\u016a\3\2\2\2k\u0172\3\2\2\2m\u0177\3\2\2\2o"+
		"\u0180\3\2\2\2q\u018a\3\2\2\2s\u0192\3\2\2\2u\u0199\3\2\2\2w\u019e\3\2"+
		"\2\2y\u01a8\3\2\2\2{\u01b0\3\2\2\2}\u01ba\3\2\2\2\177\u01c1\3\2\2\2\u0081"+
		"\u01c5\3\2\2\2\u0083\u01ca\3\2\2\2\u0085\u01d4\3\2\2\2\u0087\u01db\3\2"+
		"\2\2\u0089\u01ed\3\2\2\2\u008b\u01fc\3\2\2\2\u008d\u0202\3\2\2\2\u008f"+
		"\u0207\3\2\2\2\u0091\u020d\3\2\2\2\u0093\u0210\3\2\2\2\u0095\u0213\3\2"+
		"\2\2\u0097\u0098\7x\2\2\u0098\u0099\7q\2\2\u0099\u009a\7k\2\2\u009a\u009b"+
		"\7f\2\2\u009b\4\3\2\2\2\u009c\u009d\7u\2\2\u009d\u009e\7v\2\2\u009e\u009f"+
		"\7t\2\2\u009f\u00a0\7w\2\2\u00a0\u00a1\7e\2\2\u00a1\u00a2\7v\2\2\u00a2"+
		"\6\3\2\2\2\u00a3\u00a4\7*\2\2\u00a4\b\3\2\2\2\u00a5\u00a6\7+\2\2\u00a6"+
		"\n\3\2\2\2\u00a7\u00a8\7}\2\2\u00a8\f\3\2\2\2\u00a9\u00aa\7\177\2\2\u00aa"+
		"\16\3\2\2\2\u00ab\u00ac\7.\2\2\u00ac\20\3\2\2\2\u00ad\u00ae\7=\2\2\u00ae"+
		"\22\3\2\2\2\u00af\u00b0\7]\2\2\u00b0\24\3\2\2\2\u00b1\u00b2\7_\2\2\u00b2"+
		"\26\3\2\2\2\u00b3\u00b4\7\60\2\2\u00b4\30\3\2\2\2\u00b5\u00b6\7k\2\2\u00b6"+
		"\u00b7\7p\2\2\u00b7\u00b8\7v\2\2\u00b8\32\3\2\2\2\u00b9\u00ba\7h\2\2\u00ba"+
		"\u00bb\7n\2\2\u00bb\u00bc\7q\2\2\u00bc\u00bd\7c\2\2\u00bd\u00be\7v\2\2"+
		"\u00be\34\3\2\2\2\u00bf\u00c0\7d\2\2\u00c0\u00c1\7q\2\2\u00c1\u00c2\7"+
		"q\2\2\u00c2\u00c3\7n\2\2\u00c3\36\3\2\2\2\u00c4\u00c5\7k\2\2\u00c5\u00c6"+
		"\7h\2\2\u00c6 \3\2\2\2\u00c7\u00c8\7g\2\2\u00c8\u00c9\7n\2\2\u00c9\u00ca"+
		"\7u\2\2\u00ca\u00cb\7g\2\2\u00cb\"\3\2\2\2\u00cc\u00cd\7d\2\2\u00cd\u00ce"+
		"\7t\2\2\u00ce\u00cf\7g\2\2\u00cf\u00d0\7c\2\2\u00d0\u00d1\7m\2\2\u00d1"+
		"$\3\2\2\2\u00d2\u00d3\7e\2\2\u00d3\u00d4\7q\2\2\u00d4\u00d5\7p\2\2\u00d5"+
		"\u00d6\7v\2\2\u00d6\u00d7\7k\2\2\u00d7\u00d8\7p\2\2\u00d8\u00d9\7w\2\2"+
		"\u00d9\u00da\7g\2\2\u00da&\3\2\2\2\u00db\u00dc\7t\2\2\u00dc\u00dd\7g\2"+
		"\2\u00dd\u00de\7v\2\2\u00de\u00df\7w\2\2\u00df\u00e0\7t\2\2\u00e0\u00e1"+
		"\7p\2\2\u00e1(\3\2\2\2\u00e2\u00e3\7y\2\2\u00e3\u00e4\7j\2\2\u00e4\u00e5"+
		"\7k\2\2\u00e5\u00e6\7n\2\2\u00e6\u00e7\7g\2\2\u00e7*\3\2\2\2\u00e8\u00e9"+
		"\7f\2\2\u00e9\u00ea\7q\2\2\u00ea,\3\2\2\2\u00eb\u00ec\7h\2\2\u00ec\u00ed"+
		"\7q\2\2\u00ed\u00ee\7t\2\2\u00ee.\3\2\2\2\u00ef\u00f0\7?\2\2\u00f0\60"+
		"\3\2\2\2\u00f1\u00f2\7?\2\2\u00f2\u00f3\7?\2\2\u00f3\62\3\2\2\2\u00f4"+
		"\u00f5\7#\2\2\u00f5\u00f6\7?\2\2\u00f6\64\3\2\2\2\u00f7\u00f8\7>\2\2\u00f8"+
		"\u00f9\7?\2\2\u00f9\66\3\2\2\2\u00fa\u00fb\7>\2\2\u00fb8\3\2\2\2\u00fc"+
		"\u00fd\7@\2\2\u00fd\u00fe\7?\2\2\u00fe:\3\2\2\2\u00ff\u0100\7@\2\2\u0100"+
		"<\3\2\2\2\u0101\u0102\7-\2\2\u0102>\3\2\2\2\u0103\u0104\7/\2\2\u0104@"+
		"\3\2\2\2\u0105\u0106\7,\2\2\u0106B\3\2\2\2\u0107\u0108\7\61\2\2\u0108"+
		"D\3\2\2\2\u0109\u010a\7#\2\2\u010aF\3\2\2\2\u010b\u010c\7\'\2\2\u010c"+
		"H\3\2\2\2\u010d\u010e\7(\2\2\u010e\u010f\7(\2\2\u010fJ\3\2\2\2\u0110\u0111"+
		"\7~\2\2\u0111\u0112\7~\2\2\u0112L\3\2\2\2\u0113\u0114\7v\2\2\u0114\u0115"+
		"\7t\2\2\u0115\u0116\7w\2\2\u0116\u0117\7g\2\2\u0117N\3\2\2\2\u0118\u0119"+
		"\7h\2\2\u0119\u011a\7c\2\2\u011a\u011b\7n\2\2\u011b\u011c\7u\2\2\u011c"+
		"\u011d\7g\2\2\u011dP\3\2\2\2\u011e\u011f\7^\2\2\u011f\u0120\7v\2\2\u0120"+
		"\u0121\7t\2\2\u0121\u0122\7w\2\2\u0122\u0123\7g\2\2\u0123R\3\2\2\2\u0124"+
		"\u0125\7^\2\2\u0125\u0126\7h\2\2\u0126\u0127\7c\2\2\u0127\u0128\7n\2\2"+
		"\u0128\u0129\7u\2\2\u0129\u012a\7g\2\2\u012aT\3\2\2\2\u012b\u012c\7^\2"+
		"\2\u012c\u012d\7t\2\2\u012d\u012e\7g\2\2\u012e\u012f\7u\2\2\u012f\u0130"+
		"\7w\2\2\u0130\u0131\7n\2\2\u0131\u0132\7v\2\2\u0132V\3\2\2\2\u0133\u0134"+
		"\7^\2\2\u0134\u0135\7n\2\2\u0135\u0136\7g\2\2\u0136\u0137\7p\2\2\u0137"+
		"\u0138\7i\2\2\u0138\u0139\7v\2\2\u0139\u013a\7j\2\2\u013aX\3\2\2\2\u013b"+
		"\u013c\7^\2\2\u013c\u013d\7q\2\2\u013d\u013e\7n\2\2\u013e\u013f\7f\2\2"+
		"\u013fZ\3\2\2\2\u0140\u0141\7^\2\2\u0141\u0142\7y\2\2\u0142\u0143\7k\2"+
		"\2\u0143\u0144\7v\2\2\u0144\u0145\7j\2\2\u0145\\\3\2\2\2\u0146\u0147\7"+
		"?\2\2\u0147\u0148\7?\2\2\u0148\u0149\7@\2\2\u0149^\3\2\2\2\u014a\u014b"+
		"\7>\2\2\u014b\u014c\7?\2\2\u014c\u014d\7?\2\2\u014d\u014e\7@\2\2\u014e"+
		"`\3\2\2\2\u014f\u0150\7`\2\2\u0150\u0151\7`\2\2\u0151b\3\2\2\2\u0152\u0153"+
		"\7^\2\2\u0153\u0154\7h\2\2\u0154\u0155\7q\2\2\u0155\u0156\7t\2\2\u0156"+
		"\u0157\7c\2\2\u0157\u0158\7n\2\2\u0158\u0159\7n\2\2\u0159d\3\2\2\2\u015a"+
		"\u015b\7^\2\2\u015b\u015c\7g\2\2\u015c\u015d\7z\2\2\u015d\u015e\7k\2\2"+
		"\u015e\u015f\7u\2\2\u015f\u0160\7v\2\2\u0160\u0161\7u\2\2\u0161f\3\2\2"+
		"\2\u0162\u0163\7d\2\2\u0163\u0164\7q\2\2\u0164\u0165\7q\2\2\u0165\u0166"+
		"\7n\2\2\u0166\u0167\7g\2\2\u0167\u0168\7c\2\2\u0168\u0169\7p\2\2\u0169"+
		"h\3\2\2\2\u016a\u016b\7k\2\2\u016b\u016c\7p\2\2\u016c\u016d\7v\2\2\u016d"+
		"\u016e\7g\2\2\u016e\u016f\7i\2\2\u016f\u0170\7g\2\2\u0170\u0171\7t\2\2"+
		"\u0171j\3\2\2\2\u0172\u0173\7t\2\2\u0173\u0174\7g\2\2\u0174\u0175\7c\2"+
		"\2\u0175\u0176\7n\2\2\u0176l\3\2\2\2\u0177\u0178\7t\2\2\u0178\u0179\7"+
		"g\2\2\u0179\u017a\7s\2\2\u017a\u017b\7w\2\2\u017b\u017c\7k\2\2\u017c\u017d"+
		"\7t\2\2\u017d\u017e\7g\2\2\u017e\u017f\7u\2\2\u017fn\3\2\2\2\u0180\u0181"+
		"\7f\2\2\u0181\u0182\7g\2\2\u0182\u0183\7e\2\2\u0183\u0184\7t\2\2\u0184"+
		"\u0185\7g\2\2\u0185\u0186\7c\2\2\u0186\u0187\7u\2\2\u0187\u0188\7g\2\2"+
		"\u0188\u0189\7u\2\2\u0189p\3\2\2\2\u018a\u018b\7g\2\2\u018b\u018c\7p\2"+
		"\2\u018c\u018d\7u\2\2\u018d\u018e\7w\2\2\u018e\u018f\7t\2\2\u018f\u0190"+
		"\7g\2\2\u0190\u0191\7u\2\2\u0191r\3\2\2\2\u0192\u0193\7c\2\2\u0193\u0194"+
		"\7u\2\2\u0194\u0195\7u\2\2\u0195\u0196\7g\2\2\u0196\u0197\7t\2\2\u0197"+
		"\u0198\7v\2\2\u0198t\3\2\2\2\u0199\u019a\7n\2\2\u019a\u019b\7q\2\2\u019b"+
		"\u019c\7q\2\2\u019c\u019d\7r\2\2\u019dv\3\2\2\2\u019e\u019f\7k\2\2\u019f"+
		"\u01a0\7p\2\2\u01a0\u01a1\7x\2\2\u01a1\u01a2\7c\2\2\u01a2\u01a3\7t\2\2"+
		"\u01a3\u01a4\7k\2\2\u01a4\u01a5\7c\2\2\u01a5\u01a6\7p\2\2\u01a6\u01a7"+
		"\7v\2\2\u01a7x\3\2\2\2\u01a8\u01a9\7x\2\2\u01a9\u01aa\7c\2\2\u01aa\u01ab"+
		"\7t\2\2\u01ab\u01ac\7k\2\2\u01ac\u01ad\7c\2\2\u01ad\u01ae\7p\2\2\u01ae"+
		"\u01af\7v\2\2\u01afz\3\2\2\2\u01b0\u01b1\7r\2\2\u01b1\u01b2\7t\2\2\u01b2"+
		"\u01b3\7g\2\2\u01b3\u01b4\7f\2\2\u01b4\u01b5\7k\2\2\u01b5\u01b6\7e\2\2"+
		"\u01b6\u01b7\7c\2\2\u01b7\u01b8\7v\2\2\u01b8\u01b9\7g\2\2\u01b9|\3\2\2"+
		"\2\u01ba\u01bb\7^\2\2\u01bb\u01bc\7x\2\2\u01bc\u01bd\7c\2\2\u01bd\u01be"+
		"\7n\2\2\u01be\u01bf\7k\2\2\u01bf\u01c0\7f\2\2\u01c0~\3\2\2\2\u01c1\u01c2"+
		"\7\60\2\2\u01c2\u01c3\7\60\2\2\u01c3\u0080\3\2\2\2\u01c4\u01c6\t\2\2\2"+
		"\u01c5\u01c4\3\2\2\2\u01c6\u01c7\3\2\2\2\u01c7\u01c5\3\2\2\2\u01c7\u01c8"+
		"\3\2\2\2\u01c8\u0082\3\2\2\2\u01c9\u01cb\t\2\2\2\u01ca\u01c9\3\2\2\2\u01cb"+
		"\u01cc\3\2\2\2\u01cc\u01ca\3\2\2\2\u01cc\u01cd\3\2\2\2\u01cd\u01ce\3\2"+
		"\2\2\u01ce\u01d0\7\60\2\2\u01cf\u01d1\t\2\2\2\u01d0\u01cf\3\2\2\2\u01d1"+
		"\u01d2\3\2\2\2\u01d2\u01d0\3\2\2\2\u01d2\u01d3\3\2\2\2\u01d3\u0084\3\2"+
		"\2\2\u01d4\u01d8\t\3\2\2\u01d5\u01d7\t\4\2\2\u01d6\u01d5\3\2\2\2\u01d7"+
		"\u01da\3\2\2\2\u01d8\u01d6\3\2\2\2\u01d8\u01d9\3\2\2\2\u01d9\u0086\3\2"+
		"\2\2\u01da\u01d8\3\2\2\2\u01db\u01dc\7\61\2\2\u01dc\u01dd\7,\2\2\u01dd"+
		"\u01e9\3\2\2\2\u01de\u01df\7,\2\2\u01df\u01ea\7\61\2\2\u01e0\u01e4\n\5"+
		"\2\2\u01e1\u01e3\13\2\2\2\u01e2\u01e1\3\2\2\2\u01e3\u01e6\3\2\2\2\u01e4"+
		"\u01e5\3\2\2\2\u01e4\u01e2\3\2\2\2\u01e5\u01e7\3\2\2\2\u01e6\u01e4\3\2"+
		"\2\2\u01e7\u01e8\7,\2\2\u01e8\u01ea\7\61\2\2\u01e9\u01de\3\2\2\2\u01e9"+
		"\u01e0\3\2\2\2\u01ea\u01eb\3\2\2\2\u01eb\u01ec\bD\2\2\u01ec\u0088\3\2"+
		"\2\2\u01ed\u01ee\7\61\2\2\u01ee\u01ef\7\61\2\2\u01ef\u01f8\3\2\2\2\u01f0"+
		"\u01f9\t\6\2\2\u01f1\u01f5\n\5\2\2\u01f2\u01f4\n\6\2\2\u01f3\u01f2\3\2"+
		"\2\2\u01f4\u01f7\3\2\2\2\u01f5\u01f3\3\2\2\2\u01f5\u01f6\3\2\2\2\u01f6"+
		"\u01f9\3\2\2\2\u01f7\u01f5\3\2\2\2\u01f8\u01f0\3\2\2\2\u01f8\u01f1\3\2"+
		"\2\2\u01f9\u01fa\3\2\2\2\u01fa\u01fb\bE\2\2\u01fb\u008a\3\2\2\2\u01fc"+
		"\u01fd\7\61\2\2\u01fd\u01fe\7,\2\2\u01fe\u01ff\7B\2\2\u01ff\u0200\3\2"+
		"\2\2\u0200\u0201\bF\3\2\u0201\u008c\3\2\2\2\u0202\u0203\7,\2\2\u0203\u0204"+
		"\7\61\2\2\u0204\u0205\3\2\2\2\u0205\u0206\bG\4\2\u0206\u008e\3\2\2\2\u0207"+
		"\u0208\7\61\2\2\u0208\u0209\7\61\2\2\u0209\u020a\7B\2\2\u020a\u020b\3"+
		"\2\2\2\u020b\u020c\bH\5\2\u020c\u0090\3\2\2\2\u020d\u020e\7B\2\2\u020e"+
		"\u020f\bI\6\2\u020f\u0092\3\2\2\2\u0210\u0211\t\6\2\2\u0211\u0212\bJ\7"+
		"\2\u0212\u0094\3\2\2\2\u0213\u0214\t\7\2\2\u0214\u0215\3\2\2\2\u0215\u0216"+
		"\bK\2\2\u0216\u0096\3\2\2\2\13\2\u01c7\u01cc\u01d2\u01d8\u01e4\u01e9\u01f5"+
		"\u01f8\b\b\2\2\3F\2\3G\3\3H\4\3I\5\3J\6";
	public static final ATN _ATN =
		new ATNDeserializer().deserialize(_serializedATN.toCharArray());
	static {
		_decisionToDFA = new DFA[_ATN.getNumberOfDecisions()];
		for (int i = 0; i < _ATN.getNumberOfDecisions(); i++) {
			_decisionToDFA[i] = new DFA(_ATN.getDecisionState(i), i);
		}
	}
}