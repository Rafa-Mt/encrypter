use rsa::pkcs1::DecodeRsaPrivateKey;
use::rsa::{pkcs1::{DecodeRsaPublicKey, EncodeRsaPrivateKey, EncodeRsaPublicKey}, pkcs8::LineEnding, Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey };
use std::{fs, io::Write};

mod args;
use args::Action;

fn main() {
    let args = args::parse_args()
        .expect(&args::get_usage_message());

    let target_buffer = args.target.as_bytes();

    let data = match args.action.clone() {
        Action::Encrypt (keystring) => Some(encrypt(&keystring, target_buffer)),
        Action::Decrypt (keystring) => Some(decrypt(&keystring, target_buffer)),
        Action::CreateKeys => {
            create_keys(&args.target);
            None
        },
    };

    match data {
        None => {},
        Some(data) => {
            let file_extension = match args.action.clone() {
                Action::Encrypt(_) => "cif",
                Action::Decrypt(_) => "txt",
                Action::CreateKeys => panic!("How.")
            };

            fs::File::create(format!("./out.{file_extension}"))
                .expect("Failed to create output file")
                .write(&data)
                .expect("Failed to write output file");
        }
    }
}

fn encrypt(keystring: &str, target_buffer: &[u8]) -> Vec<u8> {
    let public_key = RsaPublicKey::read_pkcs1_pem_file(&keystring)
    .expect("Failed to read public key");

    let mut rng = rand::thread_rng();

    public_key.encrypt(&mut rng, Pkcs1v15Encrypt, target_buffer)
        .expect("Failed to encrypt")
}

fn decrypt(keystring: &str, target_buffer: &[u8]) -> Vec<u8> {
    let private_key = RsaPrivateKey::read_pkcs1_pem_file(keystring)
        .expect("Failed to read private key");  

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


#[cfg(test)]
mod tests;