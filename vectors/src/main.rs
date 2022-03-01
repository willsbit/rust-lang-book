fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);
                            // [] gives us a reference;
    match v.get(2) {  //  get method with the index passed as an argument gives us an Option<&T>
        //When the get method is passed an index that is outside the vector, it returns None without panicking
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
        println!("{}", i);
    }

    // Using an Enum to Store Multiple Types
    // the variants of an enum are defined under the same enum type,
    // so when we need to store elements of a different type in a vector,
    // we can define and use an enum

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}
