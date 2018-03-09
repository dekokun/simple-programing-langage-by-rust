use std::env;
use std::process;
use std::collections::HashMap;

struct State<'a> {
    program: &'a str,
    func: HashMap<char, String>,
    pointer: usize,
    args: Vec<usize>,
}

fn main() {
    let program = env::args().nth(1).expect("Missing argument");

    let func = HashMap::new();
    let args = Vec::new();
    let mut state = State {
        program: &program,
        func: func,
        pointer: 0,
        args: args,
    };
    println!("{:?}", eval(&mut state).unwrap().0);
}

fn eval(state: &mut State) -> Option<(usize, usize)> {
    let p = state.program.chars().nth(state.pointer)?;
    state.pointer += 1;

    match p {
        // skip space
        _ if p.is_whitespace() => {
            return eval(state);
        }
        // Function parameter
        'a'...'z' => {
            // println!("args: {:?}", args);
            return Some((state.args[p as usize - 'a' as usize], state.pointer));
        }
        'P' => {
            if next(state) != '(' {
                error(format!("expect: (, actual: {}", next(state)));
            }
            // '('
            state.pointer += 1;
            let (val, mut pointer) = eval(state)?;
            println!("{}", val);
            // ')'
            pointer += 1;
            return Some((val, pointer));
        }
        // Function definition
        'A'...'Z' if next(state) == '[' => {
            let func_name = p;
            // '['
            state.pointer += 1;
            let mut func_string: String = "".to_string();
            while state.program.chars().nth(state.pointer)? != ']' {
                func_string.push(state.program.chars().nth(state.pointer)?);
                state.pointer += 1;
            }
            state.func.insert(func_name, func_string);
            // ']'
            state.pointer += 1;
            return eval(state);
        }
        // Function application
        'A'...'Z' if next(state) == '(' => {
            let func_name = p;
            // '('
            state.pointer += 1;
            let mut newargs = Vec::new();
            while state.program.chars().nth(state.pointer)? != ')' {
                if state.program.chars().nth(state.pointer)?.is_whitespace() {
                    state.pointer += 1;
                    continue;
                }
                let result = eval(state)?;
                newargs.push(result.0);
                state.pointer = result.1;
            }

            let func_string = state.func.get(&func_name)?;
            let mut func_pointer = 0;
            let mut val = 0;
            while func_pointer <= func_string.len() - 1 {
                let result = eval(&mut State {
                    program: func_string,
                    func: state.func.clone(),
                    pointer: func_pointer,
                    args: newargs.clone(),
                })?;
                val = result.0;
                func_pointer = result.1;
                while func_pointer <= func_string.len() - 1
                    && func_string.chars().nth(func_pointer)?.is_whitespace()
                {
                    func_pointer += 1;
                }
            }
            // ')'
            state.pointer += 1;
            return Some((val, state.pointer));
        }
        // Literal numbers
        '0'...'9' => {
            let mut val = p.to_digit(10)?;
            while state.pointer <= state.program.len() - 1
                && state.program.chars().nth(state.pointer)?.is_digit(10)
            {
                val = val * 10 + state.program.chars().nth(state.pointer)?.to_digit(10)?;
                state.pointer += 1;
            }
            return Some((val as usize, state.pointer));
        }
        // arithmetic operators
        '+' | '-' | '*' | '/' => {
            let (x, pointer) = eval(state)?;
            state.pointer = pointer;
            let (y, pointer) = eval(state)?;
            state.pointer = pointer;
            let val = match p {
                '+' => x + y,
                '-' => x - y,
                '*' => x * y,
                '/' => x / y,
                _ => error(format!("Invalid operator: {:?}", p)),
            };
            return Some((val, pointer));
        }
        _ => error(format!(
            "Invalid character: {:?}, pointer: {:?}",
            p, state.pointer
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

fn next(state: &State) -> char {
    return state
        .program
        .get(state.pointer..state.pointer + 1)
        .unwrap()
        .chars()
        .last()
        .unwrap();
}
