An interpreter and a compiler written in Rust by following the books from
[Thorsten Ball](https://www.amazon.com/Thorsten-Ball/e/B06XCKTCRW/ref=dp_byline_cont_pop_book_1).

The interpreter is built from scratch which includes a lexer (tokenizer), a parser and a tree walking interpreter.

---

## Features

- The language supports int, string and boolean data types.
- Supports common operators like +, -, ==, !=, <, > etc.
- Supports let assignments and return statements.
- If/else expressions and functional expressions are also supported.

## Methodology

- The lexer initially does the tokenization of the code input.
- The tokens will be fed in to the parser, which creates the relevant AST (Abstract syntax tree) nodes.
