
// This file is a part of Simple-XX/SimpleCompiler
// (https://github.com/Simple-XX/SimpleCompiler).
//
// parser.cpp for Simple-XX/SimpleCompiler.

#include "parser.h"

Parser::Parser(Lexer &lex) : lexer(lex) {
    return;
}

Parser::~Parser() {
    return;
}

// 获取下一个 token
void Parser::next(void) {
    token = lexer.lexing();
    return;
}

// 匹配指定 Token
bool Parser::match_token(Tag tag) {
    if (token->tag == tag) {
        return true;
    }
    else {
        return false;
    }
}

// 进行解析，返回解析结果(AST)
ASTPtr Parser::parsing(void) {
    this->next(); // 读入第一个token

    ASTPtr prog = program();
    return prog;
}

// 程序由代码片段与程序组成
ASTPtr Parser::program(void) {
    ASTPtrList nodes;
    while (is_done() == false) {
        ASTPtr node = statement();
        nodes.push_back(move(node));
    }
    return make_unique<CompUnitAST>(move(nodes));
}

// 代码片段由声明与定义组成
ASTPtr Parser::statement(void) {
    ASTPtr stmt = binary_add();
    return make_unique<StmtAST>(move(stmt));
}

// 二元表达式
ASTPtr Parser::binary(const function<ASTPtr()> &parser, initializer_list<Operator> ops) {
    auto lhs = parser();
    if (!lhs) {
        cout << "error 100" << endl;
        exit(100);
    }
    while (find(ops.begin(), ops.end(), tag_to_op(token->tag)) != ops.end()) {
        Operator op = tag_to_op(token->tag);
        next();
        auto rhs = parser();
        if (!rhs) {
            cout << "error 101" << endl;
            exit(101);
        }
        lhs = make_unique<BinaryAST>(op, move(lhs), move(rhs));
    }
    return lhs;
}

ASTPtr Parser::binary_add(void){
    return binary([this]
                  { return binary_mul(); }, {Operator::add_op, Operator::sub_op});
}

ASTPtr Parser::binary_mul(void){
    return binary([this]
                  { return unary(); }, {Operator::mul_op, Operator::div_op, Operator::mod_op});
}

// 一元表达式
ASTPtr Parser::unary(void)
{
    if (match_token(Tag::LPAREN)) {
        // (  EXP  )
        next(); // 消耗左括号
        ASTPtr exp = binary_add();
        if (token->tag != Tag::RPAREN){
            cout << "error 102" << endl;
            exit(102);
        }
        next(); // 消耗右括号
        return exp;
    } else if (match_token(Tag::NUM)){
        // NUM
        Num* token_casted = (Num*)token;
        ASTPtr num = make_unique<NumAST>(token_casted->val);
        next();
        return num;
    } else if (match_token(Tag::ADD)){
        // + EXP
        next();
        ASTPtr exp = unary();
        if (!exp) {
            exit(103);
        }
        return make_unique<UnaryAST>(Operator::add_op, move(exp));
    } else if (match_token(Tag::SUB)){
        // - EXP
        next();
        ASTPtr exp = unary();
        if (!exp) {
            exit(104);
        }
        return make_unique<UnaryAST>(Operator::sub_op, move(exp));
    } else if (match_token(Tag::NOT)){
        // ! EXP
        next();
        ASTPtr exp = unary();
        if (!exp) {
            exit(105);
        }
        return make_unique<UnaryAST>(Operator::not_op, move(exp));
    } else if (match_token(Tag::ID)){
        Id* token_casted = (Id*)token;
        string id_name = token_casted->name;
        next();
        // Function call: Id (params)
        if (match_token(Tag::LPAREN)) {
            next();
            // id(): no params
            if (match_token(Tag::RPAREN)) {
                ASTPtr function_call = make_unique<FuncCallAST>(id_name);
                next(); // 消耗右括号
                return function_call;
            } else {
                ASTPtrList params;
                while (true) {
                    ASTPtr param = binary_add();
                    if (!param) {
                        exit(106);
                    }
                    params.push_back(move(param));
                    // id(a,b,c)
                    if (match_token(Tag::COMMA) == false) break;
                    next(); // ,
                }
                if (match_token(Tag::RPAREN) == false) {
                    exit(107);
                }
                next(); // )
                return make_unique<FuncCallAST>(id_name, move(params));
            }
        } else if (match_token(Tag::LBRACKET)) { // LVal: array (id[exp])
            ASTPtrList position;
            while (match_token(Tag::LBRACKET)) {
                next(); // [
                ASTPtr sub_position = binary_add();
                position.push_back(move(sub_position));
                if (match_token(Tag::RBRACKET) == false) {
                    exit(108);
                }
                next();
            }
            return make_unique<LValAST>(id_name, array_t, move(position));
        } else { // LVal: var (id)
            return make_unique<LValAST>(id_name, var_t);
        }
    }
    cout << "error 55" << endl;
    exit(55);
}

bool Parser::is_done(void) const {
    return lexer.is_done();
}
