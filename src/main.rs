use std::env;
use std::io;

mod checks;

fn main() -> io::Result<()> {
    let args: Vec<_> = env::args().collect();

    for arg in args[1..].iter() {
        let empty = checks::is_empty(arg);
        println!("{:?}", empty);
    }

    Ok(())
}