
# This file is a part of MRNIU/SimpleKernel (https://github.com/MRNIU/SimpleCompiler).
#
# Makefile for MRNIU/SimpleCompiler.

# 设置 make 环境
CURR_DIR := $(shell pwd)
ROOT_DIR := $(CURR_DIR)
include $(ROOT_DIR)/Makefile.env

.PHONY: default
default:
	@$(MAKE) all

.PHONY: all
all: $(SUB_DIR)
	@echo Entry $(CURR_DIR)
	@for subdir in $(SUB_DIR); \
    	do $(MAKE) -C $$subdir all || exit 1; \
    done
	$(MAKE) c-sub
	@echo Leave $(CURR_DIR)

.PHONY: remake
remake:
	$(MAKE) clean
	@$(MAKE) all

# 删除源码外的所有文件
.PHONY: clean
clean:
	@echo Entry $(CURR_DIR)
	@echo 正在删除... $<
	@find . -name "*.o"  | xargs rm -f
	@find . -name "*.gch"  | xargs rm -f
	@rm -f *.nm
	@rm -f c-sub
	@for subdir in $(SUB_DIR); \
		do $(MAKE) -C $$subdir clean || exit 1; \
	done
	@echo 删除完毕
	@echo Leave $(CURR_DIR)

CXX_SOURCES := $(shell find . -name "*.cpp")
S_SOURCES := $(shell find . -name "*.s")
OBJ := $(patsubst %.cpp, %.o, $(CXX_SOURCES)) \
	   $(patsubst %.s, %.o, $(S_SOURCES))

.PHONY: c-sub
c-sub: $(OBJ)
	@echo 正在生成编译器... $<
	@$(LD) $(LDFLAGS) $^ -o $@

.PHONY: code_line_count
code_line_count:
	cloc ./

.PHONY: info
info:
	@echo Current dir: $(CURR_DIR)
	@echo CXX: $(CXX)
	@echo CXXFLAGS: $(CXXFLAGS)
	@echo LD: $(LD)
	@echo LDFLAGS: $(LDFLAGS)
	@echo AS: $(AS)
	@echo ASFLAGS: $(ASFLAGS)
	@echo exclude_dirs: $(exclude_dirs)
	@echo SUBDIRS: $(SUB_DIR)
	@echo Include dir: $(INCLUDE_DIR)
	@echo OBJ: $(OBJ)
