use super::*;
extern crate rsa;
use rsa::{pkcs1::DecodeRsaPrivateKey, Pkcs1v15Encrypt}; 

#[test]
fn file_writing() {
    let mut keys = crate::rsa::generate_keys();
    let data = b"Hello, World!";
    let encrypted = crate::rsa::encrypt(data, &mut keys)
        .expect("Failed to encrypt data");
    
    save(&encrypted, &keys);
}

#[test]
fn file_reading() {
    let private_key = fs::read_to_string("out/private.pem")
        .expect("Failed to read private key file");

    let private_key = rsa::RsaPrivateKey::from_pkcs1_pem(&private_key)
        .expect("Failed to read private key");

    let enc_file = fs::File::open("out/out.cif")
        .expect("Failed to open encrypted file");

    let enc_data: Vec<u8> = enc_file.bytes().map(|b| b.unwrap()).collect();

    let decrypted = private_key
        .decrypt(Pkcs1v15Encrypt, &enc_data)
        .expect("Failed to decrypt data");

    println!("Decrypted: {:?}", String::from_utf8(decrypted).unwrap());
}