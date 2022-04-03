use std::io::{self, Write};

use booltable::{compiler::Compiler, parser::Parser, vm::VM};

fn main() {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let mut parser = Parser::new(&input);
        let parsed_equation = parser.parse_equation().unwrap();

        let compiler = Compiler::new(parsed_equation);
        let compiled_equation = compiler.compile();

        let mut vm = VM::new(compiled_equation);
        let truth_table = vm.gen();

        println!("{}", truth_table);
    }
}
