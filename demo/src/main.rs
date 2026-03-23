fn main() {
    println!("{}", add('+', 2, 3));
    println!("{}", add('*', 2, 3));
    println!("Hello, world!");
    println!("{}", add(19, 23));
}

fn add(op: char, a: u8, b: u8) -> u8 {
    if op == '+' {
        a + b
    } else {
        a * b
    }
}

