use std::fs;
use std::fs::File;
use std::io::prelude::*;

pub struct Config {
    pub folder_name: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Not Enough Arguments. Provide Folder Name.");
        }
        let folder_name = args[1].clone();

        Ok(Config { folder_name })
    }
}

pub fn create_dir(folder_name: &String) -> std::io::Result<()> {
    fs::create_dir(folder_name.clone())?;
    Ok(())
}

pub fn write_data(folder: &String, data: &[u8], file: &str) -> std::io::Result<()> {
    let path_req = format!("./{}/{}", folder, file);
    let mut file = File::create(path_req)?;
    file.write_all(data)?;
    Ok(())
}
