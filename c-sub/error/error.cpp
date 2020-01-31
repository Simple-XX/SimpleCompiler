
// This file is a part of MRNIU/SimpleCompiler (https://github.com/MRNIU/SimpleCompiler).
//
// error.cpp for MRNIU/SimpleCompiler.

#include "error.h"

Pos::Pos(unsigned int l, unsigned int c) : line(l), col(c) {
	return;
}

Pos::~Pos() {
	return;
}

Error::Error(const string &f) : filename(f) {
	err_no = 0;
	pos = new Pos(0, 0);
	return;
}

Error::~Error() {
	if(pos != NULL) {
		delete pos;
	}
	return;
}

void Error::set_line(unsigned int l) {
	pos->line = l;
	return;
}

void Error::set_col(unsigned int c) {
	pos->col = c;
	return;
}

void Error::set_err_no(int e) {
	err_no = e;
	return;
}

int Error::get_err_no() const {
	return err_no;
}

Pos * Error::get_pos() const {
	return pos;
}

void Error::display_err() const {
	cout	<< "File: " << filename << ", Line: " << pos->line << ", COL: " << pos->col
	        << ", Err: " << err_no << endl;
	return;
}
