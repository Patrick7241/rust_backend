use ulid::Ulid;

fn main() {
    let id = Ulid::new();
    println!("User ID: {}", id.to_string()); // 可存为 String
    println!("As u128: {}", id.0);          // 存为 big int
}