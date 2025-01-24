use::rsa::{pkcs1::{EncodeRsaPrivateKey, EncodeRsaPublicKey}, pkcs8::LineEnding, };
use std::{fs, io::{Read, Write}};

mod rsa;

fn main() {
    print!("Main function");
}


fn save(encrypted: &Vec<u8>, keys: &rsa::Keys) {
    let _ = fs::create_dir("out");

    fs::File::create("out/out.cif")
        .expect("Failed to create digest file")
        .write(&encrypted)
        .expect("Failed to write data to file");

    fs::File::create("out/private.pem")
        .expect("Failed to create private key file")
        .write(keys.private.to_pkcs1_pem(LineEnding::LF).unwrap().as_bytes())
        .expect("Failed to write private key to file");

    fs::File::create("out/public.pem")
        .expect("Failed to create public key file")
        .write(keys.public.to_pkcs1_pem(LineEnding::LF).unwrap().as_bytes())
        .expect("Failed to write public key to file");
}


#[cfg(test)]
mod tests;