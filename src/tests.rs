use super::*;
extern crate rsa;
use rsa::{pkcs1::{DecodeRsaPrivateKey, DecodeRsaPublicKey}, Pkcs1v15Encrypt, RsaPublicKey}; 


#[test]
fn file_reading_decrypt() {
    let private_key = fs::read_to_string("out/private.pem")
        .expect("Failed to read private key file");

    let private_key = rsa::RsaPrivateKey::from_pkcs1_pem(&private_key)
        .expect("Invalid private key");

    let enc_file = fs::File::open("out/out.cif")
        .expect("Failed to open encrypted file");

    let enc_data: Vec<u8> = enc_file.bytes()
        .map(|b| b.unwrap())
        .collect();

    let decrypted = private_key
        .decrypt(Pkcs1v15Encrypt, &enc_data)
        .expect("Failed to decrypt data");

    println!("Decrypted: {:?}", String::from_utf8(decrypted).unwrap());
}

#[test]
fn file_reading_encrypt() {
    let public_key_file = fs::read_to_string("out/public.pem")
        .expect("Failed to read public key file");

    // println!("File contents: {}", public_key_file);

    let public_key = RsaPublicKey::from_pkcs1_pem(&public_key_file)
        .expect("Failed to read public key");

    let file_to_encrypt = fs::read_to_string("out/file.txt")
        .expect("Failed to read target file");

    println!("pubkey: {}\nfile: {}", public_key.to_pkcs1_pem(LineEnding::LF).unwrap(), file_to_encrypt);
}

