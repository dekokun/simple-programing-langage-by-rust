use std::env;
use std::process;
use std::collections::HashMap;

fn main() {
    let program = env::args().nth(1).expect("Missing argument");

    let func = &mut HashMap::new();
    println!("{:?}", eval(&program, func, 0, [None; 26]).0);
}

fn eval(
    program: &str,
    func: &mut HashMap<char, String>,
    mut pointer: usize,
    args: [Option<usize>; 26],
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
            // println!("args: {:?}", args);
            return (args[p as usize - 'a' as usize].unwrap(), pointer);
        }
        'P' => {
            if next != '(' {
                error(format!("expect: (, actual: {}", next));
            }
            // '('
            pointer += 1;
            let (val, mut pointer) = eval(program, func, pointer, args);
            println!("{}", val);
            // ')'
            pointer += 1;
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
            let mut newargs = [None; 26];
            while program.chars().nth(pointer).unwrap() != ')' {
                if program.chars().nth(pointer).unwrap() == ' ' {
                    pointer += 1;
                    continue;
                }
                let result = eval(program, func, pointer, args);
                newargs[i] = Some(result.0);
                pointer = result.1;
                i += 1;
            }

            let func_string = func.get(&func_name).unwrap();
            let mut func_pointer = 0;
            let mut val = 0;
            while func_pointer <= func_string.len() - 1 {
                let result = eval(func_string, &mut func.clone(), func_pointer, newargs);
                val = result.0;
                func_pointer = result.1;
                while func_pointer <= func_string.len() - 1
                    && func_string.chars().nth(func_pointer).unwrap() == ' '
                {
                    func_pointer += 1;
                }
            }
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
        _ => error(format!(
            "Invalid character: {:?}, pointer: {:?}",
            p,
            pointer
        )),
    };
}

fn error(error: String) -> ! {
    eprintln!("{}", error);
    process::exit(1);
}
fn _log(log: String) {
    eprintln!("{}", log);
}
