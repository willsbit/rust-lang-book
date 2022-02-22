mod references_and_borrowing;

fn main() {
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", s); // This will print `hello, world!`

    // heap and pointers
    let s1 = String::from("hello");
    // let s2 = s1;
    // s1 não existe mais -> apenas s2 está apondando para a memória no heap que contém "hello"

    // para fazer uma "deep copy" (copiar a memória do heap em si), devemos usar o .clone()
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("{}, world!", s1);
}
