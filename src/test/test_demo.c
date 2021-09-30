// This file is a part of Simple-XX/SimpleCompiler
// (https://github.com/Simple-XX/SimpleCompiler).
//
// test_demo.c for Simple-XX/SimpleCompiler.

int foo(int a) {
    return foo(a - 1);
}

int main() {
    int a = 10;
    a = a * foo(a);
    return 0;
}