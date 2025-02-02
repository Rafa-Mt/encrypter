use super::*;
use algorithms;

#[test]
fn full_process_no_save() {

    use algorithms::rsa::{self, Keys};

    //crea las llaves en out y un archivo file.txt con el contenido "mensaje encriptado"
    let keys= rsa::Keys::new();
    keys.save_to_file("./out");
    //mensaje encriptado
    let message = b"mensaje encriptado";

    let public_key = Keys::read_pubkey("./out/public.pem");
    let private_key = Keys::read_privkey("./out/private.pem");

    let encrypted = rsa::encrypt(&public_key, message);
    let decrypted = rsa::decrypt(&private_key, &encrypted);

    assert_eq!(message, decrypted.as_slice());
}