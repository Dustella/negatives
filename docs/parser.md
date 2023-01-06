# 语法

```
Prog ::=
    Stmt
    FuncDeclare
    eps
```

```
Stmt ::=
    ForStmt
    IfStmt
    WhileStmt
    AssignStmt
    eps
```

```
ForStmt ::=
    for ( AssignStmt Expr ; NoSemiAssignStmt ) { Stmt } Stmt
```

```
IfStmt ::=
    if ( Expr ) { Stmt } IfExt

IfExt ::=
    else if ( Expr ){ Stmt } IfExt
    else { Stmt }
    Stmt
```

```
WhileStmt ::=
    while ( Expr ) { Stmt }
```

```
FuncDecl ::=
    function Id ( ArgList ) { Stmt }
```

```
Expr ::=
    Expr + Term
    Expr - Term
    Term

Term ::=
    Term * Factor
    Term / Factor
    Factor

Factor ::=
    ( Expr )
    id
```

```
Expr ::=
    Expr + Term
    Expr - Term
    Term

Term ::=
    Term * CntPt
    Term / CntPt
    CntPt

CntPt ::=
    CntPt == Factor
    CntPt >= Factor
    CntPt <= Factor
    CntPt > Factor
    CntPt < Factor
    Factor

Factor ::=
    ( Expr )
    id
```

```
Expr ::=
    Term Expre

Expre ::=
    + Term Expre
    - Term Expre
    eps

Term ::=
    CntPt Termt

Termt ::=
    * CntPt Termt
    / CntPt Termt
    eps

CntPt ::=
    Factor CntPtc

CntPtc ::=
    == Factor CntPtc
    >= Factor CntPtc
    <= Factor CntPtc
    > Factor CntPtc
    < Factor CntPtc
    eps

Factor ::=
    ( Expr )
    id


```

```
IfExt ::=
    else ElseExt
    Stmt


ElseExt ::=
    if ( Expr ){ Stmt } IfExt
    { Stmt }
```

```

```
