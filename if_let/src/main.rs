// if let é uma alternativa menos verbosa para a sintaxe match
fn main() {
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    } else {
        println!("Maximum not set!")
    }
}
