// This file is a part of Simple-XX/SimpleCompiler
// (https://github.com/Simple-XX/SimpleCompiler).
//
// test_tyck.c for Simple-XX/SimpleCompiler.

int global = 1;

int foo(int a) {
    return a + 1;
}

int main() {
    int a = global + 1;
    a = a + foo(a);
    return 0;
}