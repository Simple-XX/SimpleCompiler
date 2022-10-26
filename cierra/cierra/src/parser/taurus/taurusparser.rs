// Generated from Taurus.g4 by ANTLR 4.8
#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(nonstandard_style)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_braces)]

#![allow(warnings, clippy::all)]

use antlr_rust::PredictionContextCache;
use antlr_rust::parser::{Parser, BaseParser, ParserRecog, ParserNodeType};
use antlr_rust::token_stream::TokenStream;
use antlr_rust::TokenSource;
use antlr_rust::parser_atn_simulator::ParserATNSimulator;
use antlr_rust::errors::*;
use antlr_rust::rule_context::{BaseRuleContext, CustomRuleContext, RuleContext};
use antlr_rust::recognizer::{Recognizer,Actions};
use antlr_rust::atn_deserializer::ATNDeserializer;
use antlr_rust::dfa::DFA;
use antlr_rust::atn::{ATN, INVALID_ALT};
use antlr_rust::error_strategy::{ErrorStrategy, DefaultErrorStrategy};
use antlr_rust::parser_rule_context::{BaseParserRuleContext, ParserRuleContext,cast,cast_mut};
use antlr_rust::tree::*;
use antlr_rust::token::{TOKEN_EOF,OwningToken,Token};
use antlr_rust::int_stream::EOF;
use antlr_rust::vocabulary::{Vocabulary,VocabularyImpl};
use antlr_rust::token_factory::{CommonTokenFactory,TokenFactory, TokenAware};
use super::tauruslistener::*;
use antlr_rust::lazy_static;
use antlr_rust::{TidAble,TidExt};

use std::marker::PhantomData;
use std::sync::Arc;
use std::rc::Rc;
use std::convert::TryFrom;
use std::cell::RefCell;
use std::ops::{DerefMut, Deref};
use std::borrow::{Borrow,BorrowMut};
use std::any::{Any,TypeId};

		pub const VOID:isize=1; 
		pub const STRUCT:isize=2; 
		pub const LPAR:isize=3; 
		pub const RPAR:isize=4; 
		pub const LBRACE:isize=5; 
		pub const RBRACE:isize=6; 
		pub const COMMA:isize=7; 
		pub const SEMICOLON:isize=8; 
		pub const LBRACKET:isize=9; 
		pub const RBRACKET:isize=10; 
		pub const PERIOD:isize=11; 
		pub const INT:isize=12; 
		pub const FLOAT:isize=13; 
		pub const BOOL:isize=14; 
		pub const IF:isize=15; 
		pub const ELSE:isize=16; 
		pub const BREAK:isize=17; 
		pub const CONTINUE:isize=18; 
		pub const RETURN:isize=19; 
		pub const WHILE:isize=20; 
		pub const DO:isize=21; 
		pub const FOR:isize=22; 
		pub const ASSIGN:isize=23; 
		pub const EQ:isize=24; 
		pub const NE:isize=25; 
		pub const LE:isize=26; 
		pub const LT:isize=27; 
		pub const GE:isize=28; 
		pub const GT:isize=29; 
		pub const ADD:isize=30; 
		pub const MINUS:isize=31; 
		pub const MUL:isize=32; 
		pub const DIV:isize=33; 
		pub const NOT:isize=34; 
		pub const MOD:isize=35; 
		pub const AND:isize=36; 
		pub const OR:isize=37; 
		pub const EXPR_TRUE:isize=38; 
		pub const EXPR_FALSE:isize=39; 
		pub const ANNO_TRUE:isize=40; 
		pub const ANNO_FALSE:isize=41; 
		pub const RESULT:isize=42; 
		pub const LENGTH:isize=43; 
		pub const OLD:isize=44; 
		pub const WITH:isize=45; 
		pub const IMPLY:isize=46; 
		pub const EQUIV:isize=47; 
		pub const XOR:isize=48; 
		pub const FORALL:isize=49; 
		pub const EXISTS:isize=50; 
		pub const BOOLEAN:isize=51; 
		pub const INTEGER:isize=52; 
		pub const REAL:isize=53; 
		pub const REQUIRES:isize=54; 
		pub const DECREASES:isize=55; 
		pub const ENSURES:isize=56; 
		pub const ASSERT:isize=57; 
		pub const LOOP:isize=58; 
		pub const INVARIANT:isize=59; 
		pub const VARIANT:isize=60; 
		pub const PREDICATE:isize=61; 
		pub const VALID:isize=62; 
		pub const APOSTROPHE:isize=63; 
		pub const INT_CONSTANT:isize=64; 
		pub const FLOAT_CONSTANT:isize=65; 
		pub const IDENT:isize=66; 
		pub const COMMENT:isize=67; 
		pub const LINE_COMMENT:isize=68; 
		pub const ANNOT_START:isize=69; 
		pub const ANNOT_END:isize=70; 
		pub const LINE_ANNOT_START:isize=71; 
		pub const AT:isize=72; 
		pub const LINEEND:isize=73; 
		pub const WS:isize=74;
	pub const RULE_main:usize = 0; 
	pub const RULE_def:usize = 1; 
	pub const RULE_declStmt:usize = 2; 
	pub const RULE_funcDef:usize = 3; 
	pub const RULE_structDef:usize = 4; 
	pub const RULE_localVar:usize = 5; 
	pub const RULE_paraVar:usize = 6; 
	pub const RULE_retVar:usize = 7; 
	pub const RULE_retTy:usize = 8; 
	pub const RULE_atomicType:usize = 9; 
	pub const RULE_logicParaVar:usize = 10; 
	pub const RULE_logicAtomicType:usize = 11; 
	pub const RULE_stmt:usize = 12; 
	pub const RULE_forInit:usize = 13; 
	pub const RULE_assign:usize = 14; 
	pub const RULE_decl:usize = 15; 
	pub const RULE_expr:usize = 16; 
	pub const RULE_logicConstant:usize = 17; 
	pub const RULE_arithTerm:usize = 18; 
	pub const RULE_term:usize = 19; 
	pub const RULE_pred:usize = 20; 
	pub const RULE_arithOp:usize = 21; 
	pub const RULE_addOp:usize = 22; 
	pub const RULE_mulOp:usize = 23; 
	pub const RULE_cmpOp:usize = 24; 
	pub const RULE_eqOp:usize = 25; 
	pub const RULE_ordOp:usize = 26; 
	pub const RULE_unaryOp:usize = 27; 
	pub const RULE_quantifier:usize = 28; 
	pub const RULE_binder:usize = 29; 
	pub const RULE_funcContract:usize = 30; 
	pub const RULE_requiresClause:usize = 31; 
	pub const RULE_decreasesClause:usize = 32; 
	pub const RULE_ensuresClause:usize = 33; 
	pub const RULE_assertion:usize = 34; 
	pub const RULE_loopAnnot:usize = 35; 
	pub const RULE_predDefs:usize = 36; 
	pub const RULE_predDef:usize = 37; 
	pub const RULE_constant:usize = 38;
	pub const ruleNames: [&'static str; 39] =  [
		"main", "def", "declStmt", "funcDef", "structDef", "localVar", "paraVar", 
		"retVar", "retTy", "atomicType", "logicParaVar", "logicAtomicType", "stmt", 
		"forInit", "assign", "decl", "expr", "logicConstant", "arithTerm", "term", 
		"pred", "arithOp", "addOp", "mulOp", "cmpOp", "eqOp", "ordOp", "unaryOp", 
		"quantifier", "binder", "funcContract", "requiresClause", "decreasesClause", 
		"ensuresClause", "assertion", "loopAnnot", "predDefs", "predDef", "constant"
	];


	pub const _LITERAL_NAMES: [Option<&'static str>;73] = [
		None, Some("'void'"), Some("'struct'"), Some("'('"), Some("')'"), Some("'{'"), 
		Some("'}'"), Some("','"), Some("';'"), Some("'['"), Some("']'"), Some("'.'"), 
		Some("'int'"), Some("'float'"), Some("'bool'"), Some("'if'"), Some("'else'"), 
		Some("'break'"), Some("'continue'"), Some("'return'"), Some("'while'"), 
		Some("'do'"), Some("'for'"), Some("'='"), Some("'=='"), Some("'!='"), 
		Some("'<='"), Some("'<'"), Some("'>='"), Some("'>'"), Some("'+'"), Some("'-'"), 
		Some("'*'"), Some("'/'"), Some("'!'"), Some("'%'"), Some("'&&'"), Some("'||'"), 
		Some("'true'"), Some("'false'"), Some("'\\true'"), Some("'\\false'"), 
		Some("'\\result'"), Some("'\\length'"), Some("'\\old'"), Some("'\\with'"), 
		Some("'==>'"), Some("'<==>'"), Some("'^^'"), Some("'\\forall'"), Some("'\\exists'"), 
		Some("'boolean'"), Some("'integer'"), Some("'real'"), Some("'requires'"), 
		Some("'decreases'"), Some("'ensures'"), Some("'assert'"), Some("'loop'"), 
		Some("'invariant'"), Some("'variant'"), Some("'predicate'"), Some("'\\valid'"), 
		Some("'..'"), None, None, None, None, None, Some("'/*@'"), Some("'*/'"), 
		Some("'//@'"), Some("'@'")
	];
	pub const _SYMBOLIC_NAMES: [Option<&'static str>;75]  = [
		None, Some("VOID"), Some("STRUCT"), Some("LPAR"), Some("RPAR"), Some("LBRACE"), 
		Some("RBRACE"), Some("COMMA"), Some("SEMICOLON"), Some("LBRACKET"), Some("RBRACKET"), 
		Some("PERIOD"), Some("INT"), Some("FLOAT"), Some("BOOL"), Some("IF"), 
		Some("ELSE"), Some("BREAK"), Some("CONTINUE"), Some("RETURN"), Some("WHILE"), 
		Some("DO"), Some("FOR"), Some("ASSIGN"), Some("EQ"), Some("NE"), Some("LE"), 
		Some("LT"), Some("GE"), Some("GT"), Some("ADD"), Some("MINUS"), Some("MUL"), 
		Some("DIV"), Some("NOT"), Some("MOD"), Some("AND"), Some("OR"), Some("EXPR_TRUE"), 
		Some("EXPR_FALSE"), Some("ANNO_TRUE"), Some("ANNO_FALSE"), Some("RESULT"), 
		Some("LENGTH"), Some("OLD"), Some("WITH"), Some("IMPLY"), Some("EQUIV"), 
		Some("XOR"), Some("FORALL"), Some("EXISTS"), Some("BOOLEAN"), Some("INTEGER"), 
		Some("REAL"), Some("REQUIRES"), Some("DECREASES"), Some("ENSURES"), Some("ASSERT"), 
		Some("LOOP"), Some("INVARIANT"), Some("VARIANT"), Some("PREDICATE"), Some("VALID"), 
		Some("APOSTROPHE"), Some("INT_CONSTANT"), Some("FLOAT_CONSTANT"), Some("IDENT"), 
		Some("COMMENT"), Some("LINE_COMMENT"), Some("ANNOT_START"), Some("ANNOT_END"), 
		Some("LINE_ANNOT_START"), Some("AT"), Some("LINEEND"), Some("WS")
	];
	lazy_static!{
	    static ref _shared_context_cache: Arc<PredictionContextCache> = Arc::new(PredictionContextCache::new());
		static ref VOCABULARY: Box<dyn Vocabulary> = Box::new(VocabularyImpl::new(_LITERAL_NAMES.iter(), _SYMBOLIC_NAMES.iter(), None));
	}


type BaseParserType<'input, I> =
	BaseParser<'input,TaurusParserExt<'input>, I, TaurusParserContextType , dyn TaurusListener<'input> + 'input >;

type TokenType<'input> = <LocalTokenFactory<'input> as TokenFactory<'input>>::Tok;
pub type LocalTokenFactory<'input> = CommonTokenFactory;

pub type TaurusTreeWalker<'input,'a> =
	ParseTreeWalker<'input, 'a, TaurusParserContextType , dyn TaurusListener<'input> + 'a>;

/// Parser for Taurus grammar
pub struct TaurusParser<'input,I,H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	base:BaseParserType<'input,I>,
	interpreter:Arc<ParserATNSimulator>,
	_shared_context_cache: Box<PredictionContextCache>,
    pub err_handler: H,
}

impl<'input, I, H> TaurusParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn get_serialized_atn() -> &'static str { _serializedATN }

    pub fn set_error_strategy(&mut self, strategy: H) {
        self.err_handler = strategy
    }

    pub fn with_strategy(input: I, strategy: H) -> Self {
		antlr_rust::recognizer::check_version("0","3");
		let interpreter = Arc::new(ParserATNSimulator::new(
			_ATN.clone(),
			_decision_to_DFA.clone(),
			_shared_context_cache.clone(),
		));
		Self {
			base: BaseParser::new_base_parser(
				input,
				Arc::clone(&interpreter),
				TaurusParserExt{
					_pd: Default::default(),

					    in_annot: false,
					    in_line_annot: false,

				}
			),
			interpreter,
            _shared_context_cache: Box::new(PredictionContextCache::new()),
            err_handler: strategy,
        }
    }

}

type DynStrategy<'input,I> = Box<dyn ErrorStrategy<'input,BaseParserType<'input,I>> + 'input>;

impl<'input, I> TaurusParser<'input, I, DynStrategy<'input,I>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn with_dyn_strategy(input: I) -> Self{
    	Self::with_strategy(input,Box::new(DefaultErrorStrategy::new()))
    }
}

impl<'input, I> TaurusParser<'input, I, DefaultErrorStrategy<'input,TaurusParserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
    pub fn new(input: I) -> Self{
    	Self::with_strategy(input,DefaultErrorStrategy::new())
    }
}

/// Trait for monomorphized trait object that corresponds to the nodes of parse tree generated for TaurusParser
pub trait TaurusParserContext<'input>:
	for<'x> Listenable<dyn TaurusListener<'input> + 'x > + 
	ParserRuleContext<'input, TF=LocalTokenFactory<'input>, Ctx=TaurusParserContextType>
{}

antlr_rust::coerce_from!{ 'input : TaurusParserContext<'input> }

impl<'input> TaurusParserContext<'input> for TerminalNode<'input,TaurusParserContextType> {}
impl<'input> TaurusParserContext<'input> for ErrorNode<'input,TaurusParserContextType> {}

antlr_rust::tid! { impl<'input> TidAble<'input> for dyn TaurusParserContext<'input> + 'input }

antlr_rust::tid! { impl<'input> TidAble<'input> for dyn TaurusListener<'input> + 'input }

pub struct TaurusParserContextType;
antlr_rust::tid!{TaurusParserContextType}

impl<'input> ParserNodeType<'input> for TaurusParserContextType{
	type TF = LocalTokenFactory<'input>;
	type Type = dyn TaurusParserContext<'input> + 'input;
}

impl<'input, I, H> Deref for TaurusParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
    type Target = BaseParserType<'input,I>;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl<'input, I, H> DerefMut for TaurusParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

pub struct TaurusParserExt<'input>{
	_pd: PhantomData<&'input str>,

	    in_annot: bool,
	    in_line_annot: bool,

}

impl<'input> TaurusParserExt<'input>{
}
antlr_rust::tid! { TaurusParserExt<'a> }

impl<'input> TokenAware<'input> for TaurusParserExt<'input>{
	type TF = LocalTokenFactory<'input>;
}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> ParserRecog<'input, BaseParserType<'input,I>> for TaurusParserExt<'input>{}

impl<'input,I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>> Actions<'input, BaseParserType<'input,I>> for TaurusParserExt<'input>{
	fn get_grammar_file_name(&self) -> & str{ "Taurus.g4"}

   	fn get_rule_names(&self) -> &[& str] {&ruleNames}

   	fn get_vocabulary(&self) -> &dyn Vocabulary { &**VOCABULARY }
	fn sempred(_localctx: Option<&(dyn TaurusParserContext<'input> + 'input)>, rule_index: isize, pred_index: isize,
			   recog:&mut BaseParserType<'input,I>
	)->bool{
		match rule_index {
					16 => TaurusParser::<'input,I,_>::expr_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
					18 => TaurusParser::<'input,I,_>::arithTerm_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
					19 => TaurusParser::<'input,I,_>::term_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
					20 => TaurusParser::<'input,I,_>::pred_sempred(_localctx.and_then(|x|x.downcast_ref()), pred_index, recog),
			_ => true
		}
	}
}

impl<'input, I> TaurusParser<'input, I, DefaultErrorStrategy<'input,TaurusParserContextType>>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
{
	fn expr_sempred(_localctx: Option<&ExprContext<'input>>, pred_index:isize,
						recog:&mut <Self as Deref>::Target
		) -> bool {
		match pred_index {
				0=>{
					recog.precpred(None, 6)
				}
				1=>{
					recog.precpred(None, 5)
				}
				2=>{
					recog.precpred(None, 4)
				}
				3=>{
					recog.precpred(None, 3)
				}
				4=>{
					recog.precpred(None, 2)
				}
				5=>{
					recog.precpred(None, 1)
				}
				6=>{
					recog.precpred(None, 9)
				}
				7=>{
					recog.precpred(None, 8)
				}
			_ => true
		}
	}
	fn arithTerm_sempred(_localctx: Option<&ArithTermContext<'input>>, pred_index:isize,
						recog:&mut <Self as Deref>::Target
		) -> bool {
		match pred_index {
				8=>{
					recog.precpred(None, 2)
				}
				9=>{
					recog.precpred(None, 1)
				}
				10=>{
					recog.precpred(None, 5)
				}
				11=>{
					recog.precpred(None, 4)
				}
			_ => true
		}
	}
	fn term_sempred(_localctx: Option<&TermContext<'input>>, pred_index:isize,
						recog:&mut <Self as Deref>::Target
		) -> bool {
		match pred_index {
				12=>{
					recog.precpred(None, 4)
				}
				13=>{
					recog.precpred(None, 3)
				}
				14=>{
					recog.precpred(None, 2)
				}
				15=>{
					recog.precpred(None, 1)
				}
			_ => true
		}
	}
	fn pred_sempred(_localctx: Option<&PredContext<'input>>, pred_index:isize,
						recog:&mut <Self as Deref>::Target
		) -> bool {
		match pred_index {
				16=>{
					recog.precpred(None, 8)
				}
				17=>{
					recog.precpred(None, 7)
				}
				18=>{
					recog.precpred(None, 6)
				}
				19=>{
					recog.precpred(None, 5)
				}
				20=>{
					recog.precpred(None, 3)
				}
			_ => true
		}
	}
}
//------------------- main ----------------
pub type MainContextAll<'input> = MainContext<'input>;


pub type MainContext<'input> = BaseParserRuleContext<'input,MainContextExt<'input>>;

#[derive(Clone)]
pub struct MainContextExt<'input>{
	pub defs: Option<Rc<DefContextAll<'input>>>,
ph:PhantomData<&'input str>
}

impl<'input> TaurusParserContext<'input> for MainContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for MainContext<'input>{
		fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_main(self);
		}fn exit(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
			listener.exit_main(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for MainContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_main }
	//fn type_rule_index() -> usize where Self: Sized { RULE_main }
}
antlr_rust::tid!{MainContextExt<'a>}

impl<'input> MainContextExt<'input>{
	fn new(parent: Option<Rc<dyn TaurusParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<MainContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,MainContextExt{
				defs: None, 
				ph:PhantomData
			}),
		)
	}
}

pub trait MainContextAttrs<'input>: TaurusParserContext<'input> + BorrowMut<MainContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token EOF
/// Returns `None` if there is no child corresponding to token EOF
fn EOF(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
	self.get_token(EOF, 0)
}
fn def_all(&self) ->  Vec<Rc<DefContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn def(&self, i: usize) -> Option<Rc<DefContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> MainContextAttrs<'input> for MainContext<'input>{}

impl<'input, I, H> TaurusParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn main(&mut self,)
	-> Result<Rc<MainContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = MainContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 0, RULE_main);
        let mut _localctx: Rc<MainContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(81);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while _la==STRUCT || _la==ANNOT_START || _la==LINE_ANNOT_START {
				{
				{
				/*InvokeRule def*/
				recog.base.set_state(78);
				let tmp = recog.def()?;
				 cast_mut::<_,MainContext >(&mut _localctx).defs = Some(tmp.clone());
				  

				}
				}
				recog.base.set_state(83);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(84);
			recog.base.match_token(EOF,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- def ----------------
#[derive(Debug)]
pub enum DefContextAll<'input>{
	FunctionContext(FunctionContext<'input>),
	StructureContext(StructureContext<'input>),
	PredicateContext(PredicateContext<'input>),
Error(DefContext<'input>)
}
antlr_rust::tid!{DefContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for DefContextAll<'input>{}

impl<'input> TaurusParserContext<'input> for DefContextAll<'input>{}

impl<'input> Deref for DefContextAll<'input>{
	type Target = dyn DefContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use DefContextAll::*;
		match self{
			FunctionContext(inner) => inner,
			StructureContext(inner) => inner,
			PredicateContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for DefContextAll<'input>{
    fn enter(&self, listener: &mut (dyn TaurusListener<'input> + 'a)) { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn TaurusListener<'input> + 'a)) { self.deref().exit(listener) }
}



pub type DefContext<'input> = BaseParserRuleContext<'input,DefContextExt<'input>>;

#[derive(Clone)]
pub struct DefContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TaurusParserContext<'input> for DefContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for DefContext<'input>{
}

impl<'input> CustomRuleContext<'input> for DefContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_def }
	//fn type_rule_index() -> usize where Self: Sized { RULE_def }
}
antlr_rust::tid!{DefContextExt<'a>}

impl<'input> DefContextExt<'input>{
	fn new(parent: Option<Rc<dyn TaurusParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<DefContextAll<'input>> {
		Rc::new(
		DefContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,DefContextExt{
				ph:PhantomData
			}),
		)
		)
	}
}

pub trait DefContextAttrs<'input>: TaurusParserContext<'input> + BorrowMut<DefContextExt<'input>>{


}

impl<'input> DefContextAttrs<'input> for DefContext<'input>{}

pub type FunctionContext<'input> = BaseParserRuleContext<'input,FunctionContextExt<'input>>;

pub trait FunctionContextAttrs<'input>: TaurusParserContext<'input>{
	fn funcDef(&self) -> Option<Rc<FuncDefContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> FunctionContextAttrs<'input> for FunctionContext<'input>{}

pub struct FunctionContextExt<'input>{
	base:DefContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{FunctionContextExt<'a>}

impl<'input> TaurusParserContext<'input> for FunctionContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for FunctionContext<'input>{
	fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_Function(self);
	}
}

impl<'input> CustomRuleContext<'input> for FunctionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_def }
	//fn type_rule_index() -> usize where Self: Sized { RULE_def }
}

impl<'input> Borrow<DefContextExt<'input>> for FunctionContext<'input>{
	fn borrow(&self) -> &DefContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<DefContextExt<'input>> for FunctionContext<'input>{
	fn borrow_mut(&mut self) -> &mut DefContextExt<'input> { &mut self.base }
}

impl<'input> DefContextAttrs<'input> for FunctionContext<'input> {}

impl<'input> FunctionContextExt<'input>{
	fn new(ctx: &dyn DefContextAttrs<'input>) -> Rc<DefContextAll<'input>>  {
		Rc::new(
			DefContextAll::FunctionContext(
				BaseParserRuleContext::copy_from(ctx,FunctionContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type StructureContext<'input> = BaseParserRuleContext<'input,StructureContextExt<'input>>;

pub trait StructureContextAttrs<'input>: TaurusParserContext<'input>{
	fn structDef(&self) -> Option<Rc<StructDefContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> StructureContextAttrs<'input> for StructureContext<'input>{}

pub struct StructureContextExt<'input>{
	base:DefContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{StructureContextExt<'a>}

impl<'input> TaurusParserContext<'input> for StructureContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for StructureContext<'input>{
	fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_Structure(self);
	}
}

impl<'input> CustomRuleContext<'input> for StructureContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_def }
	//fn type_rule_index() -> usize where Self: Sized { RULE_def }
}

impl<'input> Borrow<DefContextExt<'input>> for StructureContext<'input>{
	fn borrow(&self) -> &DefContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<DefContextExt<'input>> for StructureContext<'input>{
	fn borrow_mut(&mut self) -> &mut DefContextExt<'input> { &mut self.base }
}

impl<'input> DefContextAttrs<'input> for StructureContext<'input> {}

impl<'input> StructureContextExt<'input>{
	fn new(ctx: &dyn DefContextAttrs<'input>) -> Rc<DefContextAll<'input>>  {
		Rc::new(
			DefContextAll::StructureContext(
				BaseParserRuleContext::copy_from(ctx,StructureContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type PredicateContext<'input> = BaseParserRuleContext<'input,PredicateContextExt<'input>>;

pub trait PredicateContextAttrs<'input>: TaurusParserContext<'input>{
	fn predDefs(&self) -> Option<Rc<PredDefsContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> PredicateContextAttrs<'input> for PredicateContext<'input>{}

pub struct PredicateContextExt<'input>{
	base:DefContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{PredicateContextExt<'a>}

impl<'input> TaurusParserContext<'input> for PredicateContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for PredicateContext<'input>{
	fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_Predicate(self);
	}
}

impl<'input> CustomRuleContext<'input> for PredicateContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_def }
	//fn type_rule_index() -> usize where Self: Sized { RULE_def }
}

impl<'input> Borrow<DefContextExt<'input>> for PredicateContext<'input>{
	fn borrow(&self) -> &DefContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<DefContextExt<'input>> for PredicateContext<'input>{
	fn borrow_mut(&mut self) -> &mut DefContextExt<'input> { &mut self.base }
}

impl<'input> DefContextAttrs<'input> for PredicateContext<'input> {}

impl<'input> PredicateContextExt<'input>{
	fn new(ctx: &dyn DefContextAttrs<'input>) -> Rc<DefContextAll<'input>>  {
		Rc::new(
			DefContextAll::PredicateContext(
				BaseParserRuleContext::copy_from(ctx,PredicateContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I, H> TaurusParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn def(&mut self,)
	-> Result<Rc<DefContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = DefContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 2, RULE_def);
        let mut _localctx: Rc<DefContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(89);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(1,&mut recog.base)? {
				1 =>{
					let tmp = FunctionContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1);
					_localctx = tmp;
					{
					/*InvokeRule funcDef*/
					recog.base.set_state(86);
					recog.funcDef()?;

					}
				}
			,
				2 =>{
					let tmp = StructureContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2);
					_localctx = tmp;
					{
					/*InvokeRule structDef*/
					recog.base.set_state(87);
					recog.structDef()?;

					}
				}
			,
				3 =>{
					let tmp = PredicateContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 3);
					_localctx = tmp;
					{
					/*InvokeRule predDefs*/
					recog.base.set_state(88);
					recog.predDefs()?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- declStmt ----------------
#[derive(Debug)]
pub enum DeclStmtContextAll<'input>{
	StatementContext(StatementContext<'input>),
	DeclarationContext(DeclarationContext<'input>),
Error(DeclStmtContext<'input>)
}
antlr_rust::tid!{DeclStmtContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for DeclStmtContextAll<'input>{}

impl<'input> TaurusParserContext<'input> for DeclStmtContextAll<'input>{}

impl<'input> Deref for DeclStmtContextAll<'input>{
	type Target = dyn DeclStmtContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use DeclStmtContextAll::*;
		match self{
			StatementContext(inner) => inner,
			DeclarationContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for DeclStmtContextAll<'input>{
    fn enter(&self, listener: &mut (dyn TaurusListener<'input> + 'a)) { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn TaurusListener<'input> + 'a)) { self.deref().exit(listener) }
}



pub type DeclStmtContext<'input> = BaseParserRuleContext<'input,DeclStmtContextExt<'input>>;

#[derive(Clone)]
pub struct DeclStmtContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TaurusParserContext<'input> for DeclStmtContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for DeclStmtContext<'input>{
}

impl<'input> CustomRuleContext<'input> for DeclStmtContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_declStmt }
	//fn type_rule_index() -> usize where Self: Sized { RULE_declStmt }
}
antlr_rust::tid!{DeclStmtContextExt<'a>}

impl<'input> DeclStmtContextExt<'input>{
	fn new(parent: Option<Rc<dyn TaurusParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<DeclStmtContextAll<'input>> {
		Rc::new(
		DeclStmtContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,DeclStmtContextExt{
				ph:PhantomData
			}),
		)
		)
	}
}

pub trait DeclStmtContextAttrs<'input>: TaurusParserContext<'input> + BorrowMut<DeclStmtContextExt<'input>>{


}

impl<'input> DeclStmtContextAttrs<'input> for DeclStmtContext<'input>{}

pub type StatementContext<'input> = BaseParserRuleContext<'input,StatementContextExt<'input>>;

pub trait StatementContextAttrs<'input>: TaurusParserContext<'input>{
	fn stmt(&self) -> Option<Rc<StmtContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> StatementContextAttrs<'input> for StatementContext<'input>{}

pub struct StatementContextExt<'input>{
	base:DeclStmtContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{StatementContextExt<'a>}

impl<'input> TaurusParserContext<'input> for StatementContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for StatementContext<'input>{
	fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_Statement(self);
	}
}

impl<'input> CustomRuleContext<'input> for StatementContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_declStmt }
	//fn type_rule_index() -> usize where Self: Sized { RULE_declStmt }
}

impl<'input> Borrow<DeclStmtContextExt<'input>> for StatementContext<'input>{
	fn borrow(&self) -> &DeclStmtContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<DeclStmtContextExt<'input>> for StatementContext<'input>{
	fn borrow_mut(&mut self) -> &mut DeclStmtContextExt<'input> { &mut self.base }
}

impl<'input> DeclStmtContextAttrs<'input> for StatementContext<'input> {}

impl<'input> StatementContextExt<'input>{
	fn new(ctx: &dyn DeclStmtContextAttrs<'input>) -> Rc<DeclStmtContextAll<'input>>  {
		Rc::new(
			DeclStmtContextAll::StatementContext(
				BaseParserRuleContext::copy_from(ctx,StatementContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type DeclarationContext<'input> = BaseParserRuleContext<'input,DeclarationContextExt<'input>>;

pub trait DeclarationContextAttrs<'input>: TaurusParserContext<'input>{
	fn decl(&self) -> Option<Rc<DeclContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> DeclarationContextAttrs<'input> for DeclarationContext<'input>{}

pub struct DeclarationContextExt<'input>{
	base:DeclStmtContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{DeclarationContextExt<'a>}

impl<'input> TaurusParserContext<'input> for DeclarationContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for DeclarationContext<'input>{
	fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_Declaration(self);
	}
}

impl<'input> CustomRuleContext<'input> for DeclarationContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_declStmt }
	//fn type_rule_index() -> usize where Self: Sized { RULE_declStmt }
}

impl<'input> Borrow<DeclStmtContextExt<'input>> for DeclarationContext<'input>{
	fn borrow(&self) -> &DeclStmtContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<DeclStmtContextExt<'input>> for DeclarationContext<'input>{
	fn borrow_mut(&mut self) -> &mut DeclStmtContextExt<'input> { &mut self.base }
}

impl<'input> DeclStmtContextAttrs<'input> for DeclarationContext<'input> {}

impl<'input> DeclarationContextExt<'input>{
	fn new(ctx: &dyn DeclStmtContextAttrs<'input>) -> Rc<DeclStmtContextAll<'input>>  {
		Rc::new(
			DeclStmtContextAll::DeclarationContext(
				BaseParserRuleContext::copy_from(ctx,DeclarationContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I, H> TaurusParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn declStmt(&mut self,)
	-> Result<Rc<DeclStmtContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = DeclStmtContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 4, RULE_declStmt);
        let mut _localctx: Rc<DeclStmtContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(93);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 STRUCT | INT | FLOAT 
				=> {
					let tmp = DeclarationContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1);
					_localctx = tmp;
					{
					/*InvokeRule decl*/
					recog.base.set_state(91);
					recog.decl()?;

					}
				}

			 LPAR | LBRACE | SEMICOLON | IF | BREAK | CONTINUE | RETURN | MINUS |
			 NOT | EXPR_TRUE | EXPR_FALSE | INT_CONSTANT | FLOAT_CONSTANT | IDENT |
			 ANNOT_START | LINE_ANNOT_START 
				=> {
					let tmp = StatementContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2);
					_localctx = tmp;
					{
					/*InvokeRule stmt*/
					recog.base.set_state(92);
					recog.stmt()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- funcDef ----------------
pub type FuncDefContextAll<'input> = FuncDefContext<'input>;


pub type FuncDefContext<'input> = BaseParserRuleContext<'input,FuncDefContextExt<'input>>;

#[derive(Clone)]
pub struct FuncDefContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TaurusParserContext<'input> for FuncDefContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for FuncDefContext<'input>{
		fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_funcDef(self);
		}fn exit(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
			listener.exit_funcDef(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for FuncDefContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_funcDef }
	//fn type_rule_index() -> usize where Self: Sized { RULE_funcDef }
}
antlr_rust::tid!{FuncDefContextExt<'a>}

impl<'input> FuncDefContextExt<'input>{
	fn new(parent: Option<Rc<dyn TaurusParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<FuncDefContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,FuncDefContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait FuncDefContextAttrs<'input>: TaurusParserContext<'input> + BorrowMut<FuncDefContextExt<'input>>{

fn funcContract(&self) -> Option<Rc<FuncContractContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn retVar(&self) -> Option<Rc<RetVarContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token LPAR
/// Returns `None` if there is no child corresponding to token LPAR
fn LPAR(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
	self.get_token(LPAR, 0)
}
/// Retrieves first TerminalNode corresponding to token RPAR
/// Returns `None` if there is no child corresponding to token RPAR
fn RPAR(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
	self.get_token(RPAR, 0)
}
/// Retrieves first TerminalNode corresponding to token LBRACE
/// Returns `None` if there is no child corresponding to token LBRACE
fn LBRACE(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
	self.get_token(LBRACE, 0)
}
/// Retrieves first TerminalNode corresponding to token RBRACE
/// Returns `None` if there is no child corresponding to token RBRACE
fn RBRACE(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
	self.get_token(RBRACE, 0)
}
fn paraVar_all(&self) ->  Vec<Rc<ParaVarContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn paraVar(&self, i: usize) -> Option<Rc<ParaVarContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn declStmt_all(&self) ->  Vec<Rc<DeclStmtContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn declStmt(&self, i: usize) -> Option<Rc<DeclStmtContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,TaurusParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
	self.get_token(COMMA, i)
}

}

impl<'input> FuncDefContextAttrs<'input> for FuncDefContext<'input>{}

impl<'input, I, H> TaurusParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn funcDef(&mut self,)
	-> Result<Rc<FuncDefContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = FuncDefContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 6, RULE_funcDef);
        let mut _localctx: Rc<FuncDefContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule funcContract*/
			recog.base.set_state(95);
			recog.funcContract()?;

			/*InvokeRule retVar*/
			recog.base.set_state(96);
			recog.retVar()?;

			recog.base.set_state(97);
			recog.base.match_token(LPAR,&mut recog.err_handler)?;

			recog.base.set_state(106);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << STRUCT) | (1usize << INT) | (1usize << FLOAT))) != 0) {
				{
				/*InvokeRule paraVar*/
				recog.base.set_state(98);
				recog.paraVar()?;

				recog.base.set_state(103);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				while _la==COMMA {
					{
					{
					recog.base.set_state(99);
					recog.base.match_token(COMMA,&mut recog.err_handler)?;

					/*InvokeRule paraVar*/
					recog.base.set_state(100);
					recog.paraVar()?;

					}
					}
					recog.base.set_state(105);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
				}
				}
			}

			recog.base.set_state(108);
			recog.base.match_token(RPAR,&mut recog.err_handler)?;

			recog.base.set_state(109);
			recog.base.match_token(LBRACE,&mut recog.err_handler)?;

			recog.base.set_state(113);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << STRUCT) | (1usize << LPAR) | (1usize << LBRACE) | (1usize << SEMICOLON) | (1usize << INT) | (1usize << FLOAT) | (1usize << IF) | (1usize << BREAK) | (1usize << CONTINUE) | (1usize << RETURN) | (1usize << MINUS))) != 0) || ((((_la - 34)) & !0x3f) == 0 && ((1usize << (_la - 34)) & ((1usize << (NOT - 34)) | (1usize << (EXPR_TRUE - 34)) | (1usize << (EXPR_FALSE - 34)) | (1usize << (INT_CONSTANT - 34)) | (1usize << (FLOAT_CONSTANT - 34)))) != 0) || ((((_la - 66)) & !0x3f) == 0 && ((1usize << (_la - 66)) & ((1usize << (IDENT - 66)) | (1usize << (ANNOT_START - 66)) | (1usize << (LINE_ANNOT_START - 66)))) != 0) {
				{
				{
				/*InvokeRule declStmt*/
				recog.base.set_state(110);
				recog.declStmt()?;

				}
				}
				recog.base.set_state(115);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(116);
			recog.base.match_token(RBRACE,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- structDef ----------------
pub type StructDefContextAll<'input> = StructDefContext<'input>;


pub type StructDefContext<'input> = BaseParserRuleContext<'input,StructDefContextExt<'input>>;

#[derive(Clone)]
pub struct StructDefContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TaurusParserContext<'input> for StructDefContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for StructDefContext<'input>{
		fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_structDef(self);
		}fn exit(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
			listener.exit_structDef(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for StructDefContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_structDef }
	//fn type_rule_index() -> usize where Self: Sized { RULE_structDef }
}
antlr_rust::tid!{StructDefContextExt<'a>}

impl<'input> StructDefContextExt<'input>{
	fn new(parent: Option<Rc<dyn TaurusParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<StructDefContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,StructDefContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait StructDefContextAttrs<'input>: TaurusParserContext<'input> + BorrowMut<StructDefContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token STRUCT
/// Returns `None` if there is no child corresponding to token STRUCT
fn STRUCT(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
	self.get_token(STRUCT, 0)
}
/// Retrieves first TerminalNode corresponding to token IDENT
/// Returns `None` if there is no child corresponding to token IDENT
fn IDENT(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
	self.get_token(IDENT, 0)
}
/// Retrieves first TerminalNode corresponding to token LBRACE
/// Returns `None` if there is no child corresponding to token LBRACE
fn LBRACE(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
	self.get_token(LBRACE, 0)
}
/// Retrieves first TerminalNode corresponding to token RBRACE
/// Returns `None` if there is no child corresponding to token RBRACE
fn RBRACE(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
	self.get_token(RBRACE, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token SEMICOLON in current rule
fn SEMICOLON_all(&self) -> Vec<Rc<TerminalNode<'input,TaurusParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token SEMICOLON, starting from 0.
/// Returns `None` if number of children corresponding to token SEMICOLON is less or equal than `i`.
fn SEMICOLON(&self, i: usize) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
	self.get_token(SEMICOLON, i)
}
fn paraVar_all(&self) ->  Vec<Rc<ParaVarContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn paraVar(&self, i: usize) -> Option<Rc<ParaVarContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}

}

impl<'input> StructDefContextAttrs<'input> for StructDefContext<'input>{}

impl<'input, I, H> TaurusParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn structDef(&mut self,)
	-> Result<Rc<StructDefContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = StructDefContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 8, RULE_structDef);
        let mut _localctx: Rc<StructDefContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(118);
			recog.base.match_token(STRUCT,&mut recog.err_handler)?;

			recog.base.set_state(119);
			recog.base.match_token(IDENT,&mut recog.err_handler)?;

			recog.base.set_state(120);
			recog.base.match_token(LBRACE,&mut recog.err_handler)?;

			recog.base.set_state(126);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			while (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << STRUCT) | (1usize << INT) | (1usize << FLOAT))) != 0) {
				{
				{
				/*InvokeRule paraVar*/
				recog.base.set_state(121);
				recog.paraVar()?;

				recog.base.set_state(122);
				recog.base.match_token(SEMICOLON,&mut recog.err_handler)?;

				}
				}
				recog.base.set_state(128);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
			}
			recog.base.set_state(129);
			recog.base.match_token(RBRACE,&mut recog.err_handler)?;

			recog.base.set_state(130);
			recog.base.match_token(SEMICOLON,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- localVar ----------------
#[derive(Debug)]
pub enum LocalVarContextAll<'input>{
	AtomLocalVarContext(AtomLocalVarContext<'input>),
	StructLocalVarContext(StructLocalVarContext<'input>),
	ArrLocalVarContext(ArrLocalVarContext<'input>),
Error(LocalVarContext<'input>)
}
antlr_rust::tid!{LocalVarContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for LocalVarContextAll<'input>{}

impl<'input> TaurusParserContext<'input> for LocalVarContextAll<'input>{}

impl<'input> Deref for LocalVarContextAll<'input>{
	type Target = dyn LocalVarContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use LocalVarContextAll::*;
		match self{
			AtomLocalVarContext(inner) => inner,
			StructLocalVarContext(inner) => inner,
			ArrLocalVarContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for LocalVarContextAll<'input>{
    fn enter(&self, listener: &mut (dyn TaurusListener<'input> + 'a)) { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn TaurusListener<'input> + 'a)) { self.deref().exit(listener) }
}



pub type LocalVarContext<'input> = BaseParserRuleContext<'input,LocalVarContextExt<'input>>;

#[derive(Clone)]
pub struct LocalVarContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TaurusParserContext<'input> for LocalVarContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for LocalVarContext<'input>{
}

impl<'input> CustomRuleContext<'input> for LocalVarContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_localVar }
	//fn type_rule_index() -> usize where Self: Sized { RULE_localVar }
}
antlr_rust::tid!{LocalVarContextExt<'a>}

impl<'input> LocalVarContextExt<'input>{
	fn new(parent: Option<Rc<dyn TaurusParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<LocalVarContextAll<'input>> {
		Rc::new(
		LocalVarContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,LocalVarContextExt{
				ph:PhantomData
			}),
		)
		)
	}
}

pub trait LocalVarContextAttrs<'input>: TaurusParserContext<'input> + BorrowMut<LocalVarContextExt<'input>>{


}

impl<'input> LocalVarContextAttrs<'input> for LocalVarContext<'input>{}

pub type AtomLocalVarContext<'input> = BaseParserRuleContext<'input,AtomLocalVarContextExt<'input>>;

pub trait AtomLocalVarContextAttrs<'input>: TaurusParserContext<'input>{
	fn atomicType(&self) -> Option<Rc<AtomicTypeContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token IDENT
	/// Returns `None` if there is no child corresponding to token IDENT
	fn IDENT(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(IDENT, 0)
	}
}

impl<'input> AtomLocalVarContextAttrs<'input> for AtomLocalVarContext<'input>{}

pub struct AtomLocalVarContextExt<'input>{
	base:LocalVarContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{AtomLocalVarContextExt<'a>}

impl<'input> TaurusParserContext<'input> for AtomLocalVarContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for AtomLocalVarContext<'input>{
	fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_AtomLocalVar(self);
	}
}

impl<'input> CustomRuleContext<'input> for AtomLocalVarContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_localVar }
	//fn type_rule_index() -> usize where Self: Sized { RULE_localVar }
}

impl<'input> Borrow<LocalVarContextExt<'input>> for AtomLocalVarContext<'input>{
	fn borrow(&self) -> &LocalVarContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<LocalVarContextExt<'input>> for AtomLocalVarContext<'input>{
	fn borrow_mut(&mut self) -> &mut LocalVarContextExt<'input> { &mut self.base }
}

impl<'input> LocalVarContextAttrs<'input> for AtomLocalVarContext<'input> {}

impl<'input> AtomLocalVarContextExt<'input>{
	fn new(ctx: &dyn LocalVarContextAttrs<'input>) -> Rc<LocalVarContextAll<'input>>  {
		Rc::new(
			LocalVarContextAll::AtomLocalVarContext(
				BaseParserRuleContext::copy_from(ctx,AtomLocalVarContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type StructLocalVarContext<'input> = BaseParserRuleContext<'input,StructLocalVarContextExt<'input>>;

pub trait StructLocalVarContextAttrs<'input>: TaurusParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token STRUCT
	/// Returns `None` if there is no child corresponding to token STRUCT
	fn STRUCT(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(STRUCT, 0)
	}
	/// Retrieves all `TerminalNode`s corresponding to token IDENT in current rule
	fn IDENT_all(&self) -> Vec<Rc<TerminalNode<'input,TaurusParserContextType>>>  where Self:Sized{
		self.children_of_type()
	}
	/// Retrieves 'i's TerminalNode corresponding to token IDENT, starting from 0.
	/// Returns `None` if number of children corresponding to token IDENT is less or equal than `i`.
	fn IDENT(&self, i: usize) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(IDENT, i)
	}
}

impl<'input> StructLocalVarContextAttrs<'input> for StructLocalVarContext<'input>{}

pub struct StructLocalVarContextExt<'input>{
	base:LocalVarContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{StructLocalVarContextExt<'a>}

impl<'input> TaurusParserContext<'input> for StructLocalVarContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for StructLocalVarContext<'input>{
	fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_StructLocalVar(self);
	}
}

impl<'input> CustomRuleContext<'input> for StructLocalVarContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_localVar }
	//fn type_rule_index() -> usize where Self: Sized { RULE_localVar }
}

impl<'input> Borrow<LocalVarContextExt<'input>> for StructLocalVarContext<'input>{
	fn borrow(&self) -> &LocalVarContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<LocalVarContextExt<'input>> for StructLocalVarContext<'input>{
	fn borrow_mut(&mut self) -> &mut LocalVarContextExt<'input> { &mut self.base }
}

impl<'input> LocalVarContextAttrs<'input> for StructLocalVarContext<'input> {}

impl<'input> StructLocalVarContextExt<'input>{
	fn new(ctx: &dyn LocalVarContextAttrs<'input>) -> Rc<LocalVarContextAll<'input>>  {
		Rc::new(
			LocalVarContextAll::StructLocalVarContext(
				BaseParserRuleContext::copy_from(ctx,StructLocalVarContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type ArrLocalVarContext<'input> = BaseParserRuleContext<'input,ArrLocalVarContextExt<'input>>;

pub trait ArrLocalVarContextAttrs<'input>: TaurusParserContext<'input>{
	fn atomicType(&self) -> Option<Rc<AtomicTypeContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token IDENT
	/// Returns `None` if there is no child corresponding to token IDENT
	fn IDENT(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(IDENT, 0)
	}
	/// Retrieves first TerminalNode corresponding to token LBRACKET
	/// Returns `None` if there is no child corresponding to token LBRACKET
	fn LBRACKET(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(LBRACKET, 0)
	}
	/// Retrieves first TerminalNode corresponding to token INT_CONSTANT
	/// Returns `None` if there is no child corresponding to token INT_CONSTANT
	fn INT_CONSTANT(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(INT_CONSTANT, 0)
	}
	/// Retrieves first TerminalNode corresponding to token RBRACKET
	/// Returns `None` if there is no child corresponding to token RBRACKET
	fn RBRACKET(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(RBRACKET, 0)
	}
}

impl<'input> ArrLocalVarContextAttrs<'input> for ArrLocalVarContext<'input>{}

pub struct ArrLocalVarContextExt<'input>{
	base:LocalVarContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{ArrLocalVarContextExt<'a>}

impl<'input> TaurusParserContext<'input> for ArrLocalVarContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for ArrLocalVarContext<'input>{
	fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_ArrLocalVar(self);
	}
}

impl<'input> CustomRuleContext<'input> for ArrLocalVarContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_localVar }
	//fn type_rule_index() -> usize where Self: Sized { RULE_localVar }
}

impl<'input> Borrow<LocalVarContextExt<'input>> for ArrLocalVarContext<'input>{
	fn borrow(&self) -> &LocalVarContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<LocalVarContextExt<'input>> for ArrLocalVarContext<'input>{
	fn borrow_mut(&mut self) -> &mut LocalVarContextExt<'input> { &mut self.base }
}

impl<'input> LocalVarContextAttrs<'input> for ArrLocalVarContext<'input> {}

impl<'input> ArrLocalVarContextExt<'input>{
	fn new(ctx: &dyn LocalVarContextAttrs<'input>) -> Rc<LocalVarContextAll<'input>>  {
		Rc::new(
			LocalVarContextAll::ArrLocalVarContext(
				BaseParserRuleContext::copy_from(ctx,ArrLocalVarContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I, H> TaurusParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn localVar(&mut self,)
	-> Result<Rc<LocalVarContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = LocalVarContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 10, RULE_localVar);
        let mut _localctx: Rc<LocalVarContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(144);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(7,&mut recog.base)? {
				1 =>{
					let tmp = AtomLocalVarContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1);
					_localctx = tmp;
					{
					/*InvokeRule atomicType*/
					recog.base.set_state(132);
					recog.atomicType()?;

					recog.base.set_state(133);
					recog.base.match_token(IDENT,&mut recog.err_handler)?;

					}
				}
			,
				2 =>{
					let tmp = StructLocalVarContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2);
					_localctx = tmp;
					{
					recog.base.set_state(135);
					recog.base.match_token(STRUCT,&mut recog.err_handler)?;

					recog.base.set_state(136);
					recog.base.match_token(IDENT,&mut recog.err_handler)?;

					recog.base.set_state(137);
					recog.base.match_token(IDENT,&mut recog.err_handler)?;

					}
				}
			,
				3 =>{
					let tmp = ArrLocalVarContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 3);
					_localctx = tmp;
					{
					/*InvokeRule atomicType*/
					recog.base.set_state(138);
					recog.atomicType()?;

					recog.base.set_state(139);
					recog.base.match_token(IDENT,&mut recog.err_handler)?;

					recog.base.set_state(140);
					recog.base.match_token(LBRACKET,&mut recog.err_handler)?;

					recog.base.set_state(141);
					recog.base.match_token(INT_CONSTANT,&mut recog.err_handler)?;

					recog.base.set_state(142);
					recog.base.match_token(RBRACKET,&mut recog.err_handler)?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- paraVar ----------------
#[derive(Debug)]
pub enum ParaVarContextAll<'input>{
	AtomParaVarContext(AtomParaVarContext<'input>),
	ArrParaVarContext(ArrParaVarContext<'input>),
	StructParaVarContext(StructParaVarContext<'input>),
Error(ParaVarContext<'input>)
}
antlr_rust::tid!{ParaVarContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for ParaVarContextAll<'input>{}

impl<'input> TaurusParserContext<'input> for ParaVarContextAll<'input>{}

impl<'input> Deref for ParaVarContextAll<'input>{
	type Target = dyn ParaVarContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use ParaVarContextAll::*;
		match self{
			AtomParaVarContext(inner) => inner,
			ArrParaVarContext(inner) => inner,
			StructParaVarContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for ParaVarContextAll<'input>{
    fn enter(&self, listener: &mut (dyn TaurusListener<'input> + 'a)) { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn TaurusListener<'input> + 'a)) { self.deref().exit(listener) }
}



pub type ParaVarContext<'input> = BaseParserRuleContext<'input,ParaVarContextExt<'input>>;

#[derive(Clone)]
pub struct ParaVarContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TaurusParserContext<'input> for ParaVarContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for ParaVarContext<'input>{
}

impl<'input> CustomRuleContext<'input> for ParaVarContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_paraVar }
	//fn type_rule_index() -> usize where Self: Sized { RULE_paraVar }
}
antlr_rust::tid!{ParaVarContextExt<'a>}

impl<'input> ParaVarContextExt<'input>{
	fn new(parent: Option<Rc<dyn TaurusParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ParaVarContextAll<'input>> {
		Rc::new(
		ParaVarContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ParaVarContextExt{
				ph:PhantomData
			}),
		)
		)
	}
}

pub trait ParaVarContextAttrs<'input>: TaurusParserContext<'input> + BorrowMut<ParaVarContextExt<'input>>{


}

impl<'input> ParaVarContextAttrs<'input> for ParaVarContext<'input>{}

pub type AtomParaVarContext<'input> = BaseParserRuleContext<'input,AtomParaVarContextExt<'input>>;

pub trait AtomParaVarContextAttrs<'input>: TaurusParserContext<'input>{
	fn atomicType(&self) -> Option<Rc<AtomicTypeContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token IDENT
	/// Returns `None` if there is no child corresponding to token IDENT
	fn IDENT(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(IDENT, 0)
	}
}

impl<'input> AtomParaVarContextAttrs<'input> for AtomParaVarContext<'input>{}

pub struct AtomParaVarContextExt<'input>{
	base:ParaVarContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{AtomParaVarContextExt<'a>}

impl<'input> TaurusParserContext<'input> for AtomParaVarContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for AtomParaVarContext<'input>{
	fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_AtomParaVar(self);
	}
}

impl<'input> CustomRuleContext<'input> for AtomParaVarContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_paraVar }
	//fn type_rule_index() -> usize where Self: Sized { RULE_paraVar }
}

impl<'input> Borrow<ParaVarContextExt<'input>> for AtomParaVarContext<'input>{
	fn borrow(&self) -> &ParaVarContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ParaVarContextExt<'input>> for AtomParaVarContext<'input>{
	fn borrow_mut(&mut self) -> &mut ParaVarContextExt<'input> { &mut self.base }
}

impl<'input> ParaVarContextAttrs<'input> for AtomParaVarContext<'input> {}

impl<'input> AtomParaVarContextExt<'input>{
	fn new(ctx: &dyn ParaVarContextAttrs<'input>) -> Rc<ParaVarContextAll<'input>>  {
		Rc::new(
			ParaVarContextAll::AtomParaVarContext(
				BaseParserRuleContext::copy_from(ctx,AtomParaVarContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type ArrParaVarContext<'input> = BaseParserRuleContext<'input,ArrParaVarContextExt<'input>>;

pub trait ArrParaVarContextAttrs<'input>: TaurusParserContext<'input>{
	fn atomicType(&self) -> Option<Rc<AtomicTypeContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token IDENT
	/// Returns `None` if there is no child corresponding to token IDENT
	fn IDENT(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(IDENT, 0)
	}
	/// Retrieves first TerminalNode corresponding to token LBRACKET
	/// Returns `None` if there is no child corresponding to token LBRACKET
	fn LBRACKET(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(LBRACKET, 0)
	}
	/// Retrieves first TerminalNode corresponding to token RBRACKET
	/// Returns `None` if there is no child corresponding to token RBRACKET
	fn RBRACKET(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(RBRACKET, 0)
	}
}

impl<'input> ArrParaVarContextAttrs<'input> for ArrParaVarContext<'input>{}

pub struct ArrParaVarContextExt<'input>{
	base:ParaVarContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{ArrParaVarContextExt<'a>}

impl<'input> TaurusParserContext<'input> for ArrParaVarContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for ArrParaVarContext<'input>{
	fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_ArrParaVar(self);
	}
}

impl<'input> CustomRuleContext<'input> for ArrParaVarContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_paraVar }
	//fn type_rule_index() -> usize where Self: Sized { RULE_paraVar }
}

impl<'input> Borrow<ParaVarContextExt<'input>> for ArrParaVarContext<'input>{
	fn borrow(&self) -> &ParaVarContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ParaVarContextExt<'input>> for ArrParaVarContext<'input>{
	fn borrow_mut(&mut self) -> &mut ParaVarContextExt<'input> { &mut self.base }
}

impl<'input> ParaVarContextAttrs<'input> for ArrParaVarContext<'input> {}

impl<'input> ArrParaVarContextExt<'input>{
	fn new(ctx: &dyn ParaVarContextAttrs<'input>) -> Rc<ParaVarContextAll<'input>>  {
		Rc::new(
			ParaVarContextAll::ArrParaVarContext(
				BaseParserRuleContext::copy_from(ctx,ArrParaVarContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type StructParaVarContext<'input> = BaseParserRuleContext<'input,StructParaVarContextExt<'input>>;

pub trait StructParaVarContextAttrs<'input>: TaurusParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token STRUCT
	/// Returns `None` if there is no child corresponding to token STRUCT
	fn STRUCT(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(STRUCT, 0)
	}
	/// Retrieves all `TerminalNode`s corresponding to token IDENT in current rule
	fn IDENT_all(&self) -> Vec<Rc<TerminalNode<'input,TaurusParserContextType>>>  where Self:Sized{
		self.children_of_type()
	}
	/// Retrieves 'i's TerminalNode corresponding to token IDENT, starting from 0.
	/// Returns `None` if number of children corresponding to token IDENT is less or equal than `i`.
	fn IDENT(&self, i: usize) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(IDENT, i)
	}
}

impl<'input> StructParaVarContextAttrs<'input> for StructParaVarContext<'input>{}

pub struct StructParaVarContextExt<'input>{
	base:ParaVarContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{StructParaVarContextExt<'a>}

impl<'input> TaurusParserContext<'input> for StructParaVarContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for StructParaVarContext<'input>{
	fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_StructParaVar(self);
	}
}

impl<'input> CustomRuleContext<'input> for StructParaVarContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_paraVar }
	//fn type_rule_index() -> usize where Self: Sized { RULE_paraVar }
}

impl<'input> Borrow<ParaVarContextExt<'input>> for StructParaVarContext<'input>{
	fn borrow(&self) -> &ParaVarContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ParaVarContextExt<'input>> for StructParaVarContext<'input>{
	fn borrow_mut(&mut self) -> &mut ParaVarContextExt<'input> { &mut self.base }
}

impl<'input> ParaVarContextAttrs<'input> for StructParaVarContext<'input> {}

impl<'input> StructParaVarContextExt<'input>{
	fn new(ctx: &dyn ParaVarContextAttrs<'input>) -> Rc<ParaVarContextAll<'input>>  {
		Rc::new(
			ParaVarContextAll::StructParaVarContext(
				BaseParserRuleContext::copy_from(ctx,StructParaVarContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I, H> TaurusParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn paraVar(&mut self,)
	-> Result<Rc<ParaVarContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ParaVarContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 12, RULE_paraVar);
        let mut _localctx: Rc<ParaVarContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(157);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(8,&mut recog.base)? {
				1 =>{
					let tmp = AtomParaVarContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1);
					_localctx = tmp;
					{
					/*InvokeRule atomicType*/
					recog.base.set_state(146);
					recog.atomicType()?;

					recog.base.set_state(147);
					recog.base.match_token(IDENT,&mut recog.err_handler)?;

					}
				}
			,
				2 =>{
					let tmp = StructParaVarContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2);
					_localctx = tmp;
					{
					recog.base.set_state(149);
					recog.base.match_token(STRUCT,&mut recog.err_handler)?;

					recog.base.set_state(150);
					recog.base.match_token(IDENT,&mut recog.err_handler)?;

					recog.base.set_state(151);
					recog.base.match_token(IDENT,&mut recog.err_handler)?;

					}
				}
			,
				3 =>{
					let tmp = ArrParaVarContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 3);
					_localctx = tmp;
					{
					/*InvokeRule atomicType*/
					recog.base.set_state(152);
					recog.atomicType()?;

					recog.base.set_state(153);
					recog.base.match_token(IDENT,&mut recog.err_handler)?;

					recog.base.set_state(154);
					recog.base.match_token(LBRACKET,&mut recog.err_handler)?;

					recog.base.set_state(155);
					recog.base.match_token(RBRACKET,&mut recog.err_handler)?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- retVar ----------------
pub type RetVarContextAll<'input> = RetVarContext<'input>;


pub type RetVarContext<'input> = BaseParserRuleContext<'input,RetVarContextExt<'input>>;

#[derive(Clone)]
pub struct RetVarContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TaurusParserContext<'input> for RetVarContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for RetVarContext<'input>{
		fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_retVar(self);
		}fn exit(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
			listener.exit_retVar(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for RetVarContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_retVar }
	//fn type_rule_index() -> usize where Self: Sized { RULE_retVar }
}
antlr_rust::tid!{RetVarContextExt<'a>}

impl<'input> RetVarContextExt<'input>{
	fn new(parent: Option<Rc<dyn TaurusParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<RetVarContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,RetVarContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait RetVarContextAttrs<'input>: TaurusParserContext<'input> + BorrowMut<RetVarContextExt<'input>>{

fn retTy(&self) -> Option<Rc<RetTyContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token IDENT
/// Returns `None` if there is no child corresponding to token IDENT
fn IDENT(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
	self.get_token(IDENT, 0)
}

}

impl<'input> RetVarContextAttrs<'input> for RetVarContext<'input>{}

impl<'input, I, H> TaurusParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn retVar(&mut self,)
	-> Result<Rc<RetVarContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = RetVarContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 14, RULE_retVar);
        let mut _localctx: Rc<RetVarContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule retTy*/
			recog.base.set_state(159);
			recog.retTy()?;

			recog.base.set_state(160);
			recog.base.match_token(IDENT,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- retTy ----------------
#[derive(Debug)]
pub enum RetTyContextAll<'input>{
	VoidRetTyContext(VoidRetTyContext<'input>),
	AtomRetTyContext(AtomRetTyContext<'input>),
	ArrRetTyContext(ArrRetTyContext<'input>),
	StructRetTyContext(StructRetTyContext<'input>),
Error(RetTyContext<'input>)
}
antlr_rust::tid!{RetTyContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for RetTyContextAll<'input>{}

impl<'input> TaurusParserContext<'input> for RetTyContextAll<'input>{}

impl<'input> Deref for RetTyContextAll<'input>{
	type Target = dyn RetTyContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use RetTyContextAll::*;
		match self{
			VoidRetTyContext(inner) => inner,
			AtomRetTyContext(inner) => inner,
			ArrRetTyContext(inner) => inner,
			StructRetTyContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for RetTyContextAll<'input>{
    fn enter(&self, listener: &mut (dyn TaurusListener<'input> + 'a)) { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn TaurusListener<'input> + 'a)) { self.deref().exit(listener) }
}



pub type RetTyContext<'input> = BaseParserRuleContext<'input,RetTyContextExt<'input>>;

#[derive(Clone)]
pub struct RetTyContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TaurusParserContext<'input> for RetTyContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for RetTyContext<'input>{
}

impl<'input> CustomRuleContext<'input> for RetTyContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_retTy }
	//fn type_rule_index() -> usize where Self: Sized { RULE_retTy }
}
antlr_rust::tid!{RetTyContextExt<'a>}

impl<'input> RetTyContextExt<'input>{
	fn new(parent: Option<Rc<dyn TaurusParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<RetTyContextAll<'input>> {
		Rc::new(
		RetTyContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,RetTyContextExt{
				ph:PhantomData
			}),
		)
		)
	}
}

pub trait RetTyContextAttrs<'input>: TaurusParserContext<'input> + BorrowMut<RetTyContextExt<'input>>{


}

impl<'input> RetTyContextAttrs<'input> for RetTyContext<'input>{}

pub type VoidRetTyContext<'input> = BaseParserRuleContext<'input,VoidRetTyContextExt<'input>>;

pub trait VoidRetTyContextAttrs<'input>: TaurusParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token VOID
	/// Returns `None` if there is no child corresponding to token VOID
	fn VOID(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(VOID, 0)
	}
}

impl<'input> VoidRetTyContextAttrs<'input> for VoidRetTyContext<'input>{}

pub struct VoidRetTyContextExt<'input>{
	base:RetTyContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{VoidRetTyContextExt<'a>}

impl<'input> TaurusParserContext<'input> for VoidRetTyContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for VoidRetTyContext<'input>{
	fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_VoidRetTy(self);
	}
}

impl<'input> CustomRuleContext<'input> for VoidRetTyContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_retTy }
	//fn type_rule_index() -> usize where Self: Sized { RULE_retTy }
}

impl<'input> Borrow<RetTyContextExt<'input>> for VoidRetTyContext<'input>{
	fn borrow(&self) -> &RetTyContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<RetTyContextExt<'input>> for VoidRetTyContext<'input>{
	fn borrow_mut(&mut self) -> &mut RetTyContextExt<'input> { &mut self.base }
}

impl<'input> RetTyContextAttrs<'input> for VoidRetTyContext<'input> {}

impl<'input> VoidRetTyContextExt<'input>{
	fn new(ctx: &dyn RetTyContextAttrs<'input>) -> Rc<RetTyContextAll<'input>>  {
		Rc::new(
			RetTyContextAll::VoidRetTyContext(
				BaseParserRuleContext::copy_from(ctx,VoidRetTyContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type AtomRetTyContext<'input> = BaseParserRuleContext<'input,AtomRetTyContextExt<'input>>;

pub trait AtomRetTyContextAttrs<'input>: TaurusParserContext<'input>{
	fn atomicType(&self) -> Option<Rc<AtomicTypeContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> AtomRetTyContextAttrs<'input> for AtomRetTyContext<'input>{}

pub struct AtomRetTyContextExt<'input>{
	base:RetTyContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{AtomRetTyContextExt<'a>}

impl<'input> TaurusParserContext<'input> for AtomRetTyContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for AtomRetTyContext<'input>{
	fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_AtomRetTy(self);
	}
}

impl<'input> CustomRuleContext<'input> for AtomRetTyContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_retTy }
	//fn type_rule_index() -> usize where Self: Sized { RULE_retTy }
}

impl<'input> Borrow<RetTyContextExt<'input>> for AtomRetTyContext<'input>{
	fn borrow(&self) -> &RetTyContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<RetTyContextExt<'input>> for AtomRetTyContext<'input>{
	fn borrow_mut(&mut self) -> &mut RetTyContextExt<'input> { &mut self.base }
}

impl<'input> RetTyContextAttrs<'input> for AtomRetTyContext<'input> {}

impl<'input> AtomRetTyContextExt<'input>{
	fn new(ctx: &dyn RetTyContextAttrs<'input>) -> Rc<RetTyContextAll<'input>>  {
		Rc::new(
			RetTyContextAll::AtomRetTyContext(
				BaseParserRuleContext::copy_from(ctx,AtomRetTyContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type ArrRetTyContext<'input> = BaseParserRuleContext<'input,ArrRetTyContextExt<'input>>;

pub trait ArrRetTyContextAttrs<'input>: TaurusParserContext<'input>{
	fn atomicType(&self) -> Option<Rc<AtomicTypeContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token LBRACKET
	/// Returns `None` if there is no child corresponding to token LBRACKET
	fn LBRACKET(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(LBRACKET, 0)
	}
	/// Retrieves first TerminalNode corresponding to token RBRACKET
	/// Returns `None` if there is no child corresponding to token RBRACKET
	fn RBRACKET(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(RBRACKET, 0)
	}
}

impl<'input> ArrRetTyContextAttrs<'input> for ArrRetTyContext<'input>{}

pub struct ArrRetTyContextExt<'input>{
	base:RetTyContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{ArrRetTyContextExt<'a>}

impl<'input> TaurusParserContext<'input> for ArrRetTyContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for ArrRetTyContext<'input>{
	fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_ArrRetTy(self);
	}
}

impl<'input> CustomRuleContext<'input> for ArrRetTyContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_retTy }
	//fn type_rule_index() -> usize where Self: Sized { RULE_retTy }
}

impl<'input> Borrow<RetTyContextExt<'input>> for ArrRetTyContext<'input>{
	fn borrow(&self) -> &RetTyContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<RetTyContextExt<'input>> for ArrRetTyContext<'input>{
	fn borrow_mut(&mut self) -> &mut RetTyContextExt<'input> { &mut self.base }
}

impl<'input> RetTyContextAttrs<'input> for ArrRetTyContext<'input> {}

impl<'input> ArrRetTyContextExt<'input>{
	fn new(ctx: &dyn RetTyContextAttrs<'input>) -> Rc<RetTyContextAll<'input>>  {
		Rc::new(
			RetTyContextAll::ArrRetTyContext(
				BaseParserRuleContext::copy_from(ctx,ArrRetTyContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type StructRetTyContext<'input> = BaseParserRuleContext<'input,StructRetTyContextExt<'input>>;

pub trait StructRetTyContextAttrs<'input>: TaurusParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token STRUCT
	/// Returns `None` if there is no child corresponding to token STRUCT
	fn STRUCT(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(STRUCT, 0)
	}
	/// Retrieves first TerminalNode corresponding to token IDENT
	/// Returns `None` if there is no child corresponding to token IDENT
	fn IDENT(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(IDENT, 0)
	}
}

impl<'input> StructRetTyContextAttrs<'input> for StructRetTyContext<'input>{}

pub struct StructRetTyContextExt<'input>{
	base:RetTyContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{StructRetTyContextExt<'a>}

impl<'input> TaurusParserContext<'input> for StructRetTyContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for StructRetTyContext<'input>{
	fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_StructRetTy(self);
	}
}

impl<'input> CustomRuleContext<'input> for StructRetTyContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_retTy }
	//fn type_rule_index() -> usize where Self: Sized { RULE_retTy }
}

impl<'input> Borrow<RetTyContextExt<'input>> for StructRetTyContext<'input>{
	fn borrow(&self) -> &RetTyContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<RetTyContextExt<'input>> for StructRetTyContext<'input>{
	fn borrow_mut(&mut self) -> &mut RetTyContextExt<'input> { &mut self.base }
}

impl<'input> RetTyContextAttrs<'input> for StructRetTyContext<'input> {}

impl<'input> StructRetTyContextExt<'input>{
	fn new(ctx: &dyn RetTyContextAttrs<'input>) -> Rc<RetTyContextAll<'input>>  {
		Rc::new(
			RetTyContextAll::StructRetTyContext(
				BaseParserRuleContext::copy_from(ctx,StructRetTyContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I, H> TaurusParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn retTy(&mut self,)
	-> Result<Rc<RetTyContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = RetTyContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 16, RULE_retTy);
        let mut _localctx: Rc<RetTyContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(170);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(9,&mut recog.base)? {
				1 =>{
					let tmp = AtomRetTyContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1);
					_localctx = tmp;
					{
					/*InvokeRule atomicType*/
					recog.base.set_state(162);
					recog.atomicType()?;

					}
				}
			,
				2 =>{
					let tmp = StructRetTyContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2);
					_localctx = tmp;
					{
					recog.base.set_state(163);
					recog.base.match_token(STRUCT,&mut recog.err_handler)?;

					recog.base.set_state(164);
					recog.base.match_token(IDENT,&mut recog.err_handler)?;

					}
				}
			,
				3 =>{
					let tmp = ArrRetTyContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 3);
					_localctx = tmp;
					{
					/*InvokeRule atomicType*/
					recog.base.set_state(165);
					recog.atomicType()?;

					recog.base.set_state(166);
					recog.base.match_token(LBRACKET,&mut recog.err_handler)?;

					recog.base.set_state(167);
					recog.base.match_token(RBRACKET,&mut recog.err_handler)?;

					}
				}
			,
				4 =>{
					let tmp = VoidRetTyContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 4);
					_localctx = tmp;
					{
					recog.base.set_state(169);
					recog.base.match_token(VOID,&mut recog.err_handler)?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- atomicType ----------------
#[derive(Debug)]
pub enum AtomicTypeContextAll<'input>{
	AtomFloatContext(AtomFloatContext<'input>),
	AtomIntContext(AtomIntContext<'input>),
Error(AtomicTypeContext<'input>)
}
antlr_rust::tid!{AtomicTypeContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for AtomicTypeContextAll<'input>{}

impl<'input> TaurusParserContext<'input> for AtomicTypeContextAll<'input>{}

impl<'input> Deref for AtomicTypeContextAll<'input>{
	type Target = dyn AtomicTypeContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use AtomicTypeContextAll::*;
		match self{
			AtomFloatContext(inner) => inner,
			AtomIntContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for AtomicTypeContextAll<'input>{
    fn enter(&self, listener: &mut (dyn TaurusListener<'input> + 'a)) { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn TaurusListener<'input> + 'a)) { self.deref().exit(listener) }
}



pub type AtomicTypeContext<'input> = BaseParserRuleContext<'input,AtomicTypeContextExt<'input>>;

#[derive(Clone)]
pub struct AtomicTypeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TaurusParserContext<'input> for AtomicTypeContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for AtomicTypeContext<'input>{
}

impl<'input> CustomRuleContext<'input> for AtomicTypeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_atomicType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_atomicType }
}
antlr_rust::tid!{AtomicTypeContextExt<'a>}

impl<'input> AtomicTypeContextExt<'input>{
	fn new(parent: Option<Rc<dyn TaurusParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<AtomicTypeContextAll<'input>> {
		Rc::new(
		AtomicTypeContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AtomicTypeContextExt{
				ph:PhantomData
			}),
		)
		)
	}
}

pub trait AtomicTypeContextAttrs<'input>: TaurusParserContext<'input> + BorrowMut<AtomicTypeContextExt<'input>>{


}

impl<'input> AtomicTypeContextAttrs<'input> for AtomicTypeContext<'input>{}

pub type AtomFloatContext<'input> = BaseParserRuleContext<'input,AtomFloatContextExt<'input>>;

pub trait AtomFloatContextAttrs<'input>: TaurusParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token FLOAT
	/// Returns `None` if there is no child corresponding to token FLOAT
	fn FLOAT(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(FLOAT, 0)
	}
}

impl<'input> AtomFloatContextAttrs<'input> for AtomFloatContext<'input>{}

pub struct AtomFloatContextExt<'input>{
	base:AtomicTypeContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{AtomFloatContextExt<'a>}

impl<'input> TaurusParserContext<'input> for AtomFloatContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for AtomFloatContext<'input>{
	fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_AtomFloat(self);
	}
}

impl<'input> CustomRuleContext<'input> for AtomFloatContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_atomicType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_atomicType }
}

impl<'input> Borrow<AtomicTypeContextExt<'input>> for AtomFloatContext<'input>{
	fn borrow(&self) -> &AtomicTypeContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<AtomicTypeContextExt<'input>> for AtomFloatContext<'input>{
	fn borrow_mut(&mut self) -> &mut AtomicTypeContextExt<'input> { &mut self.base }
}

impl<'input> AtomicTypeContextAttrs<'input> for AtomFloatContext<'input> {}

impl<'input> AtomFloatContextExt<'input>{
	fn new(ctx: &dyn AtomicTypeContextAttrs<'input>) -> Rc<AtomicTypeContextAll<'input>>  {
		Rc::new(
			AtomicTypeContextAll::AtomFloatContext(
				BaseParserRuleContext::copy_from(ctx,AtomFloatContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type AtomIntContext<'input> = BaseParserRuleContext<'input,AtomIntContextExt<'input>>;

pub trait AtomIntContextAttrs<'input>: TaurusParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token INT
	/// Returns `None` if there is no child corresponding to token INT
	fn INT(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(INT, 0)
	}
}

impl<'input> AtomIntContextAttrs<'input> for AtomIntContext<'input>{}

pub struct AtomIntContextExt<'input>{
	base:AtomicTypeContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{AtomIntContextExt<'a>}

impl<'input> TaurusParserContext<'input> for AtomIntContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for AtomIntContext<'input>{
	fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_AtomInt(self);
	}
}

impl<'input> CustomRuleContext<'input> for AtomIntContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_atomicType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_atomicType }
}

impl<'input> Borrow<AtomicTypeContextExt<'input>> for AtomIntContext<'input>{
	fn borrow(&self) -> &AtomicTypeContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<AtomicTypeContextExt<'input>> for AtomIntContext<'input>{
	fn borrow_mut(&mut self) -> &mut AtomicTypeContextExt<'input> { &mut self.base }
}

impl<'input> AtomicTypeContextAttrs<'input> for AtomIntContext<'input> {}

impl<'input> AtomIntContextExt<'input>{
	fn new(ctx: &dyn AtomicTypeContextAttrs<'input>) -> Rc<AtomicTypeContextAll<'input>>  {
		Rc::new(
			AtomicTypeContextAll::AtomIntContext(
				BaseParserRuleContext::copy_from(ctx,AtomIntContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I, H> TaurusParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn atomicType(&mut self,)
	-> Result<Rc<AtomicTypeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = AtomicTypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 18, RULE_atomicType);
        let mut _localctx: Rc<AtomicTypeContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(174);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 INT 
				=> {
					let tmp = AtomIntContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1);
					_localctx = tmp;
					{
					recog.base.set_state(172);
					recog.base.match_token(INT,&mut recog.err_handler)?;

					}
				}

			 FLOAT 
				=> {
					let tmp = AtomFloatContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2);
					_localctx = tmp;
					{
					recog.base.set_state(173);
					recog.base.match_token(FLOAT,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- logicParaVar ----------------
#[derive(Debug)]
pub enum LogicParaVarContextAll<'input>{
	ArrLogicParaVarContext(ArrLogicParaVarContext<'input>),
	StructLogicParaVarContext(StructLogicParaVarContext<'input>),
	AtomLogicParaVarContext(AtomLogicParaVarContext<'input>),
Error(LogicParaVarContext<'input>)
}
antlr_rust::tid!{LogicParaVarContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for LogicParaVarContextAll<'input>{}

impl<'input> TaurusParserContext<'input> for LogicParaVarContextAll<'input>{}

impl<'input> Deref for LogicParaVarContextAll<'input>{
	type Target = dyn LogicParaVarContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use LogicParaVarContextAll::*;
		match self{
			ArrLogicParaVarContext(inner) => inner,
			StructLogicParaVarContext(inner) => inner,
			AtomLogicParaVarContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for LogicParaVarContextAll<'input>{
    fn enter(&self, listener: &mut (dyn TaurusListener<'input> + 'a)) { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn TaurusListener<'input> + 'a)) { self.deref().exit(listener) }
}



pub type LogicParaVarContext<'input> = BaseParserRuleContext<'input,LogicParaVarContextExt<'input>>;

#[derive(Clone)]
pub struct LogicParaVarContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TaurusParserContext<'input> for LogicParaVarContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for LogicParaVarContext<'input>{
}

impl<'input> CustomRuleContext<'input> for LogicParaVarContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_logicParaVar }
	//fn type_rule_index() -> usize where Self: Sized { RULE_logicParaVar }
}
antlr_rust::tid!{LogicParaVarContextExt<'a>}

impl<'input> LogicParaVarContextExt<'input>{
	fn new(parent: Option<Rc<dyn TaurusParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<LogicParaVarContextAll<'input>> {
		Rc::new(
		LogicParaVarContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,LogicParaVarContextExt{
				ph:PhantomData
			}),
		)
		)
	}
}

pub trait LogicParaVarContextAttrs<'input>: TaurusParserContext<'input> + BorrowMut<LogicParaVarContextExt<'input>>{


}

impl<'input> LogicParaVarContextAttrs<'input> for LogicParaVarContext<'input>{}

pub type ArrLogicParaVarContext<'input> = BaseParserRuleContext<'input,ArrLogicParaVarContextExt<'input>>;

pub trait ArrLogicParaVarContextAttrs<'input>: TaurusParserContext<'input>{
	fn logicAtomicType(&self) -> Option<Rc<LogicAtomicTypeContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token IDENT
	/// Returns `None` if there is no child corresponding to token IDENT
	fn IDENT(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(IDENT, 0)
	}
	/// Retrieves first TerminalNode corresponding to token LBRACKET
	/// Returns `None` if there is no child corresponding to token LBRACKET
	fn LBRACKET(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(LBRACKET, 0)
	}
	/// Retrieves first TerminalNode corresponding to token RBRACKET
	/// Returns `None` if there is no child corresponding to token RBRACKET
	fn RBRACKET(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(RBRACKET, 0)
	}
}

impl<'input> ArrLogicParaVarContextAttrs<'input> for ArrLogicParaVarContext<'input>{}

pub struct ArrLogicParaVarContextExt<'input>{
	base:LogicParaVarContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{ArrLogicParaVarContextExt<'a>}

impl<'input> TaurusParserContext<'input> for ArrLogicParaVarContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for ArrLogicParaVarContext<'input>{
	fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_ArrLogicParaVar(self);
	}
}

impl<'input> CustomRuleContext<'input> for ArrLogicParaVarContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_logicParaVar }
	//fn type_rule_index() -> usize where Self: Sized { RULE_logicParaVar }
}

impl<'input> Borrow<LogicParaVarContextExt<'input>> for ArrLogicParaVarContext<'input>{
	fn borrow(&self) -> &LogicParaVarContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<LogicParaVarContextExt<'input>> for ArrLogicParaVarContext<'input>{
	fn borrow_mut(&mut self) -> &mut LogicParaVarContextExt<'input> { &mut self.base }
}

impl<'input> LogicParaVarContextAttrs<'input> for ArrLogicParaVarContext<'input> {}

impl<'input> ArrLogicParaVarContextExt<'input>{
	fn new(ctx: &dyn LogicParaVarContextAttrs<'input>) -> Rc<LogicParaVarContextAll<'input>>  {
		Rc::new(
			LogicParaVarContextAll::ArrLogicParaVarContext(
				BaseParserRuleContext::copy_from(ctx,ArrLogicParaVarContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type StructLogicParaVarContext<'input> = BaseParserRuleContext<'input,StructLogicParaVarContextExt<'input>>;

pub trait StructLogicParaVarContextAttrs<'input>: TaurusParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token STRUCT
	/// Returns `None` if there is no child corresponding to token STRUCT
	fn STRUCT(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(STRUCT, 0)
	}
	/// Retrieves all `TerminalNode`s corresponding to token IDENT in current rule
	fn IDENT_all(&self) -> Vec<Rc<TerminalNode<'input,TaurusParserContextType>>>  where Self:Sized{
		self.children_of_type()
	}
	/// Retrieves 'i's TerminalNode corresponding to token IDENT, starting from 0.
	/// Returns `None` if number of children corresponding to token IDENT is less or equal than `i`.
	fn IDENT(&self, i: usize) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(IDENT, i)
	}
}

impl<'input> StructLogicParaVarContextAttrs<'input> for StructLogicParaVarContext<'input>{}

pub struct StructLogicParaVarContextExt<'input>{
	base:LogicParaVarContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{StructLogicParaVarContextExt<'a>}

impl<'input> TaurusParserContext<'input> for StructLogicParaVarContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for StructLogicParaVarContext<'input>{
	fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_StructLogicParaVar(self);
	}
}

impl<'input> CustomRuleContext<'input> for StructLogicParaVarContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_logicParaVar }
	//fn type_rule_index() -> usize where Self: Sized { RULE_logicParaVar }
}

impl<'input> Borrow<LogicParaVarContextExt<'input>> for StructLogicParaVarContext<'input>{
	fn borrow(&self) -> &LogicParaVarContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<LogicParaVarContextExt<'input>> for StructLogicParaVarContext<'input>{
	fn borrow_mut(&mut self) -> &mut LogicParaVarContextExt<'input> { &mut self.base }
}

impl<'input> LogicParaVarContextAttrs<'input> for StructLogicParaVarContext<'input> {}

impl<'input> StructLogicParaVarContextExt<'input>{
	fn new(ctx: &dyn LogicParaVarContextAttrs<'input>) -> Rc<LogicParaVarContextAll<'input>>  {
		Rc::new(
			LogicParaVarContextAll::StructLogicParaVarContext(
				BaseParserRuleContext::copy_from(ctx,StructLogicParaVarContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type AtomLogicParaVarContext<'input> = BaseParserRuleContext<'input,AtomLogicParaVarContextExt<'input>>;

pub trait AtomLogicParaVarContextAttrs<'input>: TaurusParserContext<'input>{
	fn logicAtomicType(&self) -> Option<Rc<LogicAtomicTypeContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token IDENT
	/// Returns `None` if there is no child corresponding to token IDENT
	fn IDENT(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(IDENT, 0)
	}
}

impl<'input> AtomLogicParaVarContextAttrs<'input> for AtomLogicParaVarContext<'input>{}

pub struct AtomLogicParaVarContextExt<'input>{
	base:LogicParaVarContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{AtomLogicParaVarContextExt<'a>}

impl<'input> TaurusParserContext<'input> for AtomLogicParaVarContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for AtomLogicParaVarContext<'input>{
	fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_AtomLogicParaVar(self);
	}
}

impl<'input> CustomRuleContext<'input> for AtomLogicParaVarContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_logicParaVar }
	//fn type_rule_index() -> usize where Self: Sized { RULE_logicParaVar }
}

impl<'input> Borrow<LogicParaVarContextExt<'input>> for AtomLogicParaVarContext<'input>{
	fn borrow(&self) -> &LogicParaVarContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<LogicParaVarContextExt<'input>> for AtomLogicParaVarContext<'input>{
	fn borrow_mut(&mut self) -> &mut LogicParaVarContextExt<'input> { &mut self.base }
}

impl<'input> LogicParaVarContextAttrs<'input> for AtomLogicParaVarContext<'input> {}

impl<'input> AtomLogicParaVarContextExt<'input>{
	fn new(ctx: &dyn LogicParaVarContextAttrs<'input>) -> Rc<LogicParaVarContextAll<'input>>  {
		Rc::new(
			LogicParaVarContextAll::AtomLogicParaVarContext(
				BaseParserRuleContext::copy_from(ctx,AtomLogicParaVarContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I, H> TaurusParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn logicParaVar(&mut self,)
	-> Result<Rc<LogicParaVarContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = LogicParaVarContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 20, RULE_logicParaVar);
        let mut _localctx: Rc<LogicParaVarContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(187);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(11,&mut recog.base)? {
				1 =>{
					let tmp = AtomLogicParaVarContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1);
					_localctx = tmp;
					{
					/*InvokeRule logicAtomicType*/
					recog.base.set_state(176);
					recog.logicAtomicType()?;

					recog.base.set_state(177);
					recog.base.match_token(IDENT,&mut recog.err_handler)?;

					}
				}
			,
				2 =>{
					let tmp = StructLogicParaVarContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2);
					_localctx = tmp;
					{
					recog.base.set_state(179);
					recog.base.match_token(STRUCT,&mut recog.err_handler)?;

					recog.base.set_state(180);
					recog.base.match_token(IDENT,&mut recog.err_handler)?;

					recog.base.set_state(181);
					recog.base.match_token(IDENT,&mut recog.err_handler)?;

					}
				}
			,
				3 =>{
					let tmp = ArrLogicParaVarContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 3);
					_localctx = tmp;
					{
					/*InvokeRule logicAtomicType*/
					recog.base.set_state(182);
					recog.logicAtomicType()?;

					recog.base.set_state(183);
					recog.base.match_token(IDENT,&mut recog.err_handler)?;

					recog.base.set_state(184);
					recog.base.match_token(LBRACKET,&mut recog.err_handler)?;

					recog.base.set_state(185);
					recog.base.match_token(RBRACKET,&mut recog.err_handler)?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- logicAtomicType ----------------
#[derive(Debug)]
pub enum LogicAtomicTypeContextAll<'input>{
	LogicAtomBoolContext(LogicAtomBoolContext<'input>),
	LogicAtomRealContext(LogicAtomRealContext<'input>),
	LogicAtomIntContext(LogicAtomIntContext<'input>),
Error(LogicAtomicTypeContext<'input>)
}
antlr_rust::tid!{LogicAtomicTypeContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for LogicAtomicTypeContextAll<'input>{}

impl<'input> TaurusParserContext<'input> for LogicAtomicTypeContextAll<'input>{}

impl<'input> Deref for LogicAtomicTypeContextAll<'input>{
	type Target = dyn LogicAtomicTypeContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use LogicAtomicTypeContextAll::*;
		match self{
			LogicAtomBoolContext(inner) => inner,
			LogicAtomRealContext(inner) => inner,
			LogicAtomIntContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for LogicAtomicTypeContextAll<'input>{
    fn enter(&self, listener: &mut (dyn TaurusListener<'input> + 'a)) { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn TaurusListener<'input> + 'a)) { self.deref().exit(listener) }
}



pub type LogicAtomicTypeContext<'input> = BaseParserRuleContext<'input,LogicAtomicTypeContextExt<'input>>;

#[derive(Clone)]
pub struct LogicAtomicTypeContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TaurusParserContext<'input> for LogicAtomicTypeContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for LogicAtomicTypeContext<'input>{
}

impl<'input> CustomRuleContext<'input> for LogicAtomicTypeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_logicAtomicType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_logicAtomicType }
}
antlr_rust::tid!{LogicAtomicTypeContextExt<'a>}

impl<'input> LogicAtomicTypeContextExt<'input>{
	fn new(parent: Option<Rc<dyn TaurusParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<LogicAtomicTypeContextAll<'input>> {
		Rc::new(
		LogicAtomicTypeContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,LogicAtomicTypeContextExt{
				ph:PhantomData
			}),
		)
		)
	}
}

pub trait LogicAtomicTypeContextAttrs<'input>: TaurusParserContext<'input> + BorrowMut<LogicAtomicTypeContextExt<'input>>{


}

impl<'input> LogicAtomicTypeContextAttrs<'input> for LogicAtomicTypeContext<'input>{}

pub type LogicAtomBoolContext<'input> = BaseParserRuleContext<'input,LogicAtomBoolContextExt<'input>>;

pub trait LogicAtomBoolContextAttrs<'input>: TaurusParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token BOOLEAN
	/// Returns `None` if there is no child corresponding to token BOOLEAN
	fn BOOLEAN(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(BOOLEAN, 0)
	}
}

impl<'input> LogicAtomBoolContextAttrs<'input> for LogicAtomBoolContext<'input>{}

pub struct LogicAtomBoolContextExt<'input>{
	base:LogicAtomicTypeContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{LogicAtomBoolContextExt<'a>}

impl<'input> TaurusParserContext<'input> for LogicAtomBoolContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for LogicAtomBoolContext<'input>{
	fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_LogicAtomBool(self);
	}
}

impl<'input> CustomRuleContext<'input> for LogicAtomBoolContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_logicAtomicType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_logicAtomicType }
}

impl<'input> Borrow<LogicAtomicTypeContextExt<'input>> for LogicAtomBoolContext<'input>{
	fn borrow(&self) -> &LogicAtomicTypeContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<LogicAtomicTypeContextExt<'input>> for LogicAtomBoolContext<'input>{
	fn borrow_mut(&mut self) -> &mut LogicAtomicTypeContextExt<'input> { &mut self.base }
}

impl<'input> LogicAtomicTypeContextAttrs<'input> for LogicAtomBoolContext<'input> {}

impl<'input> LogicAtomBoolContextExt<'input>{
	fn new(ctx: &dyn LogicAtomicTypeContextAttrs<'input>) -> Rc<LogicAtomicTypeContextAll<'input>>  {
		Rc::new(
			LogicAtomicTypeContextAll::LogicAtomBoolContext(
				BaseParserRuleContext::copy_from(ctx,LogicAtomBoolContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type LogicAtomRealContext<'input> = BaseParserRuleContext<'input,LogicAtomRealContextExt<'input>>;

pub trait LogicAtomRealContextAttrs<'input>: TaurusParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token REAL
	/// Returns `None` if there is no child corresponding to token REAL
	fn REAL(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(REAL, 0)
	}
}

impl<'input> LogicAtomRealContextAttrs<'input> for LogicAtomRealContext<'input>{}

pub struct LogicAtomRealContextExt<'input>{
	base:LogicAtomicTypeContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{LogicAtomRealContextExt<'a>}

impl<'input> TaurusParserContext<'input> for LogicAtomRealContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for LogicAtomRealContext<'input>{
	fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_LogicAtomReal(self);
	}
}

impl<'input> CustomRuleContext<'input> for LogicAtomRealContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_logicAtomicType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_logicAtomicType }
}

impl<'input> Borrow<LogicAtomicTypeContextExt<'input>> for LogicAtomRealContext<'input>{
	fn borrow(&self) -> &LogicAtomicTypeContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<LogicAtomicTypeContextExt<'input>> for LogicAtomRealContext<'input>{
	fn borrow_mut(&mut self) -> &mut LogicAtomicTypeContextExt<'input> { &mut self.base }
}

impl<'input> LogicAtomicTypeContextAttrs<'input> for LogicAtomRealContext<'input> {}

impl<'input> LogicAtomRealContextExt<'input>{
	fn new(ctx: &dyn LogicAtomicTypeContextAttrs<'input>) -> Rc<LogicAtomicTypeContextAll<'input>>  {
		Rc::new(
			LogicAtomicTypeContextAll::LogicAtomRealContext(
				BaseParserRuleContext::copy_from(ctx,LogicAtomRealContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type LogicAtomIntContext<'input> = BaseParserRuleContext<'input,LogicAtomIntContextExt<'input>>;

pub trait LogicAtomIntContextAttrs<'input>: TaurusParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token INTEGER
	/// Returns `None` if there is no child corresponding to token INTEGER
	fn INTEGER(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(INTEGER, 0)
	}
}

impl<'input> LogicAtomIntContextAttrs<'input> for LogicAtomIntContext<'input>{}

pub struct LogicAtomIntContextExt<'input>{
	base:LogicAtomicTypeContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{LogicAtomIntContextExt<'a>}

impl<'input> TaurusParserContext<'input> for LogicAtomIntContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for LogicAtomIntContext<'input>{
	fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_LogicAtomInt(self);
	}
}

impl<'input> CustomRuleContext<'input> for LogicAtomIntContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_logicAtomicType }
	//fn type_rule_index() -> usize where Self: Sized { RULE_logicAtomicType }
}

impl<'input> Borrow<LogicAtomicTypeContextExt<'input>> for LogicAtomIntContext<'input>{
	fn borrow(&self) -> &LogicAtomicTypeContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<LogicAtomicTypeContextExt<'input>> for LogicAtomIntContext<'input>{
	fn borrow_mut(&mut self) -> &mut LogicAtomicTypeContextExt<'input> { &mut self.base }
}

impl<'input> LogicAtomicTypeContextAttrs<'input> for LogicAtomIntContext<'input> {}

impl<'input> LogicAtomIntContextExt<'input>{
	fn new(ctx: &dyn LogicAtomicTypeContextAttrs<'input>) -> Rc<LogicAtomicTypeContextAll<'input>>  {
		Rc::new(
			LogicAtomicTypeContextAll::LogicAtomIntContext(
				BaseParserRuleContext::copy_from(ctx,LogicAtomIntContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I, H> TaurusParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn logicAtomicType(&mut self,)
	-> Result<Rc<LogicAtomicTypeContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = LogicAtomicTypeContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 22, RULE_logicAtomicType);
        let mut _localctx: Rc<LogicAtomicTypeContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(192);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 INTEGER 
				=> {
					let tmp = LogicAtomIntContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1);
					_localctx = tmp;
					{
					recog.base.set_state(189);
					recog.base.match_token(INTEGER,&mut recog.err_handler)?;

					}
				}

			 REAL 
				=> {
					let tmp = LogicAtomRealContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2);
					_localctx = tmp;
					{
					recog.base.set_state(190);
					recog.base.match_token(REAL,&mut recog.err_handler)?;

					}
				}

			 BOOLEAN 
				=> {
					let tmp = LogicAtomBoolContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 3);
					_localctx = tmp;
					{
					recog.base.set_state(191);
					recog.base.match_token(BOOLEAN,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- stmt ----------------
#[derive(Debug)]
pub enum StmtContextAll<'input>{
	IfStmtContext(IfStmtContext<'input>),
	ExprStmtContext(ExprStmtContext<'input>),
	WhileStmtContext(WhileStmtContext<'input>),
	AssignStmtContext(AssignStmtContext<'input>),
	BreakStmtContext(BreakStmtContext<'input>),
	BlockStmtContext(BlockStmtContext<'input>),
	EmptyStmtContext(EmptyStmtContext<'input>),
	ContStmtContext(ContStmtContext<'input>),
	AssertStmtContext(AssertStmtContext<'input>),
	ForStmtContext(ForStmtContext<'input>),
	ReturnStmtContext(ReturnStmtContext<'input>),
	DoStmtContext(DoStmtContext<'input>),
Error(StmtContext<'input>)
}
antlr_rust::tid!{StmtContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for StmtContextAll<'input>{}

impl<'input> TaurusParserContext<'input> for StmtContextAll<'input>{}

impl<'input> Deref for StmtContextAll<'input>{
	type Target = dyn StmtContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use StmtContextAll::*;
		match self{
			IfStmtContext(inner) => inner,
			ExprStmtContext(inner) => inner,
			WhileStmtContext(inner) => inner,
			AssignStmtContext(inner) => inner,
			BreakStmtContext(inner) => inner,
			BlockStmtContext(inner) => inner,
			EmptyStmtContext(inner) => inner,
			ContStmtContext(inner) => inner,
			AssertStmtContext(inner) => inner,
			ForStmtContext(inner) => inner,
			ReturnStmtContext(inner) => inner,
			DoStmtContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for StmtContextAll<'input>{
    fn enter(&self, listener: &mut (dyn TaurusListener<'input> + 'a)) { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn TaurusListener<'input> + 'a)) { self.deref().exit(listener) }
}



pub type StmtContext<'input> = BaseParserRuleContext<'input,StmtContextExt<'input>>;

#[derive(Clone)]
pub struct StmtContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TaurusParserContext<'input> for StmtContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for StmtContext<'input>{
}

impl<'input> CustomRuleContext<'input> for StmtContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_stmt }
	//fn type_rule_index() -> usize where Self: Sized { RULE_stmt }
}
antlr_rust::tid!{StmtContextExt<'a>}

impl<'input> StmtContextExt<'input>{
	fn new(parent: Option<Rc<dyn TaurusParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<StmtContextAll<'input>> {
		Rc::new(
		StmtContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,StmtContextExt{
				ph:PhantomData
			}),
		)
		)
	}
}

pub trait StmtContextAttrs<'input>: TaurusParserContext<'input> + BorrowMut<StmtContextExt<'input>>{


}

impl<'input> StmtContextAttrs<'input> for StmtContext<'input>{}

pub type IfStmtContext<'input> = BaseParserRuleContext<'input,IfStmtContextExt<'input>>;

pub trait IfStmtContextAttrs<'input>: TaurusParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token IF
	/// Returns `None` if there is no child corresponding to token IF
	fn IF(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(IF, 0)
	}
	/// Retrieves first TerminalNode corresponding to token LPAR
	/// Returns `None` if there is no child corresponding to token LPAR
	fn LPAR(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(LPAR, 0)
	}
	fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token RPAR
	/// Returns `None` if there is no child corresponding to token RPAR
	fn RPAR(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(RPAR, 0)
	}
	fn stmt_all(&self) ->  Vec<Rc<StmtContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn stmt(&self, i: usize) -> Option<Rc<StmtContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
	/// Retrieves first TerminalNode corresponding to token ELSE
	/// Returns `None` if there is no child corresponding to token ELSE
	fn ELSE(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(ELSE, 0)
	}
}

impl<'input> IfStmtContextAttrs<'input> for IfStmtContext<'input>{}

pub struct IfStmtContextExt<'input>{
	base:StmtContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{IfStmtContextExt<'a>}

impl<'input> TaurusParserContext<'input> for IfStmtContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for IfStmtContext<'input>{
	fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_IfStmt(self);
	}
}

impl<'input> CustomRuleContext<'input> for IfStmtContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_stmt }
	//fn type_rule_index() -> usize where Self: Sized { RULE_stmt }
}

impl<'input> Borrow<StmtContextExt<'input>> for IfStmtContext<'input>{
	fn borrow(&self) -> &StmtContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<StmtContextExt<'input>> for IfStmtContext<'input>{
	fn borrow_mut(&mut self) -> &mut StmtContextExt<'input> { &mut self.base }
}

impl<'input> StmtContextAttrs<'input> for IfStmtContext<'input> {}

impl<'input> IfStmtContextExt<'input>{
	fn new(ctx: &dyn StmtContextAttrs<'input>) -> Rc<StmtContextAll<'input>>  {
		Rc::new(
			StmtContextAll::IfStmtContext(
				BaseParserRuleContext::copy_from(ctx,IfStmtContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type ExprStmtContext<'input> = BaseParserRuleContext<'input,ExprStmtContextExt<'input>>;

pub trait ExprStmtContextAttrs<'input>: TaurusParserContext<'input>{
	fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token SEMICOLON
	/// Returns `None` if there is no child corresponding to token SEMICOLON
	fn SEMICOLON(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(SEMICOLON, 0)
	}
}

impl<'input> ExprStmtContextAttrs<'input> for ExprStmtContext<'input>{}

pub struct ExprStmtContextExt<'input>{
	base:StmtContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{ExprStmtContextExt<'a>}

impl<'input> TaurusParserContext<'input> for ExprStmtContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for ExprStmtContext<'input>{
	fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_ExprStmt(self);
	}
}

impl<'input> CustomRuleContext<'input> for ExprStmtContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_stmt }
	//fn type_rule_index() -> usize where Self: Sized { RULE_stmt }
}

impl<'input> Borrow<StmtContextExt<'input>> for ExprStmtContext<'input>{
	fn borrow(&self) -> &StmtContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<StmtContextExt<'input>> for ExprStmtContext<'input>{
	fn borrow_mut(&mut self) -> &mut StmtContextExt<'input> { &mut self.base }
}

impl<'input> StmtContextAttrs<'input> for ExprStmtContext<'input> {}

impl<'input> ExprStmtContextExt<'input>{
	fn new(ctx: &dyn StmtContextAttrs<'input>) -> Rc<StmtContextAll<'input>>  {
		Rc::new(
			StmtContextAll::ExprStmtContext(
				BaseParserRuleContext::copy_from(ctx,ExprStmtContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type WhileStmtContext<'input> = BaseParserRuleContext<'input,WhileStmtContextExt<'input>>;

pub trait WhileStmtContextAttrs<'input>: TaurusParserContext<'input>{
	fn loopAnnot(&self) -> Option<Rc<LoopAnnotContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token WHILE
	/// Returns `None` if there is no child corresponding to token WHILE
	fn WHILE(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(WHILE, 0)
	}
	/// Retrieves first TerminalNode corresponding to token LPAR
	/// Returns `None` if there is no child corresponding to token LPAR
	fn LPAR(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(LPAR, 0)
	}
	fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token RPAR
	/// Returns `None` if there is no child corresponding to token RPAR
	fn RPAR(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(RPAR, 0)
	}
	fn stmt(&self) -> Option<Rc<StmtContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> WhileStmtContextAttrs<'input> for WhileStmtContext<'input>{}

pub struct WhileStmtContextExt<'input>{
	base:StmtContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{WhileStmtContextExt<'a>}

impl<'input> TaurusParserContext<'input> for WhileStmtContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for WhileStmtContext<'input>{
	fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_WhileStmt(self);
	}
}

impl<'input> CustomRuleContext<'input> for WhileStmtContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_stmt }
	//fn type_rule_index() -> usize where Self: Sized { RULE_stmt }
}

impl<'input> Borrow<StmtContextExt<'input>> for WhileStmtContext<'input>{
	fn borrow(&self) -> &StmtContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<StmtContextExt<'input>> for WhileStmtContext<'input>{
	fn borrow_mut(&mut self) -> &mut StmtContextExt<'input> { &mut self.base }
}

impl<'input> StmtContextAttrs<'input> for WhileStmtContext<'input> {}

impl<'input> WhileStmtContextExt<'input>{
	fn new(ctx: &dyn StmtContextAttrs<'input>) -> Rc<StmtContextAll<'input>>  {
		Rc::new(
			StmtContextAll::WhileStmtContext(
				BaseParserRuleContext::copy_from(ctx,WhileStmtContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type AssignStmtContext<'input> = BaseParserRuleContext<'input,AssignStmtContextExt<'input>>;

pub trait AssignStmtContextAttrs<'input>: TaurusParserContext<'input>{
	fn assign(&self) -> Option<Rc<AssignContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token SEMICOLON
	/// Returns `None` if there is no child corresponding to token SEMICOLON
	fn SEMICOLON(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(SEMICOLON, 0)
	}
}

impl<'input> AssignStmtContextAttrs<'input> for AssignStmtContext<'input>{}

pub struct AssignStmtContextExt<'input>{
	base:StmtContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{AssignStmtContextExt<'a>}

impl<'input> TaurusParserContext<'input> for AssignStmtContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for AssignStmtContext<'input>{
	fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_AssignStmt(self);
	}
}

impl<'input> CustomRuleContext<'input> for AssignStmtContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_stmt }
	//fn type_rule_index() -> usize where Self: Sized { RULE_stmt }
}

impl<'input> Borrow<StmtContextExt<'input>> for AssignStmtContext<'input>{
	fn borrow(&self) -> &StmtContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<StmtContextExt<'input>> for AssignStmtContext<'input>{
	fn borrow_mut(&mut self) -> &mut StmtContextExt<'input> { &mut self.base }
}

impl<'input> StmtContextAttrs<'input> for AssignStmtContext<'input> {}

impl<'input> AssignStmtContextExt<'input>{
	fn new(ctx: &dyn StmtContextAttrs<'input>) -> Rc<StmtContextAll<'input>>  {
		Rc::new(
			StmtContextAll::AssignStmtContext(
				BaseParserRuleContext::copy_from(ctx,AssignStmtContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type BreakStmtContext<'input> = BaseParserRuleContext<'input,BreakStmtContextExt<'input>>;

pub trait BreakStmtContextAttrs<'input>: TaurusParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token BREAK
	/// Returns `None` if there is no child corresponding to token BREAK
	fn BREAK(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(BREAK, 0)
	}
	/// Retrieves first TerminalNode corresponding to token SEMICOLON
	/// Returns `None` if there is no child corresponding to token SEMICOLON
	fn SEMICOLON(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(SEMICOLON, 0)
	}
}

impl<'input> BreakStmtContextAttrs<'input> for BreakStmtContext<'input>{}

pub struct BreakStmtContextExt<'input>{
	base:StmtContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{BreakStmtContextExt<'a>}

impl<'input> TaurusParserContext<'input> for BreakStmtContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for BreakStmtContext<'input>{
	fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_BreakStmt(self);
	}
}

impl<'input> CustomRuleContext<'input> for BreakStmtContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_stmt }
	//fn type_rule_index() -> usize where Self: Sized { RULE_stmt }
}

impl<'input> Borrow<StmtContextExt<'input>> for BreakStmtContext<'input>{
	fn borrow(&self) -> &StmtContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<StmtContextExt<'input>> for BreakStmtContext<'input>{
	fn borrow_mut(&mut self) -> &mut StmtContextExt<'input> { &mut self.base }
}

impl<'input> StmtContextAttrs<'input> for BreakStmtContext<'input> {}

impl<'input> BreakStmtContextExt<'input>{
	fn new(ctx: &dyn StmtContextAttrs<'input>) -> Rc<StmtContextAll<'input>>  {
		Rc::new(
			StmtContextAll::BreakStmtContext(
				BaseParserRuleContext::copy_from(ctx,BreakStmtContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type BlockStmtContext<'input> = BaseParserRuleContext<'input,BlockStmtContextExt<'input>>;

pub trait BlockStmtContextAttrs<'input>: TaurusParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token LBRACE
	/// Returns `None` if there is no child corresponding to token LBRACE
	fn LBRACE(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(LBRACE, 0)
	}
	/// Retrieves first TerminalNode corresponding to token RBRACE
	/// Returns `None` if there is no child corresponding to token RBRACE
	fn RBRACE(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(RBRACE, 0)
	}
	fn declStmt_all(&self) ->  Vec<Rc<DeclStmtContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn declStmt(&self, i: usize) -> Option<Rc<DeclStmtContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
}

impl<'input> BlockStmtContextAttrs<'input> for BlockStmtContext<'input>{}

pub struct BlockStmtContextExt<'input>{
	base:StmtContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{BlockStmtContextExt<'a>}

impl<'input> TaurusParserContext<'input> for BlockStmtContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for BlockStmtContext<'input>{
	fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_BlockStmt(self);
	}
}

impl<'input> CustomRuleContext<'input> for BlockStmtContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_stmt }
	//fn type_rule_index() -> usize where Self: Sized { RULE_stmt }
}

impl<'input> Borrow<StmtContextExt<'input>> for BlockStmtContext<'input>{
	fn borrow(&self) -> &StmtContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<StmtContextExt<'input>> for BlockStmtContext<'input>{
	fn borrow_mut(&mut self) -> &mut StmtContextExt<'input> { &mut self.base }
}

impl<'input> StmtContextAttrs<'input> for BlockStmtContext<'input> {}

impl<'input> BlockStmtContextExt<'input>{
	fn new(ctx: &dyn StmtContextAttrs<'input>) -> Rc<StmtContextAll<'input>>  {
		Rc::new(
			StmtContextAll::BlockStmtContext(
				BaseParserRuleContext::copy_from(ctx,BlockStmtContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type EmptyStmtContext<'input> = BaseParserRuleContext<'input,EmptyStmtContextExt<'input>>;

pub trait EmptyStmtContextAttrs<'input>: TaurusParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token SEMICOLON
	/// Returns `None` if there is no child corresponding to token SEMICOLON
	fn SEMICOLON(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(SEMICOLON, 0)
	}
}

impl<'input> EmptyStmtContextAttrs<'input> for EmptyStmtContext<'input>{}

pub struct EmptyStmtContextExt<'input>{
	base:StmtContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{EmptyStmtContextExt<'a>}

impl<'input> TaurusParserContext<'input> for EmptyStmtContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for EmptyStmtContext<'input>{
	fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_EmptyStmt(self);
	}
}

impl<'input> CustomRuleContext<'input> for EmptyStmtContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_stmt }
	//fn type_rule_index() -> usize where Self: Sized { RULE_stmt }
}

impl<'input> Borrow<StmtContextExt<'input>> for EmptyStmtContext<'input>{
	fn borrow(&self) -> &StmtContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<StmtContextExt<'input>> for EmptyStmtContext<'input>{
	fn borrow_mut(&mut self) -> &mut StmtContextExt<'input> { &mut self.base }
}

impl<'input> StmtContextAttrs<'input> for EmptyStmtContext<'input> {}

impl<'input> EmptyStmtContextExt<'input>{
	fn new(ctx: &dyn StmtContextAttrs<'input>) -> Rc<StmtContextAll<'input>>  {
		Rc::new(
			StmtContextAll::EmptyStmtContext(
				BaseParserRuleContext::copy_from(ctx,EmptyStmtContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type ContStmtContext<'input> = BaseParserRuleContext<'input,ContStmtContextExt<'input>>;

pub trait ContStmtContextAttrs<'input>: TaurusParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token CONTINUE
	/// Returns `None` if there is no child corresponding to token CONTINUE
	fn CONTINUE(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(CONTINUE, 0)
	}
	/// Retrieves first TerminalNode corresponding to token SEMICOLON
	/// Returns `None` if there is no child corresponding to token SEMICOLON
	fn SEMICOLON(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(SEMICOLON, 0)
	}
}

impl<'input> ContStmtContextAttrs<'input> for ContStmtContext<'input>{}

pub struct ContStmtContextExt<'input>{
	base:StmtContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{ContStmtContextExt<'a>}

impl<'input> TaurusParserContext<'input> for ContStmtContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for ContStmtContext<'input>{
	fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_ContStmt(self);
	}
}

impl<'input> CustomRuleContext<'input> for ContStmtContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_stmt }
	//fn type_rule_index() -> usize where Self: Sized { RULE_stmt }
}

impl<'input> Borrow<StmtContextExt<'input>> for ContStmtContext<'input>{
	fn borrow(&self) -> &StmtContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<StmtContextExt<'input>> for ContStmtContext<'input>{
	fn borrow_mut(&mut self) -> &mut StmtContextExt<'input> { &mut self.base }
}

impl<'input> StmtContextAttrs<'input> for ContStmtContext<'input> {}

impl<'input> ContStmtContextExt<'input>{
	fn new(ctx: &dyn StmtContextAttrs<'input>) -> Rc<StmtContextAll<'input>>  {
		Rc::new(
			StmtContextAll::ContStmtContext(
				BaseParserRuleContext::copy_from(ctx,ContStmtContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type AssertStmtContext<'input> = BaseParserRuleContext<'input,AssertStmtContextExt<'input>>;

pub trait AssertStmtContextAttrs<'input>: TaurusParserContext<'input>{
	fn assertion(&self) -> Option<Rc<AssertionContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> AssertStmtContextAttrs<'input> for AssertStmtContext<'input>{}

pub struct AssertStmtContextExt<'input>{
	base:StmtContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{AssertStmtContextExt<'a>}

impl<'input> TaurusParserContext<'input> for AssertStmtContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for AssertStmtContext<'input>{
	fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_AssertStmt(self);
	}
}

impl<'input> CustomRuleContext<'input> for AssertStmtContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_stmt }
	//fn type_rule_index() -> usize where Self: Sized { RULE_stmt }
}

impl<'input> Borrow<StmtContextExt<'input>> for AssertStmtContext<'input>{
	fn borrow(&self) -> &StmtContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<StmtContextExt<'input>> for AssertStmtContext<'input>{
	fn borrow_mut(&mut self) -> &mut StmtContextExt<'input> { &mut self.base }
}

impl<'input> StmtContextAttrs<'input> for AssertStmtContext<'input> {}

impl<'input> AssertStmtContextExt<'input>{
	fn new(ctx: &dyn StmtContextAttrs<'input>) -> Rc<StmtContextAll<'input>>  {
		Rc::new(
			StmtContextAll::AssertStmtContext(
				BaseParserRuleContext::copy_from(ctx,AssertStmtContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type ForStmtContext<'input> = BaseParserRuleContext<'input,ForStmtContextExt<'input>>;

pub trait ForStmtContextAttrs<'input>: TaurusParserContext<'input>{
	fn loopAnnot(&self) -> Option<Rc<LoopAnnotContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token FOR
	/// Returns `None` if there is no child corresponding to token FOR
	fn FOR(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(FOR, 0)
	}
	/// Retrieves first TerminalNode corresponding to token LPAR
	/// Returns `None` if there is no child corresponding to token LPAR
	fn LPAR(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(LPAR, 0)
	}
	/// Retrieves all `TerminalNode`s corresponding to token SEMICOLON in current rule
	fn SEMICOLON_all(&self) -> Vec<Rc<TerminalNode<'input,TaurusParserContextType>>>  where Self:Sized{
		self.children_of_type()
	}
	/// Retrieves 'i's TerminalNode corresponding to token SEMICOLON, starting from 0.
	/// Returns `None` if number of children corresponding to token SEMICOLON is less or equal than `i`.
	fn SEMICOLON(&self, i: usize) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(SEMICOLON, i)
	}
	/// Retrieves first TerminalNode corresponding to token RPAR
	/// Returns `None` if there is no child corresponding to token RPAR
	fn RPAR(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(RPAR, 0)
	}
	fn stmt_all(&self) ->  Vec<Rc<StmtContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn stmt(&self, i: usize) -> Option<Rc<StmtContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
	fn forInit(&self) -> Option<Rc<ForInitContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> ForStmtContextAttrs<'input> for ForStmtContext<'input>{}

pub struct ForStmtContextExt<'input>{
	base:StmtContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{ForStmtContextExt<'a>}

impl<'input> TaurusParserContext<'input> for ForStmtContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for ForStmtContext<'input>{
	fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_ForStmt(self);
	}
}

impl<'input> CustomRuleContext<'input> for ForStmtContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_stmt }
	//fn type_rule_index() -> usize where Self: Sized { RULE_stmt }
}

impl<'input> Borrow<StmtContextExt<'input>> for ForStmtContext<'input>{
	fn borrow(&self) -> &StmtContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<StmtContextExt<'input>> for ForStmtContext<'input>{
	fn borrow_mut(&mut self) -> &mut StmtContextExt<'input> { &mut self.base }
}

impl<'input> StmtContextAttrs<'input> for ForStmtContext<'input> {}

impl<'input> ForStmtContextExt<'input>{
	fn new(ctx: &dyn StmtContextAttrs<'input>) -> Rc<StmtContextAll<'input>>  {
		Rc::new(
			StmtContextAll::ForStmtContext(
				BaseParserRuleContext::copy_from(ctx,ForStmtContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type ReturnStmtContext<'input> = BaseParserRuleContext<'input,ReturnStmtContextExt<'input>>;

pub trait ReturnStmtContextAttrs<'input>: TaurusParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token RETURN
	/// Returns `None` if there is no child corresponding to token RETURN
	fn RETURN(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(RETURN, 0)
	}
	/// Retrieves first TerminalNode corresponding to token SEMICOLON
	/// Returns `None` if there is no child corresponding to token SEMICOLON
	fn SEMICOLON(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(SEMICOLON, 0)
	}
	fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> ReturnStmtContextAttrs<'input> for ReturnStmtContext<'input>{}

pub struct ReturnStmtContextExt<'input>{
	base:StmtContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{ReturnStmtContextExt<'a>}

impl<'input> TaurusParserContext<'input> for ReturnStmtContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for ReturnStmtContext<'input>{
	fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_ReturnStmt(self);
	}
}

impl<'input> CustomRuleContext<'input> for ReturnStmtContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_stmt }
	//fn type_rule_index() -> usize where Self: Sized { RULE_stmt }
}

impl<'input> Borrow<StmtContextExt<'input>> for ReturnStmtContext<'input>{
	fn borrow(&self) -> &StmtContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<StmtContextExt<'input>> for ReturnStmtContext<'input>{
	fn borrow_mut(&mut self) -> &mut StmtContextExt<'input> { &mut self.base }
}

impl<'input> StmtContextAttrs<'input> for ReturnStmtContext<'input> {}

impl<'input> ReturnStmtContextExt<'input>{
	fn new(ctx: &dyn StmtContextAttrs<'input>) -> Rc<StmtContextAll<'input>>  {
		Rc::new(
			StmtContextAll::ReturnStmtContext(
				BaseParserRuleContext::copy_from(ctx,ReturnStmtContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type DoStmtContext<'input> = BaseParserRuleContext<'input,DoStmtContextExt<'input>>;

pub trait DoStmtContextAttrs<'input>: TaurusParserContext<'input>{
	fn loopAnnot(&self) -> Option<Rc<LoopAnnotContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token DO
	/// Returns `None` if there is no child corresponding to token DO
	fn DO(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(DO, 0)
	}
	fn stmt(&self) -> Option<Rc<StmtContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token WHILE
	/// Returns `None` if there is no child corresponding to token WHILE
	fn WHILE(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(WHILE, 0)
	}
	/// Retrieves first TerminalNode corresponding to token LPAR
	/// Returns `None` if there is no child corresponding to token LPAR
	fn LPAR(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(LPAR, 0)
	}
	fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token RPAR
	/// Returns `None` if there is no child corresponding to token RPAR
	fn RPAR(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(RPAR, 0)
	}
}

impl<'input> DoStmtContextAttrs<'input> for DoStmtContext<'input>{}

pub struct DoStmtContextExt<'input>{
	base:StmtContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{DoStmtContextExt<'a>}

impl<'input> TaurusParserContext<'input> for DoStmtContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for DoStmtContext<'input>{
	fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_DoStmt(self);
	}
}

impl<'input> CustomRuleContext<'input> for DoStmtContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_stmt }
	//fn type_rule_index() -> usize where Self: Sized { RULE_stmt }
}

impl<'input> Borrow<StmtContextExt<'input>> for DoStmtContext<'input>{
	fn borrow(&self) -> &StmtContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<StmtContextExt<'input>> for DoStmtContext<'input>{
	fn borrow_mut(&mut self) -> &mut StmtContextExt<'input> { &mut self.base }
}

impl<'input> StmtContextAttrs<'input> for DoStmtContext<'input> {}

impl<'input> DoStmtContextExt<'input>{
	fn new(ctx: &dyn StmtContextAttrs<'input>) -> Rc<StmtContextAll<'input>>  {
		Rc::new(
			StmtContextAll::DoStmtContext(
				BaseParserRuleContext::copy_from(ctx,DoStmtContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I, H> TaurusParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn stmt(&mut self,)
	-> Result<Rc<StmtContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = StmtContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 24, RULE_stmt);
        let mut _localctx: Rc<StmtContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(260);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(19,&mut recog.base)? {
				1 =>{
					let tmp = EmptyStmtContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1);
					_localctx = tmp;
					{
					recog.base.set_state(194);
					recog.base.match_token(SEMICOLON,&mut recog.err_handler)?;

					}
				}
			,
				2 =>{
					let tmp = ExprStmtContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2);
					_localctx = tmp;
					{
					/*InvokeRule expr*/
					recog.base.set_state(195);
					recog.expr_rec(0)?;

					recog.base.set_state(196);
					recog.base.match_token(SEMICOLON,&mut recog.err_handler)?;

					}
				}
			,
				3 =>{
					let tmp = AssignStmtContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 3);
					_localctx = tmp;
					{
					/*InvokeRule assign*/
					recog.base.set_state(198);
					recog.assign()?;

					recog.base.set_state(199);
					recog.base.match_token(SEMICOLON,&mut recog.err_handler)?;

					}
				}
			,
				4 =>{
					let tmp = IfStmtContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 4);
					_localctx = tmp;
					{
					recog.base.set_state(201);
					recog.base.match_token(IF,&mut recog.err_handler)?;

					recog.base.set_state(202);
					recog.base.match_token(LPAR,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(203);
					recog.expr_rec(0)?;

					recog.base.set_state(204);
					recog.base.match_token(RPAR,&mut recog.err_handler)?;

					/*InvokeRule stmt*/
					recog.base.set_state(205);
					recog.stmt()?;

					recog.base.set_state(208);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(13,&mut recog.base)? {
						x if x == 1=>{
							{
							recog.base.set_state(206);
							recog.base.match_token(ELSE,&mut recog.err_handler)?;

							/*InvokeRule stmt*/
							recog.base.set_state(207);
							recog.stmt()?;

							}
						}

						_ => {}
					}
					}
				}
			,
				5 =>{
					let tmp = WhileStmtContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 5);
					_localctx = tmp;
					{
					/*InvokeRule loopAnnot*/
					recog.base.set_state(210);
					recog.loopAnnot()?;

					recog.base.set_state(211);
					recog.base.match_token(WHILE,&mut recog.err_handler)?;

					recog.base.set_state(212);
					recog.base.match_token(LPAR,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(213);
					recog.expr_rec(0)?;

					recog.base.set_state(214);
					recog.base.match_token(RPAR,&mut recog.err_handler)?;

					/*InvokeRule stmt*/
					recog.base.set_state(215);
					recog.stmt()?;

					}
				}
			,
				6 =>{
					let tmp = DoStmtContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 6);
					_localctx = tmp;
					{
					/*InvokeRule loopAnnot*/
					recog.base.set_state(217);
					recog.loopAnnot()?;

					recog.base.set_state(218);
					recog.base.match_token(DO,&mut recog.err_handler)?;

					/*InvokeRule stmt*/
					recog.base.set_state(219);
					recog.stmt()?;

					recog.base.set_state(220);
					recog.base.match_token(WHILE,&mut recog.err_handler)?;

					recog.base.set_state(221);
					recog.base.match_token(LPAR,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(222);
					recog.expr_rec(0)?;

					recog.base.set_state(223);
					recog.base.match_token(RPAR,&mut recog.err_handler)?;

					}
				}
			,
				7 =>{
					let tmp = ForStmtContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 7);
					_localctx = tmp;
					{
					/*InvokeRule loopAnnot*/
					recog.base.set_state(225);
					recog.loopAnnot()?;

					recog.base.set_state(226);
					recog.base.match_token(FOR,&mut recog.err_handler)?;

					recog.base.set_state(227);
					recog.base.match_token(LPAR,&mut recog.err_handler)?;

					recog.base.set_state(229);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << STRUCT) | (1usize << INT) | (1usize << FLOAT))) != 0) || _la==IDENT {
						{
						/*InvokeRule forInit*/
						recog.base.set_state(228);
						recog.forInit()?;

						}
					}

					recog.base.set_state(231);
					recog.base.match_token(SEMICOLON,&mut recog.err_handler)?;

					recog.base.set_state(233);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if ((((_la - 3)) & !0x3f) == 0 && ((1usize << (_la - 3)) & ((1usize << (LPAR - 3)) | (1usize << (MINUS - 3)) | (1usize << (NOT - 3)))) != 0) || ((((_la - 38)) & !0x3f) == 0 && ((1usize << (_la - 38)) & ((1usize << (EXPR_TRUE - 38)) | (1usize << (EXPR_FALSE - 38)) | (1usize << (INT_CONSTANT - 38)) | (1usize << (FLOAT_CONSTANT - 38)) | (1usize << (IDENT - 38)))) != 0) {
						{
						/*InvokeRule expr*/
						recog.base.set_state(232);
						recog.expr_rec(0)?;

						}
					}

					recog.base.set_state(235);
					recog.base.match_token(SEMICOLON,&mut recog.err_handler)?;

					recog.base.set_state(237);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << LPAR) | (1usize << LBRACE) | (1usize << SEMICOLON) | (1usize << IF) | (1usize << BREAK) | (1usize << CONTINUE) | (1usize << RETURN) | (1usize << MINUS))) != 0) || ((((_la - 34)) & !0x3f) == 0 && ((1usize << (_la - 34)) & ((1usize << (NOT - 34)) | (1usize << (EXPR_TRUE - 34)) | (1usize << (EXPR_FALSE - 34)) | (1usize << (INT_CONSTANT - 34)) | (1usize << (FLOAT_CONSTANT - 34)))) != 0) || ((((_la - 66)) & !0x3f) == 0 && ((1usize << (_la - 66)) & ((1usize << (IDENT - 66)) | (1usize << (ANNOT_START - 66)) | (1usize << (LINE_ANNOT_START - 66)))) != 0) {
						{
						/*InvokeRule stmt*/
						recog.base.set_state(236);
						recog.stmt()?;

						}
					}

					recog.base.set_state(239);
					recog.base.match_token(RPAR,&mut recog.err_handler)?;

					/*InvokeRule stmt*/
					recog.base.set_state(240);
					recog.stmt()?;

					}
				}
			,
				8 =>{
					let tmp = BreakStmtContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 8);
					_localctx = tmp;
					{
					recog.base.set_state(242);
					recog.base.match_token(BREAK,&mut recog.err_handler)?;

					recog.base.set_state(243);
					recog.base.match_token(SEMICOLON,&mut recog.err_handler)?;

					}
				}
			,
				9 =>{
					let tmp = ContStmtContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 9);
					_localctx = tmp;
					{
					recog.base.set_state(244);
					recog.base.match_token(CONTINUE,&mut recog.err_handler)?;

					recog.base.set_state(245);
					recog.base.match_token(SEMICOLON,&mut recog.err_handler)?;

					}
				}
			,
				10 =>{
					let tmp = ReturnStmtContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 10);
					_localctx = tmp;
					{
					recog.base.set_state(246);
					recog.base.match_token(RETURN,&mut recog.err_handler)?;

					recog.base.set_state(248);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if ((((_la - 3)) & !0x3f) == 0 && ((1usize << (_la - 3)) & ((1usize << (LPAR - 3)) | (1usize << (MINUS - 3)) | (1usize << (NOT - 3)))) != 0) || ((((_la - 38)) & !0x3f) == 0 && ((1usize << (_la - 38)) & ((1usize << (EXPR_TRUE - 38)) | (1usize << (EXPR_FALSE - 38)) | (1usize << (INT_CONSTANT - 38)) | (1usize << (FLOAT_CONSTANT - 38)) | (1usize << (IDENT - 38)))) != 0) {
						{
						/*InvokeRule expr*/
						recog.base.set_state(247);
						recog.expr_rec(0)?;

						}
					}

					recog.base.set_state(250);
					recog.base.match_token(SEMICOLON,&mut recog.err_handler)?;

					}
				}
			,
				11 =>{
					let tmp = AssertStmtContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 11);
					_localctx = tmp;
					{
					/*InvokeRule assertion*/
					recog.base.set_state(251);
					recog.assertion()?;

					}
				}
			,
				12 =>{
					let tmp = BlockStmtContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 12);
					_localctx = tmp;
					{
					recog.base.set_state(252);
					recog.base.match_token(LBRACE,&mut recog.err_handler)?;

					recog.base.set_state(256);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while (((_la) & !0x3f) == 0 && ((1usize << _la) & ((1usize << STRUCT) | (1usize << LPAR) | (1usize << LBRACE) | (1usize << SEMICOLON) | (1usize << INT) | (1usize << FLOAT) | (1usize << IF) | (1usize << BREAK) | (1usize << CONTINUE) | (1usize << RETURN) | (1usize << MINUS))) != 0) || ((((_la - 34)) & !0x3f) == 0 && ((1usize << (_la - 34)) & ((1usize << (NOT - 34)) | (1usize << (EXPR_TRUE - 34)) | (1usize << (EXPR_FALSE - 34)) | (1usize << (INT_CONSTANT - 34)) | (1usize << (FLOAT_CONSTANT - 34)))) != 0) || ((((_la - 66)) & !0x3f) == 0 && ((1usize << (_la - 66)) & ((1usize << (IDENT - 66)) | (1usize << (ANNOT_START - 66)) | (1usize << (LINE_ANNOT_START - 66)))) != 0) {
						{
						{
						/*InvokeRule declStmt*/
						recog.base.set_state(253);
						recog.declStmt()?;

						}
						}
						recog.base.set_state(258);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					recog.base.set_state(259);
					recog.base.match_token(RBRACE,&mut recog.err_handler)?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- forInit ----------------
#[derive(Debug)]
pub enum ForInitContextAll<'input>{
	ForInitLocalVarContext(ForInitLocalVarContext<'input>),
	ForInitAssignContext(ForInitAssignContext<'input>),
Error(ForInitContext<'input>)
}
antlr_rust::tid!{ForInitContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for ForInitContextAll<'input>{}

impl<'input> TaurusParserContext<'input> for ForInitContextAll<'input>{}

impl<'input> Deref for ForInitContextAll<'input>{
	type Target = dyn ForInitContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use ForInitContextAll::*;
		match self{
			ForInitLocalVarContext(inner) => inner,
			ForInitAssignContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for ForInitContextAll<'input>{
    fn enter(&self, listener: &mut (dyn TaurusListener<'input> + 'a)) { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn TaurusListener<'input> + 'a)) { self.deref().exit(listener) }
}



pub type ForInitContext<'input> = BaseParserRuleContext<'input,ForInitContextExt<'input>>;

#[derive(Clone)]
pub struct ForInitContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TaurusParserContext<'input> for ForInitContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for ForInitContext<'input>{
}

impl<'input> CustomRuleContext<'input> for ForInitContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_forInit }
	//fn type_rule_index() -> usize where Self: Sized { RULE_forInit }
}
antlr_rust::tid!{ForInitContextExt<'a>}

impl<'input> ForInitContextExt<'input>{
	fn new(parent: Option<Rc<dyn TaurusParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ForInitContextAll<'input>> {
		Rc::new(
		ForInitContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ForInitContextExt{
				ph:PhantomData
			}),
		)
		)
	}
}

pub trait ForInitContextAttrs<'input>: TaurusParserContext<'input> + BorrowMut<ForInitContextExt<'input>>{


}

impl<'input> ForInitContextAttrs<'input> for ForInitContext<'input>{}

pub type ForInitLocalVarContext<'input> = BaseParserRuleContext<'input,ForInitLocalVarContextExt<'input>>;

pub trait ForInitLocalVarContextAttrs<'input>: TaurusParserContext<'input>{
	fn localVar(&self) -> Option<Rc<LocalVarContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token ASSIGN
	/// Returns `None` if there is no child corresponding to token ASSIGN
	fn ASSIGN(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(ASSIGN, 0)
	}
	fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> ForInitLocalVarContextAttrs<'input> for ForInitLocalVarContext<'input>{}

pub struct ForInitLocalVarContextExt<'input>{
	base:ForInitContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{ForInitLocalVarContextExt<'a>}

impl<'input> TaurusParserContext<'input> for ForInitLocalVarContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for ForInitLocalVarContext<'input>{
	fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_ForInitLocalVar(self);
	}
}

impl<'input> CustomRuleContext<'input> for ForInitLocalVarContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_forInit }
	//fn type_rule_index() -> usize where Self: Sized { RULE_forInit }
}

impl<'input> Borrow<ForInitContextExt<'input>> for ForInitLocalVarContext<'input>{
	fn borrow(&self) -> &ForInitContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ForInitContextExt<'input>> for ForInitLocalVarContext<'input>{
	fn borrow_mut(&mut self) -> &mut ForInitContextExt<'input> { &mut self.base }
}

impl<'input> ForInitContextAttrs<'input> for ForInitLocalVarContext<'input> {}

impl<'input> ForInitLocalVarContextExt<'input>{
	fn new(ctx: &dyn ForInitContextAttrs<'input>) -> Rc<ForInitContextAll<'input>>  {
		Rc::new(
			ForInitContextAll::ForInitLocalVarContext(
				BaseParserRuleContext::copy_from(ctx,ForInitLocalVarContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type ForInitAssignContext<'input> = BaseParserRuleContext<'input,ForInitAssignContextExt<'input>>;

pub trait ForInitAssignContextAttrs<'input>: TaurusParserContext<'input>{
	fn assign(&self) -> Option<Rc<AssignContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> ForInitAssignContextAttrs<'input> for ForInitAssignContext<'input>{}

pub struct ForInitAssignContextExt<'input>{
	base:ForInitContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{ForInitAssignContextExt<'a>}

impl<'input> TaurusParserContext<'input> for ForInitAssignContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for ForInitAssignContext<'input>{
	fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_ForInitAssign(self);
	}
}

impl<'input> CustomRuleContext<'input> for ForInitAssignContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_forInit }
	//fn type_rule_index() -> usize where Self: Sized { RULE_forInit }
}

impl<'input> Borrow<ForInitContextExt<'input>> for ForInitAssignContext<'input>{
	fn borrow(&self) -> &ForInitContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ForInitContextExt<'input>> for ForInitAssignContext<'input>{
	fn borrow_mut(&mut self) -> &mut ForInitContextExt<'input> { &mut self.base }
}

impl<'input> ForInitContextAttrs<'input> for ForInitAssignContext<'input> {}

impl<'input> ForInitAssignContextExt<'input>{
	fn new(ctx: &dyn ForInitContextAttrs<'input>) -> Rc<ForInitContextAll<'input>>  {
		Rc::new(
			ForInitContextAll::ForInitAssignContext(
				BaseParserRuleContext::copy_from(ctx,ForInitAssignContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I, H> TaurusParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn forInit(&mut self,)
	-> Result<Rc<ForInitContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ForInitContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 26, RULE_forInit);
        let mut _localctx: Rc<ForInitContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(268);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 STRUCT | INT | FLOAT 
				=> {
					let tmp = ForInitLocalVarContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1);
					_localctx = tmp;
					{
					/*InvokeRule localVar*/
					recog.base.set_state(262);
					recog.localVar()?;

					recog.base.set_state(265);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==ASSIGN {
						{
						recog.base.set_state(263);
						recog.base.match_token(ASSIGN,&mut recog.err_handler)?;

						/*InvokeRule expr*/
						recog.base.set_state(264);
						recog.expr_rec(0)?;

						}
					}

					}
				}

			 IDENT 
				=> {
					let tmp = ForInitAssignContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2);
					_localctx = tmp;
					{
					/*InvokeRule assign*/
					recog.base.set_state(267);
					recog.assign()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- assign ----------------
#[derive(Debug)]
pub enum AssignContextAll<'input>{
	MemAssignContext(MemAssignContext<'input>),
	VarAssignContext(VarAssignContext<'input>),
	SubAssignContext(SubAssignContext<'input>),
Error(AssignContext<'input>)
}
antlr_rust::tid!{AssignContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for AssignContextAll<'input>{}

impl<'input> TaurusParserContext<'input> for AssignContextAll<'input>{}

impl<'input> Deref for AssignContextAll<'input>{
	type Target = dyn AssignContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use AssignContextAll::*;
		match self{
			MemAssignContext(inner) => inner,
			VarAssignContext(inner) => inner,
			SubAssignContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for AssignContextAll<'input>{
    fn enter(&self, listener: &mut (dyn TaurusListener<'input> + 'a)) { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn TaurusListener<'input> + 'a)) { self.deref().exit(listener) }
}



pub type AssignContext<'input> = BaseParserRuleContext<'input,AssignContextExt<'input>>;

#[derive(Clone)]
pub struct AssignContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TaurusParserContext<'input> for AssignContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for AssignContext<'input>{
}

impl<'input> CustomRuleContext<'input> for AssignContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_assign }
	//fn type_rule_index() -> usize where Self: Sized { RULE_assign }
}
antlr_rust::tid!{AssignContextExt<'a>}

impl<'input> AssignContextExt<'input>{
	fn new(parent: Option<Rc<dyn TaurusParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<AssignContextAll<'input>> {
		Rc::new(
		AssignContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AssignContextExt{
				ph:PhantomData
			}),
		)
		)
	}
}

pub trait AssignContextAttrs<'input>: TaurusParserContext<'input> + BorrowMut<AssignContextExt<'input>>{


}

impl<'input> AssignContextAttrs<'input> for AssignContext<'input>{}

pub type MemAssignContext<'input> = BaseParserRuleContext<'input,MemAssignContextExt<'input>>;

pub trait MemAssignContextAttrs<'input>: TaurusParserContext<'input>{
	/// Retrieves all `TerminalNode`s corresponding to token IDENT in current rule
	fn IDENT_all(&self) -> Vec<Rc<TerminalNode<'input,TaurusParserContextType>>>  where Self:Sized{
		self.children_of_type()
	}
	/// Retrieves 'i's TerminalNode corresponding to token IDENT, starting from 0.
	/// Returns `None` if number of children corresponding to token IDENT is less or equal than `i`.
	fn IDENT(&self, i: usize) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(IDENT, i)
	}
	/// Retrieves first TerminalNode corresponding to token PERIOD
	/// Returns `None` if there is no child corresponding to token PERIOD
	fn PERIOD(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(PERIOD, 0)
	}
	/// Retrieves first TerminalNode corresponding to token ASSIGN
	/// Returns `None` if there is no child corresponding to token ASSIGN
	fn ASSIGN(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(ASSIGN, 0)
	}
	fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> MemAssignContextAttrs<'input> for MemAssignContext<'input>{}

pub struct MemAssignContextExt<'input>{
	base:AssignContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{MemAssignContextExt<'a>}

impl<'input> TaurusParserContext<'input> for MemAssignContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for MemAssignContext<'input>{
	fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_MemAssign(self);
	}
}

impl<'input> CustomRuleContext<'input> for MemAssignContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_assign }
	//fn type_rule_index() -> usize where Self: Sized { RULE_assign }
}

impl<'input> Borrow<AssignContextExt<'input>> for MemAssignContext<'input>{
	fn borrow(&self) -> &AssignContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<AssignContextExt<'input>> for MemAssignContext<'input>{
	fn borrow_mut(&mut self) -> &mut AssignContextExt<'input> { &mut self.base }
}

impl<'input> AssignContextAttrs<'input> for MemAssignContext<'input> {}

impl<'input> MemAssignContextExt<'input>{
	fn new(ctx: &dyn AssignContextAttrs<'input>) -> Rc<AssignContextAll<'input>>  {
		Rc::new(
			AssignContextAll::MemAssignContext(
				BaseParserRuleContext::copy_from(ctx,MemAssignContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type VarAssignContext<'input> = BaseParserRuleContext<'input,VarAssignContextExt<'input>>;

pub trait VarAssignContextAttrs<'input>: TaurusParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token IDENT
	/// Returns `None` if there is no child corresponding to token IDENT
	fn IDENT(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(IDENT, 0)
	}
	/// Retrieves first TerminalNode corresponding to token ASSIGN
	/// Returns `None` if there is no child corresponding to token ASSIGN
	fn ASSIGN(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(ASSIGN, 0)
	}
	fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> VarAssignContextAttrs<'input> for VarAssignContext<'input>{}

pub struct VarAssignContextExt<'input>{
	base:AssignContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{VarAssignContextExt<'a>}

impl<'input> TaurusParserContext<'input> for VarAssignContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for VarAssignContext<'input>{
	fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_VarAssign(self);
	}
}

impl<'input> CustomRuleContext<'input> for VarAssignContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_assign }
	//fn type_rule_index() -> usize where Self: Sized { RULE_assign }
}

impl<'input> Borrow<AssignContextExt<'input>> for VarAssignContext<'input>{
	fn borrow(&self) -> &AssignContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<AssignContextExt<'input>> for VarAssignContext<'input>{
	fn borrow_mut(&mut self) -> &mut AssignContextExt<'input> { &mut self.base }
}

impl<'input> AssignContextAttrs<'input> for VarAssignContext<'input> {}

impl<'input> VarAssignContextExt<'input>{
	fn new(ctx: &dyn AssignContextAttrs<'input>) -> Rc<AssignContextAll<'input>>  {
		Rc::new(
			AssignContextAll::VarAssignContext(
				BaseParserRuleContext::copy_from(ctx,VarAssignContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type SubAssignContext<'input> = BaseParserRuleContext<'input,SubAssignContextExt<'input>>;

pub trait SubAssignContextAttrs<'input>: TaurusParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token IDENT
	/// Returns `None` if there is no child corresponding to token IDENT
	fn IDENT(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(IDENT, 0)
	}
	/// Retrieves first TerminalNode corresponding to token LBRACKET
	/// Returns `None` if there is no child corresponding to token LBRACKET
	fn LBRACKET(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(LBRACKET, 0)
	}
	fn expr_all(&self) ->  Vec<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn expr(&self, i: usize) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
	/// Retrieves first TerminalNode corresponding to token RBRACKET
	/// Returns `None` if there is no child corresponding to token RBRACKET
	fn RBRACKET(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(RBRACKET, 0)
	}
	/// Retrieves first TerminalNode corresponding to token ASSIGN
	/// Returns `None` if there is no child corresponding to token ASSIGN
	fn ASSIGN(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(ASSIGN, 0)
	}
}

impl<'input> SubAssignContextAttrs<'input> for SubAssignContext<'input>{}

pub struct SubAssignContextExt<'input>{
	base:AssignContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{SubAssignContextExt<'a>}

impl<'input> TaurusParserContext<'input> for SubAssignContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for SubAssignContext<'input>{
	fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_SubAssign(self);
	}
}

impl<'input> CustomRuleContext<'input> for SubAssignContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_assign }
	//fn type_rule_index() -> usize where Self: Sized { RULE_assign }
}

impl<'input> Borrow<AssignContextExt<'input>> for SubAssignContext<'input>{
	fn borrow(&self) -> &AssignContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<AssignContextExt<'input>> for SubAssignContext<'input>{
	fn borrow_mut(&mut self) -> &mut AssignContextExt<'input> { &mut self.base }
}

impl<'input> AssignContextAttrs<'input> for SubAssignContext<'input> {}

impl<'input> SubAssignContextExt<'input>{
	fn new(ctx: &dyn AssignContextAttrs<'input>) -> Rc<AssignContextAll<'input>>  {
		Rc::new(
			AssignContextAll::SubAssignContext(
				BaseParserRuleContext::copy_from(ctx,SubAssignContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I, H> TaurusParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn assign(&mut self,)
	-> Result<Rc<AssignContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = AssignContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 28, RULE_assign);
        let mut _localctx: Rc<AssignContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(285);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(22,&mut recog.base)? {
				1 =>{
					let tmp = VarAssignContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1);
					_localctx = tmp;
					{
					recog.base.set_state(270);
					recog.base.match_token(IDENT,&mut recog.err_handler)?;

					recog.base.set_state(271);
					recog.base.match_token(ASSIGN,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(272);
					recog.expr_rec(0)?;

					}
				}
			,
				2 =>{
					let tmp = SubAssignContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2);
					_localctx = tmp;
					{
					recog.base.set_state(273);
					recog.base.match_token(IDENT,&mut recog.err_handler)?;

					recog.base.set_state(274);
					recog.base.match_token(LBRACKET,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(275);
					recog.expr_rec(0)?;

					recog.base.set_state(276);
					recog.base.match_token(RBRACKET,&mut recog.err_handler)?;

					recog.base.set_state(277);
					recog.base.match_token(ASSIGN,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(278);
					recog.expr_rec(0)?;

					}
				}
			,
				3 =>{
					let tmp = MemAssignContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 3);
					_localctx = tmp;
					{
					recog.base.set_state(280);
					recog.base.match_token(IDENT,&mut recog.err_handler)?;

					recog.base.set_state(281);
					recog.base.match_token(PERIOD,&mut recog.err_handler)?;

					recog.base.set_state(282);
					recog.base.match_token(IDENT,&mut recog.err_handler)?;

					recog.base.set_state(283);
					recog.base.match_token(ASSIGN,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(284);
					recog.expr_rec(0)?;

					}
				}

				_ => {}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- decl ----------------
pub type DeclContextAll<'input> = DeclContext<'input>;


pub type DeclContext<'input> = BaseParserRuleContext<'input,DeclContextExt<'input>>;

#[derive(Clone)]
pub struct DeclContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TaurusParserContext<'input> for DeclContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for DeclContext<'input>{
		fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_decl(self);
		}fn exit(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
			listener.exit_decl(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for DeclContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_decl }
	//fn type_rule_index() -> usize where Self: Sized { RULE_decl }
}
antlr_rust::tid!{DeclContextExt<'a>}

impl<'input> DeclContextExt<'input>{
	fn new(parent: Option<Rc<dyn TaurusParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<DeclContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,DeclContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait DeclContextAttrs<'input>: TaurusParserContext<'input> + BorrowMut<DeclContextExt<'input>>{

fn localVar(&self) -> Option<Rc<LocalVarContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token SEMICOLON
/// Returns `None` if there is no child corresponding to token SEMICOLON
fn SEMICOLON(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
	self.get_token(SEMICOLON, 0)
}
/// Retrieves first TerminalNode corresponding to token ASSIGN
/// Returns `None` if there is no child corresponding to token ASSIGN
fn ASSIGN(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
	self.get_token(ASSIGN, 0)
}
fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}

}

impl<'input> DeclContextAttrs<'input> for DeclContext<'input>{}

impl<'input, I, H> TaurusParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn decl(&mut self,)
	-> Result<Rc<DeclContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = DeclContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 30, RULE_decl);
        let mut _localctx: Rc<DeclContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule localVar*/
			recog.base.set_state(287);
			recog.localVar()?;

			recog.base.set_state(290);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==ASSIGN {
				{
				recog.base.set_state(288);
				recog.base.match_token(ASSIGN,&mut recog.err_handler)?;

				/*InvokeRule expr*/
				recog.base.set_state(289);
				recog.expr_rec(0)?;

				}
			}

			recog.base.set_state(292);
			recog.base.match_token(SEMICOLON,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- expr ----------------
#[derive(Debug)]
pub enum ExprContextAll<'input>{
	MulExprContext(MulExprContext<'input>),
	AndExprContext(AndExprContext<'input>),
	OrdExprContext(OrdExprContext<'input>),
	IdentExprContext(IdentExprContext<'input>),
	ConstExprContext(ConstExprContext<'input>),
	AddExprContext(AddExprContext<'input>),
	ArrAccessExprContext(ArrAccessExprContext<'input>),
	UnaryExprContext(UnaryExprContext<'input>),
	OrExprContext(OrExprContext<'input>),
	MemExprContext(MemExprContext<'input>),
	ParExprContext(ParExprContext<'input>),
	EqExprContext(EqExprContext<'input>),
	CallExprContext(CallExprContext<'input>),
Error(ExprContext<'input>)
}
antlr_rust::tid!{ExprContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for ExprContextAll<'input>{}

impl<'input> TaurusParserContext<'input> for ExprContextAll<'input>{}

impl<'input> Deref for ExprContextAll<'input>{
	type Target = dyn ExprContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use ExprContextAll::*;
		match self{
			MulExprContext(inner) => inner,
			AndExprContext(inner) => inner,
			OrdExprContext(inner) => inner,
			IdentExprContext(inner) => inner,
			ConstExprContext(inner) => inner,
			AddExprContext(inner) => inner,
			ArrAccessExprContext(inner) => inner,
			UnaryExprContext(inner) => inner,
			OrExprContext(inner) => inner,
			MemExprContext(inner) => inner,
			ParExprContext(inner) => inner,
			EqExprContext(inner) => inner,
			CallExprContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for ExprContextAll<'input>{
    fn enter(&self, listener: &mut (dyn TaurusListener<'input> + 'a)) { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn TaurusListener<'input> + 'a)) { self.deref().exit(listener) }
}



pub type ExprContext<'input> = BaseParserRuleContext<'input,ExprContextExt<'input>>;

#[derive(Clone)]
pub struct ExprContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TaurusParserContext<'input> for ExprContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for ExprContext<'input>{
}

impl<'input> CustomRuleContext<'input> for ExprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expr }
}
antlr_rust::tid!{ExprContextExt<'a>}

impl<'input> ExprContextExt<'input>{
	fn new(parent: Option<Rc<dyn TaurusParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ExprContextAll<'input>> {
		Rc::new(
		ExprContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ExprContextExt{
				ph:PhantomData
			}),
		)
		)
	}
}

pub trait ExprContextAttrs<'input>: TaurusParserContext<'input> + BorrowMut<ExprContextExt<'input>>{


}

impl<'input> ExprContextAttrs<'input> for ExprContext<'input>{}

pub type MulExprContext<'input> = BaseParserRuleContext<'input,MulExprContextExt<'input>>;

pub trait MulExprContextAttrs<'input>: TaurusParserContext<'input>{
	fn expr_all(&self) ->  Vec<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn expr(&self, i: usize) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
	fn mulOp(&self) -> Option<Rc<MulOpContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> MulExprContextAttrs<'input> for MulExprContext<'input>{}

pub struct MulExprContextExt<'input>{
	base:ExprContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{MulExprContextExt<'a>}

impl<'input> TaurusParserContext<'input> for MulExprContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for MulExprContext<'input>{
	fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_MulExpr(self);
	}
}

impl<'input> CustomRuleContext<'input> for MulExprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expr }
}

impl<'input> Borrow<ExprContextExt<'input>> for MulExprContext<'input>{
	fn borrow(&self) -> &ExprContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExprContextExt<'input>> for MulExprContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExprContextExt<'input> { &mut self.base }
}

impl<'input> ExprContextAttrs<'input> for MulExprContext<'input> {}

impl<'input> MulExprContextExt<'input>{
	fn new(ctx: &dyn ExprContextAttrs<'input>) -> Rc<ExprContextAll<'input>>  {
		Rc::new(
			ExprContextAll::MulExprContext(
				BaseParserRuleContext::copy_from(ctx,MulExprContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type AndExprContext<'input> = BaseParserRuleContext<'input,AndExprContextExt<'input>>;

pub trait AndExprContextAttrs<'input>: TaurusParserContext<'input>{
	fn expr_all(&self) ->  Vec<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn expr(&self, i: usize) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
	/// Retrieves first TerminalNode corresponding to token AND
	/// Returns `None` if there is no child corresponding to token AND
	fn AND(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(AND, 0)
	}
}

impl<'input> AndExprContextAttrs<'input> for AndExprContext<'input>{}

pub struct AndExprContextExt<'input>{
	base:ExprContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{AndExprContextExt<'a>}

impl<'input> TaurusParserContext<'input> for AndExprContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for AndExprContext<'input>{
	fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_AndExpr(self);
	}
}

impl<'input> CustomRuleContext<'input> for AndExprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expr }
}

impl<'input> Borrow<ExprContextExt<'input>> for AndExprContext<'input>{
	fn borrow(&self) -> &ExprContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExprContextExt<'input>> for AndExprContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExprContextExt<'input> { &mut self.base }
}

impl<'input> ExprContextAttrs<'input> for AndExprContext<'input> {}

impl<'input> AndExprContextExt<'input>{
	fn new(ctx: &dyn ExprContextAttrs<'input>) -> Rc<ExprContextAll<'input>>  {
		Rc::new(
			ExprContextAll::AndExprContext(
				BaseParserRuleContext::copy_from(ctx,AndExprContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type OrdExprContext<'input> = BaseParserRuleContext<'input,OrdExprContextExt<'input>>;

pub trait OrdExprContextAttrs<'input>: TaurusParserContext<'input>{
	fn expr_all(&self) ->  Vec<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn expr(&self, i: usize) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
	fn ordOp(&self) -> Option<Rc<OrdOpContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> OrdExprContextAttrs<'input> for OrdExprContext<'input>{}

pub struct OrdExprContextExt<'input>{
	base:ExprContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{OrdExprContextExt<'a>}

impl<'input> TaurusParserContext<'input> for OrdExprContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for OrdExprContext<'input>{
	fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_OrdExpr(self);
	}
}

impl<'input> CustomRuleContext<'input> for OrdExprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expr }
}

impl<'input> Borrow<ExprContextExt<'input>> for OrdExprContext<'input>{
	fn borrow(&self) -> &ExprContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExprContextExt<'input>> for OrdExprContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExprContextExt<'input> { &mut self.base }
}

impl<'input> ExprContextAttrs<'input> for OrdExprContext<'input> {}

impl<'input> OrdExprContextExt<'input>{
	fn new(ctx: &dyn ExprContextAttrs<'input>) -> Rc<ExprContextAll<'input>>  {
		Rc::new(
			ExprContextAll::OrdExprContext(
				BaseParserRuleContext::copy_from(ctx,OrdExprContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type IdentExprContext<'input> = BaseParserRuleContext<'input,IdentExprContextExt<'input>>;

pub trait IdentExprContextAttrs<'input>: TaurusParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token IDENT
	/// Returns `None` if there is no child corresponding to token IDENT
	fn IDENT(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(IDENT, 0)
	}
}

impl<'input> IdentExprContextAttrs<'input> for IdentExprContext<'input>{}

pub struct IdentExprContextExt<'input>{
	base:ExprContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{IdentExprContextExt<'a>}

impl<'input> TaurusParserContext<'input> for IdentExprContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for IdentExprContext<'input>{
	fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_IdentExpr(self);
	}
}

impl<'input> CustomRuleContext<'input> for IdentExprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expr }
}

impl<'input> Borrow<ExprContextExt<'input>> for IdentExprContext<'input>{
	fn borrow(&self) -> &ExprContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExprContextExt<'input>> for IdentExprContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExprContextExt<'input> { &mut self.base }
}

impl<'input> ExprContextAttrs<'input> for IdentExprContext<'input> {}

impl<'input> IdentExprContextExt<'input>{
	fn new(ctx: &dyn ExprContextAttrs<'input>) -> Rc<ExprContextAll<'input>>  {
		Rc::new(
			ExprContextAll::IdentExprContext(
				BaseParserRuleContext::copy_from(ctx,IdentExprContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type ConstExprContext<'input> = BaseParserRuleContext<'input,ConstExprContextExt<'input>>;

pub trait ConstExprContextAttrs<'input>: TaurusParserContext<'input>{
	fn constant(&self) -> Option<Rc<ConstantContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> ConstExprContextAttrs<'input> for ConstExprContext<'input>{}

pub struct ConstExprContextExt<'input>{
	base:ExprContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{ConstExprContextExt<'a>}

impl<'input> TaurusParserContext<'input> for ConstExprContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for ConstExprContext<'input>{
	fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_ConstExpr(self);
	}
}

impl<'input> CustomRuleContext<'input> for ConstExprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expr }
}

impl<'input> Borrow<ExprContextExt<'input>> for ConstExprContext<'input>{
	fn borrow(&self) -> &ExprContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExprContextExt<'input>> for ConstExprContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExprContextExt<'input> { &mut self.base }
}

impl<'input> ExprContextAttrs<'input> for ConstExprContext<'input> {}

impl<'input> ConstExprContextExt<'input>{
	fn new(ctx: &dyn ExprContextAttrs<'input>) -> Rc<ExprContextAll<'input>>  {
		Rc::new(
			ExprContextAll::ConstExprContext(
				BaseParserRuleContext::copy_from(ctx,ConstExprContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type AddExprContext<'input> = BaseParserRuleContext<'input,AddExprContextExt<'input>>;

pub trait AddExprContextAttrs<'input>: TaurusParserContext<'input>{
	fn expr_all(&self) ->  Vec<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn expr(&self, i: usize) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
	fn addOp(&self) -> Option<Rc<AddOpContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> AddExprContextAttrs<'input> for AddExprContext<'input>{}

pub struct AddExprContextExt<'input>{
	base:ExprContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{AddExprContextExt<'a>}

impl<'input> TaurusParserContext<'input> for AddExprContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for AddExprContext<'input>{
	fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_AddExpr(self);
	}
}

impl<'input> CustomRuleContext<'input> for AddExprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expr }
}

impl<'input> Borrow<ExprContextExt<'input>> for AddExprContext<'input>{
	fn borrow(&self) -> &ExprContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExprContextExt<'input>> for AddExprContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExprContextExt<'input> { &mut self.base }
}

impl<'input> ExprContextAttrs<'input> for AddExprContext<'input> {}

impl<'input> AddExprContextExt<'input>{
	fn new(ctx: &dyn ExprContextAttrs<'input>) -> Rc<ExprContextAll<'input>>  {
		Rc::new(
			ExprContextAll::AddExprContext(
				BaseParserRuleContext::copy_from(ctx,AddExprContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type ArrAccessExprContext<'input> = BaseParserRuleContext<'input,ArrAccessExprContextExt<'input>>;

pub trait ArrAccessExprContextAttrs<'input>: TaurusParserContext<'input>{
	fn expr_all(&self) ->  Vec<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn expr(&self, i: usize) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
	/// Retrieves first TerminalNode corresponding to token LBRACKET
	/// Returns `None` if there is no child corresponding to token LBRACKET
	fn LBRACKET(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(LBRACKET, 0)
	}
	/// Retrieves first TerminalNode corresponding to token RBRACKET
	/// Returns `None` if there is no child corresponding to token RBRACKET
	fn RBRACKET(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(RBRACKET, 0)
	}
}

impl<'input> ArrAccessExprContextAttrs<'input> for ArrAccessExprContext<'input>{}

pub struct ArrAccessExprContextExt<'input>{
	base:ExprContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{ArrAccessExprContextExt<'a>}

impl<'input> TaurusParserContext<'input> for ArrAccessExprContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for ArrAccessExprContext<'input>{
	fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_ArrAccessExpr(self);
	}
}

impl<'input> CustomRuleContext<'input> for ArrAccessExprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expr }
}

impl<'input> Borrow<ExprContextExt<'input>> for ArrAccessExprContext<'input>{
	fn borrow(&self) -> &ExprContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExprContextExt<'input>> for ArrAccessExprContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExprContextExt<'input> { &mut self.base }
}

impl<'input> ExprContextAttrs<'input> for ArrAccessExprContext<'input> {}

impl<'input> ArrAccessExprContextExt<'input>{
	fn new(ctx: &dyn ExprContextAttrs<'input>) -> Rc<ExprContextAll<'input>>  {
		Rc::new(
			ExprContextAll::ArrAccessExprContext(
				BaseParserRuleContext::copy_from(ctx,ArrAccessExprContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type UnaryExprContext<'input> = BaseParserRuleContext<'input,UnaryExprContextExt<'input>>;

pub trait UnaryExprContextAttrs<'input>: TaurusParserContext<'input>{
	fn unaryOp(&self) -> Option<Rc<UnaryOpContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> UnaryExprContextAttrs<'input> for UnaryExprContext<'input>{}

pub struct UnaryExprContextExt<'input>{
	base:ExprContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{UnaryExprContextExt<'a>}

impl<'input> TaurusParserContext<'input> for UnaryExprContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for UnaryExprContext<'input>{
	fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_UnaryExpr(self);
	}
}

impl<'input> CustomRuleContext<'input> for UnaryExprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expr }
}

impl<'input> Borrow<ExprContextExt<'input>> for UnaryExprContext<'input>{
	fn borrow(&self) -> &ExprContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExprContextExt<'input>> for UnaryExprContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExprContextExt<'input> { &mut self.base }
}

impl<'input> ExprContextAttrs<'input> for UnaryExprContext<'input> {}

impl<'input> UnaryExprContextExt<'input>{
	fn new(ctx: &dyn ExprContextAttrs<'input>) -> Rc<ExprContextAll<'input>>  {
		Rc::new(
			ExprContextAll::UnaryExprContext(
				BaseParserRuleContext::copy_from(ctx,UnaryExprContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type OrExprContext<'input> = BaseParserRuleContext<'input,OrExprContextExt<'input>>;

pub trait OrExprContextAttrs<'input>: TaurusParserContext<'input>{
	fn expr_all(&self) ->  Vec<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn expr(&self, i: usize) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
	/// Retrieves first TerminalNode corresponding to token OR
	/// Returns `None` if there is no child corresponding to token OR
	fn OR(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(OR, 0)
	}
}

impl<'input> OrExprContextAttrs<'input> for OrExprContext<'input>{}

pub struct OrExprContextExt<'input>{
	base:ExprContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{OrExprContextExt<'a>}

impl<'input> TaurusParserContext<'input> for OrExprContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for OrExprContext<'input>{
	fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_OrExpr(self);
	}
}

impl<'input> CustomRuleContext<'input> for OrExprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expr }
}

impl<'input> Borrow<ExprContextExt<'input>> for OrExprContext<'input>{
	fn borrow(&self) -> &ExprContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExprContextExt<'input>> for OrExprContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExprContextExt<'input> { &mut self.base }
}

impl<'input> ExprContextAttrs<'input> for OrExprContext<'input> {}

impl<'input> OrExprContextExt<'input>{
	fn new(ctx: &dyn ExprContextAttrs<'input>) -> Rc<ExprContextAll<'input>>  {
		Rc::new(
			ExprContextAll::OrExprContext(
				BaseParserRuleContext::copy_from(ctx,OrExprContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type MemExprContext<'input> = BaseParserRuleContext<'input,MemExprContextExt<'input>>;

pub trait MemExprContextAttrs<'input>: TaurusParserContext<'input>{
	fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token PERIOD
	/// Returns `None` if there is no child corresponding to token PERIOD
	fn PERIOD(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(PERIOD, 0)
	}
	/// Retrieves first TerminalNode corresponding to token IDENT
	/// Returns `None` if there is no child corresponding to token IDENT
	fn IDENT(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(IDENT, 0)
	}
}

impl<'input> MemExprContextAttrs<'input> for MemExprContext<'input>{}

pub struct MemExprContextExt<'input>{
	base:ExprContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{MemExprContextExt<'a>}

impl<'input> TaurusParserContext<'input> for MemExprContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for MemExprContext<'input>{
	fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_MemExpr(self);
	}
}

impl<'input> CustomRuleContext<'input> for MemExprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expr }
}

impl<'input> Borrow<ExprContextExt<'input>> for MemExprContext<'input>{
	fn borrow(&self) -> &ExprContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExprContextExt<'input>> for MemExprContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExprContextExt<'input> { &mut self.base }
}

impl<'input> ExprContextAttrs<'input> for MemExprContext<'input> {}

impl<'input> MemExprContextExt<'input>{
	fn new(ctx: &dyn ExprContextAttrs<'input>) -> Rc<ExprContextAll<'input>>  {
		Rc::new(
			ExprContextAll::MemExprContext(
				BaseParserRuleContext::copy_from(ctx,MemExprContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type ParExprContext<'input> = BaseParserRuleContext<'input,ParExprContextExt<'input>>;

pub trait ParExprContextAttrs<'input>: TaurusParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token LPAR
	/// Returns `None` if there is no child corresponding to token LPAR
	fn LPAR(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(LPAR, 0)
	}
	fn expr(&self) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token RPAR
	/// Returns `None` if there is no child corresponding to token RPAR
	fn RPAR(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(RPAR, 0)
	}
}

impl<'input> ParExprContextAttrs<'input> for ParExprContext<'input>{}

pub struct ParExprContextExt<'input>{
	base:ExprContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{ParExprContextExt<'a>}

impl<'input> TaurusParserContext<'input> for ParExprContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for ParExprContext<'input>{
	fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_ParExpr(self);
	}
}

impl<'input> CustomRuleContext<'input> for ParExprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expr }
}

impl<'input> Borrow<ExprContextExt<'input>> for ParExprContext<'input>{
	fn borrow(&self) -> &ExprContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExprContextExt<'input>> for ParExprContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExprContextExt<'input> { &mut self.base }
}

impl<'input> ExprContextAttrs<'input> for ParExprContext<'input> {}

impl<'input> ParExprContextExt<'input>{
	fn new(ctx: &dyn ExprContextAttrs<'input>) -> Rc<ExprContextAll<'input>>  {
		Rc::new(
			ExprContextAll::ParExprContext(
				BaseParserRuleContext::copy_from(ctx,ParExprContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type EqExprContext<'input> = BaseParserRuleContext<'input,EqExprContextExt<'input>>;

pub trait EqExprContextAttrs<'input>: TaurusParserContext<'input>{
	fn expr_all(&self) ->  Vec<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn expr(&self, i: usize) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
	fn eqOp(&self) -> Option<Rc<EqOpContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> EqExprContextAttrs<'input> for EqExprContext<'input>{}

pub struct EqExprContextExt<'input>{
	base:ExprContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{EqExprContextExt<'a>}

impl<'input> TaurusParserContext<'input> for EqExprContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for EqExprContext<'input>{
	fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_EqExpr(self);
	}
}

impl<'input> CustomRuleContext<'input> for EqExprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expr }
}

impl<'input> Borrow<ExprContextExt<'input>> for EqExprContext<'input>{
	fn borrow(&self) -> &ExprContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExprContextExt<'input>> for EqExprContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExprContextExt<'input> { &mut self.base }
}

impl<'input> ExprContextAttrs<'input> for EqExprContext<'input> {}

impl<'input> EqExprContextExt<'input>{
	fn new(ctx: &dyn ExprContextAttrs<'input>) -> Rc<ExprContextAll<'input>>  {
		Rc::new(
			ExprContextAll::EqExprContext(
				BaseParserRuleContext::copy_from(ctx,EqExprContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type CallExprContext<'input> = BaseParserRuleContext<'input,CallExprContextExt<'input>>;

pub trait CallExprContextAttrs<'input>: TaurusParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token IDENT
	/// Returns `None` if there is no child corresponding to token IDENT
	fn IDENT(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(IDENT, 0)
	}
	/// Retrieves first TerminalNode corresponding to token LPAR
	/// Returns `None` if there is no child corresponding to token LPAR
	fn LPAR(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(LPAR, 0)
	}
	/// Retrieves first TerminalNode corresponding to token RPAR
	/// Returns `None` if there is no child corresponding to token RPAR
	fn RPAR(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(RPAR, 0)
	}
	fn expr_all(&self) ->  Vec<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn expr(&self, i: usize) -> Option<Rc<ExprContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
	/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
	fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,TaurusParserContextType>>>  where Self:Sized{
		self.children_of_type()
	}
	/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
	/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
	fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(COMMA, i)
	}
}

impl<'input> CallExprContextAttrs<'input> for CallExprContext<'input>{}

pub struct CallExprContextExt<'input>{
	base:ExprContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{CallExprContextExt<'a>}

impl<'input> TaurusParserContext<'input> for CallExprContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for CallExprContext<'input>{
	fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_CallExpr(self);
	}
}

impl<'input> CustomRuleContext<'input> for CallExprContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_expr }
	//fn type_rule_index() -> usize where Self: Sized { RULE_expr }
}

impl<'input> Borrow<ExprContextExt<'input>> for CallExprContext<'input>{
	fn borrow(&self) -> &ExprContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ExprContextExt<'input>> for CallExprContext<'input>{
	fn borrow_mut(&mut self) -> &mut ExprContextExt<'input> { &mut self.base }
}

impl<'input> ExprContextAttrs<'input> for CallExprContext<'input> {}

impl<'input> CallExprContextExt<'input>{
	fn new(ctx: &dyn ExprContextAttrs<'input>) -> Rc<ExprContextAll<'input>>  {
		Rc::new(
			ExprContextAll::CallExprContext(
				BaseParserRuleContext::copy_from(ctx,CallExprContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I, H> TaurusParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn  expr(&mut self,)
	-> Result<Rc<ExprContextAll<'input>>,ANTLRError> {
		self.expr_rec(0)
	}

	fn expr_rec(&mut self, _p: isize)
	-> Result<Rc<ExprContextAll<'input>>,ANTLRError> {
		let recog = self;
		let _parentctx = recog.ctx.take();
		let _parentState = recog.base.get_state();
		let mut _localctx = ExprContextExt::new(_parentctx.clone(), recog.base.get_state());
		recog.base.enter_recursion_rule(_localctx.clone(), 32, RULE_expr, _p);
	    let mut _localctx: Rc<ExprContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
		let _startState = 32;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {
			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(317);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(26,&mut recog.base)? {
				1 =>{
					{
					let mut tmp = IdentExprContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();


					recog.base.set_state(295);
					recog.base.match_token(IDENT,&mut recog.err_handler)?;

					}
				}
			,
				2 =>{
					{
					let mut tmp = ConstExprContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();
					/*InvokeRule constant*/
					recog.base.set_state(296);
					recog.constant()?;

					}
				}
			,
				3 =>{
					{
					let mut tmp = CallExprContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();
					recog.base.set_state(297);
					recog.base.match_token(IDENT,&mut recog.err_handler)?;

					recog.base.set_state(298);
					recog.base.match_token(LPAR,&mut recog.err_handler)?;

					recog.base.set_state(307);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if ((((_la - 3)) & !0x3f) == 0 && ((1usize << (_la - 3)) & ((1usize << (LPAR - 3)) | (1usize << (MINUS - 3)) | (1usize << (NOT - 3)))) != 0) || ((((_la - 38)) & !0x3f) == 0 && ((1usize << (_la - 38)) & ((1usize << (EXPR_TRUE - 38)) | (1usize << (EXPR_FALSE - 38)) | (1usize << (INT_CONSTANT - 38)) | (1usize << (FLOAT_CONSTANT - 38)) | (1usize << (IDENT - 38)))) != 0) {
						{
						/*InvokeRule expr*/
						recog.base.set_state(299);
						recog.expr_rec(0)?;

						recog.base.set_state(304);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
						while _la==COMMA {
							{
							{
							recog.base.set_state(300);
							recog.base.match_token(COMMA,&mut recog.err_handler)?;

							/*InvokeRule expr*/
							recog.base.set_state(301);
							recog.expr_rec(0)?;

							}
							}
							recog.base.set_state(306);
							recog.err_handler.sync(&mut recog.base)?;
							_la = recog.base.input.la(1);
						}
						}
					}

					recog.base.set_state(309);
					recog.base.match_token(RPAR,&mut recog.err_handler)?;

					}
				}
			,
				4 =>{
					{
					let mut tmp = ParExprContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();
					recog.base.set_state(310);
					recog.base.match_token(LPAR,&mut recog.err_handler)?;

					/*InvokeRule expr*/
					recog.base.set_state(311);
					recog.expr_rec(0)?;

					recog.base.set_state(312);
					recog.base.match_token(RPAR,&mut recog.err_handler)?;

					}
				}
			,
				5 =>{
					{
					let mut tmp = UnaryExprContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();
					/*InvokeRule unaryOp*/
					recog.base.set_state(314);
					recog.unaryOp()?;

					/*InvokeRule expr*/
					recog.base.set_state(315);
					recog.expr_rec(7)?;

					}
				}

				_ => {}
			}

			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(351);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(28,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					recog.trigger_exit_rule_event();
					_prevctx = _localctx.clone();
					{
					recog.base.set_state(349);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(27,&mut recog.base)? {
						1 =>{
							{
							/*recRuleLabeledAltStartAction*/
							let mut tmp = MulExprContextExt::new(&**ExprContextExt::new(_parentctx.clone(), _parentState));
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expr);
							_localctx = tmp;
							recog.base.set_state(319);
							if !({recog.precpred(None, 6)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 6)".to_owned()), None))?;
							}
							/*InvokeRule mulOp*/
							recog.base.set_state(320);
							recog.mulOp()?;

							/*InvokeRule expr*/
							recog.base.set_state(321);
							recog.expr_rec(7)?;

							}
						}
					,
						2 =>{
							{
							/*recRuleLabeledAltStartAction*/
							let mut tmp = AddExprContextExt::new(&**ExprContextExt::new(_parentctx.clone(), _parentState));
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expr);
							_localctx = tmp;
							recog.base.set_state(323);
							if !({recog.precpred(None, 5)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 5)".to_owned()), None))?;
							}
							/*InvokeRule addOp*/
							recog.base.set_state(324);
							recog.addOp()?;

							/*InvokeRule expr*/
							recog.base.set_state(325);
							recog.expr_rec(6)?;

							}
						}
					,
						3 =>{
							{
							/*recRuleLabeledAltStartAction*/
							let mut tmp = OrdExprContextExt::new(&**ExprContextExt::new(_parentctx.clone(), _parentState));
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expr);
							_localctx = tmp;
							recog.base.set_state(327);
							if !({recog.precpred(None, 4)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 4)".to_owned()), None))?;
							}
							/*InvokeRule ordOp*/
							recog.base.set_state(328);
							recog.ordOp()?;

							/*InvokeRule expr*/
							recog.base.set_state(329);
							recog.expr_rec(5)?;

							}
						}
					,
						4 =>{
							{
							/*recRuleLabeledAltStartAction*/
							let mut tmp = EqExprContextExt::new(&**ExprContextExt::new(_parentctx.clone(), _parentState));
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expr);
							_localctx = tmp;
							recog.base.set_state(331);
							if !({recog.precpred(None, 3)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 3)".to_owned()), None))?;
							}
							/*InvokeRule eqOp*/
							recog.base.set_state(332);
							recog.eqOp()?;

							/*InvokeRule expr*/
							recog.base.set_state(333);
							recog.expr_rec(4)?;

							}
						}
					,
						5 =>{
							{
							/*recRuleLabeledAltStartAction*/
							let mut tmp = AndExprContextExt::new(&**ExprContextExt::new(_parentctx.clone(), _parentState));
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expr);
							_localctx = tmp;
							recog.base.set_state(335);
							if !({recog.precpred(None, 2)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 2)".to_owned()), None))?;
							}
							recog.base.set_state(336);
							recog.base.match_token(AND,&mut recog.err_handler)?;

							/*InvokeRule expr*/
							recog.base.set_state(337);
							recog.expr_rec(3)?;

							}
						}
					,
						6 =>{
							{
							/*recRuleLabeledAltStartAction*/
							let mut tmp = OrExprContextExt::new(&**ExprContextExt::new(_parentctx.clone(), _parentState));
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expr);
							_localctx = tmp;
							recog.base.set_state(338);
							if !({recog.precpred(None, 1)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 1)".to_owned()), None))?;
							}
							recog.base.set_state(339);
							recog.base.match_token(OR,&mut recog.err_handler)?;

							/*InvokeRule expr*/
							recog.base.set_state(340);
							recog.expr_rec(2)?;

							}
						}
					,
						7 =>{
							{
							/*recRuleLabeledAltStartAction*/
							let mut tmp = ArrAccessExprContextExt::new(&**ExprContextExt::new(_parentctx.clone(), _parentState));
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expr);
							_localctx = tmp;
							recog.base.set_state(341);
							if !({recog.precpred(None, 9)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 9)".to_owned()), None))?;
							}
							recog.base.set_state(342);
							recog.base.match_token(LBRACKET,&mut recog.err_handler)?;

							/*InvokeRule expr*/
							recog.base.set_state(343);
							recog.expr_rec(0)?;

							recog.base.set_state(344);
							recog.base.match_token(RBRACKET,&mut recog.err_handler)?;

							}
						}
					,
						8 =>{
							{
							/*recRuleLabeledAltStartAction*/
							let mut tmp = MemExprContextExt::new(&**ExprContextExt::new(_parentctx.clone(), _parentState));
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_expr);
							_localctx = tmp;
							recog.base.set_state(346);
							if !({recog.precpred(None, 8)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 8)".to_owned()), None))?;
							}
							recog.base.set_state(347);
							recog.base.match_token(PERIOD,&mut recog.err_handler)?;

							recog.base.set_state(348);
							recog.base.match_token(IDENT,&mut recog.err_handler)?;

							}
						}

						_ => {}
					}
					} 
				}
				recog.base.set_state(353);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(28,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_) => {},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re)=>{
			//_localctx.exception = re;
			recog.err_handler.report_error(&mut recog.base, re);
	        recog.err_handler.recover(&mut recog.base, re)?;}
		}
		recog.base.unroll_recursion_context(_parentctx);

		Ok(_localctx)
	}
}
//------------------- logicConstant ----------------
#[derive(Debug)]
pub enum LogicConstantContextAll<'input>{
	LogicTrueConstContext(LogicTrueConstContext<'input>),
	LogicFalseConstContext(LogicFalseConstContext<'input>),
	LogicIntConstContext(LogicIntConstContext<'input>),
	LogicFloatConstContext(LogicFloatConstContext<'input>),
Error(LogicConstantContext<'input>)
}
antlr_rust::tid!{LogicConstantContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for LogicConstantContextAll<'input>{}

impl<'input> TaurusParserContext<'input> for LogicConstantContextAll<'input>{}

impl<'input> Deref for LogicConstantContextAll<'input>{
	type Target = dyn LogicConstantContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use LogicConstantContextAll::*;
		match self{
			LogicTrueConstContext(inner) => inner,
			LogicFalseConstContext(inner) => inner,
			LogicIntConstContext(inner) => inner,
			LogicFloatConstContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for LogicConstantContextAll<'input>{
    fn enter(&self, listener: &mut (dyn TaurusListener<'input> + 'a)) { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn TaurusListener<'input> + 'a)) { self.deref().exit(listener) }
}



pub type LogicConstantContext<'input> = BaseParserRuleContext<'input,LogicConstantContextExt<'input>>;

#[derive(Clone)]
pub struct LogicConstantContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TaurusParserContext<'input> for LogicConstantContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for LogicConstantContext<'input>{
}

impl<'input> CustomRuleContext<'input> for LogicConstantContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_logicConstant }
	//fn type_rule_index() -> usize where Self: Sized { RULE_logicConstant }
}
antlr_rust::tid!{LogicConstantContextExt<'a>}

impl<'input> LogicConstantContextExt<'input>{
	fn new(parent: Option<Rc<dyn TaurusParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<LogicConstantContextAll<'input>> {
		Rc::new(
		LogicConstantContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,LogicConstantContextExt{
				ph:PhantomData
			}),
		)
		)
	}
}

pub trait LogicConstantContextAttrs<'input>: TaurusParserContext<'input> + BorrowMut<LogicConstantContextExt<'input>>{


}

impl<'input> LogicConstantContextAttrs<'input> for LogicConstantContext<'input>{}

pub type LogicTrueConstContext<'input> = BaseParserRuleContext<'input,LogicTrueConstContextExt<'input>>;

pub trait LogicTrueConstContextAttrs<'input>: TaurusParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token ANNO_TRUE
	/// Returns `None` if there is no child corresponding to token ANNO_TRUE
	fn ANNO_TRUE(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(ANNO_TRUE, 0)
	}
}

impl<'input> LogicTrueConstContextAttrs<'input> for LogicTrueConstContext<'input>{}

pub struct LogicTrueConstContextExt<'input>{
	base:LogicConstantContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{LogicTrueConstContextExt<'a>}

impl<'input> TaurusParserContext<'input> for LogicTrueConstContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for LogicTrueConstContext<'input>{
	fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_LogicTrueConst(self);
	}
}

impl<'input> CustomRuleContext<'input> for LogicTrueConstContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_logicConstant }
	//fn type_rule_index() -> usize where Self: Sized { RULE_logicConstant }
}

impl<'input> Borrow<LogicConstantContextExt<'input>> for LogicTrueConstContext<'input>{
	fn borrow(&self) -> &LogicConstantContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<LogicConstantContextExt<'input>> for LogicTrueConstContext<'input>{
	fn borrow_mut(&mut self) -> &mut LogicConstantContextExt<'input> { &mut self.base }
}

impl<'input> LogicConstantContextAttrs<'input> for LogicTrueConstContext<'input> {}

impl<'input> LogicTrueConstContextExt<'input>{
	fn new(ctx: &dyn LogicConstantContextAttrs<'input>) -> Rc<LogicConstantContextAll<'input>>  {
		Rc::new(
			LogicConstantContextAll::LogicTrueConstContext(
				BaseParserRuleContext::copy_from(ctx,LogicTrueConstContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type LogicFalseConstContext<'input> = BaseParserRuleContext<'input,LogicFalseConstContextExt<'input>>;

pub trait LogicFalseConstContextAttrs<'input>: TaurusParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token ANNO_FALSE
	/// Returns `None` if there is no child corresponding to token ANNO_FALSE
	fn ANNO_FALSE(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(ANNO_FALSE, 0)
	}
}

impl<'input> LogicFalseConstContextAttrs<'input> for LogicFalseConstContext<'input>{}

pub struct LogicFalseConstContextExt<'input>{
	base:LogicConstantContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{LogicFalseConstContextExt<'a>}

impl<'input> TaurusParserContext<'input> for LogicFalseConstContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for LogicFalseConstContext<'input>{
	fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_LogicFalseConst(self);
	}
}

impl<'input> CustomRuleContext<'input> for LogicFalseConstContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_logicConstant }
	//fn type_rule_index() -> usize where Self: Sized { RULE_logicConstant }
}

impl<'input> Borrow<LogicConstantContextExt<'input>> for LogicFalseConstContext<'input>{
	fn borrow(&self) -> &LogicConstantContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<LogicConstantContextExt<'input>> for LogicFalseConstContext<'input>{
	fn borrow_mut(&mut self) -> &mut LogicConstantContextExt<'input> { &mut self.base }
}

impl<'input> LogicConstantContextAttrs<'input> for LogicFalseConstContext<'input> {}

impl<'input> LogicFalseConstContextExt<'input>{
	fn new(ctx: &dyn LogicConstantContextAttrs<'input>) -> Rc<LogicConstantContextAll<'input>>  {
		Rc::new(
			LogicConstantContextAll::LogicFalseConstContext(
				BaseParserRuleContext::copy_from(ctx,LogicFalseConstContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type LogicIntConstContext<'input> = BaseParserRuleContext<'input,LogicIntConstContextExt<'input>>;

pub trait LogicIntConstContextAttrs<'input>: TaurusParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token INT_CONSTANT
	/// Returns `None` if there is no child corresponding to token INT_CONSTANT
	fn INT_CONSTANT(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(INT_CONSTANT, 0)
	}
}

impl<'input> LogicIntConstContextAttrs<'input> for LogicIntConstContext<'input>{}

pub struct LogicIntConstContextExt<'input>{
	base:LogicConstantContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{LogicIntConstContextExt<'a>}

impl<'input> TaurusParserContext<'input> for LogicIntConstContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for LogicIntConstContext<'input>{
	fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_LogicIntConst(self);
	}
}

impl<'input> CustomRuleContext<'input> for LogicIntConstContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_logicConstant }
	//fn type_rule_index() -> usize where Self: Sized { RULE_logicConstant }
}

impl<'input> Borrow<LogicConstantContextExt<'input>> for LogicIntConstContext<'input>{
	fn borrow(&self) -> &LogicConstantContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<LogicConstantContextExt<'input>> for LogicIntConstContext<'input>{
	fn borrow_mut(&mut self) -> &mut LogicConstantContextExt<'input> { &mut self.base }
}

impl<'input> LogicConstantContextAttrs<'input> for LogicIntConstContext<'input> {}

impl<'input> LogicIntConstContextExt<'input>{
	fn new(ctx: &dyn LogicConstantContextAttrs<'input>) -> Rc<LogicConstantContextAll<'input>>  {
		Rc::new(
			LogicConstantContextAll::LogicIntConstContext(
				BaseParserRuleContext::copy_from(ctx,LogicIntConstContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type LogicFloatConstContext<'input> = BaseParserRuleContext<'input,LogicFloatConstContextExt<'input>>;

pub trait LogicFloatConstContextAttrs<'input>: TaurusParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token FLOAT_CONSTANT
	/// Returns `None` if there is no child corresponding to token FLOAT_CONSTANT
	fn FLOAT_CONSTANT(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(FLOAT_CONSTANT, 0)
	}
}

impl<'input> LogicFloatConstContextAttrs<'input> for LogicFloatConstContext<'input>{}

pub struct LogicFloatConstContextExt<'input>{
	base:LogicConstantContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{LogicFloatConstContextExt<'a>}

impl<'input> TaurusParserContext<'input> for LogicFloatConstContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for LogicFloatConstContext<'input>{
	fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_LogicFloatConst(self);
	}
}

impl<'input> CustomRuleContext<'input> for LogicFloatConstContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_logicConstant }
	//fn type_rule_index() -> usize where Self: Sized { RULE_logicConstant }
}

impl<'input> Borrow<LogicConstantContextExt<'input>> for LogicFloatConstContext<'input>{
	fn borrow(&self) -> &LogicConstantContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<LogicConstantContextExt<'input>> for LogicFloatConstContext<'input>{
	fn borrow_mut(&mut self) -> &mut LogicConstantContextExt<'input> { &mut self.base }
}

impl<'input> LogicConstantContextAttrs<'input> for LogicFloatConstContext<'input> {}

impl<'input> LogicFloatConstContextExt<'input>{
	fn new(ctx: &dyn LogicConstantContextAttrs<'input>) -> Rc<LogicConstantContextAll<'input>>  {
		Rc::new(
			LogicConstantContextAll::LogicFloatConstContext(
				BaseParserRuleContext::copy_from(ctx,LogicFloatConstContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I, H> TaurusParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn logicConstant(&mut self,)
	-> Result<Rc<LogicConstantContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = LogicConstantContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 34, RULE_logicConstant);
        let mut _localctx: Rc<LogicConstantContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(358);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 INT_CONSTANT 
				=> {
					let tmp = LogicIntConstContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1);
					_localctx = tmp;
					{
					recog.base.set_state(354);
					recog.base.match_token(INT_CONSTANT,&mut recog.err_handler)?;

					}
				}

			 FLOAT_CONSTANT 
				=> {
					let tmp = LogicFloatConstContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2);
					_localctx = tmp;
					{
					recog.base.set_state(355);
					recog.base.match_token(FLOAT_CONSTANT,&mut recog.err_handler)?;

					}
				}

			 ANNO_TRUE 
				=> {
					let tmp = LogicTrueConstContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 3);
					_localctx = tmp;
					{
					recog.base.set_state(356);
					recog.base.match_token(ANNO_TRUE,&mut recog.err_handler)?;

					}
				}

			 ANNO_FALSE 
				=> {
					let tmp = LogicFalseConstContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 4);
					_localctx = tmp;
					{
					recog.base.set_state(357);
					recog.base.match_token(ANNO_FALSE,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- arithTerm ----------------
#[derive(Debug)]
pub enum ArithTermContextAll<'input>{
	ParArithTermContext(ParArithTermContext<'input>),
	MulTermContext(MulTermContext<'input>),
	MemTermContext(MemTermContext<'input>),
	IdentTermContext(IdentTermContext<'input>),
	UnaryTermContext(UnaryTermContext<'input>),
	ArrAccessTermContext(ArrAccessTermContext<'input>),
	ResTermContext(ResTermContext<'input>),
	ArrUpdTermContext(ArrUpdTermContext<'input>),
	ConstTermContext(ConstTermContext<'input>),
	AddTermContext(AddTermContext<'input>),
Error(ArithTermContext<'input>)
}
antlr_rust::tid!{ArithTermContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for ArithTermContextAll<'input>{}

impl<'input> TaurusParserContext<'input> for ArithTermContextAll<'input>{}

impl<'input> Deref for ArithTermContextAll<'input>{
	type Target = dyn ArithTermContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use ArithTermContextAll::*;
		match self{
			ParArithTermContext(inner) => inner,
			MulTermContext(inner) => inner,
			MemTermContext(inner) => inner,
			IdentTermContext(inner) => inner,
			UnaryTermContext(inner) => inner,
			ArrAccessTermContext(inner) => inner,
			ResTermContext(inner) => inner,
			ArrUpdTermContext(inner) => inner,
			ConstTermContext(inner) => inner,
			AddTermContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for ArithTermContextAll<'input>{
    fn enter(&self, listener: &mut (dyn TaurusListener<'input> + 'a)) { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn TaurusListener<'input> + 'a)) { self.deref().exit(listener) }
}



pub type ArithTermContext<'input> = BaseParserRuleContext<'input,ArithTermContextExt<'input>>;

#[derive(Clone)]
pub struct ArithTermContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TaurusParserContext<'input> for ArithTermContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for ArithTermContext<'input>{
}

impl<'input> CustomRuleContext<'input> for ArithTermContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_arithTerm }
	//fn type_rule_index() -> usize where Self: Sized { RULE_arithTerm }
}
antlr_rust::tid!{ArithTermContextExt<'a>}

impl<'input> ArithTermContextExt<'input>{
	fn new(parent: Option<Rc<dyn TaurusParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ArithTermContextAll<'input>> {
		Rc::new(
		ArithTermContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ArithTermContextExt{
				ph:PhantomData
			}),
		)
		)
	}
}

pub trait ArithTermContextAttrs<'input>: TaurusParserContext<'input> + BorrowMut<ArithTermContextExt<'input>>{


}

impl<'input> ArithTermContextAttrs<'input> for ArithTermContext<'input>{}

pub type ParArithTermContext<'input> = BaseParserRuleContext<'input,ParArithTermContextExt<'input>>;

pub trait ParArithTermContextAttrs<'input>: TaurusParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token LPAR
	/// Returns `None` if there is no child corresponding to token LPAR
	fn LPAR(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(LPAR, 0)
	}
	fn arithTerm(&self) -> Option<Rc<ArithTermContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token RPAR
	/// Returns `None` if there is no child corresponding to token RPAR
	fn RPAR(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(RPAR, 0)
	}
}

impl<'input> ParArithTermContextAttrs<'input> for ParArithTermContext<'input>{}

pub struct ParArithTermContextExt<'input>{
	base:ArithTermContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{ParArithTermContextExt<'a>}

impl<'input> TaurusParserContext<'input> for ParArithTermContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for ParArithTermContext<'input>{
	fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_ParArithTerm(self);
	}
}

impl<'input> CustomRuleContext<'input> for ParArithTermContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_arithTerm }
	//fn type_rule_index() -> usize where Self: Sized { RULE_arithTerm }
}

impl<'input> Borrow<ArithTermContextExt<'input>> for ParArithTermContext<'input>{
	fn borrow(&self) -> &ArithTermContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ArithTermContextExt<'input>> for ParArithTermContext<'input>{
	fn borrow_mut(&mut self) -> &mut ArithTermContextExt<'input> { &mut self.base }
}

impl<'input> ArithTermContextAttrs<'input> for ParArithTermContext<'input> {}

impl<'input> ParArithTermContextExt<'input>{
	fn new(ctx: &dyn ArithTermContextAttrs<'input>) -> Rc<ArithTermContextAll<'input>>  {
		Rc::new(
			ArithTermContextAll::ParArithTermContext(
				BaseParserRuleContext::copy_from(ctx,ParArithTermContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type MulTermContext<'input> = BaseParserRuleContext<'input,MulTermContextExt<'input>>;

pub trait MulTermContextAttrs<'input>: TaurusParserContext<'input>{
	fn arithTerm_all(&self) ->  Vec<Rc<ArithTermContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn arithTerm(&self, i: usize) -> Option<Rc<ArithTermContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
	fn mulOp(&self) -> Option<Rc<MulOpContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> MulTermContextAttrs<'input> for MulTermContext<'input>{}

pub struct MulTermContextExt<'input>{
	base:ArithTermContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{MulTermContextExt<'a>}

impl<'input> TaurusParserContext<'input> for MulTermContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for MulTermContext<'input>{
	fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_MulTerm(self);
	}
}

impl<'input> CustomRuleContext<'input> for MulTermContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_arithTerm }
	//fn type_rule_index() -> usize where Self: Sized { RULE_arithTerm }
}

impl<'input> Borrow<ArithTermContextExt<'input>> for MulTermContext<'input>{
	fn borrow(&self) -> &ArithTermContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ArithTermContextExt<'input>> for MulTermContext<'input>{
	fn borrow_mut(&mut self) -> &mut ArithTermContextExt<'input> { &mut self.base }
}

impl<'input> ArithTermContextAttrs<'input> for MulTermContext<'input> {}

impl<'input> MulTermContextExt<'input>{
	fn new(ctx: &dyn ArithTermContextAttrs<'input>) -> Rc<ArithTermContextAll<'input>>  {
		Rc::new(
			ArithTermContextAll::MulTermContext(
				BaseParserRuleContext::copy_from(ctx,MulTermContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type MemTermContext<'input> = BaseParserRuleContext<'input,MemTermContextExt<'input>>;

pub trait MemTermContextAttrs<'input>: TaurusParserContext<'input>{
	fn arithTerm(&self) -> Option<Rc<ArithTermContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token PERIOD
	/// Returns `None` if there is no child corresponding to token PERIOD
	fn PERIOD(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(PERIOD, 0)
	}
	/// Retrieves first TerminalNode corresponding to token IDENT
	/// Returns `None` if there is no child corresponding to token IDENT
	fn IDENT(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(IDENT, 0)
	}
}

impl<'input> MemTermContextAttrs<'input> for MemTermContext<'input>{}

pub struct MemTermContextExt<'input>{
	base:ArithTermContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{MemTermContextExt<'a>}

impl<'input> TaurusParserContext<'input> for MemTermContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for MemTermContext<'input>{
	fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_MemTerm(self);
	}
}

impl<'input> CustomRuleContext<'input> for MemTermContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_arithTerm }
	//fn type_rule_index() -> usize where Self: Sized { RULE_arithTerm }
}

impl<'input> Borrow<ArithTermContextExt<'input>> for MemTermContext<'input>{
	fn borrow(&self) -> &ArithTermContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ArithTermContextExt<'input>> for MemTermContext<'input>{
	fn borrow_mut(&mut self) -> &mut ArithTermContextExt<'input> { &mut self.base }
}

impl<'input> ArithTermContextAttrs<'input> for MemTermContext<'input> {}

impl<'input> MemTermContextExt<'input>{
	fn new(ctx: &dyn ArithTermContextAttrs<'input>) -> Rc<ArithTermContextAll<'input>>  {
		Rc::new(
			ArithTermContextAll::MemTermContext(
				BaseParserRuleContext::copy_from(ctx,MemTermContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type IdentTermContext<'input> = BaseParserRuleContext<'input,IdentTermContextExt<'input>>;

pub trait IdentTermContextAttrs<'input>: TaurusParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token IDENT
	/// Returns `None` if there is no child corresponding to token IDENT
	fn IDENT(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(IDENT, 0)
	}
}

impl<'input> IdentTermContextAttrs<'input> for IdentTermContext<'input>{}

pub struct IdentTermContextExt<'input>{
	base:ArithTermContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{IdentTermContextExt<'a>}

impl<'input> TaurusParserContext<'input> for IdentTermContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for IdentTermContext<'input>{
	fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_IdentTerm(self);
	}
}

impl<'input> CustomRuleContext<'input> for IdentTermContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_arithTerm }
	//fn type_rule_index() -> usize where Self: Sized { RULE_arithTerm }
}

impl<'input> Borrow<ArithTermContextExt<'input>> for IdentTermContext<'input>{
	fn borrow(&self) -> &ArithTermContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ArithTermContextExt<'input>> for IdentTermContext<'input>{
	fn borrow_mut(&mut self) -> &mut ArithTermContextExt<'input> { &mut self.base }
}

impl<'input> ArithTermContextAttrs<'input> for IdentTermContext<'input> {}

impl<'input> IdentTermContextExt<'input>{
	fn new(ctx: &dyn ArithTermContextAttrs<'input>) -> Rc<ArithTermContextAll<'input>>  {
		Rc::new(
			ArithTermContextAll::IdentTermContext(
				BaseParserRuleContext::copy_from(ctx,IdentTermContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type UnaryTermContext<'input> = BaseParserRuleContext<'input,UnaryTermContextExt<'input>>;

pub trait UnaryTermContextAttrs<'input>: TaurusParserContext<'input>{
	fn unaryOp(&self) -> Option<Rc<UnaryOpContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	fn arithTerm(&self) -> Option<Rc<ArithTermContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> UnaryTermContextAttrs<'input> for UnaryTermContext<'input>{}

pub struct UnaryTermContextExt<'input>{
	base:ArithTermContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{UnaryTermContextExt<'a>}

impl<'input> TaurusParserContext<'input> for UnaryTermContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for UnaryTermContext<'input>{
	fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_UnaryTerm(self);
	}
}

impl<'input> CustomRuleContext<'input> for UnaryTermContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_arithTerm }
	//fn type_rule_index() -> usize where Self: Sized { RULE_arithTerm }
}

impl<'input> Borrow<ArithTermContextExt<'input>> for UnaryTermContext<'input>{
	fn borrow(&self) -> &ArithTermContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ArithTermContextExt<'input>> for UnaryTermContext<'input>{
	fn borrow_mut(&mut self) -> &mut ArithTermContextExt<'input> { &mut self.base }
}

impl<'input> ArithTermContextAttrs<'input> for UnaryTermContext<'input> {}

impl<'input> UnaryTermContextExt<'input>{
	fn new(ctx: &dyn ArithTermContextAttrs<'input>) -> Rc<ArithTermContextAll<'input>>  {
		Rc::new(
			ArithTermContextAll::UnaryTermContext(
				BaseParserRuleContext::copy_from(ctx,UnaryTermContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type ArrAccessTermContext<'input> = BaseParserRuleContext<'input,ArrAccessTermContextExt<'input>>;

pub trait ArrAccessTermContextAttrs<'input>: TaurusParserContext<'input>{
	fn arithTerm_all(&self) ->  Vec<Rc<ArithTermContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn arithTerm(&self, i: usize) -> Option<Rc<ArithTermContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
	/// Retrieves first TerminalNode corresponding to token LBRACKET
	/// Returns `None` if there is no child corresponding to token LBRACKET
	fn LBRACKET(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(LBRACKET, 0)
	}
	/// Retrieves first TerminalNode corresponding to token RBRACKET
	/// Returns `None` if there is no child corresponding to token RBRACKET
	fn RBRACKET(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(RBRACKET, 0)
	}
}

impl<'input> ArrAccessTermContextAttrs<'input> for ArrAccessTermContext<'input>{}

pub struct ArrAccessTermContextExt<'input>{
	base:ArithTermContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{ArrAccessTermContextExt<'a>}

impl<'input> TaurusParserContext<'input> for ArrAccessTermContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for ArrAccessTermContext<'input>{
	fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_ArrAccessTerm(self);
	}
}

impl<'input> CustomRuleContext<'input> for ArrAccessTermContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_arithTerm }
	//fn type_rule_index() -> usize where Self: Sized { RULE_arithTerm }
}

impl<'input> Borrow<ArithTermContextExt<'input>> for ArrAccessTermContext<'input>{
	fn borrow(&self) -> &ArithTermContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ArithTermContextExt<'input>> for ArrAccessTermContext<'input>{
	fn borrow_mut(&mut self) -> &mut ArithTermContextExt<'input> { &mut self.base }
}

impl<'input> ArithTermContextAttrs<'input> for ArrAccessTermContext<'input> {}

impl<'input> ArrAccessTermContextExt<'input>{
	fn new(ctx: &dyn ArithTermContextAttrs<'input>) -> Rc<ArithTermContextAll<'input>>  {
		Rc::new(
			ArithTermContextAll::ArrAccessTermContext(
				BaseParserRuleContext::copy_from(ctx,ArrAccessTermContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type ResTermContext<'input> = BaseParserRuleContext<'input,ResTermContextExt<'input>>;

pub trait ResTermContextAttrs<'input>: TaurusParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token RESULT
	/// Returns `None` if there is no child corresponding to token RESULT
	fn RESULT(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(RESULT, 0)
	}
}

impl<'input> ResTermContextAttrs<'input> for ResTermContext<'input>{}

pub struct ResTermContextExt<'input>{
	base:ArithTermContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{ResTermContextExt<'a>}

impl<'input> TaurusParserContext<'input> for ResTermContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for ResTermContext<'input>{
	fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_ResTerm(self);
	}
}

impl<'input> CustomRuleContext<'input> for ResTermContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_arithTerm }
	//fn type_rule_index() -> usize where Self: Sized { RULE_arithTerm }
}

impl<'input> Borrow<ArithTermContextExt<'input>> for ResTermContext<'input>{
	fn borrow(&self) -> &ArithTermContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ArithTermContextExt<'input>> for ResTermContext<'input>{
	fn borrow_mut(&mut self) -> &mut ArithTermContextExt<'input> { &mut self.base }
}

impl<'input> ArithTermContextAttrs<'input> for ResTermContext<'input> {}

impl<'input> ResTermContextExt<'input>{
	fn new(ctx: &dyn ArithTermContextAttrs<'input>) -> Rc<ArithTermContextAll<'input>>  {
		Rc::new(
			ArithTermContextAll::ResTermContext(
				BaseParserRuleContext::copy_from(ctx,ResTermContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type ArrUpdTermContext<'input> = BaseParserRuleContext<'input,ArrUpdTermContextExt<'input>>;

pub trait ArrUpdTermContextAttrs<'input>: TaurusParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token LBRACE
	/// Returns `None` if there is no child corresponding to token LBRACE
	fn LBRACE(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(LBRACE, 0)
	}
	fn arithTerm_all(&self) ->  Vec<Rc<ArithTermContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn arithTerm(&self, i: usize) -> Option<Rc<ArithTermContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
	/// Retrieves first TerminalNode corresponding to token WITH
	/// Returns `None` if there is no child corresponding to token WITH
	fn WITH(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(WITH, 0)
	}
	/// Retrieves first TerminalNode corresponding to token LBRACKET
	/// Returns `None` if there is no child corresponding to token LBRACKET
	fn LBRACKET(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(LBRACKET, 0)
	}
	/// Retrieves first TerminalNode corresponding to token RBRACKET
	/// Returns `None` if there is no child corresponding to token RBRACKET
	fn RBRACKET(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(RBRACKET, 0)
	}
	/// Retrieves first TerminalNode corresponding to token ASSIGN
	/// Returns `None` if there is no child corresponding to token ASSIGN
	fn ASSIGN(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(ASSIGN, 0)
	}
	/// Retrieves first TerminalNode corresponding to token RBRACE
	/// Returns `None` if there is no child corresponding to token RBRACE
	fn RBRACE(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(RBRACE, 0)
	}
}

impl<'input> ArrUpdTermContextAttrs<'input> for ArrUpdTermContext<'input>{}

pub struct ArrUpdTermContextExt<'input>{
	base:ArithTermContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{ArrUpdTermContextExt<'a>}

impl<'input> TaurusParserContext<'input> for ArrUpdTermContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for ArrUpdTermContext<'input>{
	fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_ArrUpdTerm(self);
	}
}

impl<'input> CustomRuleContext<'input> for ArrUpdTermContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_arithTerm }
	//fn type_rule_index() -> usize where Self: Sized { RULE_arithTerm }
}

impl<'input> Borrow<ArithTermContextExt<'input>> for ArrUpdTermContext<'input>{
	fn borrow(&self) -> &ArithTermContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ArithTermContextExt<'input>> for ArrUpdTermContext<'input>{
	fn borrow_mut(&mut self) -> &mut ArithTermContextExt<'input> { &mut self.base }
}

impl<'input> ArithTermContextAttrs<'input> for ArrUpdTermContext<'input> {}

impl<'input> ArrUpdTermContextExt<'input>{
	fn new(ctx: &dyn ArithTermContextAttrs<'input>) -> Rc<ArithTermContextAll<'input>>  {
		Rc::new(
			ArithTermContextAll::ArrUpdTermContext(
				BaseParserRuleContext::copy_from(ctx,ArrUpdTermContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type ConstTermContext<'input> = BaseParserRuleContext<'input,ConstTermContextExt<'input>>;

pub trait ConstTermContextAttrs<'input>: TaurusParserContext<'input>{
	fn logicConstant(&self) -> Option<Rc<LogicConstantContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> ConstTermContextAttrs<'input> for ConstTermContext<'input>{}

pub struct ConstTermContextExt<'input>{
	base:ArithTermContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{ConstTermContextExt<'a>}

impl<'input> TaurusParserContext<'input> for ConstTermContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for ConstTermContext<'input>{
	fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_ConstTerm(self);
	}
}

impl<'input> CustomRuleContext<'input> for ConstTermContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_arithTerm }
	//fn type_rule_index() -> usize where Self: Sized { RULE_arithTerm }
}

impl<'input> Borrow<ArithTermContextExt<'input>> for ConstTermContext<'input>{
	fn borrow(&self) -> &ArithTermContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ArithTermContextExt<'input>> for ConstTermContext<'input>{
	fn borrow_mut(&mut self) -> &mut ArithTermContextExt<'input> { &mut self.base }
}

impl<'input> ArithTermContextAttrs<'input> for ConstTermContext<'input> {}

impl<'input> ConstTermContextExt<'input>{
	fn new(ctx: &dyn ArithTermContextAttrs<'input>) -> Rc<ArithTermContextAll<'input>>  {
		Rc::new(
			ArithTermContextAll::ConstTermContext(
				BaseParserRuleContext::copy_from(ctx,ConstTermContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type AddTermContext<'input> = BaseParserRuleContext<'input,AddTermContextExt<'input>>;

pub trait AddTermContextAttrs<'input>: TaurusParserContext<'input>{
	fn arithTerm_all(&self) ->  Vec<Rc<ArithTermContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn arithTerm(&self, i: usize) -> Option<Rc<ArithTermContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
	fn addOp(&self) -> Option<Rc<AddOpContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> AddTermContextAttrs<'input> for AddTermContext<'input>{}

pub struct AddTermContextExt<'input>{
	base:ArithTermContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{AddTermContextExt<'a>}

impl<'input> TaurusParserContext<'input> for AddTermContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for AddTermContext<'input>{
	fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_AddTerm(self);
	}
}

impl<'input> CustomRuleContext<'input> for AddTermContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_arithTerm }
	//fn type_rule_index() -> usize where Self: Sized { RULE_arithTerm }
}

impl<'input> Borrow<ArithTermContextExt<'input>> for AddTermContext<'input>{
	fn borrow(&self) -> &ArithTermContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ArithTermContextExt<'input>> for AddTermContext<'input>{
	fn borrow_mut(&mut self) -> &mut ArithTermContextExt<'input> { &mut self.base }
}

impl<'input> ArithTermContextAttrs<'input> for AddTermContext<'input> {}

impl<'input> AddTermContextExt<'input>{
	fn new(ctx: &dyn ArithTermContextAttrs<'input>) -> Rc<ArithTermContextAll<'input>>  {
		Rc::new(
			ArithTermContextAll::AddTermContext(
				BaseParserRuleContext::copy_from(ctx,AddTermContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I, H> TaurusParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn  arithTerm(&mut self,)
	-> Result<Rc<ArithTermContextAll<'input>>,ANTLRError> {
		self.arithTerm_rec(0)
	}

	fn arithTerm_rec(&mut self, _p: isize)
	-> Result<Rc<ArithTermContextAll<'input>>,ANTLRError> {
		let recog = self;
		let _parentctx = recog.ctx.take();
		let _parentState = recog.base.get_state();
		let mut _localctx = ArithTermContextExt::new(_parentctx.clone(), recog.base.get_state());
		recog.base.enter_recursion_rule(_localctx.clone(), 36, RULE_arithTerm, _p);
	    let mut _localctx: Rc<ArithTermContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
		let _startState = 36;
		let result: Result<(), ANTLRError> = (|| {
			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(381);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 IDENT 
				=> {
					{
					let mut tmp = IdentTermContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();


					recog.base.set_state(361);
					recog.base.match_token(IDENT,&mut recog.err_handler)?;

					}
				}

			 RESULT 
				=> {
					{
					let mut tmp = ResTermContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();
					recog.base.set_state(362);
					recog.base.match_token(RESULT,&mut recog.err_handler)?;

					}
				}

			 ANNO_TRUE | ANNO_FALSE | INT_CONSTANT | FLOAT_CONSTANT 
				=> {
					{
					let mut tmp = ConstTermContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();
					/*InvokeRule logicConstant*/
					recog.base.set_state(363);
					recog.logicConstant()?;

					}
				}

			 LBRACE 
				=> {
					{
					let mut tmp = ArrUpdTermContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();
					recog.base.set_state(364);
					recog.base.match_token(LBRACE,&mut recog.err_handler)?;

					/*InvokeRule arithTerm*/
					recog.base.set_state(365);
					recog.arithTerm_rec(0)?;

					recog.base.set_state(366);
					recog.base.match_token(WITH,&mut recog.err_handler)?;

					recog.base.set_state(367);
					recog.base.match_token(LBRACKET,&mut recog.err_handler)?;

					/*InvokeRule arithTerm*/
					recog.base.set_state(368);
					recog.arithTerm_rec(0)?;

					recog.base.set_state(369);
					recog.base.match_token(RBRACKET,&mut recog.err_handler)?;

					recog.base.set_state(370);
					recog.base.match_token(ASSIGN,&mut recog.err_handler)?;

					/*InvokeRule arithTerm*/
					recog.base.set_state(371);
					recog.arithTerm_rec(0)?;

					recog.base.set_state(372);
					recog.base.match_token(RBRACE,&mut recog.err_handler)?;

					}
				}

			 LPAR 
				=> {
					{
					let mut tmp = ParArithTermContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();
					recog.base.set_state(374);
					recog.base.match_token(LPAR,&mut recog.err_handler)?;

					/*InvokeRule arithTerm*/
					recog.base.set_state(375);
					recog.arithTerm_rec(0)?;

					recog.base.set_state(376);
					recog.base.match_token(RPAR,&mut recog.err_handler)?;

					}
				}

			 MINUS | NOT 
				=> {
					{
					let mut tmp = UnaryTermContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();
					/*InvokeRule unaryOp*/
					recog.base.set_state(378);
					recog.unaryOp()?;

					/*InvokeRule arithTerm*/
					recog.base.set_state(379);
					recog.arithTerm_rec(3)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}

			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(401);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(32,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					recog.trigger_exit_rule_event();
					_prevctx = _localctx.clone();
					{
					recog.base.set_state(399);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(31,&mut recog.base)? {
						1 =>{
							{
							/*recRuleLabeledAltStartAction*/
							let mut tmp = MulTermContextExt::new(&**ArithTermContextExt::new(_parentctx.clone(), _parentState));
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_arithTerm);
							_localctx = tmp;
							recog.base.set_state(383);
							if !({recog.precpred(None, 2)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 2)".to_owned()), None))?;
							}
							/*InvokeRule mulOp*/
							recog.base.set_state(384);
							recog.mulOp()?;

							/*InvokeRule arithTerm*/
							recog.base.set_state(385);
							recog.arithTerm_rec(3)?;

							}
						}
					,
						2 =>{
							{
							/*recRuleLabeledAltStartAction*/
							let mut tmp = AddTermContextExt::new(&**ArithTermContextExt::new(_parentctx.clone(), _parentState));
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_arithTerm);
							_localctx = tmp;
							recog.base.set_state(387);
							if !({recog.precpred(None, 1)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 1)".to_owned()), None))?;
							}
							/*InvokeRule addOp*/
							recog.base.set_state(388);
							recog.addOp()?;

							/*InvokeRule arithTerm*/
							recog.base.set_state(389);
							recog.arithTerm_rec(2)?;

							}
						}
					,
						3 =>{
							{
							/*recRuleLabeledAltStartAction*/
							let mut tmp = ArrAccessTermContextExt::new(&**ArithTermContextExt::new(_parentctx.clone(), _parentState));
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_arithTerm);
							_localctx = tmp;
							recog.base.set_state(391);
							if !({recog.precpred(None, 5)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 5)".to_owned()), None))?;
							}
							recog.base.set_state(392);
							recog.base.match_token(LBRACKET,&mut recog.err_handler)?;

							/*InvokeRule arithTerm*/
							recog.base.set_state(393);
							recog.arithTerm_rec(0)?;

							recog.base.set_state(394);
							recog.base.match_token(RBRACKET,&mut recog.err_handler)?;

							}
						}
					,
						4 =>{
							{
							/*recRuleLabeledAltStartAction*/
							let mut tmp = MemTermContextExt::new(&**ArithTermContextExt::new(_parentctx.clone(), _parentState));
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_arithTerm);
							_localctx = tmp;
							recog.base.set_state(396);
							if !({recog.precpred(None, 4)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 4)".to_owned()), None))?;
							}
							recog.base.set_state(397);
							recog.base.match_token(PERIOD,&mut recog.err_handler)?;

							recog.base.set_state(398);
							recog.base.match_token(IDENT,&mut recog.err_handler)?;

							}
						}

						_ => {}
					}
					} 
				}
				recog.base.set_state(403);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(32,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_) => {},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re)=>{
			//_localctx.exception = re;
			recog.err_handler.report_error(&mut recog.base, re);
	        recog.err_handler.recover(&mut recog.base, re)?;}
		}
		recog.base.unroll_recursion_context(_parentctx);

		Ok(_localctx)
	}
}
//------------------- term ----------------
#[derive(Debug)]
pub enum TermContextAll<'input>{
	OrdTermContext(OrdTermContext<'input>),
	AndTermContext(AndTermContext<'input>),
	OrTermContext(OrTermContext<'input>),
	EqTermContext(EqTermContext<'input>),
	AriTermContext(AriTermContext<'input>),
	ParTermContext(ParTermContext<'input>),
Error(TermContext<'input>)
}
antlr_rust::tid!{TermContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for TermContextAll<'input>{}

impl<'input> TaurusParserContext<'input> for TermContextAll<'input>{}

impl<'input> Deref for TermContextAll<'input>{
	type Target = dyn TermContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use TermContextAll::*;
		match self{
			OrdTermContext(inner) => inner,
			AndTermContext(inner) => inner,
			OrTermContext(inner) => inner,
			EqTermContext(inner) => inner,
			AriTermContext(inner) => inner,
			ParTermContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for TermContextAll<'input>{
    fn enter(&self, listener: &mut (dyn TaurusListener<'input> + 'a)) { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn TaurusListener<'input> + 'a)) { self.deref().exit(listener) }
}



pub type TermContext<'input> = BaseParserRuleContext<'input,TermContextExt<'input>>;

#[derive(Clone)]
pub struct TermContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TaurusParserContext<'input> for TermContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for TermContext<'input>{
}

impl<'input> CustomRuleContext<'input> for TermContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_term }
	//fn type_rule_index() -> usize where Self: Sized { RULE_term }
}
antlr_rust::tid!{TermContextExt<'a>}

impl<'input> TermContextExt<'input>{
	fn new(parent: Option<Rc<dyn TaurusParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<TermContextAll<'input>> {
		Rc::new(
		TermContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,TermContextExt{
				ph:PhantomData
			}),
		)
		)
	}
}

pub trait TermContextAttrs<'input>: TaurusParserContext<'input> + BorrowMut<TermContextExt<'input>>{


}

impl<'input> TermContextAttrs<'input> for TermContext<'input>{}

pub type OrdTermContext<'input> = BaseParserRuleContext<'input,OrdTermContextExt<'input>>;

pub trait OrdTermContextAttrs<'input>: TaurusParserContext<'input>{
	fn term_all(&self) ->  Vec<Rc<TermContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn term(&self, i: usize) -> Option<Rc<TermContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
	fn ordOp(&self) -> Option<Rc<OrdOpContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> OrdTermContextAttrs<'input> for OrdTermContext<'input>{}

pub struct OrdTermContextExt<'input>{
	base:TermContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{OrdTermContextExt<'a>}

impl<'input> TaurusParserContext<'input> for OrdTermContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for OrdTermContext<'input>{
	fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_OrdTerm(self);
	}
}

impl<'input> CustomRuleContext<'input> for OrdTermContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_term }
	//fn type_rule_index() -> usize where Self: Sized { RULE_term }
}

impl<'input> Borrow<TermContextExt<'input>> for OrdTermContext<'input>{
	fn borrow(&self) -> &TermContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<TermContextExt<'input>> for OrdTermContext<'input>{
	fn borrow_mut(&mut self) -> &mut TermContextExt<'input> { &mut self.base }
}

impl<'input> TermContextAttrs<'input> for OrdTermContext<'input> {}

impl<'input> OrdTermContextExt<'input>{
	fn new(ctx: &dyn TermContextAttrs<'input>) -> Rc<TermContextAll<'input>>  {
		Rc::new(
			TermContextAll::OrdTermContext(
				BaseParserRuleContext::copy_from(ctx,OrdTermContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type AndTermContext<'input> = BaseParserRuleContext<'input,AndTermContextExt<'input>>;

pub trait AndTermContextAttrs<'input>: TaurusParserContext<'input>{
	fn term_all(&self) ->  Vec<Rc<TermContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn term(&self, i: usize) -> Option<Rc<TermContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
	/// Retrieves first TerminalNode corresponding to token AND
	/// Returns `None` if there is no child corresponding to token AND
	fn AND(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(AND, 0)
	}
}

impl<'input> AndTermContextAttrs<'input> for AndTermContext<'input>{}

pub struct AndTermContextExt<'input>{
	base:TermContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{AndTermContextExt<'a>}

impl<'input> TaurusParserContext<'input> for AndTermContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for AndTermContext<'input>{
	fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_AndTerm(self);
	}
}

impl<'input> CustomRuleContext<'input> for AndTermContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_term }
	//fn type_rule_index() -> usize where Self: Sized { RULE_term }
}

impl<'input> Borrow<TermContextExt<'input>> for AndTermContext<'input>{
	fn borrow(&self) -> &TermContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<TermContextExt<'input>> for AndTermContext<'input>{
	fn borrow_mut(&mut self) -> &mut TermContextExt<'input> { &mut self.base }
}

impl<'input> TermContextAttrs<'input> for AndTermContext<'input> {}

impl<'input> AndTermContextExt<'input>{
	fn new(ctx: &dyn TermContextAttrs<'input>) -> Rc<TermContextAll<'input>>  {
		Rc::new(
			TermContextAll::AndTermContext(
				BaseParserRuleContext::copy_from(ctx,AndTermContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type OrTermContext<'input> = BaseParserRuleContext<'input,OrTermContextExt<'input>>;

pub trait OrTermContextAttrs<'input>: TaurusParserContext<'input>{
	fn term_all(&self) ->  Vec<Rc<TermContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn term(&self, i: usize) -> Option<Rc<TermContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
	/// Retrieves first TerminalNode corresponding to token OR
	/// Returns `None` if there is no child corresponding to token OR
	fn OR(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(OR, 0)
	}
}

impl<'input> OrTermContextAttrs<'input> for OrTermContext<'input>{}

pub struct OrTermContextExt<'input>{
	base:TermContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{OrTermContextExt<'a>}

impl<'input> TaurusParserContext<'input> for OrTermContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for OrTermContext<'input>{
	fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_OrTerm(self);
	}
}

impl<'input> CustomRuleContext<'input> for OrTermContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_term }
	//fn type_rule_index() -> usize where Self: Sized { RULE_term }
}

impl<'input> Borrow<TermContextExt<'input>> for OrTermContext<'input>{
	fn borrow(&self) -> &TermContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<TermContextExt<'input>> for OrTermContext<'input>{
	fn borrow_mut(&mut self) -> &mut TermContextExt<'input> { &mut self.base }
}

impl<'input> TermContextAttrs<'input> for OrTermContext<'input> {}

impl<'input> OrTermContextExt<'input>{
	fn new(ctx: &dyn TermContextAttrs<'input>) -> Rc<TermContextAll<'input>>  {
		Rc::new(
			TermContextAll::OrTermContext(
				BaseParserRuleContext::copy_from(ctx,OrTermContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type EqTermContext<'input> = BaseParserRuleContext<'input,EqTermContextExt<'input>>;

pub trait EqTermContextAttrs<'input>: TaurusParserContext<'input>{
	fn term_all(&self) ->  Vec<Rc<TermContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn term(&self, i: usize) -> Option<Rc<TermContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
	fn eqOp(&self) -> Option<Rc<EqOpContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> EqTermContextAttrs<'input> for EqTermContext<'input>{}

pub struct EqTermContextExt<'input>{
	base:TermContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{EqTermContextExt<'a>}

impl<'input> TaurusParserContext<'input> for EqTermContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for EqTermContext<'input>{
	fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_EqTerm(self);
	}
}

impl<'input> CustomRuleContext<'input> for EqTermContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_term }
	//fn type_rule_index() -> usize where Self: Sized { RULE_term }
}

impl<'input> Borrow<TermContextExt<'input>> for EqTermContext<'input>{
	fn borrow(&self) -> &TermContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<TermContextExt<'input>> for EqTermContext<'input>{
	fn borrow_mut(&mut self) -> &mut TermContextExt<'input> { &mut self.base }
}

impl<'input> TermContextAttrs<'input> for EqTermContext<'input> {}

impl<'input> EqTermContextExt<'input>{
	fn new(ctx: &dyn TermContextAttrs<'input>) -> Rc<TermContextAll<'input>>  {
		Rc::new(
			TermContextAll::EqTermContext(
				BaseParserRuleContext::copy_from(ctx,EqTermContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type AriTermContext<'input> = BaseParserRuleContext<'input,AriTermContextExt<'input>>;

pub trait AriTermContextAttrs<'input>: TaurusParserContext<'input>{
	fn arithTerm(&self) -> Option<Rc<ArithTermContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> AriTermContextAttrs<'input> for AriTermContext<'input>{}

pub struct AriTermContextExt<'input>{
	base:TermContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{AriTermContextExt<'a>}

impl<'input> TaurusParserContext<'input> for AriTermContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for AriTermContext<'input>{
	fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_AriTerm(self);
	}
}

impl<'input> CustomRuleContext<'input> for AriTermContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_term }
	//fn type_rule_index() -> usize where Self: Sized { RULE_term }
}

impl<'input> Borrow<TermContextExt<'input>> for AriTermContext<'input>{
	fn borrow(&self) -> &TermContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<TermContextExt<'input>> for AriTermContext<'input>{
	fn borrow_mut(&mut self) -> &mut TermContextExt<'input> { &mut self.base }
}

impl<'input> TermContextAttrs<'input> for AriTermContext<'input> {}

impl<'input> AriTermContextExt<'input>{
	fn new(ctx: &dyn TermContextAttrs<'input>) -> Rc<TermContextAll<'input>>  {
		Rc::new(
			TermContextAll::AriTermContext(
				BaseParserRuleContext::copy_from(ctx,AriTermContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type ParTermContext<'input> = BaseParserRuleContext<'input,ParTermContextExt<'input>>;

pub trait ParTermContextAttrs<'input>: TaurusParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token LPAR
	/// Returns `None` if there is no child corresponding to token LPAR
	fn LPAR(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(LPAR, 0)
	}
	fn term(&self) -> Option<Rc<TermContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token RPAR
	/// Returns `None` if there is no child corresponding to token RPAR
	fn RPAR(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(RPAR, 0)
	}
}

impl<'input> ParTermContextAttrs<'input> for ParTermContext<'input>{}

pub struct ParTermContextExt<'input>{
	base:TermContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{ParTermContextExt<'a>}

impl<'input> TaurusParserContext<'input> for ParTermContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for ParTermContext<'input>{
	fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_ParTerm(self);
	}
}

impl<'input> CustomRuleContext<'input> for ParTermContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_term }
	//fn type_rule_index() -> usize where Self: Sized { RULE_term }
}

impl<'input> Borrow<TermContextExt<'input>> for ParTermContext<'input>{
	fn borrow(&self) -> &TermContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<TermContextExt<'input>> for ParTermContext<'input>{
	fn borrow_mut(&mut self) -> &mut TermContextExt<'input> { &mut self.base }
}

impl<'input> TermContextAttrs<'input> for ParTermContext<'input> {}

impl<'input> ParTermContextExt<'input>{
	fn new(ctx: &dyn TermContextAttrs<'input>) -> Rc<TermContextAll<'input>>  {
		Rc::new(
			TermContextAll::ParTermContext(
				BaseParserRuleContext::copy_from(ctx,ParTermContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I, H> TaurusParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn  term(&mut self,)
	-> Result<Rc<TermContextAll<'input>>,ANTLRError> {
		self.term_rec(0)
	}

	fn term_rec(&mut self, _p: isize)
	-> Result<Rc<TermContextAll<'input>>,ANTLRError> {
		let recog = self;
		let _parentctx = recog.ctx.take();
		let _parentState = recog.base.get_state();
		let mut _localctx = TermContextExt::new(_parentctx.clone(), recog.base.get_state());
		recog.base.enter_recursion_rule(_localctx.clone(), 38, RULE_term, _p);
	    let mut _localctx: Rc<TermContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
		let _startState = 38;
		let result: Result<(), ANTLRError> = (|| {
			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(410);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(33,&mut recog.base)? {
				1 =>{
					{
					let mut tmp = AriTermContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();


					/*InvokeRule arithTerm*/
					recog.base.set_state(405);
					recog.arithTerm_rec(0)?;

					}
				}
			,
				2 =>{
					{
					let mut tmp = ParTermContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();
					recog.base.set_state(406);
					recog.base.match_token(LPAR,&mut recog.err_handler)?;

					/*InvokeRule term*/
					recog.base.set_state(407);
					recog.term_rec(0)?;

					recog.base.set_state(408);
					recog.base.match_token(RPAR,&mut recog.err_handler)?;

					}
				}

				_ => {}
			}

			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(428);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(35,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					recog.trigger_exit_rule_event();
					_prevctx = _localctx.clone();
					{
					recog.base.set_state(426);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(34,&mut recog.base)? {
						1 =>{
							{
							/*recRuleLabeledAltStartAction*/
							let mut tmp = OrdTermContextExt::new(&**TermContextExt::new(_parentctx.clone(), _parentState));
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_term);
							_localctx = tmp;
							recog.base.set_state(412);
							if !({recog.precpred(None, 4)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 4)".to_owned()), None))?;
							}
							/*InvokeRule ordOp*/
							recog.base.set_state(413);
							recog.ordOp()?;

							/*InvokeRule term*/
							recog.base.set_state(414);
							recog.term_rec(5)?;

							}
						}
					,
						2 =>{
							{
							/*recRuleLabeledAltStartAction*/
							let mut tmp = EqTermContextExt::new(&**TermContextExt::new(_parentctx.clone(), _parentState));
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_term);
							_localctx = tmp;
							recog.base.set_state(416);
							if !({recog.precpred(None, 3)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 3)".to_owned()), None))?;
							}
							/*InvokeRule eqOp*/
							recog.base.set_state(417);
							recog.eqOp()?;

							/*InvokeRule term*/
							recog.base.set_state(418);
							recog.term_rec(4)?;

							}
						}
					,
						3 =>{
							{
							/*recRuleLabeledAltStartAction*/
							let mut tmp = AndTermContextExt::new(&**TermContextExt::new(_parentctx.clone(), _parentState));
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_term);
							_localctx = tmp;
							recog.base.set_state(420);
							if !({recog.precpred(None, 2)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 2)".to_owned()), None))?;
							}
							recog.base.set_state(421);
							recog.base.match_token(AND,&mut recog.err_handler)?;

							/*InvokeRule term*/
							recog.base.set_state(422);
							recog.term_rec(3)?;

							}
						}
					,
						4 =>{
							{
							/*recRuleLabeledAltStartAction*/
							let mut tmp = OrTermContextExt::new(&**TermContextExt::new(_parentctx.clone(), _parentState));
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_term);
							_localctx = tmp;
							recog.base.set_state(423);
							if !({recog.precpred(None, 1)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 1)".to_owned()), None))?;
							}
							recog.base.set_state(424);
							recog.base.match_token(OR,&mut recog.err_handler)?;

							/*InvokeRule term*/
							recog.base.set_state(425);
							recog.term_rec(2)?;

							}
						}

						_ => {}
					}
					} 
				}
				recog.base.set_state(430);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(35,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_) => {},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re)=>{
			//_localctx.exception = re;
			recog.err_handler.report_error(&mut recog.base, re);
	        recog.err_handler.recover(&mut recog.base, re)?;}
		}
		recog.base.unroll_recursion_context(_parentctx);

		Ok(_localctx)
	}
}
//------------------- pred ----------------
#[derive(Debug)]
pub enum PredContextAll<'input>{
	DisPredContext(DisPredContext<'input>),
	LengthPredContext(LengthPredContext<'input>),
	QuantiPredContext(QuantiPredContext<'input>),
	XorPredContext(XorPredContext<'input>),
	ParPredContext(ParPredContext<'input>),
	TruePredContext(TruePredContext<'input>),
	NotPredContext(NotPredContext<'input>),
	CallPredContext(CallPredContext<'input>),
	IffPredContext(IffPredContext<'input>),
	CmpPredContext(CmpPredContext<'input>),
	FalsePredContext(FalsePredContext<'input>),
	ImpPredContext(ImpPredContext<'input>),
	ConPredContext(ConPredContext<'input>),
Error(PredContext<'input>)
}
antlr_rust::tid!{PredContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for PredContextAll<'input>{}

impl<'input> TaurusParserContext<'input> for PredContextAll<'input>{}

impl<'input> Deref for PredContextAll<'input>{
	type Target = dyn PredContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use PredContextAll::*;
		match self{
			DisPredContext(inner) => inner,
			LengthPredContext(inner) => inner,
			QuantiPredContext(inner) => inner,
			XorPredContext(inner) => inner,
			ParPredContext(inner) => inner,
			TruePredContext(inner) => inner,
			NotPredContext(inner) => inner,
			CallPredContext(inner) => inner,
			IffPredContext(inner) => inner,
			CmpPredContext(inner) => inner,
			FalsePredContext(inner) => inner,
			ImpPredContext(inner) => inner,
			ConPredContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for PredContextAll<'input>{
    fn enter(&self, listener: &mut (dyn TaurusListener<'input> + 'a)) { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn TaurusListener<'input> + 'a)) { self.deref().exit(listener) }
}



pub type PredContext<'input> = BaseParserRuleContext<'input,PredContextExt<'input>>;

#[derive(Clone)]
pub struct PredContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TaurusParserContext<'input> for PredContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for PredContext<'input>{
}

impl<'input> CustomRuleContext<'input> for PredContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_pred }
	//fn type_rule_index() -> usize where Self: Sized { RULE_pred }
}
antlr_rust::tid!{PredContextExt<'a>}

impl<'input> PredContextExt<'input>{
	fn new(parent: Option<Rc<dyn TaurusParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<PredContextAll<'input>> {
		Rc::new(
		PredContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PredContextExt{
				ph:PhantomData
			}),
		)
		)
	}
}

pub trait PredContextAttrs<'input>: TaurusParserContext<'input> + BorrowMut<PredContextExt<'input>>{


}

impl<'input> PredContextAttrs<'input> for PredContext<'input>{}

pub type DisPredContext<'input> = BaseParserRuleContext<'input,DisPredContextExt<'input>>;

pub trait DisPredContextAttrs<'input>: TaurusParserContext<'input>{
	fn pred_all(&self) ->  Vec<Rc<PredContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn pred(&self, i: usize) -> Option<Rc<PredContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
	/// Retrieves first TerminalNode corresponding to token OR
	/// Returns `None` if there is no child corresponding to token OR
	fn OR(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(OR, 0)
	}
}

impl<'input> DisPredContextAttrs<'input> for DisPredContext<'input>{}

pub struct DisPredContextExt<'input>{
	base:PredContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{DisPredContextExt<'a>}

impl<'input> TaurusParserContext<'input> for DisPredContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for DisPredContext<'input>{
	fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_DisPred(self);
	}
}

impl<'input> CustomRuleContext<'input> for DisPredContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_pred }
	//fn type_rule_index() -> usize where Self: Sized { RULE_pred }
}

impl<'input> Borrow<PredContextExt<'input>> for DisPredContext<'input>{
	fn borrow(&self) -> &PredContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<PredContextExt<'input>> for DisPredContext<'input>{
	fn borrow_mut(&mut self) -> &mut PredContextExt<'input> { &mut self.base }
}

impl<'input> PredContextAttrs<'input> for DisPredContext<'input> {}

impl<'input> DisPredContextExt<'input>{
	fn new(ctx: &dyn PredContextAttrs<'input>) -> Rc<PredContextAll<'input>>  {
		Rc::new(
			PredContextAll::DisPredContext(
				BaseParserRuleContext::copy_from(ctx,DisPredContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type LengthPredContext<'input> = BaseParserRuleContext<'input,LengthPredContextExt<'input>>;

pub trait LengthPredContextAttrs<'input>: TaurusParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token VALID
	/// Returns `None` if there is no child corresponding to token VALID
	fn VALID(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(VALID, 0)
	}
	/// Retrieves all `TerminalNode`s corresponding to token LPAR in current rule
	fn LPAR_all(&self) -> Vec<Rc<TerminalNode<'input,TaurusParserContextType>>>  where Self:Sized{
		self.children_of_type()
	}
	/// Retrieves 'i's TerminalNode corresponding to token LPAR, starting from 0.
	/// Returns `None` if number of children corresponding to token LPAR is less or equal than `i`.
	fn LPAR(&self, i: usize) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(LPAR, i)
	}
	/// Retrieves first TerminalNode corresponding to token IDENT
	/// Returns `None` if there is no child corresponding to token IDENT
	fn IDENT(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(IDENT, 0)
	}
	/// Retrieves first TerminalNode corresponding to token ADD
	/// Returns `None` if there is no child corresponding to token ADD
	fn ADD(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(ADD, 0)
	}
	/// Retrieves first TerminalNode corresponding to token INT_CONSTANT
	/// Returns `None` if there is no child corresponding to token INT_CONSTANT
	fn INT_CONSTANT(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(INT_CONSTANT, 0)
	}
	/// Retrieves first TerminalNode corresponding to token APOSTROPHE
	/// Returns `None` if there is no child corresponding to token APOSTROPHE
	fn APOSTROPHE(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(APOSTROPHE, 0)
	}
	fn arithTerm(&self) -> Option<Rc<ArithTermContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves all `TerminalNode`s corresponding to token RPAR in current rule
	fn RPAR_all(&self) -> Vec<Rc<TerminalNode<'input,TaurusParserContextType>>>  where Self:Sized{
		self.children_of_type()
	}
	/// Retrieves 'i's TerminalNode corresponding to token RPAR, starting from 0.
	/// Returns `None` if number of children corresponding to token RPAR is less or equal than `i`.
	fn RPAR(&self, i: usize) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(RPAR, i)
	}
}

impl<'input> LengthPredContextAttrs<'input> for LengthPredContext<'input>{}

pub struct LengthPredContextExt<'input>{
	base:PredContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{LengthPredContextExt<'a>}

impl<'input> TaurusParserContext<'input> for LengthPredContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for LengthPredContext<'input>{
	fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_LengthPred(self);
	}
}

impl<'input> CustomRuleContext<'input> for LengthPredContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_pred }
	//fn type_rule_index() -> usize where Self: Sized { RULE_pred }
}

impl<'input> Borrow<PredContextExt<'input>> for LengthPredContext<'input>{
	fn borrow(&self) -> &PredContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<PredContextExt<'input>> for LengthPredContext<'input>{
	fn borrow_mut(&mut self) -> &mut PredContextExt<'input> { &mut self.base }
}

impl<'input> PredContextAttrs<'input> for LengthPredContext<'input> {}

impl<'input> LengthPredContextExt<'input>{
	fn new(ctx: &dyn PredContextAttrs<'input>) -> Rc<PredContextAll<'input>>  {
		Rc::new(
			PredContextAll::LengthPredContext(
				BaseParserRuleContext::copy_from(ctx,LengthPredContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type QuantiPredContext<'input> = BaseParserRuleContext<'input,QuantiPredContextExt<'input>>;

pub trait QuantiPredContextAttrs<'input>: TaurusParserContext<'input>{
	fn quantifier(&self) -> Option<Rc<QuantifierContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	fn binder_all(&self) ->  Vec<Rc<BinderContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn binder(&self, i: usize) -> Option<Rc<BinderContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
	/// Retrieves first TerminalNode corresponding to token SEMICOLON
	/// Returns `None` if there is no child corresponding to token SEMICOLON
	fn SEMICOLON(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(SEMICOLON, 0)
	}
	fn pred(&self) -> Option<Rc<PredContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
	fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,TaurusParserContextType>>>  where Self:Sized{
		self.children_of_type()
	}
	/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
	/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
	fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(COMMA, i)
	}
}

impl<'input> QuantiPredContextAttrs<'input> for QuantiPredContext<'input>{}

pub struct QuantiPredContextExt<'input>{
	base:PredContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{QuantiPredContextExt<'a>}

impl<'input> TaurusParserContext<'input> for QuantiPredContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for QuantiPredContext<'input>{
	fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_QuantiPred(self);
	}
}

impl<'input> CustomRuleContext<'input> for QuantiPredContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_pred }
	//fn type_rule_index() -> usize where Self: Sized { RULE_pred }
}

impl<'input> Borrow<PredContextExt<'input>> for QuantiPredContext<'input>{
	fn borrow(&self) -> &PredContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<PredContextExt<'input>> for QuantiPredContext<'input>{
	fn borrow_mut(&mut self) -> &mut PredContextExt<'input> { &mut self.base }
}

impl<'input> PredContextAttrs<'input> for QuantiPredContext<'input> {}

impl<'input> QuantiPredContextExt<'input>{
	fn new(ctx: &dyn PredContextAttrs<'input>) -> Rc<PredContextAll<'input>>  {
		Rc::new(
			PredContextAll::QuantiPredContext(
				BaseParserRuleContext::copy_from(ctx,QuantiPredContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type XorPredContext<'input> = BaseParserRuleContext<'input,XorPredContextExt<'input>>;

pub trait XorPredContextAttrs<'input>: TaurusParserContext<'input>{
	fn pred_all(&self) ->  Vec<Rc<PredContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn pred(&self, i: usize) -> Option<Rc<PredContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
	/// Retrieves first TerminalNode corresponding to token XOR
	/// Returns `None` if there is no child corresponding to token XOR
	fn XOR(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(XOR, 0)
	}
}

impl<'input> XorPredContextAttrs<'input> for XorPredContext<'input>{}

pub struct XorPredContextExt<'input>{
	base:PredContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{XorPredContextExt<'a>}

impl<'input> TaurusParserContext<'input> for XorPredContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for XorPredContext<'input>{
	fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_XorPred(self);
	}
}

impl<'input> CustomRuleContext<'input> for XorPredContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_pred }
	//fn type_rule_index() -> usize where Self: Sized { RULE_pred }
}

impl<'input> Borrow<PredContextExt<'input>> for XorPredContext<'input>{
	fn borrow(&self) -> &PredContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<PredContextExt<'input>> for XorPredContext<'input>{
	fn borrow_mut(&mut self) -> &mut PredContextExt<'input> { &mut self.base }
}

impl<'input> PredContextAttrs<'input> for XorPredContext<'input> {}

impl<'input> XorPredContextExt<'input>{
	fn new(ctx: &dyn PredContextAttrs<'input>) -> Rc<PredContextAll<'input>>  {
		Rc::new(
			PredContextAll::XorPredContext(
				BaseParserRuleContext::copy_from(ctx,XorPredContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type ParPredContext<'input> = BaseParserRuleContext<'input,ParPredContextExt<'input>>;

pub trait ParPredContextAttrs<'input>: TaurusParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token LPAR
	/// Returns `None` if there is no child corresponding to token LPAR
	fn LPAR(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(LPAR, 0)
	}
	fn pred(&self) -> Option<Rc<PredContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
	/// Retrieves first TerminalNode corresponding to token RPAR
	/// Returns `None` if there is no child corresponding to token RPAR
	fn RPAR(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(RPAR, 0)
	}
}

impl<'input> ParPredContextAttrs<'input> for ParPredContext<'input>{}

pub struct ParPredContextExt<'input>{
	base:PredContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{ParPredContextExt<'a>}

impl<'input> TaurusParserContext<'input> for ParPredContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for ParPredContext<'input>{
	fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_ParPred(self);
	}
}

impl<'input> CustomRuleContext<'input> for ParPredContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_pred }
	//fn type_rule_index() -> usize where Self: Sized { RULE_pred }
}

impl<'input> Borrow<PredContextExt<'input>> for ParPredContext<'input>{
	fn borrow(&self) -> &PredContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<PredContextExt<'input>> for ParPredContext<'input>{
	fn borrow_mut(&mut self) -> &mut PredContextExt<'input> { &mut self.base }
}

impl<'input> PredContextAttrs<'input> for ParPredContext<'input> {}

impl<'input> ParPredContextExt<'input>{
	fn new(ctx: &dyn PredContextAttrs<'input>) -> Rc<PredContextAll<'input>>  {
		Rc::new(
			PredContextAll::ParPredContext(
				BaseParserRuleContext::copy_from(ctx,ParPredContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type TruePredContext<'input> = BaseParserRuleContext<'input,TruePredContextExt<'input>>;

pub trait TruePredContextAttrs<'input>: TaurusParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token ANNO_TRUE
	/// Returns `None` if there is no child corresponding to token ANNO_TRUE
	fn ANNO_TRUE(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(ANNO_TRUE, 0)
	}
}

impl<'input> TruePredContextAttrs<'input> for TruePredContext<'input>{}

pub struct TruePredContextExt<'input>{
	base:PredContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{TruePredContextExt<'a>}

impl<'input> TaurusParserContext<'input> for TruePredContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for TruePredContext<'input>{
	fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_TruePred(self);
	}
}

impl<'input> CustomRuleContext<'input> for TruePredContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_pred }
	//fn type_rule_index() -> usize where Self: Sized { RULE_pred }
}

impl<'input> Borrow<PredContextExt<'input>> for TruePredContext<'input>{
	fn borrow(&self) -> &PredContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<PredContextExt<'input>> for TruePredContext<'input>{
	fn borrow_mut(&mut self) -> &mut PredContextExt<'input> { &mut self.base }
}

impl<'input> PredContextAttrs<'input> for TruePredContext<'input> {}

impl<'input> TruePredContextExt<'input>{
	fn new(ctx: &dyn PredContextAttrs<'input>) -> Rc<PredContextAll<'input>>  {
		Rc::new(
			PredContextAll::TruePredContext(
				BaseParserRuleContext::copy_from(ctx,TruePredContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type NotPredContext<'input> = BaseParserRuleContext<'input,NotPredContextExt<'input>>;

pub trait NotPredContextAttrs<'input>: TaurusParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token NOT
	/// Returns `None` if there is no child corresponding to token NOT
	fn NOT(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(NOT, 0)
	}
	fn pred(&self) -> Option<Rc<PredContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> NotPredContextAttrs<'input> for NotPredContext<'input>{}

pub struct NotPredContextExt<'input>{
	base:PredContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{NotPredContextExt<'a>}

impl<'input> TaurusParserContext<'input> for NotPredContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for NotPredContext<'input>{
	fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_NotPred(self);
	}
}

impl<'input> CustomRuleContext<'input> for NotPredContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_pred }
	//fn type_rule_index() -> usize where Self: Sized { RULE_pred }
}

impl<'input> Borrow<PredContextExt<'input>> for NotPredContext<'input>{
	fn borrow(&self) -> &PredContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<PredContextExt<'input>> for NotPredContext<'input>{
	fn borrow_mut(&mut self) -> &mut PredContextExt<'input> { &mut self.base }
}

impl<'input> PredContextAttrs<'input> for NotPredContext<'input> {}

impl<'input> NotPredContextExt<'input>{
	fn new(ctx: &dyn PredContextAttrs<'input>) -> Rc<PredContextAll<'input>>  {
		Rc::new(
			PredContextAll::NotPredContext(
				BaseParserRuleContext::copy_from(ctx,NotPredContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type CallPredContext<'input> = BaseParserRuleContext<'input,CallPredContextExt<'input>>;

pub trait CallPredContextAttrs<'input>: TaurusParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token IDENT
	/// Returns `None` if there is no child corresponding to token IDENT
	fn IDENT(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(IDENT, 0)
	}
	/// Retrieves first TerminalNode corresponding to token LPAR
	/// Returns `None` if there is no child corresponding to token LPAR
	fn LPAR(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(LPAR, 0)
	}
	fn term_all(&self) ->  Vec<Rc<TermContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn term(&self, i: usize) -> Option<Rc<TermContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
	/// Retrieves first TerminalNode corresponding to token RPAR
	/// Returns `None` if there is no child corresponding to token RPAR
	fn RPAR(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(RPAR, 0)
	}
	/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
	fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,TaurusParserContextType>>>  where Self:Sized{
		self.children_of_type()
	}
	/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
	/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
	fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(COMMA, i)
	}
}

impl<'input> CallPredContextAttrs<'input> for CallPredContext<'input>{}

pub struct CallPredContextExt<'input>{
	base:PredContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{CallPredContextExt<'a>}

impl<'input> TaurusParserContext<'input> for CallPredContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for CallPredContext<'input>{
	fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_CallPred(self);
	}
}

impl<'input> CustomRuleContext<'input> for CallPredContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_pred }
	//fn type_rule_index() -> usize where Self: Sized { RULE_pred }
}

impl<'input> Borrow<PredContextExt<'input>> for CallPredContext<'input>{
	fn borrow(&self) -> &PredContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<PredContextExt<'input>> for CallPredContext<'input>{
	fn borrow_mut(&mut self) -> &mut PredContextExt<'input> { &mut self.base }
}

impl<'input> PredContextAttrs<'input> for CallPredContext<'input> {}

impl<'input> CallPredContextExt<'input>{
	fn new(ctx: &dyn PredContextAttrs<'input>) -> Rc<PredContextAll<'input>>  {
		Rc::new(
			PredContextAll::CallPredContext(
				BaseParserRuleContext::copy_from(ctx,CallPredContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type IffPredContext<'input> = BaseParserRuleContext<'input,IffPredContextExt<'input>>;

pub trait IffPredContextAttrs<'input>: TaurusParserContext<'input>{
	fn pred_all(&self) ->  Vec<Rc<PredContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn pred(&self, i: usize) -> Option<Rc<PredContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
	/// Retrieves first TerminalNode corresponding to token EQUIV
	/// Returns `None` if there is no child corresponding to token EQUIV
	fn EQUIV(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(EQUIV, 0)
	}
}

impl<'input> IffPredContextAttrs<'input> for IffPredContext<'input>{}

pub struct IffPredContextExt<'input>{
	base:PredContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{IffPredContextExt<'a>}

impl<'input> TaurusParserContext<'input> for IffPredContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for IffPredContext<'input>{
	fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_IffPred(self);
	}
}

impl<'input> CustomRuleContext<'input> for IffPredContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_pred }
	//fn type_rule_index() -> usize where Self: Sized { RULE_pred }
}

impl<'input> Borrow<PredContextExt<'input>> for IffPredContext<'input>{
	fn borrow(&self) -> &PredContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<PredContextExt<'input>> for IffPredContext<'input>{
	fn borrow_mut(&mut self) -> &mut PredContextExt<'input> { &mut self.base }
}

impl<'input> PredContextAttrs<'input> for IffPredContext<'input> {}

impl<'input> IffPredContextExt<'input>{
	fn new(ctx: &dyn PredContextAttrs<'input>) -> Rc<PredContextAll<'input>>  {
		Rc::new(
			PredContextAll::IffPredContext(
				BaseParserRuleContext::copy_from(ctx,IffPredContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type CmpPredContext<'input> = BaseParserRuleContext<'input,CmpPredContextExt<'input>>;

pub trait CmpPredContextAttrs<'input>: TaurusParserContext<'input>{
	fn arithTerm_all(&self) ->  Vec<Rc<ArithTermContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn arithTerm(&self, i: usize) -> Option<Rc<ArithTermContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
	fn cmpOp_all(&self) ->  Vec<Rc<CmpOpContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn cmpOp(&self, i: usize) -> Option<Rc<CmpOpContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
}

impl<'input> CmpPredContextAttrs<'input> for CmpPredContext<'input>{}

pub struct CmpPredContextExt<'input>{
	base:PredContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{CmpPredContextExt<'a>}

impl<'input> TaurusParserContext<'input> for CmpPredContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for CmpPredContext<'input>{
	fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_CmpPred(self);
	}
}

impl<'input> CustomRuleContext<'input> for CmpPredContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_pred }
	//fn type_rule_index() -> usize where Self: Sized { RULE_pred }
}

impl<'input> Borrow<PredContextExt<'input>> for CmpPredContext<'input>{
	fn borrow(&self) -> &PredContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<PredContextExt<'input>> for CmpPredContext<'input>{
	fn borrow_mut(&mut self) -> &mut PredContextExt<'input> { &mut self.base }
}

impl<'input> PredContextAttrs<'input> for CmpPredContext<'input> {}

impl<'input> CmpPredContextExt<'input>{
	fn new(ctx: &dyn PredContextAttrs<'input>) -> Rc<PredContextAll<'input>>  {
		Rc::new(
			PredContextAll::CmpPredContext(
				BaseParserRuleContext::copy_from(ctx,CmpPredContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type FalsePredContext<'input> = BaseParserRuleContext<'input,FalsePredContextExt<'input>>;

pub trait FalsePredContextAttrs<'input>: TaurusParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token ANNO_FALSE
	/// Returns `None` if there is no child corresponding to token ANNO_FALSE
	fn ANNO_FALSE(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(ANNO_FALSE, 0)
	}
}

impl<'input> FalsePredContextAttrs<'input> for FalsePredContext<'input>{}

pub struct FalsePredContextExt<'input>{
	base:PredContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{FalsePredContextExt<'a>}

impl<'input> TaurusParserContext<'input> for FalsePredContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for FalsePredContext<'input>{
	fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_FalsePred(self);
	}
}

impl<'input> CustomRuleContext<'input> for FalsePredContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_pred }
	//fn type_rule_index() -> usize where Self: Sized { RULE_pred }
}

impl<'input> Borrow<PredContextExt<'input>> for FalsePredContext<'input>{
	fn borrow(&self) -> &PredContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<PredContextExt<'input>> for FalsePredContext<'input>{
	fn borrow_mut(&mut self) -> &mut PredContextExt<'input> { &mut self.base }
}

impl<'input> PredContextAttrs<'input> for FalsePredContext<'input> {}

impl<'input> FalsePredContextExt<'input>{
	fn new(ctx: &dyn PredContextAttrs<'input>) -> Rc<PredContextAll<'input>>  {
		Rc::new(
			PredContextAll::FalsePredContext(
				BaseParserRuleContext::copy_from(ctx,FalsePredContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type ImpPredContext<'input> = BaseParserRuleContext<'input,ImpPredContextExt<'input>>;

pub trait ImpPredContextAttrs<'input>: TaurusParserContext<'input>{
	fn pred_all(&self) ->  Vec<Rc<PredContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn pred(&self, i: usize) -> Option<Rc<PredContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
	/// Retrieves first TerminalNode corresponding to token IMPLY
	/// Returns `None` if there is no child corresponding to token IMPLY
	fn IMPLY(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(IMPLY, 0)
	}
}

impl<'input> ImpPredContextAttrs<'input> for ImpPredContext<'input>{}

pub struct ImpPredContextExt<'input>{
	base:PredContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{ImpPredContextExt<'a>}

impl<'input> TaurusParserContext<'input> for ImpPredContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for ImpPredContext<'input>{
	fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_ImpPred(self);
	}
}

impl<'input> CustomRuleContext<'input> for ImpPredContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_pred }
	//fn type_rule_index() -> usize where Self: Sized { RULE_pred }
}

impl<'input> Borrow<PredContextExt<'input>> for ImpPredContext<'input>{
	fn borrow(&self) -> &PredContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<PredContextExt<'input>> for ImpPredContext<'input>{
	fn borrow_mut(&mut self) -> &mut PredContextExt<'input> { &mut self.base }
}

impl<'input> PredContextAttrs<'input> for ImpPredContext<'input> {}

impl<'input> ImpPredContextExt<'input>{
	fn new(ctx: &dyn PredContextAttrs<'input>) -> Rc<PredContextAll<'input>>  {
		Rc::new(
			PredContextAll::ImpPredContext(
				BaseParserRuleContext::copy_from(ctx,ImpPredContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type ConPredContext<'input> = BaseParserRuleContext<'input,ConPredContextExt<'input>>;

pub trait ConPredContextAttrs<'input>: TaurusParserContext<'input>{
	fn pred_all(&self) ->  Vec<Rc<PredContextAll<'input>>> where Self:Sized{
		self.children_of_type()
	}
	fn pred(&self, i: usize) -> Option<Rc<PredContextAll<'input>>> where Self:Sized{
		self.child_of_type(i)
	}
	/// Retrieves first TerminalNode corresponding to token AND
	/// Returns `None` if there is no child corresponding to token AND
	fn AND(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(AND, 0)
	}
}

impl<'input> ConPredContextAttrs<'input> for ConPredContext<'input>{}

pub struct ConPredContextExt<'input>{
	base:PredContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{ConPredContextExt<'a>}

impl<'input> TaurusParserContext<'input> for ConPredContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for ConPredContext<'input>{
	fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_ConPred(self);
	}
}

impl<'input> CustomRuleContext<'input> for ConPredContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_pred }
	//fn type_rule_index() -> usize where Self: Sized { RULE_pred }
}

impl<'input> Borrow<PredContextExt<'input>> for ConPredContext<'input>{
	fn borrow(&self) -> &PredContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<PredContextExt<'input>> for ConPredContext<'input>{
	fn borrow_mut(&mut self) -> &mut PredContextExt<'input> { &mut self.base }
}

impl<'input> PredContextAttrs<'input> for ConPredContext<'input> {}

impl<'input> ConPredContextExt<'input>{
	fn new(ctx: &dyn PredContextAttrs<'input>) -> Rc<PredContextAll<'input>>  {
		Rc::new(
			PredContextAll::ConPredContext(
				BaseParserRuleContext::copy_from(ctx,ConPredContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I, H> TaurusParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn  pred(&mut self,)
	-> Result<Rc<PredContextAll<'input>>,ANTLRError> {
		self.pred_rec(0)
	}

	fn pred_rec(&mut self, _p: isize)
	-> Result<Rc<PredContextAll<'input>>,ANTLRError> {
		let recog = self;
		let _parentctx = recog.ctx.take();
		let _parentState = recog.base.get_state();
		let mut _localctx = PredContextExt::new(_parentctx.clone(), recog.base.get_state());
		recog.base.enter_recursion_rule(_localctx.clone(), 40, RULE_pred, _p);
	    let mut _localctx: Rc<PredContextAll> = _localctx;
        let mut _prevctx = _localctx.clone();
		let _startState = 40;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {
			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(485);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(40,&mut recog.base)? {
				1 =>{
					{
					let mut tmp = TruePredContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();


					recog.base.set_state(432);
					recog.base.match_token(ANNO_TRUE,&mut recog.err_handler)?;

					}
				}
			,
				2 =>{
					{
					let mut tmp = FalsePredContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();
					recog.base.set_state(433);
					recog.base.match_token(ANNO_FALSE,&mut recog.err_handler)?;

					}
				}
			,
				3 =>{
					{
					let mut tmp = CmpPredContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();
					/*InvokeRule arithTerm*/
					recog.base.set_state(434);
					recog.arithTerm_rec(0)?;

					recog.base.set_state(438); 
					recog.err_handler.sync(&mut recog.base)?;
					_alt = 1;
					loop {
						match _alt {
						    x if x == 1=>
							{
							{
							/*InvokeRule cmpOp*/
							recog.base.set_state(435);
							recog.cmpOp()?;

							/*InvokeRule arithTerm*/
							recog.base.set_state(436);
							recog.arithTerm_rec(0)?;

							}
							}

						_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
						}
						recog.base.set_state(440); 
						recog.err_handler.sync(&mut recog.base)?;
						_alt = recog.interpreter.adaptive_predict(36,&mut recog.base)?;
						if _alt==2 || _alt==INVALID_ALT { break }
					}
					}
				}
			,
				4 =>{
					{
					let mut tmp = CallPredContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();
					recog.base.set_state(442);
					recog.base.match_token(IDENT,&mut recog.err_handler)?;

					recog.base.set_state(454);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(38,&mut recog.base)? {
						x if x == 1=>{
							{
							recog.base.set_state(443);
							recog.base.match_token(LPAR,&mut recog.err_handler)?;

							/*InvokeRule term*/
							recog.base.set_state(444);
							recog.term_rec(0)?;

							recog.base.set_state(449);
							recog.err_handler.sync(&mut recog.base)?;
							_la = recog.base.input.la(1);
							while _la==COMMA {
								{
								{
								recog.base.set_state(445);
								recog.base.match_token(COMMA,&mut recog.err_handler)?;

								/*InvokeRule term*/
								recog.base.set_state(446);
								recog.term_rec(0)?;

								}
								}
								recog.base.set_state(451);
								recog.err_handler.sync(&mut recog.base)?;
								_la = recog.base.input.la(1);
							}
							recog.base.set_state(452);
							recog.base.match_token(RPAR,&mut recog.err_handler)?;

							}
						}

						_ => {}
					}
					}
				}
			,
				5 =>{
					{
					let mut tmp = ParPredContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();
					recog.base.set_state(456);
					recog.base.match_token(LPAR,&mut recog.err_handler)?;

					/*InvokeRule pred*/
					recog.base.set_state(457);
					recog.pred_rec(0)?;

					recog.base.set_state(458);
					recog.base.match_token(RPAR,&mut recog.err_handler)?;

					}
				}
			,
				6 =>{
					{
					let mut tmp = NotPredContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();
					recog.base.set_state(460);
					recog.base.match_token(NOT,&mut recog.err_handler)?;

					/*InvokeRule pred*/
					recog.base.set_state(461);
					recog.pred_rec(4)?;

					}
				}
			,
				7 =>{
					{
					let mut tmp = LengthPredContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();
					recog.base.set_state(462);
					recog.base.match_token(VALID,&mut recog.err_handler)?;

					recog.base.set_state(463);
					recog.base.match_token(LPAR,&mut recog.err_handler)?;

					recog.base.set_state(464);
					recog.base.match_token(IDENT,&mut recog.err_handler)?;

					recog.base.set_state(465);
					recog.base.match_token(ADD,&mut recog.err_handler)?;

					recog.base.set_state(466);
					recog.base.match_token(LPAR,&mut recog.err_handler)?;

					recog.base.set_state(467);
					recog.base.match_token(INT_CONSTANT,&mut recog.err_handler)?;

					recog.base.set_state(468);
					recog.base.match_token(APOSTROPHE,&mut recog.err_handler)?;

					/*InvokeRule arithTerm*/
					recog.base.set_state(469);
					recog.arithTerm_rec(0)?;

					recog.base.set_state(470);
					recog.base.match_token(RPAR,&mut recog.err_handler)?;

					recog.base.set_state(471);
					recog.base.match_token(RPAR,&mut recog.err_handler)?;

					}
				}
			,
				8 =>{
					{
					let mut tmp = QuantiPredContextExt::new(&**_localctx);
					recog.ctx = Some(tmp.clone());
					_localctx = tmp;
					_prevctx = _localctx.clone();
					/*InvokeRule quantifier*/
					recog.base.set_state(473);
					recog.quantifier()?;

					/*InvokeRule binder*/
					recog.base.set_state(474);
					recog.binder()?;

					recog.base.set_state(479);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==COMMA {
						{
						{
						recog.base.set_state(475);
						recog.base.match_token(COMMA,&mut recog.err_handler)?;

						/*InvokeRule binder*/
						recog.base.set_state(476);
						recog.binder()?;

						}
						}
						recog.base.set_state(481);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					recog.base.set_state(482);
					recog.base.match_token(SEMICOLON,&mut recog.err_handler)?;

					/*InvokeRule pred*/
					recog.base.set_state(483);
					recog.pred_rec(1)?;

					}
				}

				_ => {}
			}

			let tmp = recog.input.lt(-1).cloned();
			recog.ctx.as_ref().unwrap().set_stop(tmp);
			recog.base.set_state(504);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(42,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					recog.trigger_exit_rule_event();
					_prevctx = _localctx.clone();
					{
					recog.base.set_state(502);
					recog.err_handler.sync(&mut recog.base)?;
					match  recog.interpreter.adaptive_predict(41,&mut recog.base)? {
						1 =>{
							{
							/*recRuleLabeledAltStartAction*/
							let mut tmp = ConPredContextExt::new(&**PredContextExt::new(_parentctx.clone(), _parentState));
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_pred);
							_localctx = tmp;
							recog.base.set_state(487);
							if !({recog.precpred(None, 8)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 8)".to_owned()), None))?;
							}
							recog.base.set_state(488);
							recog.base.match_token(AND,&mut recog.err_handler)?;

							/*InvokeRule pred*/
							recog.base.set_state(489);
							recog.pred_rec(9)?;

							}
						}
					,
						2 =>{
							{
							/*recRuleLabeledAltStartAction*/
							let mut tmp = DisPredContextExt::new(&**PredContextExt::new(_parentctx.clone(), _parentState));
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_pred);
							_localctx = tmp;
							recog.base.set_state(490);
							if !({recog.precpred(None, 7)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 7)".to_owned()), None))?;
							}
							recog.base.set_state(491);
							recog.base.match_token(OR,&mut recog.err_handler)?;

							/*InvokeRule pred*/
							recog.base.set_state(492);
							recog.pred_rec(8)?;

							}
						}
					,
						3 =>{
							{
							/*recRuleLabeledAltStartAction*/
							let mut tmp = ImpPredContextExt::new(&**PredContextExt::new(_parentctx.clone(), _parentState));
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_pred);
							_localctx = tmp;
							recog.base.set_state(493);
							if !({recog.precpred(None, 6)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 6)".to_owned()), None))?;
							}
							recog.base.set_state(494);
							recog.base.match_token(IMPLY,&mut recog.err_handler)?;

							/*InvokeRule pred*/
							recog.base.set_state(495);
							recog.pred_rec(7)?;

							}
						}
					,
						4 =>{
							{
							/*recRuleLabeledAltStartAction*/
							let mut tmp = IffPredContextExt::new(&**PredContextExt::new(_parentctx.clone(), _parentState));
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_pred);
							_localctx = tmp;
							recog.base.set_state(496);
							if !({recog.precpred(None, 5)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 5)".to_owned()), None))?;
							}
							recog.base.set_state(497);
							recog.base.match_token(EQUIV,&mut recog.err_handler)?;

							/*InvokeRule pred*/
							recog.base.set_state(498);
							recog.pred_rec(6)?;

							}
						}
					,
						5 =>{
							{
							/*recRuleLabeledAltStartAction*/
							let mut tmp = XorPredContextExt::new(&**PredContextExt::new(_parentctx.clone(), _parentState));
							recog.push_new_recursion_context(tmp.clone(), _startState, RULE_pred);
							_localctx = tmp;
							recog.base.set_state(499);
							if !({recog.precpred(None, 3)}) {
								Err(FailedPredicateError::new(&mut recog.base, Some("recog.precpred(None, 3)".to_owned()), None))?;
							}
							recog.base.set_state(500);
							recog.base.match_token(XOR,&mut recog.err_handler)?;

							/*InvokeRule pred*/
							recog.base.set_state(501);
							recog.pred_rec(4)?;

							}
						}

						_ => {}
					}
					} 
				}
				recog.base.set_state(506);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(42,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_) => {},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re)=>{
			//_localctx.exception = re;
			recog.err_handler.report_error(&mut recog.base, re);
	        recog.err_handler.recover(&mut recog.base, re)?;}
		}
		recog.base.unroll_recursion_context(_parentctx);

		Ok(_localctx)
	}
}
//------------------- arithOp ----------------
#[derive(Debug)]
pub enum ArithOpContextAll<'input>{
	MulArithOpContext(MulArithOpContext<'input>),
	AddArithOpContext(AddArithOpContext<'input>),
Error(ArithOpContext<'input>)
}
antlr_rust::tid!{ArithOpContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for ArithOpContextAll<'input>{}

impl<'input> TaurusParserContext<'input> for ArithOpContextAll<'input>{}

impl<'input> Deref for ArithOpContextAll<'input>{
	type Target = dyn ArithOpContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use ArithOpContextAll::*;
		match self{
			MulArithOpContext(inner) => inner,
			AddArithOpContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for ArithOpContextAll<'input>{
    fn enter(&self, listener: &mut (dyn TaurusListener<'input> + 'a)) { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn TaurusListener<'input> + 'a)) { self.deref().exit(listener) }
}



pub type ArithOpContext<'input> = BaseParserRuleContext<'input,ArithOpContextExt<'input>>;

#[derive(Clone)]
pub struct ArithOpContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TaurusParserContext<'input> for ArithOpContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for ArithOpContext<'input>{
}

impl<'input> CustomRuleContext<'input> for ArithOpContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_arithOp }
	//fn type_rule_index() -> usize where Self: Sized { RULE_arithOp }
}
antlr_rust::tid!{ArithOpContextExt<'a>}

impl<'input> ArithOpContextExt<'input>{
	fn new(parent: Option<Rc<dyn TaurusParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ArithOpContextAll<'input>> {
		Rc::new(
		ArithOpContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ArithOpContextExt{
				ph:PhantomData
			}),
		)
		)
	}
}

pub trait ArithOpContextAttrs<'input>: TaurusParserContext<'input> + BorrowMut<ArithOpContextExt<'input>>{


}

impl<'input> ArithOpContextAttrs<'input> for ArithOpContext<'input>{}

pub type MulArithOpContext<'input> = BaseParserRuleContext<'input,MulArithOpContextExt<'input>>;

pub trait MulArithOpContextAttrs<'input>: TaurusParserContext<'input>{
	fn mulOp(&self) -> Option<Rc<MulOpContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> MulArithOpContextAttrs<'input> for MulArithOpContext<'input>{}

pub struct MulArithOpContextExt<'input>{
	base:ArithOpContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{MulArithOpContextExt<'a>}

impl<'input> TaurusParserContext<'input> for MulArithOpContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for MulArithOpContext<'input>{
	fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_MulArithOp(self);
	}
}

impl<'input> CustomRuleContext<'input> for MulArithOpContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_arithOp }
	//fn type_rule_index() -> usize where Self: Sized { RULE_arithOp }
}

impl<'input> Borrow<ArithOpContextExt<'input>> for MulArithOpContext<'input>{
	fn borrow(&self) -> &ArithOpContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ArithOpContextExt<'input>> for MulArithOpContext<'input>{
	fn borrow_mut(&mut self) -> &mut ArithOpContextExt<'input> { &mut self.base }
}

impl<'input> ArithOpContextAttrs<'input> for MulArithOpContext<'input> {}

impl<'input> MulArithOpContextExt<'input>{
	fn new(ctx: &dyn ArithOpContextAttrs<'input>) -> Rc<ArithOpContextAll<'input>>  {
		Rc::new(
			ArithOpContextAll::MulArithOpContext(
				BaseParserRuleContext::copy_from(ctx,MulArithOpContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type AddArithOpContext<'input> = BaseParserRuleContext<'input,AddArithOpContextExt<'input>>;

pub trait AddArithOpContextAttrs<'input>: TaurusParserContext<'input>{
	fn addOp(&self) -> Option<Rc<AddOpContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> AddArithOpContextAttrs<'input> for AddArithOpContext<'input>{}

pub struct AddArithOpContextExt<'input>{
	base:ArithOpContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{AddArithOpContextExt<'a>}

impl<'input> TaurusParserContext<'input> for AddArithOpContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for AddArithOpContext<'input>{
	fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_AddArithOp(self);
	}
}

impl<'input> CustomRuleContext<'input> for AddArithOpContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_arithOp }
	//fn type_rule_index() -> usize where Self: Sized { RULE_arithOp }
}

impl<'input> Borrow<ArithOpContextExt<'input>> for AddArithOpContext<'input>{
	fn borrow(&self) -> &ArithOpContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ArithOpContextExt<'input>> for AddArithOpContext<'input>{
	fn borrow_mut(&mut self) -> &mut ArithOpContextExt<'input> { &mut self.base }
}

impl<'input> ArithOpContextAttrs<'input> for AddArithOpContext<'input> {}

impl<'input> AddArithOpContextExt<'input>{
	fn new(ctx: &dyn ArithOpContextAttrs<'input>) -> Rc<ArithOpContextAll<'input>>  {
		Rc::new(
			ArithOpContextAll::AddArithOpContext(
				BaseParserRuleContext::copy_from(ctx,AddArithOpContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I, H> TaurusParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn arithOp(&mut self,)
	-> Result<Rc<ArithOpContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ArithOpContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 42, RULE_arithOp);
        let mut _localctx: Rc<ArithOpContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(509);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 MUL | DIV | MOD 
				=> {
					let tmp = MulArithOpContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1);
					_localctx = tmp;
					{
					/*InvokeRule mulOp*/
					recog.base.set_state(507);
					recog.mulOp()?;

					}
				}

			 ADD | MINUS 
				=> {
					let tmp = AddArithOpContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2);
					_localctx = tmp;
					{
					/*InvokeRule addOp*/
					recog.base.set_state(508);
					recog.addOp()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- addOp ----------------
#[derive(Debug)]
pub enum AddOpContextAll<'input>{
	PlusContext(PlusContext<'input>),
	MinusContext(MinusContext<'input>),
Error(AddOpContext<'input>)
}
antlr_rust::tid!{AddOpContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for AddOpContextAll<'input>{}

impl<'input> TaurusParserContext<'input> for AddOpContextAll<'input>{}

impl<'input> Deref for AddOpContextAll<'input>{
	type Target = dyn AddOpContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use AddOpContextAll::*;
		match self{
			PlusContext(inner) => inner,
			MinusContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for AddOpContextAll<'input>{
    fn enter(&self, listener: &mut (dyn TaurusListener<'input> + 'a)) { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn TaurusListener<'input> + 'a)) { self.deref().exit(listener) }
}



pub type AddOpContext<'input> = BaseParserRuleContext<'input,AddOpContextExt<'input>>;

#[derive(Clone)]
pub struct AddOpContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TaurusParserContext<'input> for AddOpContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for AddOpContext<'input>{
}

impl<'input> CustomRuleContext<'input> for AddOpContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_addOp }
	//fn type_rule_index() -> usize where Self: Sized { RULE_addOp }
}
antlr_rust::tid!{AddOpContextExt<'a>}

impl<'input> AddOpContextExt<'input>{
	fn new(parent: Option<Rc<dyn TaurusParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<AddOpContextAll<'input>> {
		Rc::new(
		AddOpContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AddOpContextExt{
				ph:PhantomData
			}),
		)
		)
	}
}

pub trait AddOpContextAttrs<'input>: TaurusParserContext<'input> + BorrowMut<AddOpContextExt<'input>>{


}

impl<'input> AddOpContextAttrs<'input> for AddOpContext<'input>{}

pub type PlusContext<'input> = BaseParserRuleContext<'input,PlusContextExt<'input>>;

pub trait PlusContextAttrs<'input>: TaurusParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token ADD
	/// Returns `None` if there is no child corresponding to token ADD
	fn ADD(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(ADD, 0)
	}
}

impl<'input> PlusContextAttrs<'input> for PlusContext<'input>{}

pub struct PlusContextExt<'input>{
	base:AddOpContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{PlusContextExt<'a>}

impl<'input> TaurusParserContext<'input> for PlusContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for PlusContext<'input>{
	fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_Plus(self);
	}
}

impl<'input> CustomRuleContext<'input> for PlusContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_addOp }
	//fn type_rule_index() -> usize where Self: Sized { RULE_addOp }
}

impl<'input> Borrow<AddOpContextExt<'input>> for PlusContext<'input>{
	fn borrow(&self) -> &AddOpContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<AddOpContextExt<'input>> for PlusContext<'input>{
	fn borrow_mut(&mut self) -> &mut AddOpContextExt<'input> { &mut self.base }
}

impl<'input> AddOpContextAttrs<'input> for PlusContext<'input> {}

impl<'input> PlusContextExt<'input>{
	fn new(ctx: &dyn AddOpContextAttrs<'input>) -> Rc<AddOpContextAll<'input>>  {
		Rc::new(
			AddOpContextAll::PlusContext(
				BaseParserRuleContext::copy_from(ctx,PlusContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type MinusContext<'input> = BaseParserRuleContext<'input,MinusContextExt<'input>>;

pub trait MinusContextAttrs<'input>: TaurusParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token MINUS
	/// Returns `None` if there is no child corresponding to token MINUS
	fn MINUS(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(MINUS, 0)
	}
}

impl<'input> MinusContextAttrs<'input> for MinusContext<'input>{}

pub struct MinusContextExt<'input>{
	base:AddOpContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{MinusContextExt<'a>}

impl<'input> TaurusParserContext<'input> for MinusContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for MinusContext<'input>{
	fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_Minus(self);
	}
}

impl<'input> CustomRuleContext<'input> for MinusContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_addOp }
	//fn type_rule_index() -> usize where Self: Sized { RULE_addOp }
}

impl<'input> Borrow<AddOpContextExt<'input>> for MinusContext<'input>{
	fn borrow(&self) -> &AddOpContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<AddOpContextExt<'input>> for MinusContext<'input>{
	fn borrow_mut(&mut self) -> &mut AddOpContextExt<'input> { &mut self.base }
}

impl<'input> AddOpContextAttrs<'input> for MinusContext<'input> {}

impl<'input> MinusContextExt<'input>{
	fn new(ctx: &dyn AddOpContextAttrs<'input>) -> Rc<AddOpContextAll<'input>>  {
		Rc::new(
			AddOpContextAll::MinusContext(
				BaseParserRuleContext::copy_from(ctx,MinusContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I, H> TaurusParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn addOp(&mut self,)
	-> Result<Rc<AddOpContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = AddOpContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 44, RULE_addOp);
        let mut _localctx: Rc<AddOpContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(513);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 ADD 
				=> {
					let tmp = PlusContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1);
					_localctx = tmp;
					{
					recog.base.set_state(511);
					recog.base.match_token(ADD,&mut recog.err_handler)?;

					}
				}

			 MINUS 
				=> {
					let tmp = MinusContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2);
					_localctx = tmp;
					{
					recog.base.set_state(512);
					recog.base.match_token(MINUS,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- mulOp ----------------
#[derive(Debug)]
pub enum MulOpContextAll<'input>{
	DivContext(DivContext<'input>),
	ModContext(ModContext<'input>),
	MulContext(MulContext<'input>),
Error(MulOpContext<'input>)
}
antlr_rust::tid!{MulOpContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for MulOpContextAll<'input>{}

impl<'input> TaurusParserContext<'input> for MulOpContextAll<'input>{}

impl<'input> Deref for MulOpContextAll<'input>{
	type Target = dyn MulOpContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use MulOpContextAll::*;
		match self{
			DivContext(inner) => inner,
			ModContext(inner) => inner,
			MulContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for MulOpContextAll<'input>{
    fn enter(&self, listener: &mut (dyn TaurusListener<'input> + 'a)) { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn TaurusListener<'input> + 'a)) { self.deref().exit(listener) }
}



pub type MulOpContext<'input> = BaseParserRuleContext<'input,MulOpContextExt<'input>>;

#[derive(Clone)]
pub struct MulOpContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TaurusParserContext<'input> for MulOpContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for MulOpContext<'input>{
}

impl<'input> CustomRuleContext<'input> for MulOpContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_mulOp }
	//fn type_rule_index() -> usize where Self: Sized { RULE_mulOp }
}
antlr_rust::tid!{MulOpContextExt<'a>}

impl<'input> MulOpContextExt<'input>{
	fn new(parent: Option<Rc<dyn TaurusParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<MulOpContextAll<'input>> {
		Rc::new(
		MulOpContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,MulOpContextExt{
				ph:PhantomData
			}),
		)
		)
	}
}

pub trait MulOpContextAttrs<'input>: TaurusParserContext<'input> + BorrowMut<MulOpContextExt<'input>>{


}

impl<'input> MulOpContextAttrs<'input> for MulOpContext<'input>{}

pub type DivContext<'input> = BaseParserRuleContext<'input,DivContextExt<'input>>;

pub trait DivContextAttrs<'input>: TaurusParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token DIV
	/// Returns `None` if there is no child corresponding to token DIV
	fn DIV(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(DIV, 0)
	}
}

impl<'input> DivContextAttrs<'input> for DivContext<'input>{}

pub struct DivContextExt<'input>{
	base:MulOpContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{DivContextExt<'a>}

impl<'input> TaurusParserContext<'input> for DivContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for DivContext<'input>{
	fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_Div(self);
	}
}

impl<'input> CustomRuleContext<'input> for DivContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_mulOp }
	//fn type_rule_index() -> usize where Self: Sized { RULE_mulOp }
}

impl<'input> Borrow<MulOpContextExt<'input>> for DivContext<'input>{
	fn borrow(&self) -> &MulOpContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<MulOpContextExt<'input>> for DivContext<'input>{
	fn borrow_mut(&mut self) -> &mut MulOpContextExt<'input> { &mut self.base }
}

impl<'input> MulOpContextAttrs<'input> for DivContext<'input> {}

impl<'input> DivContextExt<'input>{
	fn new(ctx: &dyn MulOpContextAttrs<'input>) -> Rc<MulOpContextAll<'input>>  {
		Rc::new(
			MulOpContextAll::DivContext(
				BaseParserRuleContext::copy_from(ctx,DivContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type ModContext<'input> = BaseParserRuleContext<'input,ModContextExt<'input>>;

pub trait ModContextAttrs<'input>: TaurusParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token MOD
	/// Returns `None` if there is no child corresponding to token MOD
	fn MOD(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(MOD, 0)
	}
}

impl<'input> ModContextAttrs<'input> for ModContext<'input>{}

pub struct ModContextExt<'input>{
	base:MulOpContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{ModContextExt<'a>}

impl<'input> TaurusParserContext<'input> for ModContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for ModContext<'input>{
	fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_Mod(self);
	}
}

impl<'input> CustomRuleContext<'input> for ModContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_mulOp }
	//fn type_rule_index() -> usize where Self: Sized { RULE_mulOp }
}

impl<'input> Borrow<MulOpContextExt<'input>> for ModContext<'input>{
	fn borrow(&self) -> &MulOpContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<MulOpContextExt<'input>> for ModContext<'input>{
	fn borrow_mut(&mut self) -> &mut MulOpContextExt<'input> { &mut self.base }
}

impl<'input> MulOpContextAttrs<'input> for ModContext<'input> {}

impl<'input> ModContextExt<'input>{
	fn new(ctx: &dyn MulOpContextAttrs<'input>) -> Rc<MulOpContextAll<'input>>  {
		Rc::new(
			MulOpContextAll::ModContext(
				BaseParserRuleContext::copy_from(ctx,ModContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type MulContext<'input> = BaseParserRuleContext<'input,MulContextExt<'input>>;

pub trait MulContextAttrs<'input>: TaurusParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token MUL
	/// Returns `None` if there is no child corresponding to token MUL
	fn MUL(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(MUL, 0)
	}
}

impl<'input> MulContextAttrs<'input> for MulContext<'input>{}

pub struct MulContextExt<'input>{
	base:MulOpContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{MulContextExt<'a>}

impl<'input> TaurusParserContext<'input> for MulContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for MulContext<'input>{
	fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_Mul(self);
	}
}

impl<'input> CustomRuleContext<'input> for MulContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_mulOp }
	//fn type_rule_index() -> usize where Self: Sized { RULE_mulOp }
}

impl<'input> Borrow<MulOpContextExt<'input>> for MulContext<'input>{
	fn borrow(&self) -> &MulOpContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<MulOpContextExt<'input>> for MulContext<'input>{
	fn borrow_mut(&mut self) -> &mut MulOpContextExt<'input> { &mut self.base }
}

impl<'input> MulOpContextAttrs<'input> for MulContext<'input> {}

impl<'input> MulContextExt<'input>{
	fn new(ctx: &dyn MulOpContextAttrs<'input>) -> Rc<MulOpContextAll<'input>>  {
		Rc::new(
			MulOpContextAll::MulContext(
				BaseParserRuleContext::copy_from(ctx,MulContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I, H> TaurusParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn mulOp(&mut self,)
	-> Result<Rc<MulOpContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = MulOpContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 46, RULE_mulOp);
        let mut _localctx: Rc<MulOpContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(518);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 MUL 
				=> {
					let tmp = MulContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1);
					_localctx = tmp;
					{
					recog.base.set_state(515);
					recog.base.match_token(MUL,&mut recog.err_handler)?;

					}
				}

			 DIV 
				=> {
					let tmp = DivContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2);
					_localctx = tmp;
					{
					recog.base.set_state(516);
					recog.base.match_token(DIV,&mut recog.err_handler)?;

					}
				}

			 MOD 
				=> {
					let tmp = ModContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 3);
					_localctx = tmp;
					{
					recog.base.set_state(517);
					recog.base.match_token(MOD,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- cmpOp ----------------
#[derive(Debug)]
pub enum CmpOpContextAll<'input>{
	OrdCmpOpContext(OrdCmpOpContext<'input>),
	EqCmpOpContext(EqCmpOpContext<'input>),
Error(CmpOpContext<'input>)
}
antlr_rust::tid!{CmpOpContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for CmpOpContextAll<'input>{}

impl<'input> TaurusParserContext<'input> for CmpOpContextAll<'input>{}

impl<'input> Deref for CmpOpContextAll<'input>{
	type Target = dyn CmpOpContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use CmpOpContextAll::*;
		match self{
			OrdCmpOpContext(inner) => inner,
			EqCmpOpContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for CmpOpContextAll<'input>{
    fn enter(&self, listener: &mut (dyn TaurusListener<'input> + 'a)) { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn TaurusListener<'input> + 'a)) { self.deref().exit(listener) }
}



pub type CmpOpContext<'input> = BaseParserRuleContext<'input,CmpOpContextExt<'input>>;

#[derive(Clone)]
pub struct CmpOpContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TaurusParserContext<'input> for CmpOpContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for CmpOpContext<'input>{
}

impl<'input> CustomRuleContext<'input> for CmpOpContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_cmpOp }
	//fn type_rule_index() -> usize where Self: Sized { RULE_cmpOp }
}
antlr_rust::tid!{CmpOpContextExt<'a>}

impl<'input> CmpOpContextExt<'input>{
	fn new(parent: Option<Rc<dyn TaurusParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<CmpOpContextAll<'input>> {
		Rc::new(
		CmpOpContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,CmpOpContextExt{
				ph:PhantomData
			}),
		)
		)
	}
}

pub trait CmpOpContextAttrs<'input>: TaurusParserContext<'input> + BorrowMut<CmpOpContextExt<'input>>{


}

impl<'input> CmpOpContextAttrs<'input> for CmpOpContext<'input>{}

pub type OrdCmpOpContext<'input> = BaseParserRuleContext<'input,OrdCmpOpContextExt<'input>>;

pub trait OrdCmpOpContextAttrs<'input>: TaurusParserContext<'input>{
	fn ordOp(&self) -> Option<Rc<OrdOpContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> OrdCmpOpContextAttrs<'input> for OrdCmpOpContext<'input>{}

pub struct OrdCmpOpContextExt<'input>{
	base:CmpOpContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{OrdCmpOpContextExt<'a>}

impl<'input> TaurusParserContext<'input> for OrdCmpOpContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for OrdCmpOpContext<'input>{
	fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_OrdCmpOp(self);
	}
}

impl<'input> CustomRuleContext<'input> for OrdCmpOpContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_cmpOp }
	//fn type_rule_index() -> usize where Self: Sized { RULE_cmpOp }
}

impl<'input> Borrow<CmpOpContextExt<'input>> for OrdCmpOpContext<'input>{
	fn borrow(&self) -> &CmpOpContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<CmpOpContextExt<'input>> for OrdCmpOpContext<'input>{
	fn borrow_mut(&mut self) -> &mut CmpOpContextExt<'input> { &mut self.base }
}

impl<'input> CmpOpContextAttrs<'input> for OrdCmpOpContext<'input> {}

impl<'input> OrdCmpOpContextExt<'input>{
	fn new(ctx: &dyn CmpOpContextAttrs<'input>) -> Rc<CmpOpContextAll<'input>>  {
		Rc::new(
			CmpOpContextAll::OrdCmpOpContext(
				BaseParserRuleContext::copy_from(ctx,OrdCmpOpContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type EqCmpOpContext<'input> = BaseParserRuleContext<'input,EqCmpOpContextExt<'input>>;

pub trait EqCmpOpContextAttrs<'input>: TaurusParserContext<'input>{
	fn eqOp(&self) -> Option<Rc<EqOpContextAll<'input>>> where Self:Sized{
		self.child_of_type(0)
	}
}

impl<'input> EqCmpOpContextAttrs<'input> for EqCmpOpContext<'input>{}

pub struct EqCmpOpContextExt<'input>{
	base:CmpOpContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{EqCmpOpContextExt<'a>}

impl<'input> TaurusParserContext<'input> for EqCmpOpContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for EqCmpOpContext<'input>{
	fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_EqCmpOp(self);
	}
}

impl<'input> CustomRuleContext<'input> for EqCmpOpContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_cmpOp }
	//fn type_rule_index() -> usize where Self: Sized { RULE_cmpOp }
}

impl<'input> Borrow<CmpOpContextExt<'input>> for EqCmpOpContext<'input>{
	fn borrow(&self) -> &CmpOpContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<CmpOpContextExt<'input>> for EqCmpOpContext<'input>{
	fn borrow_mut(&mut self) -> &mut CmpOpContextExt<'input> { &mut self.base }
}

impl<'input> CmpOpContextAttrs<'input> for EqCmpOpContext<'input> {}

impl<'input> EqCmpOpContextExt<'input>{
	fn new(ctx: &dyn CmpOpContextAttrs<'input>) -> Rc<CmpOpContextAll<'input>>  {
		Rc::new(
			CmpOpContextAll::EqCmpOpContext(
				BaseParserRuleContext::copy_from(ctx,EqCmpOpContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I, H> TaurusParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn cmpOp(&mut self,)
	-> Result<Rc<CmpOpContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = CmpOpContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 48, RULE_cmpOp);
        let mut _localctx: Rc<CmpOpContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(522);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 LE | LT | GE | GT 
				=> {
					let tmp = OrdCmpOpContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1);
					_localctx = tmp;
					{
					/*InvokeRule ordOp*/
					recog.base.set_state(520);
					recog.ordOp()?;

					}
				}

			 EQ | NE 
				=> {
					let tmp = EqCmpOpContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2);
					_localctx = tmp;
					{
					/*InvokeRule eqOp*/
					recog.base.set_state(521);
					recog.eqOp()?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- eqOp ----------------
#[derive(Debug)]
pub enum EqOpContextAll<'input>{
	NeqContext(NeqContext<'input>),
	EqContext(EqContext<'input>),
Error(EqOpContext<'input>)
}
antlr_rust::tid!{EqOpContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for EqOpContextAll<'input>{}

impl<'input> TaurusParserContext<'input> for EqOpContextAll<'input>{}

impl<'input> Deref for EqOpContextAll<'input>{
	type Target = dyn EqOpContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use EqOpContextAll::*;
		match self{
			NeqContext(inner) => inner,
			EqContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for EqOpContextAll<'input>{
    fn enter(&self, listener: &mut (dyn TaurusListener<'input> + 'a)) { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn TaurusListener<'input> + 'a)) { self.deref().exit(listener) }
}



pub type EqOpContext<'input> = BaseParserRuleContext<'input,EqOpContextExt<'input>>;

#[derive(Clone)]
pub struct EqOpContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TaurusParserContext<'input> for EqOpContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for EqOpContext<'input>{
}

impl<'input> CustomRuleContext<'input> for EqOpContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_eqOp }
	//fn type_rule_index() -> usize where Self: Sized { RULE_eqOp }
}
antlr_rust::tid!{EqOpContextExt<'a>}

impl<'input> EqOpContextExt<'input>{
	fn new(parent: Option<Rc<dyn TaurusParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<EqOpContextAll<'input>> {
		Rc::new(
		EqOpContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,EqOpContextExt{
				ph:PhantomData
			}),
		)
		)
	}
}

pub trait EqOpContextAttrs<'input>: TaurusParserContext<'input> + BorrowMut<EqOpContextExt<'input>>{


}

impl<'input> EqOpContextAttrs<'input> for EqOpContext<'input>{}

pub type NeqContext<'input> = BaseParserRuleContext<'input,NeqContextExt<'input>>;

pub trait NeqContextAttrs<'input>: TaurusParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token NE
	/// Returns `None` if there is no child corresponding to token NE
	fn NE(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(NE, 0)
	}
}

impl<'input> NeqContextAttrs<'input> for NeqContext<'input>{}

pub struct NeqContextExt<'input>{
	base:EqOpContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{NeqContextExt<'a>}

impl<'input> TaurusParserContext<'input> for NeqContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for NeqContext<'input>{
	fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_Neq(self);
	}
}

impl<'input> CustomRuleContext<'input> for NeqContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_eqOp }
	//fn type_rule_index() -> usize where Self: Sized { RULE_eqOp }
}

impl<'input> Borrow<EqOpContextExt<'input>> for NeqContext<'input>{
	fn borrow(&self) -> &EqOpContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<EqOpContextExt<'input>> for NeqContext<'input>{
	fn borrow_mut(&mut self) -> &mut EqOpContextExt<'input> { &mut self.base }
}

impl<'input> EqOpContextAttrs<'input> for NeqContext<'input> {}

impl<'input> NeqContextExt<'input>{
	fn new(ctx: &dyn EqOpContextAttrs<'input>) -> Rc<EqOpContextAll<'input>>  {
		Rc::new(
			EqOpContextAll::NeqContext(
				BaseParserRuleContext::copy_from(ctx,NeqContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type EqContext<'input> = BaseParserRuleContext<'input,EqContextExt<'input>>;

pub trait EqContextAttrs<'input>: TaurusParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token EQ
	/// Returns `None` if there is no child corresponding to token EQ
	fn EQ(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(EQ, 0)
	}
}

impl<'input> EqContextAttrs<'input> for EqContext<'input>{}

pub struct EqContextExt<'input>{
	base:EqOpContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{EqContextExt<'a>}

impl<'input> TaurusParserContext<'input> for EqContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for EqContext<'input>{
	fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_Eq(self);
	}
}

impl<'input> CustomRuleContext<'input> for EqContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_eqOp }
	//fn type_rule_index() -> usize where Self: Sized { RULE_eqOp }
}

impl<'input> Borrow<EqOpContextExt<'input>> for EqContext<'input>{
	fn borrow(&self) -> &EqOpContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<EqOpContextExt<'input>> for EqContext<'input>{
	fn borrow_mut(&mut self) -> &mut EqOpContextExt<'input> { &mut self.base }
}

impl<'input> EqOpContextAttrs<'input> for EqContext<'input> {}

impl<'input> EqContextExt<'input>{
	fn new(ctx: &dyn EqOpContextAttrs<'input>) -> Rc<EqOpContextAll<'input>>  {
		Rc::new(
			EqOpContextAll::EqContext(
				BaseParserRuleContext::copy_from(ctx,EqContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I, H> TaurusParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn eqOp(&mut self,)
	-> Result<Rc<EqOpContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = EqOpContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 50, RULE_eqOp);
        let mut _localctx: Rc<EqOpContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(526);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 EQ 
				=> {
					let tmp = EqContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1);
					_localctx = tmp;
					{
					recog.base.set_state(524);
					recog.base.match_token(EQ,&mut recog.err_handler)?;

					}
				}

			 NE 
				=> {
					let tmp = NeqContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2);
					_localctx = tmp;
					{
					recog.base.set_state(525);
					recog.base.match_token(NE,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- ordOp ----------------
#[derive(Debug)]
pub enum OrdOpContextAll<'input>{
	LtContext(LtContext<'input>),
	LeContext(LeContext<'input>),
	GtContext(GtContext<'input>),
	GeContext(GeContext<'input>),
Error(OrdOpContext<'input>)
}
antlr_rust::tid!{OrdOpContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for OrdOpContextAll<'input>{}

impl<'input> TaurusParserContext<'input> for OrdOpContextAll<'input>{}

impl<'input> Deref for OrdOpContextAll<'input>{
	type Target = dyn OrdOpContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use OrdOpContextAll::*;
		match self{
			LtContext(inner) => inner,
			LeContext(inner) => inner,
			GtContext(inner) => inner,
			GeContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for OrdOpContextAll<'input>{
    fn enter(&self, listener: &mut (dyn TaurusListener<'input> + 'a)) { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn TaurusListener<'input> + 'a)) { self.deref().exit(listener) }
}



pub type OrdOpContext<'input> = BaseParserRuleContext<'input,OrdOpContextExt<'input>>;

#[derive(Clone)]
pub struct OrdOpContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TaurusParserContext<'input> for OrdOpContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for OrdOpContext<'input>{
}

impl<'input> CustomRuleContext<'input> for OrdOpContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_ordOp }
	//fn type_rule_index() -> usize where Self: Sized { RULE_ordOp }
}
antlr_rust::tid!{OrdOpContextExt<'a>}

impl<'input> OrdOpContextExt<'input>{
	fn new(parent: Option<Rc<dyn TaurusParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<OrdOpContextAll<'input>> {
		Rc::new(
		OrdOpContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,OrdOpContextExt{
				ph:PhantomData
			}),
		)
		)
	}
}

pub trait OrdOpContextAttrs<'input>: TaurusParserContext<'input> + BorrowMut<OrdOpContextExt<'input>>{


}

impl<'input> OrdOpContextAttrs<'input> for OrdOpContext<'input>{}

pub type LtContext<'input> = BaseParserRuleContext<'input,LtContextExt<'input>>;

pub trait LtContextAttrs<'input>: TaurusParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token LT
	/// Returns `None` if there is no child corresponding to token LT
	fn LT(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(LT, 0)
	}
}

impl<'input> LtContextAttrs<'input> for LtContext<'input>{}

pub struct LtContextExt<'input>{
	base:OrdOpContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{LtContextExt<'a>}

impl<'input> TaurusParserContext<'input> for LtContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for LtContext<'input>{
	fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_Lt(self);
	}
}

impl<'input> CustomRuleContext<'input> for LtContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_ordOp }
	//fn type_rule_index() -> usize where Self: Sized { RULE_ordOp }
}

impl<'input> Borrow<OrdOpContextExt<'input>> for LtContext<'input>{
	fn borrow(&self) -> &OrdOpContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<OrdOpContextExt<'input>> for LtContext<'input>{
	fn borrow_mut(&mut self) -> &mut OrdOpContextExt<'input> { &mut self.base }
}

impl<'input> OrdOpContextAttrs<'input> for LtContext<'input> {}

impl<'input> LtContextExt<'input>{
	fn new(ctx: &dyn OrdOpContextAttrs<'input>) -> Rc<OrdOpContextAll<'input>>  {
		Rc::new(
			OrdOpContextAll::LtContext(
				BaseParserRuleContext::copy_from(ctx,LtContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type LeContext<'input> = BaseParserRuleContext<'input,LeContextExt<'input>>;

pub trait LeContextAttrs<'input>: TaurusParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token LE
	/// Returns `None` if there is no child corresponding to token LE
	fn LE(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(LE, 0)
	}
}

impl<'input> LeContextAttrs<'input> for LeContext<'input>{}

pub struct LeContextExt<'input>{
	base:OrdOpContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{LeContextExt<'a>}

impl<'input> TaurusParserContext<'input> for LeContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for LeContext<'input>{
	fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_Le(self);
	}
}

impl<'input> CustomRuleContext<'input> for LeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_ordOp }
	//fn type_rule_index() -> usize where Self: Sized { RULE_ordOp }
}

impl<'input> Borrow<OrdOpContextExt<'input>> for LeContext<'input>{
	fn borrow(&self) -> &OrdOpContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<OrdOpContextExt<'input>> for LeContext<'input>{
	fn borrow_mut(&mut self) -> &mut OrdOpContextExt<'input> { &mut self.base }
}

impl<'input> OrdOpContextAttrs<'input> for LeContext<'input> {}

impl<'input> LeContextExt<'input>{
	fn new(ctx: &dyn OrdOpContextAttrs<'input>) -> Rc<OrdOpContextAll<'input>>  {
		Rc::new(
			OrdOpContextAll::LeContext(
				BaseParserRuleContext::copy_from(ctx,LeContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type GtContext<'input> = BaseParserRuleContext<'input,GtContextExt<'input>>;

pub trait GtContextAttrs<'input>: TaurusParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token GT
	/// Returns `None` if there is no child corresponding to token GT
	fn GT(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(GT, 0)
	}
}

impl<'input> GtContextAttrs<'input> for GtContext<'input>{}

pub struct GtContextExt<'input>{
	base:OrdOpContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{GtContextExt<'a>}

impl<'input> TaurusParserContext<'input> for GtContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for GtContext<'input>{
	fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_Gt(self);
	}
}

impl<'input> CustomRuleContext<'input> for GtContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_ordOp }
	//fn type_rule_index() -> usize where Self: Sized { RULE_ordOp }
}

impl<'input> Borrow<OrdOpContextExt<'input>> for GtContext<'input>{
	fn borrow(&self) -> &OrdOpContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<OrdOpContextExt<'input>> for GtContext<'input>{
	fn borrow_mut(&mut self) -> &mut OrdOpContextExt<'input> { &mut self.base }
}

impl<'input> OrdOpContextAttrs<'input> for GtContext<'input> {}

impl<'input> GtContextExt<'input>{
	fn new(ctx: &dyn OrdOpContextAttrs<'input>) -> Rc<OrdOpContextAll<'input>>  {
		Rc::new(
			OrdOpContextAll::GtContext(
				BaseParserRuleContext::copy_from(ctx,GtContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type GeContext<'input> = BaseParserRuleContext<'input,GeContextExt<'input>>;

pub trait GeContextAttrs<'input>: TaurusParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token GE
	/// Returns `None` if there is no child corresponding to token GE
	fn GE(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(GE, 0)
	}
}

impl<'input> GeContextAttrs<'input> for GeContext<'input>{}

pub struct GeContextExt<'input>{
	base:OrdOpContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{GeContextExt<'a>}

impl<'input> TaurusParserContext<'input> for GeContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for GeContext<'input>{
	fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_Ge(self);
	}
}

impl<'input> CustomRuleContext<'input> for GeContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_ordOp }
	//fn type_rule_index() -> usize where Self: Sized { RULE_ordOp }
}

impl<'input> Borrow<OrdOpContextExt<'input>> for GeContext<'input>{
	fn borrow(&self) -> &OrdOpContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<OrdOpContextExt<'input>> for GeContext<'input>{
	fn borrow_mut(&mut self) -> &mut OrdOpContextExt<'input> { &mut self.base }
}

impl<'input> OrdOpContextAttrs<'input> for GeContext<'input> {}

impl<'input> GeContextExt<'input>{
	fn new(ctx: &dyn OrdOpContextAttrs<'input>) -> Rc<OrdOpContextAll<'input>>  {
		Rc::new(
			OrdOpContextAll::GeContext(
				BaseParserRuleContext::copy_from(ctx,GeContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I, H> TaurusParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn ordOp(&mut self,)
	-> Result<Rc<OrdOpContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = OrdOpContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 52, RULE_ordOp);
        let mut _localctx: Rc<OrdOpContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(532);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 LT 
				=> {
					let tmp = LtContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1);
					_localctx = tmp;
					{
					recog.base.set_state(528);
					recog.base.match_token(LT,&mut recog.err_handler)?;

					}
				}

			 LE 
				=> {
					let tmp = LeContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2);
					_localctx = tmp;
					{
					recog.base.set_state(529);
					recog.base.match_token(LE,&mut recog.err_handler)?;

					}
				}

			 GT 
				=> {
					let tmp = GtContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 3);
					_localctx = tmp;
					{
					recog.base.set_state(530);
					recog.base.match_token(GT,&mut recog.err_handler)?;

					}
				}

			 GE 
				=> {
					let tmp = GeContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 4);
					_localctx = tmp;
					{
					recog.base.set_state(531);
					recog.base.match_token(GE,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- unaryOp ----------------
#[derive(Debug)]
pub enum UnaryOpContextAll<'input>{
	NegContext(NegContext<'input>),
	NotContext(NotContext<'input>),
Error(UnaryOpContext<'input>)
}
antlr_rust::tid!{UnaryOpContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for UnaryOpContextAll<'input>{}

impl<'input> TaurusParserContext<'input> for UnaryOpContextAll<'input>{}

impl<'input> Deref for UnaryOpContextAll<'input>{
	type Target = dyn UnaryOpContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use UnaryOpContextAll::*;
		match self{
			NegContext(inner) => inner,
			NotContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for UnaryOpContextAll<'input>{
    fn enter(&self, listener: &mut (dyn TaurusListener<'input> + 'a)) { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn TaurusListener<'input> + 'a)) { self.deref().exit(listener) }
}



pub type UnaryOpContext<'input> = BaseParserRuleContext<'input,UnaryOpContextExt<'input>>;

#[derive(Clone)]
pub struct UnaryOpContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TaurusParserContext<'input> for UnaryOpContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for UnaryOpContext<'input>{
}

impl<'input> CustomRuleContext<'input> for UnaryOpContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_unaryOp }
	//fn type_rule_index() -> usize where Self: Sized { RULE_unaryOp }
}
antlr_rust::tid!{UnaryOpContextExt<'a>}

impl<'input> UnaryOpContextExt<'input>{
	fn new(parent: Option<Rc<dyn TaurusParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<UnaryOpContextAll<'input>> {
		Rc::new(
		UnaryOpContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,UnaryOpContextExt{
				ph:PhantomData
			}),
		)
		)
	}
}

pub trait UnaryOpContextAttrs<'input>: TaurusParserContext<'input> + BorrowMut<UnaryOpContextExt<'input>>{


}

impl<'input> UnaryOpContextAttrs<'input> for UnaryOpContext<'input>{}

pub type NegContext<'input> = BaseParserRuleContext<'input,NegContextExt<'input>>;

pub trait NegContextAttrs<'input>: TaurusParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token MINUS
	/// Returns `None` if there is no child corresponding to token MINUS
	fn MINUS(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(MINUS, 0)
	}
}

impl<'input> NegContextAttrs<'input> for NegContext<'input>{}

pub struct NegContextExt<'input>{
	base:UnaryOpContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{NegContextExt<'a>}

impl<'input> TaurusParserContext<'input> for NegContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for NegContext<'input>{
	fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_Neg(self);
	}
}

impl<'input> CustomRuleContext<'input> for NegContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_unaryOp }
	//fn type_rule_index() -> usize where Self: Sized { RULE_unaryOp }
}

impl<'input> Borrow<UnaryOpContextExt<'input>> for NegContext<'input>{
	fn borrow(&self) -> &UnaryOpContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<UnaryOpContextExt<'input>> for NegContext<'input>{
	fn borrow_mut(&mut self) -> &mut UnaryOpContextExt<'input> { &mut self.base }
}

impl<'input> UnaryOpContextAttrs<'input> for NegContext<'input> {}

impl<'input> NegContextExt<'input>{
	fn new(ctx: &dyn UnaryOpContextAttrs<'input>) -> Rc<UnaryOpContextAll<'input>>  {
		Rc::new(
			UnaryOpContextAll::NegContext(
				BaseParserRuleContext::copy_from(ctx,NegContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type NotContext<'input> = BaseParserRuleContext<'input,NotContextExt<'input>>;

pub trait NotContextAttrs<'input>: TaurusParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token NOT
	/// Returns `None` if there is no child corresponding to token NOT
	fn NOT(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(NOT, 0)
	}
}

impl<'input> NotContextAttrs<'input> for NotContext<'input>{}

pub struct NotContextExt<'input>{
	base:UnaryOpContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{NotContextExt<'a>}

impl<'input> TaurusParserContext<'input> for NotContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for NotContext<'input>{
	fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_Not(self);
	}
}

impl<'input> CustomRuleContext<'input> for NotContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_unaryOp }
	//fn type_rule_index() -> usize where Self: Sized { RULE_unaryOp }
}

impl<'input> Borrow<UnaryOpContextExt<'input>> for NotContext<'input>{
	fn borrow(&self) -> &UnaryOpContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<UnaryOpContextExt<'input>> for NotContext<'input>{
	fn borrow_mut(&mut self) -> &mut UnaryOpContextExt<'input> { &mut self.base }
}

impl<'input> UnaryOpContextAttrs<'input> for NotContext<'input> {}

impl<'input> NotContextExt<'input>{
	fn new(ctx: &dyn UnaryOpContextAttrs<'input>) -> Rc<UnaryOpContextAll<'input>>  {
		Rc::new(
			UnaryOpContextAll::NotContext(
				BaseParserRuleContext::copy_from(ctx,NotContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I, H> TaurusParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn unaryOp(&mut self,)
	-> Result<Rc<UnaryOpContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = UnaryOpContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 54, RULE_unaryOp);
        let mut _localctx: Rc<UnaryOpContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(536);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 MINUS 
				=> {
					let tmp = NegContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1);
					_localctx = tmp;
					{
					recog.base.set_state(534);
					recog.base.match_token(MINUS,&mut recog.err_handler)?;

					}
				}

			 NOT 
				=> {
					let tmp = NotContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2);
					_localctx = tmp;
					{
					recog.base.set_state(535);
					recog.base.match_token(NOT,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- quantifier ----------------
#[derive(Debug)]
pub enum QuantifierContextAll<'input>{
	UniversalContext(UniversalContext<'input>),
	ExistentialContext(ExistentialContext<'input>),
Error(QuantifierContext<'input>)
}
antlr_rust::tid!{QuantifierContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for QuantifierContextAll<'input>{}

impl<'input> TaurusParserContext<'input> for QuantifierContextAll<'input>{}

impl<'input> Deref for QuantifierContextAll<'input>{
	type Target = dyn QuantifierContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use QuantifierContextAll::*;
		match self{
			UniversalContext(inner) => inner,
			ExistentialContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for QuantifierContextAll<'input>{
    fn enter(&self, listener: &mut (dyn TaurusListener<'input> + 'a)) { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn TaurusListener<'input> + 'a)) { self.deref().exit(listener) }
}



pub type QuantifierContext<'input> = BaseParserRuleContext<'input,QuantifierContextExt<'input>>;

#[derive(Clone)]
pub struct QuantifierContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TaurusParserContext<'input> for QuantifierContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for QuantifierContext<'input>{
}

impl<'input> CustomRuleContext<'input> for QuantifierContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_quantifier }
	//fn type_rule_index() -> usize where Self: Sized { RULE_quantifier }
}
antlr_rust::tid!{QuantifierContextExt<'a>}

impl<'input> QuantifierContextExt<'input>{
	fn new(parent: Option<Rc<dyn TaurusParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<QuantifierContextAll<'input>> {
		Rc::new(
		QuantifierContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,QuantifierContextExt{
				ph:PhantomData
			}),
		)
		)
	}
}

pub trait QuantifierContextAttrs<'input>: TaurusParserContext<'input> + BorrowMut<QuantifierContextExt<'input>>{


}

impl<'input> QuantifierContextAttrs<'input> for QuantifierContext<'input>{}

pub type UniversalContext<'input> = BaseParserRuleContext<'input,UniversalContextExt<'input>>;

pub trait UniversalContextAttrs<'input>: TaurusParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token FORALL
	/// Returns `None` if there is no child corresponding to token FORALL
	fn FORALL(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(FORALL, 0)
	}
}

impl<'input> UniversalContextAttrs<'input> for UniversalContext<'input>{}

pub struct UniversalContextExt<'input>{
	base:QuantifierContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{UniversalContextExt<'a>}

impl<'input> TaurusParserContext<'input> for UniversalContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for UniversalContext<'input>{
	fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_Universal(self);
	}
}

impl<'input> CustomRuleContext<'input> for UniversalContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_quantifier }
	//fn type_rule_index() -> usize where Self: Sized { RULE_quantifier }
}

impl<'input> Borrow<QuantifierContextExt<'input>> for UniversalContext<'input>{
	fn borrow(&self) -> &QuantifierContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<QuantifierContextExt<'input>> for UniversalContext<'input>{
	fn borrow_mut(&mut self) -> &mut QuantifierContextExt<'input> { &mut self.base }
}

impl<'input> QuantifierContextAttrs<'input> for UniversalContext<'input> {}

impl<'input> UniversalContextExt<'input>{
	fn new(ctx: &dyn QuantifierContextAttrs<'input>) -> Rc<QuantifierContextAll<'input>>  {
		Rc::new(
			QuantifierContextAll::UniversalContext(
				BaseParserRuleContext::copy_from(ctx,UniversalContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type ExistentialContext<'input> = BaseParserRuleContext<'input,ExistentialContextExt<'input>>;

pub trait ExistentialContextAttrs<'input>: TaurusParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token EXISTS
	/// Returns `None` if there is no child corresponding to token EXISTS
	fn EXISTS(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(EXISTS, 0)
	}
}

impl<'input> ExistentialContextAttrs<'input> for ExistentialContext<'input>{}

pub struct ExistentialContextExt<'input>{
	base:QuantifierContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{ExistentialContextExt<'a>}

impl<'input> TaurusParserContext<'input> for ExistentialContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for ExistentialContext<'input>{
	fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_Existential(self);
	}
}

impl<'input> CustomRuleContext<'input> for ExistentialContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_quantifier }
	//fn type_rule_index() -> usize where Self: Sized { RULE_quantifier }
}

impl<'input> Borrow<QuantifierContextExt<'input>> for ExistentialContext<'input>{
	fn borrow(&self) -> &QuantifierContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<QuantifierContextExt<'input>> for ExistentialContext<'input>{
	fn borrow_mut(&mut self) -> &mut QuantifierContextExt<'input> { &mut self.base }
}

impl<'input> QuantifierContextAttrs<'input> for ExistentialContext<'input> {}

impl<'input> ExistentialContextExt<'input>{
	fn new(ctx: &dyn QuantifierContextAttrs<'input>) -> Rc<QuantifierContextAll<'input>>  {
		Rc::new(
			QuantifierContextAll::ExistentialContext(
				BaseParserRuleContext::copy_from(ctx,ExistentialContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I, H> TaurusParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn quantifier(&mut self,)
	-> Result<Rc<QuantifierContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = QuantifierContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 56, RULE_quantifier);
        let mut _localctx: Rc<QuantifierContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(540);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 FORALL 
				=> {
					let tmp = UniversalContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1);
					_localctx = tmp;
					{
					recog.base.set_state(538);
					recog.base.match_token(FORALL,&mut recog.err_handler)?;

					}
				}

			 EXISTS 
				=> {
					let tmp = ExistentialContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2);
					_localctx = tmp;
					{
					recog.base.set_state(539);
					recog.base.match_token(EXISTS,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- binder ----------------
pub type BinderContextAll<'input> = BinderContext<'input>;


pub type BinderContext<'input> = BaseParserRuleContext<'input,BinderContextExt<'input>>;

#[derive(Clone)]
pub struct BinderContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TaurusParserContext<'input> for BinderContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for BinderContext<'input>{
		fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_binder(self);
		}fn exit(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
			listener.exit_binder(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for BinderContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_binder }
	//fn type_rule_index() -> usize where Self: Sized { RULE_binder }
}
antlr_rust::tid!{BinderContextExt<'a>}

impl<'input> BinderContextExt<'input>{
	fn new(parent: Option<Rc<dyn TaurusParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<BinderContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,BinderContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait BinderContextAttrs<'input>: TaurusParserContext<'input> + BorrowMut<BinderContextExt<'input>>{

fn logicAtomicType(&self) -> Option<Rc<LogicAtomicTypeContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves all `TerminalNode`s corresponding to token IDENT in current rule
fn IDENT_all(&self) -> Vec<Rc<TerminalNode<'input,TaurusParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token IDENT, starting from 0.
/// Returns `None` if number of children corresponding to token IDENT is less or equal than `i`.
fn IDENT(&self, i: usize) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
	self.get_token(IDENT, i)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,TaurusParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
	self.get_token(COMMA, i)
}

}

impl<'input> BinderContextAttrs<'input> for BinderContext<'input>{}

impl<'input, I, H> TaurusParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn binder(&mut self,)
	-> Result<Rc<BinderContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = BinderContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 58, RULE_binder);
        let mut _localctx: Rc<BinderContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			/*InvokeRule logicAtomicType*/
			recog.base.set_state(542);
			recog.logicAtomicType()?;

			recog.base.set_state(543);
			recog.base.match_token(IDENT,&mut recog.err_handler)?;

			recog.base.set_state(548);
			recog.err_handler.sync(&mut recog.base)?;
			_alt = recog.interpreter.adaptive_predict(51,&mut recog.base)?;
			while { _alt!=2 && _alt!=INVALID_ALT } {
				if _alt==1 {
					{
					{
					recog.base.set_state(544);
					recog.base.match_token(COMMA,&mut recog.err_handler)?;

					recog.base.set_state(545);
					recog.base.match_token(IDENT,&mut recog.err_handler)?;

					}
					} 
				}
				recog.base.set_state(550);
				recog.err_handler.sync(&mut recog.base)?;
				_alt = recog.interpreter.adaptive_predict(51,&mut recog.base)?;
			}
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- funcContract ----------------
pub type FuncContractContextAll<'input> = FuncContractContext<'input>;


pub type FuncContractContext<'input> = BaseParserRuleContext<'input,FuncContractContextExt<'input>>;

#[derive(Clone)]
pub struct FuncContractContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TaurusParserContext<'input> for FuncContractContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for FuncContractContext<'input>{
		fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_funcContract(self);
		}fn exit(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
			listener.exit_funcContract(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for FuncContractContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_funcContract }
	//fn type_rule_index() -> usize where Self: Sized { RULE_funcContract }
}
antlr_rust::tid!{FuncContractContextExt<'a>}

impl<'input> FuncContractContextExt<'input>{
	fn new(parent: Option<Rc<dyn TaurusParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<FuncContractContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,FuncContractContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait FuncContractContextAttrs<'input>: TaurusParserContext<'input> + BorrowMut<FuncContractContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token ANNOT_START
/// Returns `None` if there is no child corresponding to token ANNOT_START
fn ANNOT_START(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
	self.get_token(ANNOT_START, 0)
}
/// Retrieves first TerminalNode corresponding to token ANNOT_END
/// Returns `None` if there is no child corresponding to token ANNOT_END
fn ANNOT_END(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
	self.get_token(ANNOT_END, 0)
}
fn requiresClause_all(&self) ->  Vec<Rc<RequiresClauseContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn requiresClause(&self, i: usize) -> Option<Rc<RequiresClauseContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
fn decreasesClause(&self) -> Option<Rc<DecreasesClauseContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
fn ensuresClause_all(&self) ->  Vec<Rc<EnsuresClauseContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn ensuresClause(&self, i: usize) -> Option<Rc<EnsuresClauseContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token LINE_ANNOT_START
/// Returns `None` if there is no child corresponding to token LINE_ANNOT_START
fn LINE_ANNOT_START(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
	self.get_token(LINE_ANNOT_START, 0)
}
/// Retrieves first TerminalNode corresponding to token LINEEND
/// Returns `None` if there is no child corresponding to token LINEEND
fn LINEEND(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
	self.get_token(LINEEND, 0)
}

}

impl<'input> FuncContractContextAttrs<'input> for FuncContractContext<'input>{}

impl<'input, I, H> TaurusParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn funcContract(&mut self,)
	-> Result<Rc<FuncContractContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = FuncContractContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 60, RULE_funcContract);
        let mut _localctx: Rc<FuncContractContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(585);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 ANNOT_START 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(551);
					recog.base.match_token(ANNOT_START,&mut recog.err_handler)?;

					recog.base.set_state(555);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==REQUIRES {
						{
						{
						/*InvokeRule requiresClause*/
						recog.base.set_state(552);
						recog.requiresClause()?;

						}
						}
						recog.base.set_state(557);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					recog.base.set_state(559);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==DECREASES {
						{
						/*InvokeRule decreasesClause*/
						recog.base.set_state(558);
						recog.decreasesClause()?;

						}
					}

					recog.base.set_state(564);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==ENSURES {
						{
						{
						/*InvokeRule ensuresClause*/
						recog.base.set_state(561);
						recog.ensuresClause()?;

						}
						}
						recog.base.set_state(566);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					recog.base.set_state(567);
					recog.base.match_token(ANNOT_END,&mut recog.err_handler)?;

					}
				}

			 LINE_ANNOT_START 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(568);
					recog.base.match_token(LINE_ANNOT_START,&mut recog.err_handler)?;

					recog.base.set_state(572);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==REQUIRES {
						{
						{
						/*InvokeRule requiresClause*/
						recog.base.set_state(569);
						recog.requiresClause()?;

						}
						}
						recog.base.set_state(574);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					recog.base.set_state(576);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==DECREASES {
						{
						/*InvokeRule decreasesClause*/
						recog.base.set_state(575);
						recog.decreasesClause()?;

						}
					}

					recog.base.set_state(581);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==ENSURES {
						{
						{
						/*InvokeRule ensuresClause*/
						recog.base.set_state(578);
						recog.ensuresClause()?;

						}
						}
						recog.base.set_state(583);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					recog.base.set_state(584);
					recog.base.match_token(LINEEND,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- requiresClause ----------------
pub type RequiresClauseContextAll<'input> = RequiresClauseContext<'input>;


pub type RequiresClauseContext<'input> = BaseParserRuleContext<'input,RequiresClauseContextExt<'input>>;

#[derive(Clone)]
pub struct RequiresClauseContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TaurusParserContext<'input> for RequiresClauseContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for RequiresClauseContext<'input>{
		fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_requiresClause(self);
		}fn exit(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
			listener.exit_requiresClause(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for RequiresClauseContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_requiresClause }
	//fn type_rule_index() -> usize where Self: Sized { RULE_requiresClause }
}
antlr_rust::tid!{RequiresClauseContextExt<'a>}

impl<'input> RequiresClauseContextExt<'input>{
	fn new(parent: Option<Rc<dyn TaurusParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<RequiresClauseContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,RequiresClauseContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait RequiresClauseContextAttrs<'input>: TaurusParserContext<'input> + BorrowMut<RequiresClauseContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token REQUIRES
/// Returns `None` if there is no child corresponding to token REQUIRES
fn REQUIRES(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
	self.get_token(REQUIRES, 0)
}
fn pred(&self) -> Option<Rc<PredContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token SEMICOLON
/// Returns `None` if there is no child corresponding to token SEMICOLON
fn SEMICOLON(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
	self.get_token(SEMICOLON, 0)
}

}

impl<'input> RequiresClauseContextAttrs<'input> for RequiresClauseContext<'input>{}

impl<'input, I, H> TaurusParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn requiresClause(&mut self,)
	-> Result<Rc<RequiresClauseContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = RequiresClauseContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 62, RULE_requiresClause);
        let mut _localctx: Rc<RequiresClauseContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(587);
			recog.base.match_token(REQUIRES,&mut recog.err_handler)?;

			/*InvokeRule pred*/
			recog.base.set_state(588);
			recog.pred_rec(0)?;

			recog.base.set_state(589);
			recog.base.match_token(SEMICOLON,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- decreasesClause ----------------
pub type DecreasesClauseContextAll<'input> = DecreasesClauseContext<'input>;


pub type DecreasesClauseContext<'input> = BaseParserRuleContext<'input,DecreasesClauseContextExt<'input>>;

#[derive(Clone)]
pub struct DecreasesClauseContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TaurusParserContext<'input> for DecreasesClauseContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for DecreasesClauseContext<'input>{
		fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_decreasesClause(self);
		}fn exit(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
			listener.exit_decreasesClause(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for DecreasesClauseContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_decreasesClause }
	//fn type_rule_index() -> usize where Self: Sized { RULE_decreasesClause }
}
antlr_rust::tid!{DecreasesClauseContextExt<'a>}

impl<'input> DecreasesClauseContextExt<'input>{
	fn new(parent: Option<Rc<dyn TaurusParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<DecreasesClauseContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,DecreasesClauseContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait DecreasesClauseContextAttrs<'input>: TaurusParserContext<'input> + BorrowMut<DecreasesClauseContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token DECREASES
/// Returns `None` if there is no child corresponding to token DECREASES
fn DECREASES(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
	self.get_token(DECREASES, 0)
}
/// Retrieves first TerminalNode corresponding to token SEMICOLON
/// Returns `None` if there is no child corresponding to token SEMICOLON
fn SEMICOLON(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
	self.get_token(SEMICOLON, 0)
}
fn arithTerm_all(&self) ->  Vec<Rc<ArithTermContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn arithTerm(&self, i: usize) -> Option<Rc<ArithTermContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token LPAR
/// Returns `None` if there is no child corresponding to token LPAR
fn LPAR(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
	self.get_token(LPAR, 0)
}
/// Retrieves first TerminalNode corresponding to token RPAR
/// Returns `None` if there is no child corresponding to token RPAR
fn RPAR(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
	self.get_token(RPAR, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,TaurusParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
	self.get_token(COMMA, i)
}

}

impl<'input> DecreasesClauseContextAttrs<'input> for DecreasesClauseContext<'input>{}

impl<'input, I, H> TaurusParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn decreasesClause(&mut self,)
	-> Result<Rc<DecreasesClauseContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = DecreasesClauseContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 64, RULE_decreasesClause);
        let mut _localctx: Rc<DecreasesClauseContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(591);
			recog.base.match_token(DECREASES,&mut recog.err_handler)?;

			recog.base.set_state(603);
			recog.err_handler.sync(&mut recog.base)?;
			match  recog.interpreter.adaptive_predict(60,&mut recog.base)? {
				1 =>{
					{
					/*InvokeRule arithTerm*/
					recog.base.set_state(592);
					recog.arithTerm_rec(0)?;

					}
				}
			,
				2 =>{
					{
					recog.base.set_state(593);
					recog.base.match_token(LPAR,&mut recog.err_handler)?;

					/*InvokeRule arithTerm*/
					recog.base.set_state(594);
					recog.arithTerm_rec(0)?;

					recog.base.set_state(597); 
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					loop {
						{
						{
						recog.base.set_state(595);
						recog.base.match_token(COMMA,&mut recog.err_handler)?;

						/*InvokeRule arithTerm*/
						recog.base.set_state(596);
						recog.arithTerm_rec(0)?;

						}
						}
						recog.base.set_state(599); 
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
						if !(_la==COMMA) {break}
					}
					recog.base.set_state(601);
					recog.base.match_token(RPAR,&mut recog.err_handler)?;

					}
				}

				_ => {}
			}
			recog.base.set_state(605);
			recog.base.match_token(SEMICOLON,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- ensuresClause ----------------
pub type EnsuresClauseContextAll<'input> = EnsuresClauseContext<'input>;


pub type EnsuresClauseContext<'input> = BaseParserRuleContext<'input,EnsuresClauseContextExt<'input>>;

#[derive(Clone)]
pub struct EnsuresClauseContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TaurusParserContext<'input> for EnsuresClauseContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for EnsuresClauseContext<'input>{
		fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_ensuresClause(self);
		}fn exit(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
			listener.exit_ensuresClause(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for EnsuresClauseContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_ensuresClause }
	//fn type_rule_index() -> usize where Self: Sized { RULE_ensuresClause }
}
antlr_rust::tid!{EnsuresClauseContextExt<'a>}

impl<'input> EnsuresClauseContextExt<'input>{
	fn new(parent: Option<Rc<dyn TaurusParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<EnsuresClauseContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,EnsuresClauseContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait EnsuresClauseContextAttrs<'input>: TaurusParserContext<'input> + BorrowMut<EnsuresClauseContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token ENSURES
/// Returns `None` if there is no child corresponding to token ENSURES
fn ENSURES(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
	self.get_token(ENSURES, 0)
}
fn pred(&self) -> Option<Rc<PredContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token SEMICOLON
/// Returns `None` if there is no child corresponding to token SEMICOLON
fn SEMICOLON(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
	self.get_token(SEMICOLON, 0)
}

}

impl<'input> EnsuresClauseContextAttrs<'input> for EnsuresClauseContext<'input>{}

impl<'input, I, H> TaurusParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn ensuresClause(&mut self,)
	-> Result<Rc<EnsuresClauseContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = EnsuresClauseContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 66, RULE_ensuresClause);
        let mut _localctx: Rc<EnsuresClauseContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(607);
			recog.base.match_token(ENSURES,&mut recog.err_handler)?;

			/*InvokeRule pred*/
			recog.base.set_state(608);
			recog.pred_rec(0)?;

			recog.base.set_state(609);
			recog.base.match_token(SEMICOLON,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- assertion ----------------
pub type AssertionContextAll<'input> = AssertionContext<'input>;


pub type AssertionContext<'input> = BaseParserRuleContext<'input,AssertionContextExt<'input>>;

#[derive(Clone)]
pub struct AssertionContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TaurusParserContext<'input> for AssertionContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for AssertionContext<'input>{
		fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_assertion(self);
		}fn exit(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
			listener.exit_assertion(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for AssertionContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_assertion }
	//fn type_rule_index() -> usize where Self: Sized { RULE_assertion }
}
antlr_rust::tid!{AssertionContextExt<'a>}

impl<'input> AssertionContextExt<'input>{
	fn new(parent: Option<Rc<dyn TaurusParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<AssertionContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,AssertionContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait AssertionContextAttrs<'input>: TaurusParserContext<'input> + BorrowMut<AssertionContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token ANNOT_START
/// Returns `None` if there is no child corresponding to token ANNOT_START
fn ANNOT_START(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
	self.get_token(ANNOT_START, 0)
}
/// Retrieves first TerminalNode corresponding to token ASSERT
/// Returns `None` if there is no child corresponding to token ASSERT
fn ASSERT(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
	self.get_token(ASSERT, 0)
}
fn pred(&self) -> Option<Rc<PredContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token SEMICOLON
/// Returns `None` if there is no child corresponding to token SEMICOLON
fn SEMICOLON(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
	self.get_token(SEMICOLON, 0)
}
/// Retrieves first TerminalNode corresponding to token ANNOT_END
/// Returns `None` if there is no child corresponding to token ANNOT_END
fn ANNOT_END(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
	self.get_token(ANNOT_END, 0)
}
/// Retrieves first TerminalNode corresponding to token LINE_ANNOT_START
/// Returns `None` if there is no child corresponding to token LINE_ANNOT_START
fn LINE_ANNOT_START(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
	self.get_token(LINE_ANNOT_START, 0)
}
/// Retrieves first TerminalNode corresponding to token LINEEND
/// Returns `None` if there is no child corresponding to token LINEEND
fn LINEEND(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
	self.get_token(LINEEND, 0)
}

}

impl<'input> AssertionContextAttrs<'input> for AssertionContext<'input>{}

impl<'input, I, H> TaurusParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn assertion(&mut self,)
	-> Result<Rc<AssertionContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = AssertionContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 68, RULE_assertion);
        let mut _localctx: Rc<AssertionContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(623);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 ANNOT_START 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(611);
					recog.base.match_token(ANNOT_START,&mut recog.err_handler)?;

					recog.base.set_state(612);
					recog.base.match_token(ASSERT,&mut recog.err_handler)?;

					/*InvokeRule pred*/
					recog.base.set_state(613);
					recog.pred_rec(0)?;

					recog.base.set_state(614);
					recog.base.match_token(SEMICOLON,&mut recog.err_handler)?;

					recog.base.set_state(615);
					recog.base.match_token(ANNOT_END,&mut recog.err_handler)?;

					}
				}

			 LINE_ANNOT_START 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(617);
					recog.base.match_token(LINE_ANNOT_START,&mut recog.err_handler)?;

					recog.base.set_state(618);
					recog.base.match_token(ASSERT,&mut recog.err_handler)?;

					/*InvokeRule pred*/
					recog.base.set_state(619);
					recog.pred_rec(0)?;

					recog.base.set_state(620);
					recog.base.match_token(SEMICOLON,&mut recog.err_handler)?;

					recog.base.set_state(621);
					recog.base.match_token(LINEEND,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- loopAnnot ----------------
pub type LoopAnnotContextAll<'input> = LoopAnnotContext<'input>;


pub type LoopAnnotContext<'input> = BaseParserRuleContext<'input,LoopAnnotContextExt<'input>>;

#[derive(Clone)]
pub struct LoopAnnotContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TaurusParserContext<'input> for LoopAnnotContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for LoopAnnotContext<'input>{
		fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_loopAnnot(self);
		}fn exit(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
			listener.exit_loopAnnot(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for LoopAnnotContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_loopAnnot }
	//fn type_rule_index() -> usize where Self: Sized { RULE_loopAnnot }
}
antlr_rust::tid!{LoopAnnotContextExt<'a>}

impl<'input> LoopAnnotContextExt<'input>{
	fn new(parent: Option<Rc<dyn TaurusParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<LoopAnnotContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,LoopAnnotContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait LoopAnnotContextAttrs<'input>: TaurusParserContext<'input> + BorrowMut<LoopAnnotContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token ANNOT_START
/// Returns `None` if there is no child corresponding to token ANNOT_START
fn ANNOT_START(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
	self.get_token(ANNOT_START, 0)
}
/// Retrieves first TerminalNode corresponding to token ANNOT_END
/// Returns `None` if there is no child corresponding to token ANNOT_END
fn ANNOT_END(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
	self.get_token(ANNOT_END, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token LOOP in current rule
fn LOOP_all(&self) -> Vec<Rc<TerminalNode<'input,TaurusParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token LOOP, starting from 0.
/// Returns `None` if number of children corresponding to token LOOP is less or equal than `i`.
fn LOOP(&self, i: usize) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
	self.get_token(LOOP, i)
}
/// Retrieves all `TerminalNode`s corresponding to token INVARIANT in current rule
fn INVARIANT_all(&self) -> Vec<Rc<TerminalNode<'input,TaurusParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token INVARIANT, starting from 0.
/// Returns `None` if number of children corresponding to token INVARIANT is less or equal than `i`.
fn INVARIANT(&self, i: usize) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
	self.get_token(INVARIANT, i)
}
fn pred_all(&self) ->  Vec<Rc<PredContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn pred(&self, i: usize) -> Option<Rc<PredContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves all `TerminalNode`s corresponding to token SEMICOLON in current rule
fn SEMICOLON_all(&self) -> Vec<Rc<TerminalNode<'input,TaurusParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token SEMICOLON, starting from 0.
/// Returns `None` if number of children corresponding to token SEMICOLON is less or equal than `i`.
fn SEMICOLON(&self, i: usize) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
	self.get_token(SEMICOLON, i)
}
/// Retrieves first TerminalNode corresponding to token VARIANT
/// Returns `None` if there is no child corresponding to token VARIANT
fn VARIANT(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
	self.get_token(VARIANT, 0)
}
fn arithTerm_all(&self) ->  Vec<Rc<ArithTermContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn arithTerm(&self, i: usize) -> Option<Rc<ArithTermContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token LPAR
/// Returns `None` if there is no child corresponding to token LPAR
fn LPAR(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
	self.get_token(LPAR, 0)
}
/// Retrieves first TerminalNode corresponding to token RPAR
/// Returns `None` if there is no child corresponding to token RPAR
fn RPAR(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
	self.get_token(RPAR, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,TaurusParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
	self.get_token(COMMA, i)
}
/// Retrieves first TerminalNode corresponding to token LINE_ANNOT_START
/// Returns `None` if there is no child corresponding to token LINE_ANNOT_START
fn LINE_ANNOT_START(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
	self.get_token(LINE_ANNOT_START, 0)
}
/// Retrieves first TerminalNode corresponding to token LINEEND
/// Returns `None` if there is no child corresponding to token LINEEND
fn LINEEND(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
	self.get_token(LINEEND, 0)
}

}

impl<'input> LoopAnnotContextAttrs<'input> for LoopAnnotContext<'input>{}

impl<'input, I, H> TaurusParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn loopAnnot(&mut self,)
	-> Result<Rc<LoopAnnotContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = LoopAnnotContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 70, RULE_loopAnnot);
        let mut _localctx: Rc<LoopAnnotContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			let mut _alt: isize;
			recog.base.set_state(687);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 ANNOT_START 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(625);
					recog.base.match_token(ANNOT_START,&mut recog.err_handler)?;

					recog.base.set_state(633);
					recog.err_handler.sync(&mut recog.base)?;
					_alt = recog.interpreter.adaptive_predict(62,&mut recog.base)?;
					while { _alt!=2 && _alt!=INVALID_ALT } {
						if _alt==1 {
							{
							{
							recog.base.set_state(626);
							recog.base.match_token(LOOP,&mut recog.err_handler)?;

							recog.base.set_state(627);
							recog.base.match_token(INVARIANT,&mut recog.err_handler)?;

							/*InvokeRule pred*/
							recog.base.set_state(628);
							recog.pred_rec(0)?;

							recog.base.set_state(629);
							recog.base.match_token(SEMICOLON,&mut recog.err_handler)?;

							}
							} 
						}
						recog.base.set_state(635);
						recog.err_handler.sync(&mut recog.base)?;
						_alt = recog.interpreter.adaptive_predict(62,&mut recog.base)?;
					}
					recog.base.set_state(653);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==LOOP {
						{
						recog.base.set_state(636);
						recog.base.match_token(LOOP,&mut recog.err_handler)?;

						recog.base.set_state(637);
						recog.base.match_token(VARIANT,&mut recog.err_handler)?;

						recog.base.set_state(649);
						recog.err_handler.sync(&mut recog.base)?;
						match  recog.interpreter.adaptive_predict(64,&mut recog.base)? {
							1 =>{
								{
								/*InvokeRule arithTerm*/
								recog.base.set_state(638);
								recog.arithTerm_rec(0)?;

								}
							}
						,
							2 =>{
								{
								recog.base.set_state(639);
								recog.base.match_token(LPAR,&mut recog.err_handler)?;

								/*InvokeRule arithTerm*/
								recog.base.set_state(640);
								recog.arithTerm_rec(0)?;

								recog.base.set_state(643); 
								recog.err_handler.sync(&mut recog.base)?;
								_la = recog.base.input.la(1);
								loop {
									{
									{
									recog.base.set_state(641);
									recog.base.match_token(COMMA,&mut recog.err_handler)?;

									/*InvokeRule arithTerm*/
									recog.base.set_state(642);
									recog.arithTerm_rec(0)?;

									}
									}
									recog.base.set_state(645); 
									recog.err_handler.sync(&mut recog.base)?;
									_la = recog.base.input.la(1);
									if !(_la==COMMA) {break}
								}
								recog.base.set_state(647);
								recog.base.match_token(RPAR,&mut recog.err_handler)?;

								}
							}

							_ => {}
						}
						recog.base.set_state(651);
						recog.base.match_token(SEMICOLON,&mut recog.err_handler)?;

						}
					}

					recog.base.set_state(655);
					recog.base.match_token(ANNOT_END,&mut recog.err_handler)?;

					}
				}

			 LINE_ANNOT_START 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(656);
					recog.base.match_token(LINE_ANNOT_START,&mut recog.err_handler)?;

					recog.base.set_state(664);
					recog.err_handler.sync(&mut recog.base)?;
					_alt = recog.interpreter.adaptive_predict(66,&mut recog.base)?;
					while { _alt!=2 && _alt!=INVALID_ALT } {
						if _alt==1 {
							{
							{
							recog.base.set_state(657);
							recog.base.match_token(LOOP,&mut recog.err_handler)?;

							recog.base.set_state(658);
							recog.base.match_token(INVARIANT,&mut recog.err_handler)?;

							/*InvokeRule pred*/
							recog.base.set_state(659);
							recog.pred_rec(0)?;

							recog.base.set_state(660);
							recog.base.match_token(SEMICOLON,&mut recog.err_handler)?;

							}
							} 
						}
						recog.base.set_state(666);
						recog.err_handler.sync(&mut recog.base)?;
						_alt = recog.interpreter.adaptive_predict(66,&mut recog.base)?;
					}
					recog.base.set_state(684);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					if _la==LOOP {
						{
						recog.base.set_state(667);
						recog.base.match_token(LOOP,&mut recog.err_handler)?;

						recog.base.set_state(668);
						recog.base.match_token(VARIANT,&mut recog.err_handler)?;

						recog.base.set_state(680);
						recog.err_handler.sync(&mut recog.base)?;
						match  recog.interpreter.adaptive_predict(68,&mut recog.base)? {
							1 =>{
								{
								/*InvokeRule arithTerm*/
								recog.base.set_state(669);
								recog.arithTerm_rec(0)?;

								}
							}
						,
							2 =>{
								{
								recog.base.set_state(670);
								recog.base.match_token(LPAR,&mut recog.err_handler)?;

								/*InvokeRule arithTerm*/
								recog.base.set_state(671);
								recog.arithTerm_rec(0)?;

								recog.base.set_state(674); 
								recog.err_handler.sync(&mut recog.base)?;
								_la = recog.base.input.la(1);
								loop {
									{
									{
									recog.base.set_state(672);
									recog.base.match_token(COMMA,&mut recog.err_handler)?;

									/*InvokeRule arithTerm*/
									recog.base.set_state(673);
									recog.arithTerm_rec(0)?;

									}
									}
									recog.base.set_state(676); 
									recog.err_handler.sync(&mut recog.base)?;
									_la = recog.base.input.la(1);
									if !(_la==COMMA) {break}
								}
								recog.base.set_state(678);
								recog.base.match_token(RPAR,&mut recog.err_handler)?;

								}
							}

							_ => {}
						}
						recog.base.set_state(682);
						recog.base.match_token(SEMICOLON,&mut recog.err_handler)?;

						}
					}

					recog.base.set_state(686);
					recog.base.match_token(LINEEND,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- predDefs ----------------
pub type PredDefsContextAll<'input> = PredDefsContext<'input>;


pub type PredDefsContext<'input> = BaseParserRuleContext<'input,PredDefsContextExt<'input>>;

#[derive(Clone)]
pub struct PredDefsContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TaurusParserContext<'input> for PredDefsContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for PredDefsContext<'input>{
		fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_predDefs(self);
		}fn exit(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
			listener.exit_predDefs(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for PredDefsContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_predDefs }
	//fn type_rule_index() -> usize where Self: Sized { RULE_predDefs }
}
antlr_rust::tid!{PredDefsContextExt<'a>}

impl<'input> PredDefsContextExt<'input>{
	fn new(parent: Option<Rc<dyn TaurusParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<PredDefsContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PredDefsContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait PredDefsContextAttrs<'input>: TaurusParserContext<'input> + BorrowMut<PredDefsContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token ANNOT_START
/// Returns `None` if there is no child corresponding to token ANNOT_START
fn ANNOT_START(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
	self.get_token(ANNOT_START, 0)
}
/// Retrieves first TerminalNode corresponding to token ANNOT_END
/// Returns `None` if there is no child corresponding to token ANNOT_END
fn ANNOT_END(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
	self.get_token(ANNOT_END, 0)
}
fn predDef_all(&self) ->  Vec<Rc<PredDefContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn predDef(&self, i: usize) -> Option<Rc<PredDefContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token LINE_ANNOT_START
/// Returns `None` if there is no child corresponding to token LINE_ANNOT_START
fn LINE_ANNOT_START(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
	self.get_token(LINE_ANNOT_START, 0)
}
/// Retrieves first TerminalNode corresponding to token LINEEND
/// Returns `None` if there is no child corresponding to token LINEEND
fn LINEEND(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
	self.get_token(LINEEND, 0)
}

}

impl<'input> PredDefsContextAttrs<'input> for PredDefsContext<'input>{}

impl<'input, I, H> TaurusParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn predDefs(&mut self,)
	-> Result<Rc<PredDefsContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PredDefsContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 72, RULE_predDefs);
        let mut _localctx: Rc<PredDefsContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(705);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 ANNOT_START 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 1);
					recog.base.enter_outer_alt(None, 1);
					{
					recog.base.set_state(689);
					recog.base.match_token(ANNOT_START,&mut recog.err_handler)?;

					recog.base.set_state(693);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==PREDICATE {
						{
						{
						/*InvokeRule predDef*/
						recog.base.set_state(690);
						recog.predDef()?;

						}
						}
						recog.base.set_state(695);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					recog.base.set_state(696);
					recog.base.match_token(ANNOT_END,&mut recog.err_handler)?;

					}
				}

			 LINE_ANNOT_START 
				=> {
					//recog.base.enter_outer_alt(_localctx.clone(), 2);
					recog.base.enter_outer_alt(None, 2);
					{
					recog.base.set_state(697);
					recog.base.match_token(LINE_ANNOT_START,&mut recog.err_handler)?;

					recog.base.set_state(701);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
					while _la==PREDICATE {
						{
						{
						/*InvokeRule predDef*/
						recog.base.set_state(698);
						recog.predDef()?;

						}
						}
						recog.base.set_state(703);
						recog.err_handler.sync(&mut recog.base)?;
						_la = recog.base.input.la(1);
					}
					recog.base.set_state(704);
					recog.base.match_token(LINEEND,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- predDef ----------------
pub type PredDefContextAll<'input> = PredDefContext<'input>;


pub type PredDefContext<'input> = BaseParserRuleContext<'input,PredDefContextExt<'input>>;

#[derive(Clone)]
pub struct PredDefContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TaurusParserContext<'input> for PredDefContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for PredDefContext<'input>{
		fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
			listener.enter_every_rule(self);
			listener.enter_predDef(self);
		}fn exit(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
			listener.exit_predDef(self);
			listener.exit_every_rule(self);
		}
}

impl<'input> CustomRuleContext<'input> for PredDefContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_predDef }
	//fn type_rule_index() -> usize where Self: Sized { RULE_predDef }
}
antlr_rust::tid!{PredDefContextExt<'a>}

impl<'input> PredDefContextExt<'input>{
	fn new(parent: Option<Rc<dyn TaurusParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<PredDefContextAll<'input>> {
		Rc::new(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,PredDefContextExt{
				ph:PhantomData
			}),
		)
	}
}

pub trait PredDefContextAttrs<'input>: TaurusParserContext<'input> + BorrowMut<PredDefContextExt<'input>>{

/// Retrieves first TerminalNode corresponding to token PREDICATE
/// Returns `None` if there is no child corresponding to token PREDICATE
fn PREDICATE(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
	self.get_token(PREDICATE, 0)
}
/// Retrieves first TerminalNode corresponding to token IDENT
/// Returns `None` if there is no child corresponding to token IDENT
fn IDENT(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
	self.get_token(IDENT, 0)
}
/// Retrieves first TerminalNode corresponding to token ASSIGN
/// Returns `None` if there is no child corresponding to token ASSIGN
fn ASSIGN(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
	self.get_token(ASSIGN, 0)
}
fn pred(&self) -> Option<Rc<PredContextAll<'input>>> where Self:Sized{
	self.child_of_type(0)
}
/// Retrieves first TerminalNode corresponding to token SEMICOLON
/// Returns `None` if there is no child corresponding to token SEMICOLON
fn SEMICOLON(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
	self.get_token(SEMICOLON, 0)
}
/// Retrieves first TerminalNode corresponding to token LPAR
/// Returns `None` if there is no child corresponding to token LPAR
fn LPAR(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
	self.get_token(LPAR, 0)
}
fn logicParaVar_all(&self) ->  Vec<Rc<LogicParaVarContextAll<'input>>> where Self:Sized{
	self.children_of_type()
}
fn logicParaVar(&self, i: usize) -> Option<Rc<LogicParaVarContextAll<'input>>> where Self:Sized{
	self.child_of_type(i)
}
/// Retrieves first TerminalNode corresponding to token RPAR
/// Returns `None` if there is no child corresponding to token RPAR
fn RPAR(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
	self.get_token(RPAR, 0)
}
/// Retrieves all `TerminalNode`s corresponding to token COMMA in current rule
fn COMMA_all(&self) -> Vec<Rc<TerminalNode<'input,TaurusParserContextType>>>  where Self:Sized{
	self.children_of_type()
}
/// Retrieves 'i's TerminalNode corresponding to token COMMA, starting from 0.
/// Returns `None` if number of children corresponding to token COMMA is less or equal than `i`.
fn COMMA(&self, i: usize) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
	self.get_token(COMMA, i)
}

}

impl<'input> PredDefContextAttrs<'input> for PredDefContext<'input>{}

impl<'input, I, H> TaurusParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn predDef(&mut self,)
	-> Result<Rc<PredDefContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = PredDefContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 74, RULE_predDef);
        let mut _localctx: Rc<PredDefContextAll> = _localctx;
		let mut _la: isize = -1;
		let result: Result<(), ANTLRError> = (|| {

			//recog.base.enter_outer_alt(_localctx.clone(), 1);
			recog.base.enter_outer_alt(None, 1);
			{
			recog.base.set_state(707);
			recog.base.match_token(PREDICATE,&mut recog.err_handler)?;

			recog.base.set_state(708);
			recog.base.match_token(IDENT,&mut recog.err_handler)?;

			recog.base.set_state(720);
			recog.err_handler.sync(&mut recog.base)?;
			_la = recog.base.input.la(1);
			if _la==LPAR {
				{
				recog.base.set_state(709);
				recog.base.match_token(LPAR,&mut recog.err_handler)?;

				/*InvokeRule logicParaVar*/
				recog.base.set_state(710);
				recog.logicParaVar()?;

				recog.base.set_state(715);
				recog.err_handler.sync(&mut recog.base)?;
				_la = recog.base.input.la(1);
				while _la==COMMA {
					{
					{
					recog.base.set_state(711);
					recog.base.match_token(COMMA,&mut recog.err_handler)?;

					/*InvokeRule logicParaVar*/
					recog.base.set_state(712);
					recog.logicParaVar()?;

					}
					}
					recog.base.set_state(717);
					recog.err_handler.sync(&mut recog.base)?;
					_la = recog.base.input.la(1);
				}
				recog.base.set_state(718);
				recog.base.match_token(RPAR,&mut recog.err_handler)?;

				}
			}

			recog.base.set_state(722);
			recog.base.match_token(ASSIGN,&mut recog.err_handler)?;

			/*InvokeRule pred*/
			recog.base.set_state(723);
			recog.pred_rec(0)?;

			recog.base.set_state(724);
			recog.base.match_token(SEMICOLON,&mut recog.err_handler)?;

			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}
//------------------- constant ----------------
#[derive(Debug)]
pub enum ConstantContextAll<'input>{
	TrueConstContext(TrueConstContext<'input>),
	FalseConstContext(FalseConstContext<'input>),
	IntConstContext(IntConstContext<'input>),
	FloatConstContext(FloatConstContext<'input>),
Error(ConstantContext<'input>)
}
antlr_rust::tid!{ConstantContextAll<'a>}

impl<'input> antlr_rust::parser_rule_context::DerefSeal for ConstantContextAll<'input>{}

impl<'input> TaurusParserContext<'input> for ConstantContextAll<'input>{}

impl<'input> Deref for ConstantContextAll<'input>{
	type Target = dyn ConstantContextAttrs<'input> + 'input;
	fn deref(&self) -> &Self::Target{
		use ConstantContextAll::*;
		match self{
			TrueConstContext(inner) => inner,
			FalseConstContext(inner) => inner,
			IntConstContext(inner) => inner,
			FloatConstContext(inner) => inner,
Error(inner) => inner
		}
	}
}
impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for ConstantContextAll<'input>{
    fn enter(&self, listener: &mut (dyn TaurusListener<'input> + 'a)) { self.deref().enter(listener) }
    fn exit(&self, listener: &mut (dyn TaurusListener<'input> + 'a)) { self.deref().exit(listener) }
}



pub type ConstantContext<'input> = BaseParserRuleContext<'input,ConstantContextExt<'input>>;

#[derive(Clone)]
pub struct ConstantContextExt<'input>{
ph:PhantomData<&'input str>
}

impl<'input> TaurusParserContext<'input> for ConstantContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for ConstantContext<'input>{
}

impl<'input> CustomRuleContext<'input> for ConstantContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_constant }
	//fn type_rule_index() -> usize where Self: Sized { RULE_constant }
}
antlr_rust::tid!{ConstantContextExt<'a>}

impl<'input> ConstantContextExt<'input>{
	fn new(parent: Option<Rc<dyn TaurusParserContext<'input> + 'input > >, invoking_state: isize) -> Rc<ConstantContextAll<'input>> {
		Rc::new(
		ConstantContextAll::Error(
			BaseParserRuleContext::new_parser_ctx(parent, invoking_state,ConstantContextExt{
				ph:PhantomData
			}),
		)
		)
	}
}

pub trait ConstantContextAttrs<'input>: TaurusParserContext<'input> + BorrowMut<ConstantContextExt<'input>>{


}

impl<'input> ConstantContextAttrs<'input> for ConstantContext<'input>{}

pub type TrueConstContext<'input> = BaseParserRuleContext<'input,TrueConstContextExt<'input>>;

pub trait TrueConstContextAttrs<'input>: TaurusParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token EXPR_TRUE
	/// Returns `None` if there is no child corresponding to token EXPR_TRUE
	fn EXPR_TRUE(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(EXPR_TRUE, 0)
	}
}

impl<'input> TrueConstContextAttrs<'input> for TrueConstContext<'input>{}

pub struct TrueConstContextExt<'input>{
	base:ConstantContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{TrueConstContextExt<'a>}

impl<'input> TaurusParserContext<'input> for TrueConstContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for TrueConstContext<'input>{
	fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_TrueConst(self);
	}
}

impl<'input> CustomRuleContext<'input> for TrueConstContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_constant }
	//fn type_rule_index() -> usize where Self: Sized { RULE_constant }
}

impl<'input> Borrow<ConstantContextExt<'input>> for TrueConstContext<'input>{
	fn borrow(&self) -> &ConstantContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ConstantContextExt<'input>> for TrueConstContext<'input>{
	fn borrow_mut(&mut self) -> &mut ConstantContextExt<'input> { &mut self.base }
}

impl<'input> ConstantContextAttrs<'input> for TrueConstContext<'input> {}

impl<'input> TrueConstContextExt<'input>{
	fn new(ctx: &dyn ConstantContextAttrs<'input>) -> Rc<ConstantContextAll<'input>>  {
		Rc::new(
			ConstantContextAll::TrueConstContext(
				BaseParserRuleContext::copy_from(ctx,TrueConstContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type FalseConstContext<'input> = BaseParserRuleContext<'input,FalseConstContextExt<'input>>;

pub trait FalseConstContextAttrs<'input>: TaurusParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token EXPR_FALSE
	/// Returns `None` if there is no child corresponding to token EXPR_FALSE
	fn EXPR_FALSE(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(EXPR_FALSE, 0)
	}
}

impl<'input> FalseConstContextAttrs<'input> for FalseConstContext<'input>{}

pub struct FalseConstContextExt<'input>{
	base:ConstantContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{FalseConstContextExt<'a>}

impl<'input> TaurusParserContext<'input> for FalseConstContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for FalseConstContext<'input>{
	fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_FalseConst(self);
	}
}

impl<'input> CustomRuleContext<'input> for FalseConstContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_constant }
	//fn type_rule_index() -> usize where Self: Sized { RULE_constant }
}

impl<'input> Borrow<ConstantContextExt<'input>> for FalseConstContext<'input>{
	fn borrow(&self) -> &ConstantContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ConstantContextExt<'input>> for FalseConstContext<'input>{
	fn borrow_mut(&mut self) -> &mut ConstantContextExt<'input> { &mut self.base }
}

impl<'input> ConstantContextAttrs<'input> for FalseConstContext<'input> {}

impl<'input> FalseConstContextExt<'input>{
	fn new(ctx: &dyn ConstantContextAttrs<'input>) -> Rc<ConstantContextAll<'input>>  {
		Rc::new(
			ConstantContextAll::FalseConstContext(
				BaseParserRuleContext::copy_from(ctx,FalseConstContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type IntConstContext<'input> = BaseParserRuleContext<'input,IntConstContextExt<'input>>;

pub trait IntConstContextAttrs<'input>: TaurusParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token INT_CONSTANT
	/// Returns `None` if there is no child corresponding to token INT_CONSTANT
	fn INT_CONSTANT(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(INT_CONSTANT, 0)
	}
}

impl<'input> IntConstContextAttrs<'input> for IntConstContext<'input>{}

pub struct IntConstContextExt<'input>{
	base:ConstantContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{IntConstContextExt<'a>}

impl<'input> TaurusParserContext<'input> for IntConstContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for IntConstContext<'input>{
	fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_IntConst(self);
	}
}

impl<'input> CustomRuleContext<'input> for IntConstContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_constant }
	//fn type_rule_index() -> usize where Self: Sized { RULE_constant }
}

impl<'input> Borrow<ConstantContextExt<'input>> for IntConstContext<'input>{
	fn borrow(&self) -> &ConstantContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ConstantContextExt<'input>> for IntConstContext<'input>{
	fn borrow_mut(&mut self) -> &mut ConstantContextExt<'input> { &mut self.base }
}

impl<'input> ConstantContextAttrs<'input> for IntConstContext<'input> {}

impl<'input> IntConstContextExt<'input>{
	fn new(ctx: &dyn ConstantContextAttrs<'input>) -> Rc<ConstantContextAll<'input>>  {
		Rc::new(
			ConstantContextAll::IntConstContext(
				BaseParserRuleContext::copy_from(ctx,IntConstContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

pub type FloatConstContext<'input> = BaseParserRuleContext<'input,FloatConstContextExt<'input>>;

pub trait FloatConstContextAttrs<'input>: TaurusParserContext<'input>{
	/// Retrieves first TerminalNode corresponding to token FLOAT_CONSTANT
	/// Returns `None` if there is no child corresponding to token FLOAT_CONSTANT
	fn FLOAT_CONSTANT(&self) -> Option<Rc<TerminalNode<'input,TaurusParserContextType>>> where Self:Sized{
		self.get_token(FLOAT_CONSTANT, 0)
	}
}

impl<'input> FloatConstContextAttrs<'input> for FloatConstContext<'input>{}

pub struct FloatConstContextExt<'input>{
	base:ConstantContextExt<'input>,
	ph:PhantomData<&'input str>
}

antlr_rust::tid!{FloatConstContextExt<'a>}

impl<'input> TaurusParserContext<'input> for FloatConstContext<'input>{}

impl<'input,'a> Listenable<dyn TaurusListener<'input> + 'a> for FloatConstContext<'input>{
	fn enter(&self,listener: &mut (dyn TaurusListener<'input> + 'a)) {
		listener.enter_every_rule(self);
		listener.enter_FloatConst(self);
	}
}

impl<'input> CustomRuleContext<'input> for FloatConstContextExt<'input>{
	type TF = LocalTokenFactory<'input>;
	type Ctx = TaurusParserContextType;
	fn get_rule_index(&self) -> usize { RULE_constant }
	//fn type_rule_index() -> usize where Self: Sized { RULE_constant }
}

impl<'input> Borrow<ConstantContextExt<'input>> for FloatConstContext<'input>{
	fn borrow(&self) -> &ConstantContextExt<'input> { &self.base }
}
impl<'input> BorrowMut<ConstantContextExt<'input>> for FloatConstContext<'input>{
	fn borrow_mut(&mut self) -> &mut ConstantContextExt<'input> { &mut self.base }
}

impl<'input> ConstantContextAttrs<'input> for FloatConstContext<'input> {}

impl<'input> FloatConstContextExt<'input>{
	fn new(ctx: &dyn ConstantContextAttrs<'input>) -> Rc<ConstantContextAll<'input>>  {
		Rc::new(
			ConstantContextAll::FloatConstContext(
				BaseParserRuleContext::copy_from(ctx,FloatConstContextExt{
        			base: ctx.borrow().clone(),
        			ph:PhantomData
				})
			)
		)
	}
}

impl<'input, I, H> TaurusParser<'input, I, H>
where
    I: TokenStream<'input, TF = LocalTokenFactory<'input> > + TidAble<'input>,
    H: ErrorStrategy<'input,BaseParserType<'input,I>>
{
	pub fn constant(&mut self,)
	-> Result<Rc<ConstantContextAll<'input>>,ANTLRError> {
		let mut recog = self;
		let _parentctx = recog.ctx.take();
		let mut _localctx = ConstantContextExt::new(_parentctx.clone(), recog.base.get_state());
        recog.base.enter_rule(_localctx.clone(), 76, RULE_constant);
        let mut _localctx: Rc<ConstantContextAll> = _localctx;
		let result: Result<(), ANTLRError> = (|| {

			recog.base.set_state(730);
			recog.err_handler.sync(&mut recog.base)?;
			match recog.base.input.la(1) {
			 INT_CONSTANT 
				=> {
					let tmp = IntConstContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 1);
					_localctx = tmp;
					{
					recog.base.set_state(726);
					recog.base.match_token(INT_CONSTANT,&mut recog.err_handler)?;

					}
				}

			 FLOAT_CONSTANT 
				=> {
					let tmp = FloatConstContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 2);
					_localctx = tmp;
					{
					recog.base.set_state(727);
					recog.base.match_token(FLOAT_CONSTANT,&mut recog.err_handler)?;

					}
				}

			 EXPR_TRUE 
				=> {
					let tmp = TrueConstContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 3);
					_localctx = tmp;
					{
					recog.base.set_state(728);
					recog.base.match_token(EXPR_TRUE,&mut recog.err_handler)?;

					}
				}

			 EXPR_FALSE 
				=> {
					let tmp = FalseConstContextExt::new(&**_localctx);
					recog.base.enter_outer_alt(Some(tmp.clone()), 4);
					_localctx = tmp;
					{
					recog.base.set_state(729);
					recog.base.match_token(EXPR_FALSE,&mut recog.err_handler)?;

					}
				}

				_ => Err(ANTLRError::NoAltError(NoViableAltError::new(&mut recog.base)))?
			}
			Ok(())
		})();
		match result {
		Ok(_)=>{},
        Err(e @ ANTLRError::FallThrough(_)) => return Err(e),
		Err(ref re) => {
				//_localctx.exception = re;
				recog.err_handler.report_error(&mut recog.base, re);
				recog.err_handler.recover(&mut recog.base, re)?;
			}
		}
		recog.base.exit_rule();

		Ok(_localctx)
	}
}

lazy_static! {
    static ref _ATN: Arc<ATN> =
        Arc::new(ATNDeserializer::new(None).deserialize(_serializedATN.chars()));
    static ref _decision_to_DFA: Arc<Vec<antlr_rust::RwLock<DFA>>> = {
        let mut dfa = Vec::new();
        let size = _ATN.decision_to_state.len();
        for i in 0..size {
            dfa.push(DFA::new(
                _ATN.clone(),
                _ATN.get_decision_state(i),
                i as isize,
            ).into())
        }
        Arc::new(dfa)
    };
}



const _serializedATN:&'static str =
	"\x03\u{608b}\u{a72a}\u{8133}\u{b9ed}\u{417c}\u{3be7}\u{7786}\u{5964}\x03\
	\x4c\u{2df}\x04\x02\x09\x02\x04\x03\x09\x03\x04\x04\x09\x04\x04\x05\x09\
	\x05\x04\x06\x09\x06\x04\x07\x09\x07\x04\x08\x09\x08\x04\x09\x09\x09\x04\
	\x0a\x09\x0a\x04\x0b\x09\x0b\x04\x0c\x09\x0c\x04\x0d\x09\x0d\x04\x0e\x09\
	\x0e\x04\x0f\x09\x0f\x04\x10\x09\x10\x04\x11\x09\x11\x04\x12\x09\x12\x04\
	\x13\x09\x13\x04\x14\x09\x14\x04\x15\x09\x15\x04\x16\x09\x16\x04\x17\x09\
	\x17\x04\x18\x09\x18\x04\x19\x09\x19\x04\x1a\x09\x1a\x04\x1b\x09\x1b\x04\
	\x1c\x09\x1c\x04\x1d\x09\x1d\x04\x1e\x09\x1e\x04\x1f\x09\x1f\x04\x20\x09\
	\x20\x04\x21\x09\x21\x04\x22\x09\x22\x04\x23\x09\x23\x04\x24\x09\x24\x04\
	\x25\x09\x25\x04\x26\x09\x26\x04\x27\x09\x27\x04\x28\x09\x28\x03\x02\x07\
	\x02\x52\x0a\x02\x0c\x02\x0e\x02\x55\x0b\x02\x03\x02\x03\x02\x03\x03\x03\
	\x03\x03\x03\x05\x03\x5c\x0a\x03\x03\x04\x03\x04\x05\x04\x60\x0a\x04\x03\
	\x05\x03\x05\x03\x05\x03\x05\x03\x05\x03\x05\x07\x05\x68\x0a\x05\x0c\x05\
	\x0e\x05\x6b\x0b\x05\x05\x05\x6d\x0a\x05\x03\x05\x03\x05\x03\x05\x07\x05\
	\x72\x0a\x05\x0c\x05\x0e\x05\x75\x0b\x05\x03\x05\x03\x05\x03\x06\x03\x06\
	\x03\x06\x03\x06\x03\x06\x03\x06\x07\x06\x7f\x0a\x06\x0c\x06\x0e\x06\u{82}\
	\x0b\x06\x03\x06\x03\x06\x03\x06\x03\x07\x03\x07\x03\x07\x03\x07\x03\x07\
	\x03\x07\x03\x07\x03\x07\x03\x07\x03\x07\x03\x07\x03\x07\x05\x07\u{93}\x0a\
	\x07\x03\x08\x03\x08\x03\x08\x03\x08\x03\x08\x03\x08\x03\x08\x03\x08\x03\
	\x08\x03\x08\x03\x08\x05\x08\u{a0}\x0a\x08\x03\x09\x03\x09\x03\x09\x03\x0a\
	\x03\x0a\x03\x0a\x03\x0a\x03\x0a\x03\x0a\x03\x0a\x03\x0a\x05\x0a\u{ad}\x0a\
	\x0a\x03\x0b\x03\x0b\x05\x0b\u{b1}\x0a\x0b\x03\x0c\x03\x0c\x03\x0c\x03\x0c\
	\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x03\x0c\x05\x0c\u{be}\x0a\
	\x0c\x03\x0d\x03\x0d\x03\x0d\x05\x0d\u{c3}\x0a\x0d\x03\x0e\x03\x0e\x03\x0e\
	\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\
	\x03\x0e\x03\x0e\x05\x0e\u{d3}\x0a\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\
	\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\
	\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x05\x0e\u{e8}\x0a\x0e\x03\x0e\
	\x03\x0e\x05\x0e\u{ec}\x0a\x0e\x03\x0e\x03\x0e\x05\x0e\u{f0}\x0a\x0e\x03\
	\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x05\
	\x0e\u{fb}\x0a\x0e\x03\x0e\x03\x0e\x03\x0e\x03\x0e\x07\x0e\u{101}\x0a\x0e\
	\x0c\x0e\x0e\x0e\u{104}\x0b\x0e\x03\x0e\x05\x0e\u{107}\x0a\x0e\x03\x0f\x03\
	\x0f\x03\x0f\x05\x0f\u{10c}\x0a\x0f\x03\x0f\x05\x0f\u{10f}\x0a\x0f\x03\x10\
	\x03\x10\x03\x10\x03\x10\x03\x10\x03\x10\x03\x10\x03\x10\x03\x10\x03\x10\
	\x03\x10\x03\x10\x03\x10\x03\x10\x03\x10\x05\x10\u{120}\x0a\x10\x03\x11\
	\x03\x11\x03\x11\x05\x11\u{125}\x0a\x11\x03\x11\x03\x11\x03\x12\x03\x12\
	\x03\x12\x03\x12\x03\x12\x03\x12\x03\x12\x03\x12\x07\x12\u{131}\x0a\x12\
	\x0c\x12\x0e\x12\u{134}\x0b\x12\x05\x12\u{136}\x0a\x12\x03\x12\x03\x12\x03\
	\x12\x03\x12\x03\x12\x03\x12\x03\x12\x03\x12\x05\x12\u{140}\x0a\x12\x03\
	\x12\x03\x12\x03\x12\x03\x12\x03\x12\x03\x12\x03\x12\x03\x12\x03\x12\x03\
	\x12\x03\x12\x03\x12\x03\x12\x03\x12\x03\x12\x03\x12\x03\x12\x03\x12\x03\
	\x12\x03\x12\x03\x12\x03\x12\x03\x12\x03\x12\x03\x12\x03\x12\x03\x12\x03\
	\x12\x03\x12\x03\x12\x07\x12\u{160}\x0a\x12\x0c\x12\x0e\x12\u{163}\x0b\x12\
	\x03\x13\x03\x13\x03\x13\x03\x13\x05\x13\u{169}\x0a\x13\x03\x14\x03\x14\
	\x03\x14\x03\x14\x03\x14\x03\x14\x03\x14\x03\x14\x03\x14\x03\x14\x03\x14\
	\x03\x14\x03\x14\x03\x14\x03\x14\x03\x14\x03\x14\x03\x14\x03\x14\x03\x14\
	\x03\x14\x05\x14\u{180}\x0a\x14\x03\x14\x03\x14\x03\x14\x03\x14\x03\x14\
	\x03\x14\x03\x14\x03\x14\x03\x14\x03\x14\x03\x14\x03\x14\x03\x14\x03\x14\
	\x03\x14\x03\x14\x07\x14\u{192}\x0a\x14\x0c\x14\x0e\x14\u{195}\x0b\x14\x03\
	\x15\x03\x15\x03\x15\x03\x15\x03\x15\x03\x15\x05\x15\u{19d}\x0a\x15\x03\
	\x15\x03\x15\x03\x15\x03\x15\x03\x15\x03\x15\x03\x15\x03\x15\x03\x15\x03\
	\x15\x03\x15\x03\x15\x03\x15\x03\x15\x07\x15\u{1ad}\x0a\x15\x0c\x15\x0e\
	\x15\u{1b0}\x0b\x15\x03\x16\x03\x16\x03\x16\x03\x16\x03\x16\x03\x16\x03\
	\x16\x06\x16\u{1b9}\x0a\x16\x0d\x16\x0e\x16\u{1ba}\x03\x16\x03\x16\x03\x16\
	\x03\x16\x03\x16\x07\x16\u{1c2}\x0a\x16\x0c\x16\x0e\x16\u{1c5}\x0b\x16\x03\
	\x16\x03\x16\x05\x16\u{1c9}\x0a\x16\x03\x16\x03\x16\x03\x16\x03\x16\x03\
	\x16\x03\x16\x03\x16\x03\x16\x03\x16\x03\x16\x03\x16\x03\x16\x03\x16\x03\
	\x16\x03\x16\x03\x16\x03\x16\x03\x16\x03\x16\x03\x16\x03\x16\x07\x16\u{1e0}\
	\x0a\x16\x0c\x16\x0e\x16\u{1e3}\x0b\x16\x03\x16\x03\x16\x03\x16\x05\x16\
	\u{1e8}\x0a\x16\x03\x16\x03\x16\x03\x16\x03\x16\x03\x16\x03\x16\x03\x16\
	\x03\x16\x03\x16\x03\x16\x03\x16\x03\x16\x03\x16\x03\x16\x03\x16\x07\x16\
	\u{1f9}\x0a\x16\x0c\x16\x0e\x16\u{1fc}\x0b\x16\x03\x17\x03\x17\x05\x17\u{200}\
	\x0a\x17\x03\x18\x03\x18\x05\x18\u{204}\x0a\x18\x03\x19\x03\x19\x03\x19\
	\x05\x19\u{209}\x0a\x19\x03\x1a\x03\x1a\x05\x1a\u{20d}\x0a\x1a\x03\x1b\x03\
	\x1b\x05\x1b\u{211}\x0a\x1b\x03\x1c\x03\x1c\x03\x1c\x03\x1c\x05\x1c\u{217}\
	\x0a\x1c\x03\x1d\x03\x1d\x05\x1d\u{21b}\x0a\x1d\x03\x1e\x03\x1e\x05\x1e\
	\u{21f}\x0a\x1e\x03\x1f\x03\x1f\x03\x1f\x03\x1f\x07\x1f\u{225}\x0a\x1f\x0c\
	\x1f\x0e\x1f\u{228}\x0b\x1f\x03\x20\x03\x20\x07\x20\u{22c}\x0a\x20\x0c\x20\
	\x0e\x20\u{22f}\x0b\x20\x03\x20\x05\x20\u{232}\x0a\x20\x03\x20\x07\x20\u{235}\
	\x0a\x20\x0c\x20\x0e\x20\u{238}\x0b\x20\x03\x20\x03\x20\x03\x20\x07\x20\
	\u{23d}\x0a\x20\x0c\x20\x0e\x20\u{240}\x0b\x20\x03\x20\x05\x20\u{243}\x0a\
	\x20\x03\x20\x07\x20\u{246}\x0a\x20\x0c\x20\x0e\x20\u{249}\x0b\x20\x03\x20\
	\x05\x20\u{24c}\x0a\x20\x03\x21\x03\x21\x03\x21\x03\x21\x03\x22\x03\x22\
	\x03\x22\x03\x22\x03\x22\x03\x22\x06\x22\u{258}\x0a\x22\x0d\x22\x0e\x22\
	\u{259}\x03\x22\x03\x22\x05\x22\u{25e}\x0a\x22\x03\x22\x03\x22\x03\x23\x03\
	\x23\x03\x23\x03\x23\x03\x24\x03\x24\x03\x24\x03\x24\x03\x24\x03\x24\x03\
	\x24\x03\x24\x03\x24\x03\x24\x03\x24\x03\x24\x05\x24\u{272}\x0a\x24\x03\
	\x25\x03\x25\x03\x25\x03\x25\x03\x25\x03\x25\x07\x25\u{27a}\x0a\x25\x0c\
	\x25\x0e\x25\u{27d}\x0b\x25\x03\x25\x03\x25\x03\x25\x03\x25\x03\x25\x03\
	\x25\x03\x25\x06\x25\u{286}\x0a\x25\x0d\x25\x0e\x25\u{287}\x03\x25\x03\x25\
	\x05\x25\u{28c}\x0a\x25\x03\x25\x03\x25\x05\x25\u{290}\x0a\x25\x03\x25\x03\
	\x25\x03\x25\x03\x25\x03\x25\x03\x25\x03\x25\x07\x25\u{299}\x0a\x25\x0c\
	\x25\x0e\x25\u{29c}\x0b\x25\x03\x25\x03\x25\x03\x25\x03\x25\x03\x25\x03\
	\x25\x03\x25\x06\x25\u{2a5}\x0a\x25\x0d\x25\x0e\x25\u{2a6}\x03\x25\x03\x25\
	\x05\x25\u{2ab}\x0a\x25\x03\x25\x03\x25\x05\x25\u{2af}\x0a\x25\x03\x25\x05\
	\x25\u{2b2}\x0a\x25\x03\x26\x03\x26\x07\x26\u{2b6}\x0a\x26\x0c\x26\x0e\x26\
	\u{2b9}\x0b\x26\x03\x26\x03\x26\x03\x26\x07\x26\u{2be}\x0a\x26\x0c\x26\x0e\
	\x26\u{2c1}\x0b\x26\x03\x26\x05\x26\u{2c4}\x0a\x26\x03\x27\x03\x27\x03\x27\
	\x03\x27\x03\x27\x03\x27\x07\x27\u{2cc}\x0a\x27\x0c\x27\x0e\x27\u{2cf}\x0b\
	\x27\x03\x27\x03\x27\x05\x27\u{2d3}\x0a\x27\x03\x27\x03\x27\x03\x27\x03\
	\x27\x03\x28\x03\x28\x03\x28\x03\x28\x05\x28\u{2dd}\x0a\x28\x03\x28\x02\
	\x06\x22\x26\x28\x2a\x29\x02\x04\x06\x08\x0a\x0c\x0e\x10\x12\x14\x16\x18\
	\x1a\x1c\x1e\x20\x22\x24\x26\x28\x2a\x2c\x2e\x30\x32\x34\x36\x38\x3a\x3c\
	\x3e\x40\x42\x44\x46\x48\x4a\x4c\x4e\x02\x02\x02\u{337}\x02\x53\x03\x02\
	\x02\x02\x04\x5b\x03\x02\x02\x02\x06\x5f\x03\x02\x02\x02\x08\x61\x03\x02\
	\x02\x02\x0a\x78\x03\x02\x02\x02\x0c\u{92}\x03\x02\x02\x02\x0e\u{9f}\x03\
	\x02\x02\x02\x10\u{a1}\x03\x02\x02\x02\x12\u{ac}\x03\x02\x02\x02\x14\u{b0}\
	\x03\x02\x02\x02\x16\u{bd}\x03\x02\x02\x02\x18\u{c2}\x03\x02\x02\x02\x1a\
	\u{106}\x03\x02\x02\x02\x1c\u{10e}\x03\x02\x02\x02\x1e\u{11f}\x03\x02\x02\
	\x02\x20\u{121}\x03\x02\x02\x02\x22\u{13f}\x03\x02\x02\x02\x24\u{168}\x03\
	\x02\x02\x02\x26\u{17f}\x03\x02\x02\x02\x28\u{19c}\x03\x02\x02\x02\x2a\u{1e7}\
	\x03\x02\x02\x02\x2c\u{1ff}\x03\x02\x02\x02\x2e\u{203}\x03\x02\x02\x02\x30\
	\u{208}\x03\x02\x02\x02\x32\u{20c}\x03\x02\x02\x02\x34\u{210}\x03\x02\x02\
	\x02\x36\u{216}\x03\x02\x02\x02\x38\u{21a}\x03\x02\x02\x02\x3a\u{21e}\x03\
	\x02\x02\x02\x3c\u{220}\x03\x02\x02\x02\x3e\u{24b}\x03\x02\x02\x02\x40\u{24d}\
	\x03\x02\x02\x02\x42\u{251}\x03\x02\x02\x02\x44\u{261}\x03\x02\x02\x02\x46\
	\u{271}\x03\x02\x02\x02\x48\u{2b1}\x03\x02\x02\x02\x4a\u{2c3}\x03\x02\x02\
	\x02\x4c\u{2c5}\x03\x02\x02\x02\x4e\u{2dc}\x03\x02\x02\x02\x50\x52\x05\x04\
	\x03\x02\x51\x50\x03\x02\x02\x02\x52\x55\x03\x02\x02\x02\x53\x51\x03\x02\
	\x02\x02\x53\x54\x03\x02\x02\x02\x54\x56\x03\x02\x02\x02\x55\x53\x03\x02\
	\x02\x02\x56\x57\x07\x02\x02\x03\x57\x03\x03\x02\x02\x02\x58\x5c\x05\x08\
	\x05\x02\x59\x5c\x05\x0a\x06\x02\x5a\x5c\x05\x4a\x26\x02\x5b\x58\x03\x02\
	\x02\x02\x5b\x59\x03\x02\x02\x02\x5b\x5a\x03\x02\x02\x02\x5c\x05\x03\x02\
	\x02\x02\x5d\x60\x05\x20\x11\x02\x5e\x60\x05\x1a\x0e\x02\x5f\x5d\x03\x02\
	\x02\x02\x5f\x5e\x03\x02\x02\x02\x60\x07\x03\x02\x02\x02\x61\x62\x05\x3e\
	\x20\x02\x62\x63\x05\x10\x09\x02\x63\x6c\x07\x05\x02\x02\x64\x69\x05\x0e\
	\x08\x02\x65\x66\x07\x09\x02\x02\x66\x68\x05\x0e\x08\x02\x67\x65\x03\x02\
	\x02\x02\x68\x6b\x03\x02\x02\x02\x69\x67\x03\x02\x02\x02\x69\x6a\x03\x02\
	\x02\x02\x6a\x6d\x03\x02\x02\x02\x6b\x69\x03\x02\x02\x02\x6c\x64\x03\x02\
	\x02\x02\x6c\x6d\x03\x02\x02\x02\x6d\x6e\x03\x02\x02\x02\x6e\x6f\x07\x06\
	\x02\x02\x6f\x73\x07\x07\x02\x02\x70\x72\x05\x06\x04\x02\x71\x70\x03\x02\
	\x02\x02\x72\x75\x03\x02\x02\x02\x73\x71\x03\x02\x02\x02\x73\x74\x03\x02\
	\x02\x02\x74\x76\x03\x02\x02\x02\x75\x73\x03\x02\x02\x02\x76\x77\x07\x08\
	\x02\x02\x77\x09\x03\x02\x02\x02\x78\x79\x07\x04\x02\x02\x79\x7a\x07\x44\
	\x02\x02\x7a\u{80}\x07\x07\x02\x02\x7b\x7c\x05\x0e\x08\x02\x7c\x7d\x07\x0a\
	\x02\x02\x7d\x7f\x03\x02\x02\x02\x7e\x7b\x03\x02\x02\x02\x7f\u{82}\x03\x02\
	\x02\x02\u{80}\x7e\x03\x02\x02\x02\u{80}\u{81}\x03\x02\x02\x02\u{81}\u{83}\
	\x03\x02\x02\x02\u{82}\u{80}\x03\x02\x02\x02\u{83}\u{84}\x07\x08\x02\x02\
	\u{84}\u{85}\x07\x0a\x02\x02\u{85}\x0b\x03\x02\x02\x02\u{86}\u{87}\x05\x14\
	\x0b\x02\u{87}\u{88}\x07\x44\x02\x02\u{88}\u{93}\x03\x02\x02\x02\u{89}\u{8a}\
	\x07\x04\x02\x02\u{8a}\u{8b}\x07\x44\x02\x02\u{8b}\u{93}\x07\x44\x02\x02\
	\u{8c}\u{8d}\x05\x14\x0b\x02\u{8d}\u{8e}\x07\x44\x02\x02\u{8e}\u{8f}\x07\
	\x0b\x02\x02\u{8f}\u{90}\x07\x42\x02\x02\u{90}\u{91}\x07\x0c\x02\x02\u{91}\
	\u{93}\x03\x02\x02\x02\u{92}\u{86}\x03\x02\x02\x02\u{92}\u{89}\x03\x02\x02\
	\x02\u{92}\u{8c}\x03\x02\x02\x02\u{93}\x0d\x03\x02\x02\x02\u{94}\u{95}\x05\
	\x14\x0b\x02\u{95}\u{96}\x07\x44\x02\x02\u{96}\u{a0}\x03\x02\x02\x02\u{97}\
	\u{98}\x07\x04\x02\x02\u{98}\u{99}\x07\x44\x02\x02\u{99}\u{a0}\x07\x44\x02\
	\x02\u{9a}\u{9b}\x05\x14\x0b\x02\u{9b}\u{9c}\x07\x44\x02\x02\u{9c}\u{9d}\
	\x07\x0b\x02\x02\u{9d}\u{9e}\x07\x0c\x02\x02\u{9e}\u{a0}\x03\x02\x02\x02\
	\u{9f}\u{94}\x03\x02\x02\x02\u{9f}\u{97}\x03\x02\x02\x02\u{9f}\u{9a}\x03\
	\x02\x02\x02\u{a0}\x0f\x03\x02\x02\x02\u{a1}\u{a2}\x05\x12\x0a\x02\u{a2}\
	\u{a3}\x07\x44\x02\x02\u{a3}\x11\x03\x02\x02\x02\u{a4}\u{ad}\x05\x14\x0b\
	\x02\u{a5}\u{a6}\x07\x04\x02\x02\u{a6}\u{ad}\x07\x44\x02\x02\u{a7}\u{a8}\
	\x05\x14\x0b\x02\u{a8}\u{a9}\x07\x0b\x02\x02\u{a9}\u{aa}\x07\x0c\x02\x02\
	\u{aa}\u{ad}\x03\x02\x02\x02\u{ab}\u{ad}\x07\x03\x02\x02\u{ac}\u{a4}\x03\
	\x02\x02\x02\u{ac}\u{a5}\x03\x02\x02\x02\u{ac}\u{a7}\x03\x02\x02\x02\u{ac}\
	\u{ab}\x03\x02\x02\x02\u{ad}\x13\x03\x02\x02\x02\u{ae}\u{b1}\x07\x0e\x02\
	\x02\u{af}\u{b1}\x07\x0f\x02\x02\u{b0}\u{ae}\x03\x02\x02\x02\u{b0}\u{af}\
	\x03\x02\x02\x02\u{b1}\x15\x03\x02\x02\x02\u{b2}\u{b3}\x05\x18\x0d\x02\u{b3}\
	\u{b4}\x07\x44\x02\x02\u{b4}\u{be}\x03\x02\x02\x02\u{b5}\u{b6}\x07\x04\x02\
	\x02\u{b6}\u{b7}\x07\x44\x02\x02\u{b7}\u{be}\x07\x44\x02\x02\u{b8}\u{b9}\
	\x05\x18\x0d\x02\u{b9}\u{ba}\x07\x44\x02\x02\u{ba}\u{bb}\x07\x0b\x02\x02\
	\u{bb}\u{bc}\x07\x0c\x02\x02\u{bc}\u{be}\x03\x02\x02\x02\u{bd}\u{b2}\x03\
	\x02\x02\x02\u{bd}\u{b5}\x03\x02\x02\x02\u{bd}\u{b8}\x03\x02\x02\x02\u{be}\
	\x17\x03\x02\x02\x02\u{bf}\u{c3}\x07\x36\x02\x02\u{c0}\u{c3}\x07\x37\x02\
	\x02\u{c1}\u{c3}\x07\x35\x02\x02\u{c2}\u{bf}\x03\x02\x02\x02\u{c2}\u{c0}\
	\x03\x02\x02\x02\u{c2}\u{c1}\x03\x02\x02\x02\u{c3}\x19\x03\x02\x02\x02\u{c4}\
	\u{107}\x07\x0a\x02\x02\u{c5}\u{c6}\x05\x22\x12\x02\u{c6}\u{c7}\x07\x0a\
	\x02\x02\u{c7}\u{107}\x03\x02\x02\x02\u{c8}\u{c9}\x05\x1e\x10\x02\u{c9}\
	\u{ca}\x07\x0a\x02\x02\u{ca}\u{107}\x03\x02\x02\x02\u{cb}\u{cc}\x07\x11\
	\x02\x02\u{cc}\u{cd}\x07\x05\x02\x02\u{cd}\u{ce}\x05\x22\x12\x02\u{ce}\u{cf}\
	\x07\x06\x02\x02\u{cf}\u{d2}\x05\x1a\x0e\x02\u{d0}\u{d1}\x07\x12\x02\x02\
	\u{d1}\u{d3}\x05\x1a\x0e\x02\u{d2}\u{d0}\x03\x02\x02\x02\u{d2}\u{d3}\x03\
	\x02\x02\x02\u{d3}\u{107}\x03\x02\x02\x02\u{d4}\u{d5}\x05\x48\x25\x02\u{d5}\
	\u{d6}\x07\x16\x02\x02\u{d6}\u{d7}\x07\x05\x02\x02\u{d7}\u{d8}\x05\x22\x12\
	\x02\u{d8}\u{d9}\x07\x06\x02\x02\u{d9}\u{da}\x05\x1a\x0e\x02\u{da}\u{107}\
	\x03\x02\x02\x02\u{db}\u{dc}\x05\x48\x25\x02\u{dc}\u{dd}\x07\x17\x02\x02\
	\u{dd}\u{de}\x05\x1a\x0e\x02\u{de}\u{df}\x07\x16\x02\x02\u{df}\u{e0}\x07\
	\x05\x02\x02\u{e0}\u{e1}\x05\x22\x12\x02\u{e1}\u{e2}\x07\x06\x02\x02\u{e2}\
	\u{107}\x03\x02\x02\x02\u{e3}\u{e4}\x05\x48\x25\x02\u{e4}\u{e5}\x07\x18\
	\x02\x02\u{e5}\u{e7}\x07\x05\x02\x02\u{e6}\u{e8}\x05\x1c\x0f\x02\u{e7}\u{e6}\
	\x03\x02\x02\x02\u{e7}\u{e8}\x03\x02\x02\x02\u{e8}\u{e9}\x03\x02\x02\x02\
	\u{e9}\u{eb}\x07\x0a\x02\x02\u{ea}\u{ec}\x05\x22\x12\x02\u{eb}\u{ea}\x03\
	\x02\x02\x02\u{eb}\u{ec}\x03\x02\x02\x02\u{ec}\u{ed}\x03\x02\x02\x02\u{ed}\
	\u{ef}\x07\x0a\x02\x02\u{ee}\u{f0}\x05\x1a\x0e\x02\u{ef}\u{ee}\x03\x02\x02\
	\x02\u{ef}\u{f0}\x03\x02\x02\x02\u{f0}\u{f1}\x03\x02\x02\x02\u{f1}\u{f2}\
	\x07\x06\x02\x02\u{f2}\u{f3}\x05\x1a\x0e\x02\u{f3}\u{107}\x03\x02\x02\x02\
	\u{f4}\u{f5}\x07\x13\x02\x02\u{f5}\u{107}\x07\x0a\x02\x02\u{f6}\u{f7}\x07\
	\x14\x02\x02\u{f7}\u{107}\x07\x0a\x02\x02\u{f8}\u{fa}\x07\x15\x02\x02\u{f9}\
	\u{fb}\x05\x22\x12\x02\u{fa}\u{f9}\x03\x02\x02\x02\u{fa}\u{fb}\x03\x02\x02\
	\x02\u{fb}\u{fc}\x03\x02\x02\x02\u{fc}\u{107}\x07\x0a\x02\x02\u{fd}\u{107}\
	\x05\x46\x24\x02\u{fe}\u{102}\x07\x07\x02\x02\u{ff}\u{101}\x05\x06\x04\x02\
	\u{100}\u{ff}\x03\x02\x02\x02\u{101}\u{104}\x03\x02\x02\x02\u{102}\u{100}\
	\x03\x02\x02\x02\u{102}\u{103}\x03\x02\x02\x02\u{103}\u{105}\x03\x02\x02\
	\x02\u{104}\u{102}\x03\x02\x02\x02\u{105}\u{107}\x07\x08\x02\x02\u{106}\
	\u{c4}\x03\x02\x02\x02\u{106}\u{c5}\x03\x02\x02\x02\u{106}\u{c8}\x03\x02\
	\x02\x02\u{106}\u{cb}\x03\x02\x02\x02\u{106}\u{d4}\x03\x02\x02\x02\u{106}\
	\u{db}\x03\x02\x02\x02\u{106}\u{e3}\x03\x02\x02\x02\u{106}\u{f4}\x03\x02\
	\x02\x02\u{106}\u{f6}\x03\x02\x02\x02\u{106}\u{f8}\x03\x02\x02\x02\u{106}\
	\u{fd}\x03\x02\x02\x02\u{106}\u{fe}\x03\x02\x02\x02\u{107}\x1b\x03\x02\x02\
	\x02\u{108}\u{10b}\x05\x0c\x07\x02\u{109}\u{10a}\x07\x19\x02\x02\u{10a}\
	\u{10c}\x05\x22\x12\x02\u{10b}\u{109}\x03\x02\x02\x02\u{10b}\u{10c}\x03\
	\x02\x02\x02\u{10c}\u{10f}\x03\x02\x02\x02\u{10d}\u{10f}\x05\x1e\x10\x02\
	\u{10e}\u{108}\x03\x02\x02\x02\u{10e}\u{10d}\x03\x02\x02\x02\u{10f}\x1d\
	\x03\x02\x02\x02\u{110}\u{111}\x07\x44\x02\x02\u{111}\u{112}\x07\x19\x02\
	\x02\u{112}\u{120}\x05\x22\x12\x02\u{113}\u{114}\x07\x44\x02\x02\u{114}\
	\u{115}\x07\x0b\x02\x02\u{115}\u{116}\x05\x22\x12\x02\u{116}\u{117}\x07\
	\x0c\x02\x02\u{117}\u{118}\x07\x19\x02\x02\u{118}\u{119}\x05\x22\x12\x02\
	\u{119}\u{120}\x03\x02\x02\x02\u{11a}\u{11b}\x07\x44\x02\x02\u{11b}\u{11c}\
	\x07\x0d\x02\x02\u{11c}\u{11d}\x07\x44\x02\x02\u{11d}\u{11e}\x07\x19\x02\
	\x02\u{11e}\u{120}\x05\x22\x12\x02\u{11f}\u{110}\x03\x02\x02\x02\u{11f}\
	\u{113}\x03\x02\x02\x02\u{11f}\u{11a}\x03\x02\x02\x02\u{120}\x1f\x03\x02\
	\x02\x02\u{121}\u{124}\x05\x0c\x07\x02\u{122}\u{123}\x07\x19\x02\x02\u{123}\
	\u{125}\x05\x22\x12\x02\u{124}\u{122}\x03\x02\x02\x02\u{124}\u{125}\x03\
	\x02\x02\x02\u{125}\u{126}\x03\x02\x02\x02\u{126}\u{127}\x07\x0a\x02\x02\
	\u{127}\x21\x03\x02\x02\x02\u{128}\u{129}\x08\x12\x01\x02\u{129}\u{140}\
	\x07\x44\x02\x02\u{12a}\u{140}\x05\x4e\x28\x02\u{12b}\u{12c}\x07\x44\x02\
	\x02\u{12c}\u{135}\x07\x05\x02\x02\u{12d}\u{132}\x05\x22\x12\x02\u{12e}\
	\u{12f}\x07\x09\x02\x02\u{12f}\u{131}\x05\x22\x12\x02\u{130}\u{12e}\x03\
	\x02\x02\x02\u{131}\u{134}\x03\x02\x02\x02\u{132}\u{130}\x03\x02\x02\x02\
	\u{132}\u{133}\x03\x02\x02\x02\u{133}\u{136}\x03\x02\x02\x02\u{134}\u{132}\
	\x03\x02\x02\x02\u{135}\u{12d}\x03\x02\x02\x02\u{135}\u{136}\x03\x02\x02\
	\x02\u{136}\u{137}\x03\x02\x02\x02\u{137}\u{140}\x07\x06\x02\x02\u{138}\
	\u{139}\x07\x05\x02\x02\u{139}\u{13a}\x05\x22\x12\x02\u{13a}\u{13b}\x07\
	\x06\x02\x02\u{13b}\u{140}\x03\x02\x02\x02\u{13c}\u{13d}\x05\x38\x1d\x02\
	\u{13d}\u{13e}\x05\x22\x12\x09\u{13e}\u{140}\x03\x02\x02\x02\u{13f}\u{128}\
	\x03\x02\x02\x02\u{13f}\u{12a}\x03\x02\x02\x02\u{13f}\u{12b}\x03\x02\x02\
	\x02\u{13f}\u{138}\x03\x02\x02\x02\u{13f}\u{13c}\x03\x02\x02\x02\u{140}\
	\u{161}\x03\x02\x02\x02\u{141}\u{142}\x0c\x08\x02\x02\u{142}\u{143}\x05\
	\x30\x19\x02\u{143}\u{144}\x05\x22\x12\x09\u{144}\u{160}\x03\x02\x02\x02\
	\u{145}\u{146}\x0c\x07\x02\x02\u{146}\u{147}\x05\x2e\x18\x02\u{147}\u{148}\
	\x05\x22\x12\x08\u{148}\u{160}\x03\x02\x02\x02\u{149}\u{14a}\x0c\x06\x02\
	\x02\u{14a}\u{14b}\x05\x36\x1c\x02\u{14b}\u{14c}\x05\x22\x12\x07\u{14c}\
	\u{160}\x03\x02\x02\x02\u{14d}\u{14e}\x0c\x05\x02\x02\u{14e}\u{14f}\x05\
	\x34\x1b\x02\u{14f}\u{150}\x05\x22\x12\x06\u{150}\u{160}\x03\x02\x02\x02\
	\u{151}\u{152}\x0c\x04\x02\x02\u{152}\u{153}\x07\x26\x02\x02\u{153}\u{160}\
	\x05\x22\x12\x05\u{154}\u{155}\x0c\x03\x02\x02\u{155}\u{156}\x07\x27\x02\
	\x02\u{156}\u{160}\x05\x22\x12\x04\u{157}\u{158}\x0c\x0b\x02\x02\u{158}\
	\u{159}\x07\x0b\x02\x02\u{159}\u{15a}\x05\x22\x12\x02\u{15a}\u{15b}\x07\
	\x0c\x02\x02\u{15b}\u{160}\x03\x02\x02\x02\u{15c}\u{15d}\x0c\x0a\x02\x02\
	\u{15d}\u{15e}\x07\x0d\x02\x02\u{15e}\u{160}\x07\x44\x02\x02\u{15f}\u{141}\
	\x03\x02\x02\x02\u{15f}\u{145}\x03\x02\x02\x02\u{15f}\u{149}\x03\x02\x02\
	\x02\u{15f}\u{14d}\x03\x02\x02\x02\u{15f}\u{151}\x03\x02\x02\x02\u{15f}\
	\u{154}\x03\x02\x02\x02\u{15f}\u{157}\x03\x02\x02\x02\u{15f}\u{15c}\x03\
	\x02\x02\x02\u{160}\u{163}\x03\x02\x02\x02\u{161}\u{15f}\x03\x02\x02\x02\
	\u{161}\u{162}\x03\x02\x02\x02\u{162}\x23\x03\x02\x02\x02\u{163}\u{161}\
	\x03\x02\x02\x02\u{164}\u{169}\x07\x42\x02\x02\u{165}\u{169}\x07\x43\x02\
	\x02\u{166}\u{169}\x07\x2a\x02\x02\u{167}\u{169}\x07\x2b\x02\x02\u{168}\
	\u{164}\x03\x02\x02\x02\u{168}\u{165}\x03\x02\x02\x02\u{168}\u{166}\x03\
	\x02\x02\x02\u{168}\u{167}\x03\x02\x02\x02\u{169}\x25\x03\x02\x02\x02\u{16a}\
	\u{16b}\x08\x14\x01\x02\u{16b}\u{180}\x07\x44\x02\x02\u{16c}\u{180}\x07\
	\x2c\x02\x02\u{16d}\u{180}\x05\x24\x13\x02\u{16e}\u{16f}\x07\x07\x02\x02\
	\u{16f}\u{170}\x05\x26\x14\x02\u{170}\u{171}\x07\x2f\x02\x02\u{171}\u{172}\
	\x07\x0b\x02\x02\u{172}\u{173}\x05\x26\x14\x02\u{173}\u{174}\x07\x0c\x02\
	\x02\u{174}\u{175}\x07\x19\x02\x02\u{175}\u{176}\x05\x26\x14\x02\u{176}\
	\u{177}\x07\x08\x02\x02\u{177}\u{180}\x03\x02\x02\x02\u{178}\u{179}\x07\
	\x05\x02\x02\u{179}\u{17a}\x05\x26\x14\x02\u{17a}\u{17b}\x07\x06\x02\x02\
	\u{17b}\u{180}\x03\x02\x02\x02\u{17c}\u{17d}\x05\x38\x1d\x02\u{17d}\u{17e}\
	\x05\x26\x14\x05\u{17e}\u{180}\x03\x02\x02\x02\u{17f}\u{16a}\x03\x02\x02\
	\x02\u{17f}\u{16c}\x03\x02\x02\x02\u{17f}\u{16d}\x03\x02\x02\x02\u{17f}\
	\u{16e}\x03\x02\x02\x02\u{17f}\u{178}\x03\x02\x02\x02\u{17f}\u{17c}\x03\
	\x02\x02\x02\u{180}\u{193}\x03\x02\x02\x02\u{181}\u{182}\x0c\x04\x02\x02\
	\u{182}\u{183}\x05\x30\x19\x02\u{183}\u{184}\x05\x26\x14\x05\u{184}\u{192}\
	\x03\x02\x02\x02\u{185}\u{186}\x0c\x03\x02\x02\u{186}\u{187}\x05\x2e\x18\
	\x02\u{187}\u{188}\x05\x26\x14\x04\u{188}\u{192}\x03\x02\x02\x02\u{189}\
	\u{18a}\x0c\x07\x02\x02\u{18a}\u{18b}\x07\x0b\x02\x02\u{18b}\u{18c}\x05\
	\x26\x14\x02\u{18c}\u{18d}\x07\x0c\x02\x02\u{18d}\u{192}\x03\x02\x02\x02\
	\u{18e}\u{18f}\x0c\x06\x02\x02\u{18f}\u{190}\x07\x0d\x02\x02\u{190}\u{192}\
	\x07\x44\x02\x02\u{191}\u{181}\x03\x02\x02\x02\u{191}\u{185}\x03\x02\x02\
	\x02\u{191}\u{189}\x03\x02\x02\x02\u{191}\u{18e}\x03\x02\x02\x02\u{192}\
	\u{195}\x03\x02\x02\x02\u{193}\u{191}\x03\x02\x02\x02\u{193}\u{194}\x03\
	\x02\x02\x02\u{194}\x27\x03\x02\x02\x02\u{195}\u{193}\x03\x02\x02\x02\u{196}\
	\u{197}\x08\x15\x01\x02\u{197}\u{19d}\x05\x26\x14\x02\u{198}\u{199}\x07\
	\x05\x02\x02\u{199}\u{19a}\x05\x28\x15\x02\u{19a}\u{19b}\x07\x06\x02\x02\
	\u{19b}\u{19d}\x03\x02\x02\x02\u{19c}\u{196}\x03\x02\x02\x02\u{19c}\u{198}\
	\x03\x02\x02\x02\u{19d}\u{1ae}\x03\x02\x02\x02\u{19e}\u{19f}\x0c\x06\x02\
	\x02\u{19f}\u{1a0}\x05\x36\x1c\x02\u{1a0}\u{1a1}\x05\x28\x15\x07\u{1a1}\
	\u{1ad}\x03\x02\x02\x02\u{1a2}\u{1a3}\x0c\x05\x02\x02\u{1a3}\u{1a4}\x05\
	\x34\x1b\x02\u{1a4}\u{1a5}\x05\x28\x15\x06\u{1a5}\u{1ad}\x03\x02\x02\x02\
	\u{1a6}\u{1a7}\x0c\x04\x02\x02\u{1a7}\u{1a8}\x07\x26\x02\x02\u{1a8}\u{1ad}\
	\x05\x28\x15\x05\u{1a9}\u{1aa}\x0c\x03\x02\x02\u{1aa}\u{1ab}\x07\x27\x02\
	\x02\u{1ab}\u{1ad}\x05\x28\x15\x04\u{1ac}\u{19e}\x03\x02\x02\x02\u{1ac}\
	\u{1a2}\x03\x02\x02\x02\u{1ac}\u{1a6}\x03\x02\x02\x02\u{1ac}\u{1a9}\x03\
	\x02\x02\x02\u{1ad}\u{1b0}\x03\x02\x02\x02\u{1ae}\u{1ac}\x03\x02\x02\x02\
	\u{1ae}\u{1af}\x03\x02\x02\x02\u{1af}\x29\x03\x02\x02\x02\u{1b0}\u{1ae}\
	\x03\x02\x02\x02\u{1b1}\u{1b2}\x08\x16\x01\x02\u{1b2}\u{1e8}\x07\x2a\x02\
	\x02\u{1b3}\u{1e8}\x07\x2b\x02\x02\u{1b4}\u{1b8}\x05\x26\x14\x02\u{1b5}\
	\u{1b6}\x05\x32\x1a\x02\u{1b6}\u{1b7}\x05\x26\x14\x02\u{1b7}\u{1b9}\x03\
	\x02\x02\x02\u{1b8}\u{1b5}\x03\x02\x02\x02\u{1b9}\u{1ba}\x03\x02\x02\x02\
	\u{1ba}\u{1b8}\x03\x02\x02\x02\u{1ba}\u{1bb}\x03\x02\x02\x02\u{1bb}\u{1e8}\
	\x03\x02\x02\x02\u{1bc}\u{1c8}\x07\x44\x02\x02\u{1bd}\u{1be}\x07\x05\x02\
	\x02\u{1be}\u{1c3}\x05\x28\x15\x02\u{1bf}\u{1c0}\x07\x09\x02\x02\u{1c0}\
	\u{1c2}\x05\x28\x15\x02\u{1c1}\u{1bf}\x03\x02\x02\x02\u{1c2}\u{1c5}\x03\
	\x02\x02\x02\u{1c3}\u{1c1}\x03\x02\x02\x02\u{1c3}\u{1c4}\x03\x02\x02\x02\
	\u{1c4}\u{1c6}\x03\x02\x02\x02\u{1c5}\u{1c3}\x03\x02\x02\x02\u{1c6}\u{1c7}\
	\x07\x06\x02\x02\u{1c7}\u{1c9}\x03\x02\x02\x02\u{1c8}\u{1bd}\x03\x02\x02\
	\x02\u{1c8}\u{1c9}\x03\x02\x02\x02\u{1c9}\u{1e8}\x03\x02\x02\x02\u{1ca}\
	\u{1cb}\x07\x05\x02\x02\u{1cb}\u{1cc}\x05\x2a\x16\x02\u{1cc}\u{1cd}\x07\
	\x06\x02\x02\u{1cd}\u{1e8}\x03\x02\x02\x02\u{1ce}\u{1cf}\x07\x24\x02\x02\
	\u{1cf}\u{1e8}\x05\x2a\x16\x06\u{1d0}\u{1d1}\x07\x40\x02\x02\u{1d1}\u{1d2}\
	\x07\x05\x02\x02\u{1d2}\u{1d3}\x07\x44\x02\x02\u{1d3}\u{1d4}\x07\x20\x02\
	\x02\u{1d4}\u{1d5}\x07\x05\x02\x02\u{1d5}\u{1d6}\x07\x42\x02\x02\u{1d6}\
	\u{1d7}\x07\x41\x02\x02\u{1d7}\u{1d8}\x05\x26\x14\x02\u{1d8}\u{1d9}\x07\
	\x06\x02\x02\u{1d9}\u{1da}\x07\x06\x02\x02\u{1da}\u{1e8}\x03\x02\x02\x02\
	\u{1db}\u{1dc}\x05\x3a\x1e\x02\u{1dc}\u{1e1}\x05\x3c\x1f\x02\u{1dd}\u{1de}\
	\x07\x09\x02\x02\u{1de}\u{1e0}\x05\x3c\x1f\x02\u{1df}\u{1dd}\x03\x02\x02\
	\x02\u{1e0}\u{1e3}\x03\x02\x02\x02\u{1e1}\u{1df}\x03\x02\x02\x02\u{1e1}\
	\u{1e2}\x03\x02\x02\x02\u{1e2}\u{1e4}\x03\x02\x02\x02\u{1e3}\u{1e1}\x03\
	\x02\x02\x02\u{1e4}\u{1e5}\x07\x0a\x02\x02\u{1e5}\u{1e6}\x05\x2a\x16\x03\
	\u{1e6}\u{1e8}\x03\x02\x02\x02\u{1e7}\u{1b1}\x03\x02\x02\x02\u{1e7}\u{1b3}\
	\x03\x02\x02\x02\u{1e7}\u{1b4}\x03\x02\x02\x02\u{1e7}\u{1bc}\x03\x02\x02\
	\x02\u{1e7}\u{1ca}\x03\x02\x02\x02\u{1e7}\u{1ce}\x03\x02\x02\x02\u{1e7}\
	\u{1d0}\x03\x02\x02\x02\u{1e7}\u{1db}\x03\x02\x02\x02\u{1e8}\u{1fa}\x03\
	\x02\x02\x02\u{1e9}\u{1ea}\x0c\x0a\x02\x02\u{1ea}\u{1eb}\x07\x26\x02\x02\
	\u{1eb}\u{1f9}\x05\x2a\x16\x0b\u{1ec}\u{1ed}\x0c\x09\x02\x02\u{1ed}\u{1ee}\
	\x07\x27\x02\x02\u{1ee}\u{1f9}\x05\x2a\x16\x0a\u{1ef}\u{1f0}\x0c\x08\x02\
	\x02\u{1f0}\u{1f1}\x07\x30\x02\x02\u{1f1}\u{1f9}\x05\x2a\x16\x09\u{1f2}\
	\u{1f3}\x0c\x07\x02\x02\u{1f3}\u{1f4}\x07\x31\x02\x02\u{1f4}\u{1f9}\x05\
	\x2a\x16\x08\u{1f5}\u{1f6}\x0c\x05\x02\x02\u{1f6}\u{1f7}\x07\x32\x02\x02\
	\u{1f7}\u{1f9}\x05\x2a\x16\x06\u{1f8}\u{1e9}\x03\x02\x02\x02\u{1f8}\u{1ec}\
	\x03\x02\x02\x02\u{1f8}\u{1ef}\x03\x02\x02\x02\u{1f8}\u{1f2}\x03\x02\x02\
	\x02\u{1f8}\u{1f5}\x03\x02\x02\x02\u{1f9}\u{1fc}\x03\x02\x02\x02\u{1fa}\
	\u{1f8}\x03\x02\x02\x02\u{1fa}\u{1fb}\x03\x02\x02\x02\u{1fb}\x2b\x03\x02\
	\x02\x02\u{1fc}\u{1fa}\x03\x02\x02\x02\u{1fd}\u{200}\x05\x30\x19\x02\u{1fe}\
	\u{200}\x05\x2e\x18\x02\u{1ff}\u{1fd}\x03\x02\x02\x02\u{1ff}\u{1fe}\x03\
	\x02\x02\x02\u{200}\x2d\x03\x02\x02\x02\u{201}\u{204}\x07\x20\x02\x02\u{202}\
	\u{204}\x07\x21\x02\x02\u{203}\u{201}\x03\x02\x02\x02\u{203}\u{202}\x03\
	\x02\x02\x02\u{204}\x2f\x03\x02\x02\x02\u{205}\u{209}\x07\x22\x02\x02\u{206}\
	\u{209}\x07\x23\x02\x02\u{207}\u{209}\x07\x25\x02\x02\u{208}\u{205}\x03\
	\x02\x02\x02\u{208}\u{206}\x03\x02\x02\x02\u{208}\u{207}\x03\x02\x02\x02\
	\u{209}\x31\x03\x02\x02\x02\u{20a}\u{20d}\x05\x36\x1c\x02\u{20b}\u{20d}\
	\x05\x34\x1b\x02\u{20c}\u{20a}\x03\x02\x02\x02\u{20c}\u{20b}\x03\x02\x02\
	\x02\u{20d}\x33\x03\x02\x02\x02\u{20e}\u{211}\x07\x1a\x02\x02\u{20f}\u{211}\
	\x07\x1b\x02\x02\u{210}\u{20e}\x03\x02\x02\x02\u{210}\u{20f}\x03\x02\x02\
	\x02\u{211}\x35\x03\x02\x02\x02\u{212}\u{217}\x07\x1d\x02\x02\u{213}\u{217}\
	\x07\x1c\x02\x02\u{214}\u{217}\x07\x1f\x02\x02\u{215}\u{217}\x07\x1e\x02\
	\x02\u{216}\u{212}\x03\x02\x02\x02\u{216}\u{213}\x03\x02\x02\x02\u{216}\
	\u{214}\x03\x02\x02\x02\u{216}\u{215}\x03\x02\x02\x02\u{217}\x37\x03\x02\
	\x02\x02\u{218}\u{21b}\x07\x21\x02\x02\u{219}\u{21b}\x07\x24\x02\x02\u{21a}\
	\u{218}\x03\x02\x02\x02\u{21a}\u{219}\x03\x02\x02\x02\u{21b}\x39\x03\x02\
	\x02\x02\u{21c}\u{21f}\x07\x33\x02\x02\u{21d}\u{21f}\x07\x34\x02\x02\u{21e}\
	\u{21c}\x03\x02\x02\x02\u{21e}\u{21d}\x03\x02\x02\x02\u{21f}\x3b\x03\x02\
	\x02\x02\u{220}\u{221}\x05\x18\x0d\x02\u{221}\u{226}\x07\x44\x02\x02\u{222}\
	\u{223}\x07\x09\x02\x02\u{223}\u{225}\x07\x44\x02\x02\u{224}\u{222}\x03\
	\x02\x02\x02\u{225}\u{228}\x03\x02\x02\x02\u{226}\u{224}\x03\x02\x02\x02\
	\u{226}\u{227}\x03\x02\x02\x02\u{227}\x3d\x03\x02\x02\x02\u{228}\u{226}\
	\x03\x02\x02\x02\u{229}\u{22d}\x07\x47\x02\x02\u{22a}\u{22c}\x05\x40\x21\
	\x02\u{22b}\u{22a}\x03\x02\x02\x02\u{22c}\u{22f}\x03\x02\x02\x02\u{22d}\
	\u{22b}\x03\x02\x02\x02\u{22d}\u{22e}\x03\x02\x02\x02\u{22e}\u{231}\x03\
	\x02\x02\x02\u{22f}\u{22d}\x03\x02\x02\x02\u{230}\u{232}\x05\x42\x22\x02\
	\u{231}\u{230}\x03\x02\x02\x02\u{231}\u{232}\x03\x02\x02\x02\u{232}\u{236}\
	\x03\x02\x02\x02\u{233}\u{235}\x05\x44\x23\x02\u{234}\u{233}\x03\x02\x02\
	\x02\u{235}\u{238}\x03\x02\x02\x02\u{236}\u{234}\x03\x02\x02\x02\u{236}\
	\u{237}\x03\x02\x02\x02\u{237}\u{239}\x03\x02\x02\x02\u{238}\u{236}\x03\
	\x02\x02\x02\u{239}\u{24c}\x07\x48\x02\x02\u{23a}\u{23e}\x07\x49\x02\x02\
	\u{23b}\u{23d}\x05\x40\x21\x02\u{23c}\u{23b}\x03\x02\x02\x02\u{23d}\u{240}\
	\x03\x02\x02\x02\u{23e}\u{23c}\x03\x02\x02\x02\u{23e}\u{23f}\x03\x02\x02\
	\x02\u{23f}\u{242}\x03\x02\x02\x02\u{240}\u{23e}\x03\x02\x02\x02\u{241}\
	\u{243}\x05\x42\x22\x02\u{242}\u{241}\x03\x02\x02\x02\u{242}\u{243}\x03\
	\x02\x02\x02\u{243}\u{247}\x03\x02\x02\x02\u{244}\u{246}\x05\x44\x23\x02\
	\u{245}\u{244}\x03\x02\x02\x02\u{246}\u{249}\x03\x02\x02\x02\u{247}\u{245}\
	\x03\x02\x02\x02\u{247}\u{248}\x03\x02\x02\x02\u{248}\u{24a}\x03\x02\x02\
	\x02\u{249}\u{247}\x03\x02\x02\x02\u{24a}\u{24c}\x07\x4b\x02\x02\u{24b}\
	\u{229}\x03\x02\x02\x02\u{24b}\u{23a}\x03\x02\x02\x02\u{24c}\x3f\x03\x02\
	\x02\x02\u{24d}\u{24e}\x07\x38\x02\x02\u{24e}\u{24f}\x05\x2a\x16\x02\u{24f}\
	\u{250}\x07\x0a\x02\x02\u{250}\x41\x03\x02\x02\x02\u{251}\u{25d}\x07\x39\
	\x02\x02\u{252}\u{25e}\x05\x26\x14\x02\u{253}\u{254}\x07\x05\x02\x02\u{254}\
	\u{257}\x05\x26\x14\x02\u{255}\u{256}\x07\x09\x02\x02\u{256}\u{258}\x05\
	\x26\x14\x02\u{257}\u{255}\x03\x02\x02\x02\u{258}\u{259}\x03\x02\x02\x02\
	\u{259}\u{257}\x03\x02\x02\x02\u{259}\u{25a}\x03\x02\x02\x02\u{25a}\u{25b}\
	\x03\x02\x02\x02\u{25b}\u{25c}\x07\x06\x02\x02\u{25c}\u{25e}\x03\x02\x02\
	\x02\u{25d}\u{252}\x03\x02\x02\x02\u{25d}\u{253}\x03\x02\x02\x02\u{25e}\
	\u{25f}\x03\x02\x02\x02\u{25f}\u{260}\x07\x0a\x02\x02\u{260}\x43\x03\x02\
	\x02\x02\u{261}\u{262}\x07\x3a\x02\x02\u{262}\u{263}\x05\x2a\x16\x02\u{263}\
	\u{264}\x07\x0a\x02\x02\u{264}\x45\x03\x02\x02\x02\u{265}\u{266}\x07\x47\
	\x02\x02\u{266}\u{267}\x07\x3b\x02\x02\u{267}\u{268}\x05\x2a\x16\x02\u{268}\
	\u{269}\x07\x0a\x02\x02\u{269}\u{26a}\x07\x48\x02\x02\u{26a}\u{272}\x03\
	\x02\x02\x02\u{26b}\u{26c}\x07\x49\x02\x02\u{26c}\u{26d}\x07\x3b\x02\x02\
	\u{26d}\u{26e}\x05\x2a\x16\x02\u{26e}\u{26f}\x07\x0a\x02\x02\u{26f}\u{270}\
	\x07\x4b\x02\x02\u{270}\u{272}\x03\x02\x02\x02\u{271}\u{265}\x03\x02\x02\
	\x02\u{271}\u{26b}\x03\x02\x02\x02\u{272}\x47\x03\x02\x02\x02\u{273}\u{27b}\
	\x07\x47\x02\x02\u{274}\u{275}\x07\x3c\x02\x02\u{275}\u{276}\x07\x3d\x02\
	\x02\u{276}\u{277}\x05\x2a\x16\x02\u{277}\u{278}\x07\x0a\x02\x02\u{278}\
	\u{27a}\x03\x02\x02\x02\u{279}\u{274}\x03\x02\x02\x02\u{27a}\u{27d}\x03\
	\x02\x02\x02\u{27b}\u{279}\x03\x02\x02\x02\u{27b}\u{27c}\x03\x02\x02\x02\
	\u{27c}\u{28f}\x03\x02\x02\x02\u{27d}\u{27b}\x03\x02\x02\x02\u{27e}\u{27f}\
	\x07\x3c\x02\x02\u{27f}\u{28b}\x07\x3e\x02\x02\u{280}\u{28c}\x05\x26\x14\
	\x02\u{281}\u{282}\x07\x05\x02\x02\u{282}\u{285}\x05\x26\x14\x02\u{283}\
	\u{284}\x07\x09\x02\x02\u{284}\u{286}\x05\x26\x14\x02\u{285}\u{283}\x03\
	\x02\x02\x02\u{286}\u{287}\x03\x02\x02\x02\u{287}\u{285}\x03\x02\x02\x02\
	\u{287}\u{288}\x03\x02\x02\x02\u{288}\u{289}\x03\x02\x02\x02\u{289}\u{28a}\
	\x07\x06\x02\x02\u{28a}\u{28c}\x03\x02\x02\x02\u{28b}\u{280}\x03\x02\x02\
	\x02\u{28b}\u{281}\x03\x02\x02\x02\u{28c}\u{28d}\x03\x02\x02\x02\u{28d}\
	\u{28e}\x07\x0a\x02\x02\u{28e}\u{290}\x03\x02\x02\x02\u{28f}\u{27e}\x03\
	\x02\x02\x02\u{28f}\u{290}\x03\x02\x02\x02\u{290}\u{291}\x03\x02\x02\x02\
	\u{291}\u{2b2}\x07\x48\x02\x02\u{292}\u{29a}\x07\x49\x02\x02\u{293}\u{294}\
	\x07\x3c\x02\x02\u{294}\u{295}\x07\x3d\x02\x02\u{295}\u{296}\x05\x2a\x16\
	\x02\u{296}\u{297}\x07\x0a\x02\x02\u{297}\u{299}\x03\x02\x02\x02\u{298}\
	\u{293}\x03\x02\x02\x02\u{299}\u{29c}\x03\x02\x02\x02\u{29a}\u{298}\x03\
	\x02\x02\x02\u{29a}\u{29b}\x03\x02\x02\x02\u{29b}\u{2ae}\x03\x02\x02\x02\
	\u{29c}\u{29a}\x03\x02\x02\x02\u{29d}\u{29e}\x07\x3c\x02\x02\u{29e}\u{2aa}\
	\x07\x3e\x02\x02\u{29f}\u{2ab}\x05\x26\x14\x02\u{2a0}\u{2a1}\x07\x05\x02\
	\x02\u{2a1}\u{2a4}\x05\x26\x14\x02\u{2a2}\u{2a3}\x07\x09\x02\x02\u{2a3}\
	\u{2a5}\x05\x26\x14\x02\u{2a4}\u{2a2}\x03\x02\x02\x02\u{2a5}\u{2a6}\x03\
	\x02\x02\x02\u{2a6}\u{2a4}\x03\x02\x02\x02\u{2a6}\u{2a7}\x03\x02\x02\x02\
	\u{2a7}\u{2a8}\x03\x02\x02\x02\u{2a8}\u{2a9}\x07\x06\x02\x02\u{2a9}\u{2ab}\
	\x03\x02\x02\x02\u{2aa}\u{29f}\x03\x02\x02\x02\u{2aa}\u{2a0}\x03\x02\x02\
	\x02\u{2ab}\u{2ac}\x03\x02\x02\x02\u{2ac}\u{2ad}\x07\x0a\x02\x02\u{2ad}\
	\u{2af}\x03\x02\x02\x02\u{2ae}\u{29d}\x03\x02\x02\x02\u{2ae}\u{2af}\x03\
	\x02\x02\x02\u{2af}\u{2b0}\x03\x02\x02\x02\u{2b0}\u{2b2}\x07\x4b\x02\x02\
	\u{2b1}\u{273}\x03\x02\x02\x02\u{2b1}\u{292}\x03\x02\x02\x02\u{2b2}\x49\
	\x03\x02\x02\x02\u{2b3}\u{2b7}\x07\x47\x02\x02\u{2b4}\u{2b6}\x05\x4c\x27\
	\x02\u{2b5}\u{2b4}\x03\x02\x02\x02\u{2b6}\u{2b9}\x03\x02\x02\x02\u{2b7}\
	\u{2b5}\x03\x02\x02\x02\u{2b7}\u{2b8}\x03\x02\x02\x02\u{2b8}\u{2ba}\x03\
	\x02\x02\x02\u{2b9}\u{2b7}\x03\x02\x02\x02\u{2ba}\u{2c4}\x07\x48\x02\x02\
	\u{2bb}\u{2bf}\x07\x49\x02\x02\u{2bc}\u{2be}\x05\x4c\x27\x02\u{2bd}\u{2bc}\
	\x03\x02\x02\x02\u{2be}\u{2c1}\x03\x02\x02\x02\u{2bf}\u{2bd}\x03\x02\x02\
	\x02\u{2bf}\u{2c0}\x03\x02\x02\x02\u{2c0}\u{2c2}\x03\x02\x02\x02\u{2c1}\
	\u{2bf}\x03\x02\x02\x02\u{2c2}\u{2c4}\x07\x4b\x02\x02\u{2c3}\u{2b3}\x03\
	\x02\x02\x02\u{2c3}\u{2bb}\x03\x02\x02\x02\u{2c4}\x4b\x03\x02\x02\x02\u{2c5}\
	\u{2c6}\x07\x3f\x02\x02\u{2c6}\u{2d2}\x07\x44\x02\x02\u{2c7}\u{2c8}\x07\
	\x05\x02\x02\u{2c8}\u{2cd}\x05\x16\x0c\x02\u{2c9}\u{2ca}\x07\x09\x02\x02\
	\u{2ca}\u{2cc}\x05\x16\x0c\x02\u{2cb}\u{2c9}\x03\x02\x02\x02\u{2cc}\u{2cf}\
	\x03\x02\x02\x02\u{2cd}\u{2cb}\x03\x02\x02\x02\u{2cd}\u{2ce}\x03\x02\x02\
	\x02\u{2ce}\u{2d0}\x03\x02\x02\x02\u{2cf}\u{2cd}\x03\x02\x02\x02\u{2d0}\
	\u{2d1}\x07\x06\x02\x02\u{2d1}\u{2d3}\x03\x02\x02\x02\u{2d2}\u{2c7}\x03\
	\x02\x02\x02\u{2d2}\u{2d3}\x03\x02\x02\x02\u{2d3}\u{2d4}\x03\x02\x02\x02\
	\u{2d4}\u{2d5}\x07\x19\x02\x02\u{2d5}\u{2d6}\x05\x2a\x16\x02\u{2d6}\u{2d7}\
	\x07\x0a\x02\x02\u{2d7}\x4d\x03\x02\x02\x02\u{2d8}\u{2dd}\x07\x42\x02\x02\
	\u{2d9}\u{2dd}\x07\x43\x02\x02\u{2da}\u{2dd}\x07\x28\x02\x02\u{2db}\u{2dd}\
	\x07\x29\x02\x02\u{2dc}\u{2d8}\x03\x02\x02\x02\u{2dc}\u{2d9}\x03\x02\x02\
	\x02\u{2dc}\u{2da}\x03\x02\x02\x02\u{2dc}\u{2db}\x03\x02\x02\x02\u{2dd}\
	\x4f\x03\x02\x02\x02\x4f\x53\x5b\x5f\x69\x6c\x73\u{80}\u{92}\u{9f}\u{ac}\
	\u{b0}\u{bd}\u{c2}\u{d2}\u{e7}\u{eb}\u{ef}\u{fa}\u{102}\u{106}\u{10b}\u{10e}\
	\u{11f}\u{124}\u{132}\u{135}\u{13f}\u{15f}\u{161}\u{168}\u{17f}\u{191}\u{193}\
	\u{19c}\u{1ac}\u{1ae}\u{1ba}\u{1c3}\u{1c8}\u{1e1}\u{1e7}\u{1f8}\u{1fa}\u{1ff}\
	\u{203}\u{208}\u{20c}\u{210}\u{216}\u{21a}\u{21e}\u{226}\u{22d}\u{231}\u{236}\
	\u{23e}\u{242}\u{247}\u{24b}\u{259}\u{25d}\u{271}\u{27b}\u{287}\u{28b}\u{28f}\
	\u{29a}\u{2a6}\u{2aa}\u{2ae}\u{2b1}\u{2b7}\u{2bf}\u{2c3}\u{2cd}\u{2d2}\u{2dc}";

