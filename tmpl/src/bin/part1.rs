use std::io::{self, BufRead, Write};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    process(handle, io::stdout());
}

fn process(input: impl BufRead, mut output: impl Write) {
    output.write(0.to_string().as_bytes()).unwrap();
}