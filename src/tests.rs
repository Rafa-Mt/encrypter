use super::*;
use algorithms;
use ::aes::{cipher::{generic_array::GenericArray, KeyInit}, Aes256};


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





/* #[test]
fn aes_create_keys() {
    use algorithms::aes;
    let key = aes::generate_key();
    aes::save_to_file(&key, "AesOut/claveRs.key");
} */

/* #[test]
fn test_text_encrypt_aes() {
    use algorithms::aes;
    // Leer el contenido original de "README.md"
    let original_data = fs::read("README.md").expect("Error al leer README.md");
    
    // Generar la clave AES de 256 bits
    let key = aes::generate_key();
    
    // Clonar los datos para encriptarlos
    let mut data_to_encrypt = original_data.clone();
    
    // Encriptar los datos
    let key_array: [u8; 32] = *key.as_ref();
    let encrypted = aes::encrypt(&key_array, &mut data_to_encrypt)
        .expect("Fallo al encriptar los datos");
    
    // Asegurarse de que el ciphertext difiera (después del IV) del contenido original
    // Se omite el IV (primeros 16 bytes) para la comparación
    assert_ne!(&encrypted[16..], &original_data[..std::cmp::min(original_data.len(), encrypted.len()-16)]);
    
    // Desencriptar los datos
    let key_array: [u8; 32] = *key.as_ref();
    let decrypted = aes::decrypt(&key_array, &encrypted)
        .expect("Fallo al desencriptar los datos");
    
    // Verificar que el texto desencriptado sea igual al original
    assert_eq!(original_data, decrypted);
}
 */



#[test]
fn test_text_encrypt_aes() {
    use std::fs;
    use std::io::Write;
    use algorithms::aes;

    // Leer el contenido original de "README.md"
    let original_data = fs::read("README.md").expect("Error al leer README.md");
    
    // Generar la clave AES de 256 bits
    let key = aes::generate_key();
    aes::save_to_file(&key, "AesOut/claveRs.key");
    
    // Clonar los datos para encriptarlos
    let mut data_to_encrypt = original_data.clone();
    
    // Encriptar los datos
    let key_array: [u8; 32] = *key.as_ref();
    let encrypted = aes::encrypt(&key_array, &mut data_to_encrypt)
        .expect("Fallo al encriptar los datos");
    
    // Guardar el archivo encriptado
    fs::write("AesOut/archivos_encrypted.bin", &encrypted)
        .expect("Error al escribir el archivo encriptado");
    
    // Asegurarse de que el ciphertext (sin IV) difiera del contenido original
    assert_ne!(
        &encrypted[16..],
        &original_data[..std::cmp::min(original_data.len(), encrypted.len()-16)]
    );
    
    // Desencriptar los datos
    let decrypted = aes::decrypt(&key_array, &encrypted)
        .expect("Fallo al desencriptar los datos");
    
    // Guardar el archivo desencriptado
    fs::write("AesOut/archibos_decrypted.txt", &decrypted)
        .expect("Error al escribir el archivo desencriptado");
    
    // Verificar que el texto desencriptado sea igual al original
    assert_eq!(original_data, decrypted);
}

