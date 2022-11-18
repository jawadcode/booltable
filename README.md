# booltable

Truth table generator for basic boolean algebraic equations

## How does it work?

It's a REPL which takes in equations in the form: `<boolean expression> = <output name>`, where `<boolean expression>` can be made up of arbitrarily named boolean variables.

Syntax error diagnostics are completely non-existent so don't make any syntax errors please 😂.

| Operator | Syntax               |
|:--------:|:---------------------|
| NOT      | `NOT`, `!`, `¬`      |
| AND      | `AND`, `.`, `∨`      |
| OR       | `OR`, `+`, `∨`       |
| XOR      | `XOR`, `^`, `⊕`, `⊻` |
