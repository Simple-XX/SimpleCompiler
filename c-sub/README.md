# c-sub
c-sub
简单的 C 语言子集编译器

# 词法记号

## 类型系统
```C
int
char
void
*  // 一维指针
&  // 取址
[]  // 一维数组
```

## 常量

字符，字符串，2 8 10 16 进制整数

## 表达式
```C
+  // 加法
++  // 自增
+=
-  // 减法
--  // 自减
-=
*  // 乘法
*=
/  // 除法
/=
=  // 赋值
!=  // 不等
==  // 相等
%  // 取模
>  // 大于
>=  // 大于等于
<  // 小于
<=  // 小于等于
&  // 按位与
|  // 按位或
^  // 按位异或
~  // 按位取反
!  // 逻辑取反
&&  // 逻辑与
||  // 逻辑或
```

## 语句
```C
for
while
do
if
else
switch
case
default
return
continue
break
{}
;
:
```

## 声明与定义
```C
extern
()
,
```

## 其它

包括注释，类型转换等

# 参考资料

- 自己动手写编译器、连接器，王博俊 张宇 清华大学出版社

- [怎样写一个解释器](http://www.yinwang.org/blog-cn/2012/08/01/interpreter)

- 自己动手构造编译系统 编译、汇编与链接，范志东 张琼声 机械工业出版社
