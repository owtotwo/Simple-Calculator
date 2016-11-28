# 用Rust实现简单的命令行计算器 #

## 目的 ##
* 练习使用新语言。
* 熟练新语言的语法特性以及异常处理等常见的编程模式，并且
对简单的数组处理和字符处理以及模块管理等部分进行练手。

## 要求 ##
* 基于命令行实现一个能对32位有符号整数(-2147483648 ~ 
2147483647)进行四则运算('+', '-', '*', '/')的计算器。
* 能使用括号进行表达式嵌套('(', ')')，不同词法单元间（
数字及运算符）可以有任意空格（包括制表符及换行符）。
* 表达式输入的行中应当有对应的提示符(prompt)，如">>"。
* 当表达式输入正确后键入回车，程序就进行相应的运算并在下
一行显示结果。
* 当表达式输入错误时则显示错误信息。
* 当结果溢出时则显示溢出错误。
* 当出现除零时则显示除零错误。
* 无论输入错误与否，计算成功与否，皆在输出相应信息后另起
一行继续下一次的输入。
* 输入Ctrl+C则立即结束当前一次输入，并另起一行进行输入。
* 输入EOF则结束程序。

## Backus–Naur Form ##
``` BNF
<expr>    ::= <term> "+" <expr>
            | <term> "-" <expr>
            | <term>

<term>    ::= <factor> "*" <term>
            | <factor> "/" <term>
            | <factor>

<factor>  ::= "(" <expr> ")"
            | <integer>

<integer> ::= "-" <number>
            | "+" <number>
            | <number>

<number>  ::= <digit> <number>
            | <digit>

<digit>   ::= "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9"
```

## Project Structure ##
``` directory tree
src
├── calculator
│   ├── mod.rs
│   ├── expr
│   │   ├── mod.rs
│   │   ├── integer.rs
│   │   ├── factor.rs
│   │   ├── operator.rs
│   │   └── term.rs
│   └── utils.rs(abandoned)
└── main.rs
```

## References ##
[Elementary arithmetic](https://en.wikipedia.org/wiki/Elementary_arithmetic)
