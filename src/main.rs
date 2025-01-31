use::rsa::{pkcs1::{DecodeRsaPublicKey, DecodeRsaPrivateKey, EncodeRsaPrivateKey, EncodeRsaPublicKey}, pkcs8::LineEnding, Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey };
use std::{fs, io::{Read, Write}};

mod args;
use args::Action;

fn main() {
    let args = args::parse_args()
        .expect(&args::get_usage_message());




    let data = match args.action.clone() {
        Action::Encrypt (keystring) =>Some(encrypt(&keystring, &args.target)),
        Action::Decrypt (keystring) => Some(decrypt(&keystring, &args.target)),
        Action::CreateKeys => {
            create_keys(&args.target);
            None
        },
    };

    match data {
        None => {},
        Some(data) => {
            let file_extension = match args.action.clone() {
                Action::Encrypt(_) => "bin",
                Action::Decrypt(_) => "txt",
                Action::CreateKeys => panic!("How.")
            };

            fs::File::create(format!("./out.{file_extension}"))
                .expect("Failed to create output file")
                .write_all(&data)
                .expect("Failed to write output file");
        }
    }
}

fn encrypt(keystring: &str, file_path: &String) -> Vec<u8> {
    let public_key = RsaPublicKey::read_pkcs1_pem_file(&keystring)
    .expect("Failed to read public key");

    println!("path: {}", file_path);
    
    let content = read_file_to_bytes(&file_path);
    let target_buffer = content.as_slice();


    

    let mut rng = rand::thread_rng();

    public_key.encrypt(&mut rng, Pkcs1v15Encrypt, target_buffer)
        .expect("Failed to encrypt")

    
}

fn decrypt(keystring: &str, file_path: &String) -> Vec<u8> {
    let private_key = RsaPrivateKey::read_pkcs1_pem_file(keystring)
        .expect("Failed to read private key");  

    let content = read_file_to_bytes(&file_path);
    let target_buffer = content.as_slice();

    

    // println!("{:?}", private_key.to_pkcs1_pem(LineEnding::LF).unwrap());

    private_key.decrypt(Pkcs1v15Encrypt, target_buffer)
        .expect("Failed to decrypt")
}

fn create_keys(target: &str) {
    let mut rng = rand::thread_rng();
    let private_key = RsaPrivateKey::new(&mut rng, 2048)
        .expect("Failed to generate private key");


    let public_key = private_key.to_public_key();

    public_key.write_pkcs1_pem_file(format!("{target}/public.pem"), LineEnding::LF)
        .expect("Failed to create public key file");

    private_key.write_pkcs1_pem_file(format!("{target}/private.pem"), LineEnding::LF)
        .expect("Failed to create private key file");
}

fn read_file_to_bytes(file_path: &str) -> Vec<u8> {
    let mut file = fs::File::open(file_path)
        .expect("Failed to open file");

    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer)
        .expect("Failed to read file");


    buffer
}

#[cfg(test)]
mod tests;