use std::io;

fn main() {
    println!("Hello World!");

    add_to_number(5.0, 8.8);
    minus_to_number(9.9, 6.834);
    product_of_numbers(5, 5);
    quotient_of_numbers(44, 22);

    let x = 254_i16;
    let y = 10_i32;

    let z = x % y as i16;
    println!("{}", z);

    let xx = (i32::MAX as f32) + 1.0;
    println!("{}", xx);

    let mut input2 = String::new();
    io::stdin()
        .read_line(&mut input2)
        .expect("expected to read line");

    let parse_input2_to_i64: i64 = input2.trim().parse().unwrap();

    println!("Enter an input");
    println!("Your input ist: {}", parse_input2_to_i64);
}

fn add_to_number(x: f32, y: f32) {
    let sum_of_two_nums = x + y;
    println!("The sum of {} and {} is: {}", x, y, sum_of_two_nums);
}

fn minus_to_number(x: f32, y: f32) {
    let difference_value: f32 = x - y;
    println!("The difference of {} and {} is: {}", x, y, difference_value);
}

fn product_of_numbers(x: i32, y: i32) {
    let product_value: i32 = x * y;
    println!("The product of {} and {} is: {}", x, y, product_value);
}

fn quotient_of_numbers(x: i64, y: i64) {
    let quotient_value: i64 = x / y;
    println!("The quotient of {} and {} is: {}", x, y, quotient_value);
}
