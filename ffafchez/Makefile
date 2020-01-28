
# This file is a part of MRNIU/SimpleCompiler (https://github.com/MRNIU/SimpleCompiler).
#
# Makefile for MRNIU/SimpleCompiler.

include ./makeenv

.PHONY: all
all:
	$(MAKE) $(patsubst %.c, %.o $(wildcard *.c))

.PHONY: clean
clean:
	rm *.o

%.o: %.c:
	$(CC) $(CFLAGS) $< -o $@

%.o: %.s:
	$(AS) $(ASFLAGS) $< -o $@
