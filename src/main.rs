//! This file will execute the given command,
//! retrieve the first line of stdout, strip the eventual newline
//! and print it on stdout.
use std::env::args;
use std::process::Command;

fn main() -> std::io::Result<()> {
    let arguments = args().collect::<Vec<_>>();
    assert!(arguments.len() >= 2);
    let output = Command::new(&arguments[1]).args(&arguments[2..]).output()?;
    let res = String::from_utf8(output.stdout)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
    res.lines().next().map(|l| print!("{}", l));
    Ok(())
}
