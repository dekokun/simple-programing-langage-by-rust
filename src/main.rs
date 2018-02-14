use std::env;

fn main() {
    let p = env::args().nth(1).expect("Missing argument");

    println!("{:?}", eval(p, 0));
}

fn eval(p: String, pointer: usize) -> usize {
    if pointer == p.len() {
        return 0;
    }
    return eval(p, pointer + 1);
}
