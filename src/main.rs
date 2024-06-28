use std::io::Read;

use ink_fmt::{config::FormatConfig, format};

fn main() {
    let mut source = String::new();
    std::io::stdin()
        .lock()
        .read_to_string(&mut source)
        .expect("Why can't we read from stdin?");

    let source = format(FormatConfig::default(), source);

    println!("{}", source);
}
