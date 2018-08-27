#![allow(unused)]

use std::env;
use std::fs;
use std::fs::File;
use std::io::prelude::*;

fn write_data(folder: &String) -> std::io::Result<()> {
    let shader = b"
package main 

func main() {
     fmt.Println(\"Hey\")
}
";

    let path_req = format!("./{}/main.go", folder);
    println!("{}", path_req);
    let mut file = File::create(path_req)?;
    file.write_all(shader)?;
    Ok(())
}

fn main() {
    println!("Starting the Program..");
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Not Enough Arguments. Please provide a folder name.");
    }
    let folder_name = &args[1];
    let res = fs::create_dir(folder_name);
    match res {
        Ok(()) => println!("Folder {} Created.", folder_name),
        Err(error) => {
            println!("Error: {:?}  ", error);
        }
    }
    let response = write_data(folder_name);
}
