use std::fs::OpenOptions;
use std::io::prelude::*;
use std::fs::File;

const FILE_PATH: &str = "./logs/error.txt";


pub fn error_log (msg: String){

    match File::open(FILE_PATH){
        Ok(_) => println!("File exists !"),
        Err(e) => {
            println!("Error: {e} !");
            match File::create(FILE_PATH) {
                Ok(_) => println!("File Created !"),
                Err(e) => println!("Error: {e} !"),                
            }
        }
    }

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(FILE_PATH)
        .expect("Error while opening file");


    file.write(msg.as_bytes()).expect("Error while writing to file");

}

        