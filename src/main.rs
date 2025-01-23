use::rsa::{pkcs1::{EncodeRsaPrivateKey, EncodeRsaPublicKey}, pkcs8::LineEnding, Pkcs1v15Encrypt};

mod rsa;

fn main() {
    let mut keys = rsa::generate_keys();
    let data = b"Hello, World!";
    let encrypted = rsa::encrypt(data, &mut keys);
    // println!("Encrypted: {:?}", encrypted);
    // println!("Private: {:?}", keys.private.to_pkcs1_pem(LineEnding::LF).unwrap());
    // println!("Public: {:?}", keys.public.to_pkcs1_pem(LineEnding::LF).unwrap());

    let decrypted = keys.private.decrypt(Pkcs1v15Encrypt, &encrypted)
    .expect("Failed to decrypt data");
    println!("Decrypted: {:?}", String::from_utf8(decrypted).unwrap());
}
