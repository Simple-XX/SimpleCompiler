
# This file is a part of Simple-XX/SimpleCompiler
# (https://github.com/Simple-XX/SimpleCompiler).
#
# CMakeLists.txt for Simple-XX/SimpleCompiler.

aux_source_directory(./ main_src)
aux_source_directory(error error_src)
aux_source_directory(lexical lexical_src)
aux_source_directory(parser parser_src)
aux_source_directory(scanner scanner_src)
aux_source_directory(typechecker typechecker_src)
aux_source_directory(ir ir_src)
aux_source_directory(codegen codegen_src)

include_directories(include)

add_executable(SimpleCompiler
    ${main_src}
    ${error_src}
    ${lexical_src}
    ${parser_src}
    ${scanner_src}
    ${ir_src}
    ${typechecker_src}
    ${codegen_src}
    )
