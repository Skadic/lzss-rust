use std::fs::File;
use std::io::Read;

mod augmented;
mod lzfactor;
mod lz;

fn main() -> std::io::Result<()> {

    // Get the arguments and skip 1, since the first argument is the application path
    let mut args = std::env::args().skip(1);

    // Get the file name. If there is none, exit and send a message
    let file_name = match args.next() {
        Some(name) => name,
        None => {
            println!("Please input a file");
            return Ok(())
        }
    };

    let mut file = String::new();
    File::open(file_name).expect("Unable to open file").read_to_string(&mut file)?;

    println!("{}", lz::lz_factorized_string(file));

    Ok(())
}
