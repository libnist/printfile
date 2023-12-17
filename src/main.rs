use std::fs::File;
use std::io::{BufRead, BufReader};
use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    for arg in &args[1..]{
        read_file(arg)
    }
}

fn read_file(path: &str) {
    let file = File::open(path);

    match file {
        // The happy path
        Ok(file) => {
            let reader = BufReader::new(file);
            for line in reader.lines() {
                println!("{}", line.unwrap());
            }
        },
        // The unhappy path
        Err(error) => {
            match error.kind() {
                std::io::ErrorKind::NotFound => {
                    println!("There was no file at: {}", path)
                },
                _ => {
                    println!("There is something wrong with the file at: {}", path)
                }
            }
        },
    };
}
