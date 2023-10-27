
# This file is a part of Simple-XX/SimpleCompiler
# (https://github.com/Simple-XX/SimpleCompiler).
#
# CMakeLists.txt for Simple-XX/SimpleCompiler.

add_executable(SimpleCompiler
        codegen/codegen.cpp
        error/error.cpp
        ir/irgen.cpp
        ir/irlexer.cpp
        ir/irparser.cpp
        ir/lowirgen.cpp
        lexical/lexical.cpp
        lexical/token.cpp
        parser/parser.cpp
        parser/type.cpp
        scanner/scanner.cpp
        typechecker/eval.cpp
        typechecker/typechecker.cpp
        init.cpp
        main.cpp
)

target_include_directories(SimpleCompiler PRIVATE
        include
)
