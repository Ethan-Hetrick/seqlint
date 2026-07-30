use std::env;
use std::io;

mod checks;

fn main() -> io::Result<()> {
    let args: Vec<_> = env::args().collect();

    for arg in args[1..].iter() {
        println!{"{:?}", &arg};
        dbg!(checks::integrity_check(arg));
        dbg!(checks::special_char_check(arg));
    }

    Ok(())
}