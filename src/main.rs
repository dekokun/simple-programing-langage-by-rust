use std::env;
use std::process;

fn main() {
    let program = env::args().nth(1).expect("Missing argument");

    println!("{:?}", eval(&program, 0).0);
}

fn eval(program: &str, mut pointer: usize) -> (usize, usize) {
    while pointer <= program.len() - 1 && program.chars().nth(pointer).unwrap() == ' ' {
        pointer += 1;
    }
    let p = program.chars().nth(pointer).unwrap();
    pointer += 1;
    if p.is_digit(10) {
        let mut val = p.to_digit(10).unwrap();
        while pointer <= program.len() - 1 && program.chars().nth(pointer).unwrap().is_digit(10) {
            val = val * 10 + program.chars().nth(pointer).unwrap().to_digit(10).unwrap();
            pointer += 1;
        }
        return (val as usize, pointer);
    }
    if p == '+' {
        let (x, pointer) = eval(program, pointer);
        let (y, pointer) = eval(program, pointer);
        return (x + y, pointer);
    }
    error(format!("Invalid character: {:?}", p));
}

fn error(error: String) -> ! {
    eprintln!("{}", error);
    process::exit(1);
}
