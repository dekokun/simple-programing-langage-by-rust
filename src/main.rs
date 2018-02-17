use std::env;
use std::process;
use std::collections::HashMap;

fn main() {
    let program = env::args().nth(1).expect("Missing argument");

    let func = &mut HashMap::new();
    println!("{:?}", eval(&program, func, 0, 0).0);
}

fn eval(
    program: &str,
    func: &mut HashMap<char, String>,
    mut pointer: usize,
    arg: usize,
) -> (usize, usize) {
    // skip space
    while pointer <= program.len() - 1 && program.chars().nth(pointer).unwrap() == ' ' {
        pointer += 1;
    }
    let p = program.chars().nth(pointer).unwrap();
    pointer += 1;

    let next = program.chars().nth(pointer).unwrap_or('a');
    match p {
        // Function parameter
        '.' => return (arg, pointer),
        // Function definition
        'A'...'Z' if next == '[' => {
            let func_name = p;
            // '['
            pointer += 1;
            let mut func_string: String = "".to_string();
            while program.chars().nth(pointer).unwrap() != ']' {
                func_string.push(program.chars().nth(pointer).unwrap());
                pointer += 1;
            }
            func.insert(func_name, func_string);
            // ']'
            pointer += 1;
            return eval(program, func, pointer, arg);
        }
        // Literal numbers
        _ if p.is_digit(10) => {
            let mut val = p.to_digit(10).unwrap();
            while pointer <= program.len() - 1 && program.chars().nth(pointer).unwrap().is_digit(10)
            {
                val = val * 10 + program.chars().nth(pointer).unwrap().to_digit(10).unwrap();
                pointer += 1;
            }
            return (val as usize, pointer);
        }
        // arithmetic operators
        '+' | '-' | '*' | '/' => {
            let (x, pointer) = eval(program, func, pointer, arg);
            let (y, pointer) = eval(program, func, pointer, arg);
            let val = match p {
                '+' => x + y,
                '-' => x - y,
                '*' => x * y,
                '/' => x / y,
                _ => error(format!("Invalid operator: {:?}", p)),
            };
            return (val, pointer);
        }
        _ => error(format!("Invalid character: {:?}", p)),
    };
}

fn error(error: String) -> ! {
    eprintln!("{}", error);
    process::exit(1);
}
