
use std::{fs, io::{Read, Write}, path::Path};

mod algorithms;
mod args;
use args::Action;

fn main() {
    // let args = args::parse_args()
    //     .expect(&args::get_usage_message());

    // let buffer = if args.action != Action::CreateKeys {
    //     Some(read_file_to_bytes(&args.target))
    // } else {
    //     None
    // };

    // let data = match args.action.clone() {
    //     Action::Encrypt (keystring) =>Some(encrypt(&keystring, &buffer.unwrap().as_slice())),
    //     Action::Decrypt (keystring) => Some(decrypt(&keystring, &buffer.unwrap().as_slice())),
    //     Action::CreateKeys => {
    //         create_keys(&args.target);
    //         None
    //     },
    // };

    // match data {
    //     None => {},
    //     Some(data) => {
    //         let file_extension = match args.action.clone() {
    //             Action::Encrypt(_) => "bin",
    //             Action::Decrypt(_) => "txt",
    //             Action::CreateKeys => panic!("How.")
    //         };

    //         fs::File::create(format!("./out.{file_extension}"))
    //             .expect("Failed to create output file")
    //             .write_all(&data)
    //             .expect("Failed to write output file");
    //     }
    // }
}

fn read_file_to_bytes(file_path: &str) -> Vec<u8> {
    let path = Path::new(file_path);

    let mut file = fs::File::open(path)
        .expect("Failed to open file");

    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer)
        .expect("Failed to read file");

    buffer
}

#[cfg(test)]
mod tests;