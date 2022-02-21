fn main() {
    print_labeled_measurement(5, 'h');


    // statements and expressions

    // Statements are instructions that perform some action and do not return a value.
    // Expressions evaluate to a resulting value.

    let y = {
        let x = 3;
        x + 1 //Expressions do not include ending semicolons.
        // If you add a semicolon to the end of an expression, you turn it into a statement,
        // and it will then not return a value.
    };

    println!("The value of y is: {}", y);

    let x = five();
    println!("The value of x is: {}", x);
}

fn five() -> i32 {
    5
    // return
    //  the return value of the function is synonymous with the value
    // of the final expression in the block of the body of a function.
    // You can return early from a function by using the return keyword and specifying a value,
    // but most functions return the last expression implicitly.
}


fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}



