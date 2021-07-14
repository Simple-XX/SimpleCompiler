
// This file is a part of Simple-XX/SimpleCompiler
// (https://github.com/Simple-XX/SimpleCompiler).
//
// test_lexical.c for Simple-XX/SimpleCompiler.

// keywords
int
char
void
if
else
while
for
break
continue
return

// identifier
adffadf fdafa232
fa_efa _fwf2_

// operator
+
-
*
/
=
==
&
&&
|
||
^
!
!=
>
<
>=
<=
&& | || ^ ! != >

// separator
(
)
{
}
[
]
,
:
;

// combination
int  a  = 124;
char bb = 's';
int  c  = 2;
int  d  = a * b;

// real program
int main()
{
	int num1 = 12;
    int arr [114514];
    int *pointer;
	scanf("%d", &num1);
    for (int i = 0; i < 10; i++) {
	    printf("num1 = %d\n", num1);
    }
	return 0;
}