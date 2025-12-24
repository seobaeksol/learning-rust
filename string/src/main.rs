fn main() {
    let str = "한글".to_string();
    println!("{}", str.len());
    println!("{}", str.chars().count());

    for char in str.chars() {
        println!("{}", char);
    }

    println!("{}", str);
}
