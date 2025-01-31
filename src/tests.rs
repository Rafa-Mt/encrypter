use super::*;
extern crate rsa;


#[test]
fn full_process_no_save() {

    //crea las llaves en out y un archivo file.txt con el contenido "mensaje encriptado"
    create_keys("./out");

    //mensaje encriptado
    let message = b"mensaje encriptado";

    let public_key_path = "./out/public.pem";
    let private_key_path = "./out/private.pem";

    let encrypted = encrypt(&public_key_path, message);
    let decrypted = decrypt(&private_key_path, &encrypted);

    assert_eq!(message, decrypted.as_slice());



}