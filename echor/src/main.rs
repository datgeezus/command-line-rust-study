use clap::{App, Arg};

fn main() {
    let matches = App::new("echor")
        .version("0.1.0")
        .author("John Doe <john.doe@example.com")
        .about("Rust echo")
        .arg(
            Arg::with_name("text")
            .value_name("TEXT")
            .help("Input text")
            .required(true)
            .min_values(1),
            )
        .arg(
            Arg::with_name("omit_newline")
            .short('n')
            .help("Do not print newline")
            .takes_value(false),
            )
        .get_matches();
    let text: Vec<_> = matches.get_many::<String>("text")
        .expect("`text` is required")
        .map(|s| s.as_str())
        .collect();
    let omit_newline = matches.is_present("omit_newline");
    print!("{}{}", text.join(" "), if omit_newline { "" } else { "\n" });
}
