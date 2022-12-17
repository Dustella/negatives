# negatives 负片

这是一个用于研究编译原理的玩具。

我们准备设计一种简易的脚本语言 Negative，并实现其词法分析器、实现基于 LL（1）文法的语法分析

Negative 将是一个简单的脚本语言，设计主要参考了 ES6 之后的 JavaScript。

## 编译和运行

请确保你有 rust 相关环境和 cargo，

你需要使用以下指令调试

```bash
cargo run -- poem.ne lex
```

必须顺序在 `--` 后传入两个参数。第一个是源程序的相对路径，第二个是工作模式。工作模式可以是`lex`或者是`parse`

使用以下指令构建二进制文件

```bash
cargo build -r
```

你可以在这里查阅更多关于 negative 语言的词法和文法规则、实现细节

## 词法规则

词法单元种类

```text
token:
    keyword // 关键字
    identifier // 标识符
    constant // 数值常量
    operator // 操作符和界符
    string-literal // 字符串字面量
```

### keyword 关键字

negative 具有以下关键字

`let`, `function`, `for`, `continue`, `else`, `if`, `return`, `elif`, `while`, `break`,

我们在分析出标识符之后，会判断标识符是否在关键字表中

### identifer 标识符

negative 具有以下标识符语法规则如下。标识符必须以字母开头，之后可以是下划线，数字或者字母

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

### Constants 数值型常量

negative 只支持 10 进制的整数和浮点数，遵循以下规则

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

分析字符串的时候，我们会考虑到转译符，并且会无视除了转译和换行之外的任何字符。

字符串总是由双引号开始，由双引号结尾。当字符串中有字符串的时候，应当使用`\"`的方法插入。例如

`"I am \"Dustella\""`是一整个合法的字符串，而对于 `"I am "Dustella""`这个文本，词法分析器会认为"`"I am"`和`""`是字符串,`Dustella`被当成了标识符。这显然不是我们希望的。

具体的语法规则如下

```text
string-literal:
    " s-char-sequenceopt "

s-char-sequenceopt:
    【除了单独的\," 或者换行\n】之外的任何字符
```

### operator 操作符和界符

negatives 支持

```text
operator: one of
 [ ] ( ) { } .
 * + - ~ !
 / < > <= >= ==
 : ;
  += -=
```
