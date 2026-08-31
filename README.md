# my-compiler
Learning build a compiler with Rust Language.


TODO LIST：
### Lexer：


#### Lexer specification
| Token      | Start  | Continue  | End         |
| ---------- | ------ | --------- | ----------- |
| Identifier | 字母 `_` | 字母/数字 `_` | 非 continue  |
| Integer    | 数字     | 数字        | 非数字         |
| String     | `"`    | 非 `"`     | `"`         |
| `+`        | `+`    | 无         | 立即结束        |
| `=`        | `=`    | `=` 可选    | 看 lookahead |
| `==`       | `=`    | 第二个 `=`   | 立即结束        |
