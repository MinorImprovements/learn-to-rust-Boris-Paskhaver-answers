use std::io::{self, stdin};
use std::{fs, process};

fn main() {
    match write_to_file() {
        Ok(file) => println!("Successfully wrote to file: {file}"),
        Err(error) => {
            eprintln!("There was an error: {error:?}");
            process::exit(1);
        }
    };
}

fn write_to_file() -> io::Result<String> {
    println!("What file would you like to write to:");
    let mut file_name: String = String::new();
    stdin().read_line(&mut file_name)?;
    println!("What would you like to write to the file:");
    let mut add_to_file: String = String::new();
    stdin().read_line(&mut add_to_file)?;
    fs::write(file_name.trim(), add_to_file.trim())?;
    Ok(file_name)
}
