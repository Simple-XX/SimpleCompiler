
# This file is a part of Simple-XX/SimpleCompiler (https://github.com/Simple-XX/SimpleCompiler).
# 
# CMakeLists.txt for Simple-XX/SimpleCompiler.

# Set CXX flags for debug
if (CMAKE_BUILD_TYPE STREQUAL DEBUG)
    set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -g -ggdb -Wall -Wextra")
endif ()

if (CMAKE_BUILD_TYPE STREQUAL RELEASE)
    set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -Wall -Wextra -Werror")
endif ()

# Set common flags
set(CMAKE_CXX_FLAGS "${CMAKE_CXX_FLAGS} -O0")

aux_source_directory(${SimpleCompiler_SOURCE_CODE_DIR} main_src)
aux_source_directory(${SimpleCompiler_SOURCE_CODE_DIR}/error error_src)
aux_source_directory(${SimpleCompiler_SOURCE_CODE_DIR}/lexical lexical_src)
aux_source_directory(${SimpleCompiler_SOURCE_CODE_DIR}/parser parser_src)
aux_source_directory(${SimpleCompiler_SOURCE_CODE_DIR}/scanner scanner_src)
aux_source_directory(${SimpleCompiler_SOURCE_CODE_DIR}/sym sym_src)

include_directories(${SimpleCompiler_SOURCE_CODE_DIR}/include)

add_executable(${CompilerName}
    ${main_src}
    ${error_src}
    ${lexical_src}
    ${parser_src}
    ${scanner_src}
    ${sym_src})
