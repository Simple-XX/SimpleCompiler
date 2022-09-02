grammar Expr;
prog:	(expr NEWLINE)* ;
expr:	e1=expr ('*'|'/') e2=expr # MulDiv
    |	e1=expr ('+'|'-') e2=expr # AddSub
    |	i=INT # Int
    |	'(' e=expr ')'    # Parens
    ;
NEWLINE : [\r\n]+ ;
INT     : [0-9]+ ;