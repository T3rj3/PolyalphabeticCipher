extern crate clap;
use clap::{Arg, App};
use std::fs::File;
use std::io::{Read, Write};

static KEY_FILE: &'static str = "key_file";
static PLAINTEXT_FILE: &'static str = "plaintext_file";
static CIPHERTEXT_FILE: &'static str = "ciphertext_file";
static OUTPUT_FILE: &'static str = "output_file";

fn app_builder() -> App<'static, 'static> {
    App::new("Encrypter")
            .arg(Arg::with_name(KEY_FILE)
                .short("k")
                .long("key_file")
                .value_name("FILE")
                .help("Specifies a keyfile")
                .takes_value(true)
                .required(true))
            .arg(Arg::with_name(PLAINTEXT_FILE)
                .short("p")
                .long("plaintext_file")
                .value_name("FILE")
                .help("Specifies a plaintext file")
                .takes_value(true)
                .conflicts_with(CIPHERTEXT_FILE)
                .required_unless(CIPHERTEXT_FILE))
            .arg(Arg::with_name(CIPHERTEXT_FILE)
                .short("c")
                .long("ciphertext_file")
                .value_name("FILE")
                .help("Specifies a ciphertext file")
                .takes_value(true)
                .conflicts_with(PLAINTEXT_FILE)
                .required_unless(PLAINTEXT_FILE))
            .arg(Arg::with_name(OUTPUT_FILE)
                .short("o")
                .long("output_file")
                .help("Specifies an output file")
                .takes_value(true)
                .required(true))
}

fn main() {
    let matches = app_builder().get_matches();
    let output_filename = matches.value_of(OUTPUT_FILE).unwrap();
    let mut output_file = File::create(output_filename).expect("Could not create output file");

    let key_filename = matches.value_of(KEY_FILE).unwrap();
    let mut key_file = File::open(key_filename).expect("Could not open keyfile")
    let mut key_buffer = Vec::new();

    key_file.read_to_end(&mut key_buffer).unwrap();

    match matches.value_of(PLAINTEXT_FILE) {
        Some (plaintext_filename) => {
            let mut plaintext_file = File::open(plaintext_filename).expect("Could not open plaintext file");
            let mut plaintext_buffer = Vec::new();

            plaintext_file.read_to_end(&mut plaintext_buffer).unwrap();

            for i in plaintext_buffer.iter_mut().zip(key_buffer.iter().cycle()) {
                *i.0 = (*i.0).wrapping_add(*i.1);
            }

            output_file.write_all(&plaintext_buffer[..]).unwrap();

        },
        None => {
            let ciphertext_filename = matches.value_of(CIPHERTEXT_FILE).unwrap();
            let mut ciphertext_file = File::open(ciphertext_filename).expect("Could not open ciphertext file");
            let mut ciphertext_buffer = Vec::new();

            ciphertext_file.read_to_end(&mut ciphertext_buffer).unwrap();

            for i in ciphertext_buffer.iter_mut().zip(key_buffer.iter().cycle()) {
                *i.0 = (*i.0).wrapping_sub(*i.1);
            }

            output_file.write_all(&ciphertext_buffer[..]).unwrap();

        }
    }

    println!("Finished.");
}
