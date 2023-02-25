use std::io;

fn main() {
    println!("Input your name!");

    let mut _input = String::new();

    io::stdin()
        .read_line(&mut _input)
        .expect("Failed to read input");

    print!("Your input is {}", _input);
}
