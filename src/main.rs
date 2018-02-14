use std::env;

fn main() {
    let program = env::args().nth(1).expect("Missing argument");

    println!("{:?}", eval(program, 0));
}

fn eval(program: String, mut pointer: usize) -> usize {
    let p = program.chars().nth(pointer).unwrap();
    pointer += 1;
    let mut val = p.to_digit(10).unwrap();
    while pointer <= program.len() - 1 && program.chars().nth(pointer).unwrap().is_digit(10) {
        val = val * 10 + program.chars().nth(pointer).unwrap().to_digit(10).unwrap();
        pointer += 1;
    }
    return val as usize;
}
