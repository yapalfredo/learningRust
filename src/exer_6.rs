fn main() {
    let cond = 2.1 <= 2.005;

    print!("{}", cond);

    let cond2 = true && cond;

    println!("{}", cond2);

    let cond3 = false || cond2;

    println!("{}", cond3);

    let cond4 = "bread";

    if cond4 == "food" {
        print!("Your condition is met!");
    } else if cond4 == "cake" {
        print!("That was a good coake");
    } else {
        print!("Chocolate Crepe");
    }
}
