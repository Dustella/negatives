# 词法分析

## 词法分析做的工作

我们需要完成的任务是：输入一个字符序列，输出一个词法单元序列。

我们知道，在编译器中，后续的语法分析中，语法分析会不停读入下一个词法单元、读入下一个词法单元，也就是驱动着词法分析器读入剩余的字符，尝试找到一个合法的词法单元。

那么我设计了一个类：`Tokenizer`，因为我们需要有状态地记录当前字符串位置、行数、行内位置来进行友好的报错。这个类暴露一个方法`get_next_token`，这个方法会尝试读入剩下的字符、尝试给出一个合法的词法单元。如果已经已经读完了文件，方法会返回`Token::Operator('$')`，（因为我后续使用的语法分析是预测分析法，最后一个词法单元需要补一个`$`作为文件结尾）

我们使用字符序列可以实例化一个对象，把这个对象交给语法分析的部分。

这样，我们避免了额外的空间复杂度：我们没有也不必要在语法分析之前完整地在内存中存下所有词法单元，我们应该让语法分析驱动词法分析。

同时，这个良好的设计使得最后我们只读了一趟源程序，所有分析可以完全在一趟之内完成。

## 词法单元种类

```text
token:
    keyword
    identifier
    constant
    operator
    string-literal
```

### keyword

negative具有以下关键字

`let`, `function`, `for`, `continue`, `else`, `if`, `return`, `elif`, `while`, `break`,

### identifer

negative 具有以下标识符

```text
id -> 
 nondigit 
 id nondigit 
 id digit

no-digit ->
 _ a b c d e f g h i j k l m
 n o p q r s t u v w x y z
 A B C D E F G H I J K L M
 N O P Q R S T U V W X Y Z

digit -> 
 0 1 2 3 4 5 6 7 8 9
```

### Constants

```text
constant:
 integer-constant
 floating-constant

integer-constant:
 nonzero-digit
 integer-constant digit

nonzero-digit: one of
 1 2 3 4 5 6 7 8 9

decimal-floating-constant:
 nonzero-digit . nonzero-digit

```

### string-literal

```text
string-literal:
    " s-char-sequenceopt "

s-char-sequenceopt:
    【除了单独的\," 或者换行\n】之外的任何字符
```

### operator

```text
operator: one of
 [ ] ( ) { } .
 * + - ~ !
 / < > <= >= ==
 : ; 
  += -= 
```
