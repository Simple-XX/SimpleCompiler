
# This file is a part of Simple-XX/SimpleCompiler
# (https://github.com/Simple-XX/SimpleCompiler).
#
# CMakeLists.txt for Simple-XX/SimpleCompiler.

# 设置项目名与版本
project(
        system-test
        VERSION 0.0.1
)

enable_language(CXX)

add_executable(system_test
)

target_compile_options(system_test PRIVATE
        ${DEFAULT_TEST_COMPILE_OPTIONS}
)

target_link_options(system_test PRIVATE
        ${DEFAULT_TEST_LINK_OPTIONS}
)
