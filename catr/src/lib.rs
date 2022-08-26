use std::error::Error;
use clap::{App, Arg};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("catr")
        .version("0.1.0")
        .author("John Doe")
        .about("Rust cat")
        .arg(
            Arg::with_name("files")
                .required(true)
        )
        .arg(Arg::with_name("number_lines")
            .required(false)
            .short('n')
            .takes_value(false)
        )
        .arg(Arg::with_name("number_nonblank_lines")
            .required(false)
            .short('n')
            .takes_value(false)
        )
        .get_matches();

    Ok(Config {
        files: matches.get_many::<String>("files")
            .expect("`files` is required")
            .cloned()
            .collect(),
        number_lines: matches.is_present("number_lines"),
        number_nonblank_lines: matches.is_present("number_nonblank_lines")
    })
}

pub fn run() -> MyResult<()> {
    println!("Hello, world!");
    Ok(())
}
