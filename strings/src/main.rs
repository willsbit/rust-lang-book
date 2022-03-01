fn main() {
    let mut s = String::new();

    let data = "initial contents";
    let s = data.to_string();

    // mesma coisa que
    let s = String::from("initial contents");

    let mut  f = String::from("foo");
    f.push_str("bar");

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
    // mesma coisa que
    let s = format!("{}-{}-{}", s1, s2, s3);

    let hello = "Здравствуйте";

    let s = &hello[0..4];


    // iterando em uma string
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }


    for b in "नमस्ते".bytes() {
        println!("{}", b); // valid Unicode scalar values may be made up of more than 1 byte
    }

}
