fn main() {
    dotenvy::dotenv().expect("Failed to parse .env");
    println!("Hello, world!");
}
