use std::io::{Read, Write};

use ink_fmt::format;

fn main() {
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

    let output = format(source);

    std::io::stdout()
        .lock()
        .write_all(output.as_bytes())
        .expect("Writing to stdout should work");
}
