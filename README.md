An interpreter written in Rust by following the books from
[Thorsten Ball](https://www.amazon.com/Thorsten-Ball/e/B06XCKTCRW/ref=dp_byline_cont_pop_book_1).

The interpreter is built from scratch which includes a lexer (tokenizer), a parser and a tree walking interpreter.

---

## Features

- The language supports int, string and boolean data types.
- Supports common operators like +, -, ==, !=, <, > etc.
- Supports let and return statements.
- Supports If/else expressions and function expressions.
- Supports higher order functions and closures.
- Have a range of built-in functions such as len, print, push, sleep etc.
- Supports composite data types: Arrays and HashMaps.

## Methodology

- The lexer does the tokenization of the code input.
- The tokens will be fed in to the parser, which forwards the tokens as it parses the program statements one by one.
- To parse expressions, pratt parsing is used (recursive decent parsing).
- For each statement/expression parsed, the parser will create a corresponding AST node to be later evaluated.
- Once the parsing is finished, a tree walking evaluator will evaluate the program from beginning to the end and will return the final result
