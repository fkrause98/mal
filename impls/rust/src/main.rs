use std::{
    env::Args,
    io::{self, stdin, stdout, Read, Write},
};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    let stdin = stdin();
    let mut stdout = stdout().lock();
    // let mut stdout = stdout();
    loop {
        stdout.write(b"user> ")?;
        stdout.flush()?;
        let read_lines = stdin.read_line(&mut buffer)?;
        match read_lines {
            0 => return Ok(()),
            _ => {
                stdout.write(buffer.as_bytes())?;
                buffer.clear();
            }
        }
    }
}

pub fn READ(input: &str) {
    println!("{}", input);
}

pub fn EVAL(input: &str) {
    println!("{}", input);
}

pub fn PRINT(input: &str) {
    println!("{}", input);
}

pub fn rep(input: &str) {}
