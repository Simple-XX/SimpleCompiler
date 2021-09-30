
// This file is a part of Simple-XX/SimpleCompiler
// (https://github.com/Simple-XX/SimpleCompiler).
//
// ir.h for Simple-XX/SimpleCompiler.

#ifndef _IR_H_
#define _IR_H_

enum IRToken {
        VARDECL_IR,
        NUMBER_IR,
        SYMBOL_IR,
        LSB_IR,
        RSB_IR,
        ASSIGN_IR,
        FUNCEND_IR,
        END_IR,
        GOTO_IR,
        IF_IR,
        PARAM_IR,
        CALL_IR,
        RETURN_IR,
        OP_IR,
        COLON_IR,
        LOGICOP_IR,
        COMMENT_IR
    };

#endif /* _IR_H_ */