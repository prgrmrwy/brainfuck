use brainfuck::Compiler;
use std::fs;
use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();

    let path = &args[1];

    println!("path: {}", path);
    let code_buffer = fs::read_to_string(path).unwrap();
 
    Compiler::new(&code_buffer).interpret();
}
