use std::io;

fn split_string(s: String, delimiter: char, field: usize) -> String {
    let parts: Vec<&str> = s.split(delimiter).collect();
    let result = parts.get(field);
    return result.unwrap().to_string();
}
fn main() {
    let mut input=String::new();
    while input.trim() != "exit" {
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        println!("You entered: {}", input);
    }

    // for i in (0..=10).rev() {
    //     println!("Hello, world! {}", i);
    // }

    // let name = "Alice";
    // match name {
    //     "Alice" => println!("Hello, Alice!"),
    //     "Bob" => println!("Hello, Bob!"),
    //     _ => println!("Hello, world!"),
    // }

    let chunk = split_string("hello,world".to_string(), ',', 1);
    println!("{}", chunk);

}
