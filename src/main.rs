use std::io::Read;

use ink_fmt::{config::FormatConfig, format};

fn main() -> Result<(), String> {
    let mut source = String::new();
    let args = &mut std::env::args();
    let _ = args.next(); // that's us, we know who we are.
    let filename = args.next();
    if filename.is_none() || filename.as_ref().is_some_and(|it| it == "-") {
        eprintln!("Reading from stdin");
        std::io::stdin()
            .lock()
            .read_to_string(&mut source)
            .expect("Why can't we read from stdin?");
    } else {
        eprintln!("Reading from file {}", filename.as_ref().unwrap());
        source = std::fs::read_to_string(&filename.unwrap()).expect("File should exist");
    }

    assert!(!source.is_empty());
    format(FormatConfig::default(), source).map(|result| println!("{}", result))
}
