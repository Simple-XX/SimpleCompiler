
# This file is a part of Simple-XX/SimpleCompiler (https://github.com/Simple-XX/SimpleCompiler).
#
# CMakeLists.txt for Simple-XX/SimpleCompiler.

cmake_minimum_required(VERSION 3.10)

project(SimpleCompiler LANGUAGES CXX)

if(${SimpleCompiler_SOURCE_DIR} STREQUAL ${SimpleCompiler_BINARY_DIR})
    message(FATAL_ERROR "In-source builds not allowed. Please make a new directory (called a build directory) and run CMake from there.")
endif()

set(CMAKE_CXX_STANDARD 17)

set(CMAKE_BUILD_TYPE DEBUG)

set(CMAKE_RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin)

set(CompilerName SimpleCompiler)

set(SimpleCompiler_SOURCE_CODE_DIR ${SimpleCompiler_SOURCE_DIR}/src)
add_subdirectory(${SimpleCompiler_SOURCE_CODE_DIR})

