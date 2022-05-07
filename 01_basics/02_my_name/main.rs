use std::io;

fn main() {
    println!("Come ti chiami?");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read line");
    println!("ciao {}", name);
}