fn main() {
    let _x = 52;

    println!("Hello World!");

    //TUples
    //immutable by default
    let _tup = (1, "2", 3.0);

    println!("{}", _tup.0);

    let mut _tup2 = (1, "2", 3.0, false);
    println!("{}", _tup2.0);

    _tup2.0 = 5;
    println!("{}", _tup2.0);

    //arrays
    let _arr = [1, 2, 3, 4, 5];
    println!("{}", _arr[0]);

    let mut _arr2 = _arr;
    _arr2[0] = 5;

    println!("{}", _arr2[0]);

    let _arr3: [u32; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    println!("{}", _arr3[5]);
}
