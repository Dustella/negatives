# negative 编译器

本negative编译器实现了简答的词法分析和简答的对于LL（1）文法的语法分析，最后可以调试词法单元流、或者显示AST

## 编译和运行

请确保你有 rust 相关环境和 cargo，

你需要使用以下指令调试

```bash
cargo run -- poem.txt lex
```

必须顺序在 `--` 后传入两个参数。第一个是源程序的相对路径，第二个是工作模式。工作模式可以是`lex`或者是`parse`

使用以下指令构建二进制文件

```bash
cargo build -r
```

你可以在这里查阅更多关于negative语言的词法和文法规则、实现细节
