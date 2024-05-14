use std::fs::OpenOptions;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;


pub fn create_logs (path: &str){

    match File::open(path){
        Ok(_) => println!("File exists !"),
        Err(e) => {
            println!("Error: {e} !");
            match File::create(path) {
                Ok(_) => println!("File Created !"),
                Err(e) => println!("Error: {e} !"),                
            }
        }
    }

}

pub fn error_log (msg: String, path: &str){

    if !(Path::new(path).is_file()){
        create_logs(path);
    }

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(path)
        .expect("Error while opening file");
        


    file.write(msg.as_bytes()).expect("Error while writing to file");

}

        