// #![allow(unused)]
extern crate goc;

use goc as Util;
use goc::Config;
use std::io::ErrorKind;

use std::env;
use std::process;

const DATA: &[u8] = b"
package main 

func main() {
     fmt.Println(\"Hey\")
}
";

const FILE_NAME: &str = "main.go";

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments {}", err);
        process::exit(1);
    });

    let folder_name = config.folder_name.clone();

    if let Err(ref e) = Util::create_dir(&folder_name) {
        if e.kind() == ErrorKind::AlreadyExists {
            println!("Error in creating directory: {:?}", e.to_string());
            process::exit(1);
        }
        println!("Error in creating directory: {:?}", e.to_string());
        process::exit(1);
    }

    if let Err(e) = Util::write_data(&folder_name, DATA, FILE_NAME) {
        println!("Error in writing file: {:?}", e);
        process::exit(1);
    }
    println!("Done!");
}
