use std::io::stdin;
use clap::{Arg, App};

fn main() {
    let args = App::new("strconv")
        .version("0.0.1")
        .about("Converts mixed binary strings to ASCII")
        .arg(Arg::with_name("encoding"))
                    .short("e")
                    .long("encoding")
                    .takes_value(true)
                    .possible_values(&["ascii", "utf-8"])
                    .default("ascii")
                    .help("Output encoding")
        .get_matches();
    println!("Hello, world!");
}
