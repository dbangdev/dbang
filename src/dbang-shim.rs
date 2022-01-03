pub fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut alias = args[0].clone();
    if alias.contains("/") {
        alias = alias.split("/").last().unwrap().to_string();
    }
    println!("alias name: {}", alias);
}
