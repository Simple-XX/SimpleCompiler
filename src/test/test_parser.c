
// This file is a part of Simple-XX/SimpleCompiler
// (https://github.com/Simple-XX/SimpleCompiler).
//
// test_parser.c for Simple-XX/SimpleCompiler.

// statement
int global = 1;

int main() {
  !(11 + 45) - 14 * 19 / 19 % 810 + simple(1 + 1, 2) + xx(id(), 2) +
      comp[1][id(2)] + iler;
  simple = 1;
  simple[xx][compiler] = simple + 1;
  ;
  break;
  continue;
  return;
  return simple + xx;

  // if
  if (a && b == true)
    simple = xx;
  else
    simple = compiler;

  if (a && b == true)
    simple = xx;

  // while
  while (a && b == true)
    simple = xx;

  // block
  {}

  {
    int a[10][10], b;
    int a[10][10] = {1};
    simple = xx;
    return simple + xx;
  }

  {
    int a = 1;
    const int a1 = 1;
  }

  // All test

  {
    const int a[3] = {1, 2, 3};
    while (a != 0) {
      if (a == 1) {
        a = 2;
      } else {
        a = 3;
      }
      a = 1;
    }
  }
}