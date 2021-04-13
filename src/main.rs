use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    // Print args passed in
    println!("{:#?}", args)
}
