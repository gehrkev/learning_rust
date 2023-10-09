use std::fs::rename;
use std::fs::File;
use std::io::Error;
use std::io::ErrorKind;

fn main() {
    // panic!("panicked here");

    // let vec = vec![1];
    // vec[10]; //panic! // $env:RUST_BACKTRACE=1
    // let file = File::open("error.txt").expect("Error opening the file!");
    // let file = match file {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("error2.txt") {
    //             Ok(file_created) => file_created,
    //             Err(err) => panic!("Cannot create the file!")
    //         }
    //         _ => panic!("Other error kind")
    //     }
    // };
    rename_file().unwrap();
}

fn open_file() -> Result<File, Error> {
    let file = File::open("error.txt")?;
    Ok(file)
}

fn rename_file() -> Result<(), Error> {
    let file = rename("error.txt", "renamed.txt")?;
    Ok(file)
}

// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }
