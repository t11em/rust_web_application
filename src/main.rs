fn main() {
    println!("Hello, world!");
    println!("global: {}", std::env::var("GLOBAL").unwrap());
    println!("global: {}", std::env::var("LOCAL").unwrap());
}
