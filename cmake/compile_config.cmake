
# This file is a part of Simple-XX/SimpleCompiler
# (https://github.com/Simple-XX/SimpleCompiler).
#
# compile_config.cmake for Simple-XX/SimpleCompiler.
# 配置信息

# 编译选项
list(APPEND DEFAULT_COMPILE_OPTIONS
        -Wall
        -Wextra
        $<$<CONFIG:Release>:-O3;-Werror>
        $<$<CONFIG:Debug>:-O0;-g;-ggdb>
)

list(APPEND DEFAULT_LINK_LIB
        spdlog::spdlog
)
