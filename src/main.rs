extern crate getopts;

use getopts::Options;
use std::borrow::Borrow;
use std::env;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::prelude::*;
use std::process;

fn usage(program: &str, opts: Options) -> String {
    opts.usage(&format!("Usage: {} [options] FILE", program))
}

fn pipe(reader: &mut impl Read, path: Option<&str>, append: bool) -> io::Result<()> {
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;
    let mut writer: Box<dyn Write> = match path {
        None => Box::new(io::stdout()),
        Some(path) => Box::new(open_file(path, append)?),
    };
    writer.write_all(&buffer)?;
    Ok(())
}

fn open_file(path: &str, append: bool) -> io::Result<File> {
    OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(!append)
        .append(append)
        .open(path)
}

fn main() {
    let mut opts = Options::new();
    opts.optflag("a", "", "append to file instead of overwriting");
    opts.optflag("h", "help", "print this help menu");

    let mut args = env::args();
    let program = args.next().unwrap_or("rw".into());
    let matches = match opts.parse(args) {
        Ok(m) => m,
        Err(err) => {
            eprintln!("{}", err);
            eprintln!("{}", usage(&program, opts));
            process::exit(1);
        }
    };
    if matches.opt_present("h") {
        return println!("{}", usage(&program, opts));
    }
    let append = matches.opt_present("a");
    if matches.free.len() > 1 {
        eprintln!("Too many arguments.");
        eprintln!("{}", usage(&program, opts));
        process::exit(1);
    }
    let path = matches.free.first().map(Borrow::borrow);
    if let Err(err) = pipe(&mut io::stdin(), path, append) {
        eprintln!("{}", err);
        process::exit(1);
    }
}
