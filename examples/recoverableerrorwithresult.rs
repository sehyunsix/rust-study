use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file: Result<File, std::io::Error> = match greeting_file_result {
        Ok(file) => Ok(file),
        Err(error) => match error.kind() {
            ErrorKind::NotFound =>  {
                println!("No such file in here");
                Err(error)
            },
            _ => {
                panic!("Problem opening the file: {error:?}");
            }
        },
    };
}