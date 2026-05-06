
grammar Magi;

main : com EOF ;
                                                            /* Labels */
com:  VAR ASSIGN exp SEMICOLON                              # var
    | com com                                               # seq
    | IF LPAR exp RPAR LCUR com RCUR                        # if
    | IF LPAR exp RPAR LCUR com RCUR ELSE LCUR com RCUR     # ifElse
    | WHILE LPAR exp RPAR LCUR com RCUR                     # while
    | PRINT LPAR exp RPAR SEMICOLON                         # print
    ;

exp : FLOAT                                                 # float
    | INT                                                   # int
    | BOOL                                                  # bool
    | VAR                                                   # access
    | <assoc=right> exp POW exp                             # pow
    | exp op=(MUL | DIV | MOD) exp                          # arith2
    | exp op=(ADD | SUB) exp                                # arith1
    | exp op=(EQ | NEQ) exp                                 # eqExp
    | exp op=(AND | OR) exp                                 # andOr
    | exp op=(LT | LTE | GT | GTE) exp                      # cmpExp
    | NOT exp                                               # not
    | LPAR exp RPAR                                         # paren
    ;

LPAR : '(' ;
RPAR : ')' ;
LCUR : '{' ;
RCUR : '}' ;

// COMMANDS
IF: 'if' ;
ELSE: 'else' ;
WHILE: 'while' ;
PRINT: 'print' ;

// ARITHMETIC
FLOAT : (INT | '-' '0') '.' [0-9]+;
INT : '0' | [-]?[1-9][0-9]* ;
ADD  : '+' ;
MUL  : '*' ;
SUB : '-' ;
DIV : '/' ;
MOD  : 'mod' ;
POW : '^';

// LOGIC
BOOL : 'true' | 'false' ;
NOT: '!' ;
EQ: '==' ;
NEQ: '!=' ;
LT: '<' ;
GT: '>' ;
LTE: '<=' ;
GTE: '>=' ;
AND: '&&' ;
OR: '||' ;

// ASSIGNMENT
ASSIGN : ':=' ;
SEMICOLON : ';' ;

// STRING LITERALS
STRING : '"' STRCHAR* '"' ;
fragment STRCHAR : ~["\\] | ESC ;
fragment ESC : '\\' [btnfr"'\\] ;

VAR : [A-Za-z]+ ;

// WHAT TO SKIP: Comments, whitespaces, tabulations, etc.
WS : [ \t\n\r]+              -> skip ;
COMMENT : '/*' .*? '*/'      -> skip ;
LINE_COMMENT : '//' ~[\r\n]* -> skip ;
