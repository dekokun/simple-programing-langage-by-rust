use std::env;
use std::process;
use std::collections::HashMap;

fn main() {
    let program = env::args().nth(1).expect("Missing argument");

    let func = &mut HashMap::new();
    println!("{:?}", eval(&program, func, 0, [0; 26]).0);
}

fn eval(
    program: &str,
    func: &mut HashMap<char, String>,
    mut pointer: usize,
    mut args: [usize; 26],
) -> (usize, usize) {
    let p = program.chars().nth(pointer).unwrap();
    pointer += 1;

    let next = program.chars().nth(pointer).unwrap_or('a');
    match p {
        // skip space
        _ if p.is_whitespace() => {
            return eval(program, func, pointer, args);
        }
        // Function parameter
        'a'...'z' => {
            return (args[p as usize - 'a' as usize], pointer);
        }
        'P' => {
            if next != '(' {
                error(format!("expect: (, actual: {}", next));
            }
            // '('
            pointer += 1;
            let (val, pointer) = eval(program, func, pointer, args);
            println!("{}", val);
            return (val, pointer);
        }
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
            return eval(program, func, pointer, args);
        }
        // Function application
        'A'...'Z' if next == '(' => {
            let func_name = p;
            // '('
            pointer += 1;
            let mut i = 0;
            while program.chars().nth(pointer).unwrap() != ')' {
                if program.chars().nth(pointer).unwrap() == ' ' {
                    pointer += 1;
                    continue;
                }
                let result = eval(program, func, pointer, args);
                pointer = result.1;
                args[i] = result.0;
                i += 1;
            }

            let func_string = func.get(&func_name).unwrap();
            let (val, _) = eval(func_string, &mut func.clone(), 0, args);
            // ')'
            pointer += 1;
            return (val, pointer);
        }
        // Literal numbers
        '0'...'9' => {
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
            let (x, pointer) = eval(program, func, pointer, args);
            let (y, pointer) = eval(program, func, pointer, args);
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
