use std::io::BufRead;
use std::{io, fmt};
use clap::{Arg, App};

#[derive(Debug, Clone)]
struct ConversionError;

impl fmt::Display for ConversionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "conversion failed")
    }
}

fn convert() -> Result<String, ConversionError> {
    let mut result = String::new();
    let mut buf = vec![];
    let stdin = io::stdin();
    let _ = stdin.lock();
    let mut reader = std::io::BufReader::new(stdin);

    reader.read_until(b'\n', &mut buf).expect("Unable to read stdin");

    let mut slice: &[u8] = &buf[0..];
    loop {
        match std::str::from_utf8(slice) {
            Ok(valid) => {
                result.push_str(valid);
                println!("result: {}", result);
                break
            }

            Err(error) => {
                let(valid, after_valid) = buf.split_at(error.valid_up_to());
                //result.push_str(std::str::from_utf8(valid).unwrap());
                result.push_str("?x00");
                if let Some(invalid_sequence_length) = error.error_len() {
                    slice = &after_valid[invalid_sequence_length..]
                } else {
                    return Err(ConversionError)
                }
            }
        }
    }

    return Ok(result)
}

fn main() {
    let args = App::new("strconv")
        .version("0.0.1")
        .about("Converts mixed binary strings to ASCII")
        .arg(Arg::with_name("encoding")
                    .short('e')
                    .long("encoding")
                    .takes_value(true)
                    .possible_values(&["ascii", "utf-8"])
                    .help("Output encoding"))
        .get_matches();

    let encoding = args.value_of("encoding").unwrap_or("ascii");
    println!("Encoding is {}", encoding);
    println!("Hello, world!");
    println!("Conversion: [{}]", convert().unwrap());
}
