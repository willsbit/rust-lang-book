fn main() {
    // mutability
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // constants
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    // shadowing
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);



    // tuplas e arrays
    // Tuples have a fixed length: once declared, they cannot grow or shrink in size.
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {}", y);

    //  We can also access a tuple element directly by using a period (.)
    // followed by the index of the value we want to access.
    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    // Unlike a tuple, every element of an array must have the same type.
    // Unlike arrays in some other languages, arrays in Rust have a fixed length.

    let a = [1, 2, 3, 4, 5];


    //  A vector is a similar collection type provided by the standard library
    // that is allowed to grow or shrink in size. If you’re unsure
    // whether to use an array or a vector, chances are you should use a vector

    let months = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];

    let b = [1;5]; // igual a escrever let b = [1, 1, 1, 1, 1];

    let first = a[0];
    let second = a[1];

}


