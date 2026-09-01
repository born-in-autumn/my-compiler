# my-compiler
Learning build a compiler with Rust Language.

## Compiler Pipeline

Source Code
    ↓
Lexer
    ↓
Token Stream
    ↓
Parser
    ↓
AST
    ↓
Semantic Analysis
    ↓
IR
    ↓
Code Generation

## TODO

### Lexer
- [x] Basic tokenization

### Parser
- [ ] VariableDeclaration
- [ ] Literal
- [ ] Identifier
- [ ] BinaryExpression
- [ ] ...

### AST
- [ ] Program
- [ ] VariableDeclaration
- [ ] Expression
- [ ] ...

### Semantic Analysis
...

### IR
...

### Code Generation
...

#### Lexer specification
| Token      | Start  | Continue  | End         |
| ---------- | ------ | --------- | ----------- |
| Identifier | 字母 `_` | 字母/数字 `_` | 不是 Continue  |
| Integer    | 数字     | 数字        | 非数字         |
| String     | `"`    | 不是 `"`     | `"`         |
| `+`        | `+`    | 无        | 立刻结束        |
| `=`        | `=`    | `=` （可选）    | 往后看一个 |
| `==`       | `=`    | 第二个 `=`   | 立刻结束        |

<!--

    let input = "let a = 1 + 3 + 4 = 8;";
    输入：
    // expect: [ Keyword(Let), Space, Identifier("a"), Space, Assign, Space, Identifier("1"), Space, Plus, Space, Identifier("3"), Space, Plus, Space, Identifier("4"), Space, Assign, Space, Identifier("8"), Semicolon]

-->

#### Parser Grammar
<!--  
我们要创建的是一个类似Typescript的Compiler
在这些语言里，Program里应该是一个声明列表（Declaration），声明有很多：变量声明、函数声明等等
然后在声明里，会有expression和statement，也就是表达式和语句
比如说一个main.ts文件可能长这样：
let global = 10;        // 这是声明（Declaration）
function main() {       // 这是声明（函数声明）
    let a = 5;          // 这是声明（局部变量声明）
    a = a + 1;          // 这是语句（赋值语句）
    return a;           // 这是语句（返回语句）
}
// 对于一个变量声明来说，例如let a = 1 + 2;
首先由声明关键字、变量名、初始化符号，表达式（1+2）或者字面量（‘apple’，‘123’），这里我们暂时不考虑未赋值的场景，比如let a;
而一个表达式可以由：字面量、操作符号、另一个表达式组成
关键问题在于，知道一个表达式的所有组成，是不是还要枚举这些组成的顺序和结构才能完成文档，如果是的话，东西会非常多，而且不知道会不会遗漏，不过鉴于这个是设计文档，遗漏了后续也可以补
1+2
a++
a+=1
a==1;
这些应该都是表达式
-->

结构如下所示(注意：我们文档里可以先这么写，但是先做最简单的——VariableDeclaration）：

Parser Grammar

Program
└── Declaration*

Declaration
├── VariableDeclaration
├── FunctionDeclaration
└── ...

Statement
├── ExpressionStatement
├── ReturnStatement
├── Block
├── IfStatement
├── WhileStatement
└── ...

Expression
├── Literal
├── Identifier
├── BinaryExpression
├── UnaryExpression
├── AssignmentExpression
├── CallExpression
├── ...