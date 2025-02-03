use std::{fs::File, io::Write, path::Path};




use aes::Aes256;
use aes::cipher::{
    BlockEncrypt, BlockDecrypt, KeyInit,
    generic_array::{GenericArray, typenum::U32},
};
use rand::rngs::OsRng;
use rand::RngCore;
use rand::Rng;

/// Encripta los datos usando AES-256 en modo CBC con relleno PKCS7.
/// Genera un IV aleatorio, encripta los datos y devuelve el IV concatenado con el texto cifrado.
pub fn encrypt(key: &[u8; 32], data: &mut Vec<u8>) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let block_size = 16;
    // Aplicar padding PKCS7
    let pad_len = block_size - (data.len() % block_size);
    data.extend(std::iter::repeat(pad_len as u8).take(pad_len));

    // Generar un IV aleatorio
    let mut rng = rand::thread_rng();
    let mut iv = [0u8; 16];
    rng.fill(&mut iv);

    // Instanciar el cifrador AES-256 en modo ECB (usado internamente)
    let cipher = Aes256::new(GenericArray::from_slice(key));

    let mut prev = iv; // En CBC, el IV se usa como "bloque anterior" para el primer bloque
    let mut result = iv.to_vec(); // Prependemos el IV al resultado

    // Procesar cada bloque de 16 bytes
    for chunk in data.chunks(block_size) {
        let mut block = [0u8; 16];
        block.copy_from_slice(chunk);
        // Operación CBC: XOR con el bloque anterior (o IV para el primero)
        for i in 0..block_size {
            block[i] ^= prev[i];
        }
        // Encriptar el bloque: se usa el modo ECB 
        let mut block_ga = GenericArray::clone_from_slice(&block);
        cipher.encrypt_block(&mut block_ga);
        // El bloque encriptado se guarda como el nuevo "prev" y se añade al resultado
        prev.copy_from_slice(&block_ga);
        result.extend_from_slice(&block_ga);
    }

    Ok(result)
}

/// Desencripta los datos aplicando manualmente CBC y removiendo el padding PKCS7 sin usar métodos "padded".
pub fn decrypt(key: &[u8; 32], data: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let block_size = 16;
    // Separar el IV (primeros 16 bytes)
    if data.len() < block_size {
        return Err("Datos insuficientes".into());
    }
    let iv = &data[..block_size];
    let ciphertext = &data[block_size..];

    let cipher = Aes256::new(GenericArray::from_slice(key));

    let mut prev = iv;
    let mut plaintext = Vec::with_capacity(ciphertext.len());

    // Procesar cada bloque de 16 bytes
    for chunk in ciphertext.chunks(block_size) {
        if chunk.len() != block_size {
            return Err("Bloque de longitud incorrecta".into());
        }
        let mut block = GenericArray::clone_from_slice(chunk);
        // Desencriptar el bloque (ECB)
        cipher.decrypt_block(&mut block);
        let mut plain_block = [0u8; 16];
        // Operación inversa del CBC: XOR con el bloque previo (IV o ciphertext anterior)
        for i in 0..block_size {
            plain_block[i] = block[i] ^ prev[i];
        }
        plaintext.extend_from_slice(&plain_block);
        prev = chunk; // El bloque actual en ciphertext se vuelve el "prev" para el siguiente bloque
    }

    // Remover el padding PKCS7: el último byte indica la cantidad de bytes de padding
    if let Some(&pad_len) = plaintext.last() {
        let pad_len = pad_len as usize;
        if pad_len == 0 || pad_len > block_size || pad_len > plaintext.len() {
            return Err("Padding inválido".into());
        }
        // Verificar que los últimos pad_len bytes sean iguales a pad_len
        if !plaintext[plaintext.len()-pad_len..].iter().all(|&b| b as usize == pad_len) {
            return Err("Padding incorrecto".into());
        }
        plaintext.truncate(plaintext.len() - pad_len);
    }

    Ok(plaintext)
}

pub fn save_to_file(key: &GenericArray<u8, U32>, target: &str) {
    let path =Path::new(target);
    // Guardar la clave en un archivo .key
    let mut file = File::create(path).expect("No se pudo crear el archivo");
    file.write_all(&key).expect("No se pudo escribir en el archivo");
} 

pub fn generate_key() -> GenericArray<u8, U32> {
    let mut key  = [0u8; 32];
    let mut rng = OsRng;
    rng.fill_bytes(&mut key);
    GenericArray::clone_from_slice(&key)
}
