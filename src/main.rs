use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    //println!("{:#?}", args);
    
    // args[0] is location of file, target args[1] for user command

    let command = args[1].clone();
    
     

        if command == "get" {
            println!("Get command ran");
        } else {
            println!("{}, was Run", args[1]);
        }
}
