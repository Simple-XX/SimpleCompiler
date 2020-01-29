
// This file is a part of MRNIU/SimpleCompiler (https://github.com/MRNIU/SimpleCompiler).
//
// lexical.cpp for MRNIU/SimpleCompiler.

#include "lexical.h"

// 初始化
Keywords::Keywords() {
	keywords["int"] = KW_INT;
	keywords["char"] = KW_CHAR;
	keywords["void"] = KW_VOID;
	keywords["if"] = KW_IF;
	keywords["else"] = KW_ELSE;
	keywords["while"] = KW_WHILE;
	keywords["for"] = KW_FOR;
	keywords["break"] = KW_BREAK;
	keywords["continue"] = KW_CONTINUE;
	keywords["return"] = KW_RETURN;
}

Tag Keywords::getTag(string name) {
	return keywords.find(name) != keywords.end() ? keywords[name] : ID;
}
