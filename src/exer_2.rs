fn main() {
    println!("Hello World!");

    let x = return_sum(5, 5);
    println!("The Sum of 5 and 5 is equal to {}", x);
}

fn return_sum(x: i32, y: i32) -> i32 {
    x + y
}
