use super::*;
use algorithms;

#[test]
fn full_rsa() {
    use algorithms::rsa::{self, RsaKeys};

    //crea las llaves en out y un archivo file.txt con el contenido "mensaje encriptado"
    let keys= rsa::RsaKeys::new();
    keys.save_to_file("./out");
    //mensaje encriptado
    let message = b"mensaje encriptado";

    let public_key = RsaKeys::read_pubkey("./out/public.pem");
    let private_key = RsaKeys::read_privkey("./out/private.pem");

    let encrypted = rsa::encrypt(&public_key, message);
    let decrypted = rsa::decrypt(&private_key, &encrypted);

    assert_eq!(message, decrypted.as_slice());
}

#[test]
fn aes_no_save() {
    use algorithms::aes;
    let key = aes::generate_key();
    let message = b"mensaje encriptado";

    let encrypted = aes::encrypt(&key, &message.to_vec());
    print!("{:?}", key);
    let decrypted = aes::decrypt(&key, &encrypted);

    assert_eq!(decrypted.as_slice().len(), message.len());
}