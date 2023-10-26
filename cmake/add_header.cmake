
# This file is a part of Simple-XX/SimpleCompiler
# (https://github.com/Simple-XX/SimpleCompiler).
#
# add_header.cmake for Simple-XX/SimpleCompiler.
# 将头文件路径添加到 _target 的搜索路径中

function(add_header_3rd _target)
    target_include_directories(${_target} PRIVATE
            ${tinyobjloader_SOURCE_DIR})
endfunction()
