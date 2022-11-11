An interpreter written in Rust by following the books from
[Thorsten Ball](https://interpreterbook.com/) on writing an interpreter in Go.

The interpreter is built from scratch and it includes a lexer (tokenizer), a parser and a tree walking interpreter.

---

## Features

- The language supports int, string and boolean data types.
- Supports composite data types: Arrays and HashMaps.
- Supports common operators like +, -, ==, !=, <, > etc.
- Supports let, return and while statements.
- Supports assignments, if/else expressions and function expressions.
- Supports higher order functions and closures.
- Have a range of built-in functions such as len, print, push, sleep etc.
- Supports indexing on arrays, strings and HashMaps.
- Supports Range indexing on arrays and strings.

## Usage

- Download the asset file relevant to your platform from the latest release.
- Extract the zip and run the executable to start the REPL.
- Run the executable with relative filepath as an argument to execute a script file.

## Example Code

```go
let map = fn(arr, callback) {
    let new_arr = [];
    let i = 0;

    while (i < len(arr)) {
        let mapped = callback(arr[i]);
        push(new_arr, mapped);
        i = i + 1;
    }

    return new_arr;
}

let add_1 = fn(x) {
    return x + 1;
}

let arr = [10, 20, 30, 40, 50];
map(arr, add_1);
```

> More code examples can be found in tests/testfiles directory.

## Methodology

- The lexer does the tokenization of the code input.
- The tokens will be fed in to the parser, which forwards the tokens as it parses the program statements one by one.
- To parse expressions, pratt parsing is used (recursive decent parsing).
- For each statement/expression parsed, the parser will create a corresponding AST node to be later evaluated.
- Once the parsing is finished, a tree walking evaluator will evaluate the program from beginning to the end and will return the final result.
- There is a global state as well as block level state for functions, if-expressions, and while-statements, which are built on top of HashMaps. Each function call also generates a scope chain to facilitate closures.
