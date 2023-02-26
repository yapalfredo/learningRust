fn main() {
    print!("Hello World!");

    let x: f32 = 10.0;
    let y: f32 = 25.0;

    println!(
        "Adding {} and {} will equate to {}",
        x,
        y,
        add_two_numbers(x, y)
    );

    // expression

    let _number = {
        let x = 3;

        //no semicolon because the value in this will be assigned to _number
        x + 1
    };

    println!("{}", _number);

    print_hello_world();

    println!("{}", return_result(10, 25));
}

//semicolon must not be there for function
//that returns a value
//unless you put the keyword 'return'
fn add_two_numbers(x: f32, y: f32) -> f32 {
    x + y
}

//for function w/ no returning value
//semicolon is required
fn print_hello_world() {
    println!("Hello World");
}

fn return_result(x: i32, y: i32) -> i32 {
    let res = x + y;
    res
}
