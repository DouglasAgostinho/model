mod tools;

use tools::error_log;
use std::io;


//Contant definitions

const ERROR_PATH: &str = "./logs/error.txt";

fn main() {

    let mut msg: String = String::new();

    match io::stdin().read_line(&mut msg) {
        Ok(_) => println!("Ok"),
        Err(e) => println!("Error: {e}"),
    }

    error_log(msg, ERROR_PATH);
}