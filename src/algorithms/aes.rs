use aes::cipher::ArrayLength;
use aes::Aes256;
use aes::cipher::{
    BlockCipher, BlockEncrypt, BlockDecrypt, KeyInit,
    generic_array::{GenericArray, typenum::U32},
};
use rand::RngCore;

pub fn generate_key() -> GenericArray<u8, U32> {
    let mut key = [0u8; 32];
    let mut rng = rand::thread_rng();
    rng.fill_bytes(&mut key);
    GenericArray::clone_from_slice(&key)
}

pub fn encrypt(key: &GenericArray<u8, U32>, target_buffer: &Vec<u8>) -> Vec<u8> {
    let cipher = Aes256::new(key);
    let mut block = GenericArray::clone_from_slice(&target_buffer);
    cipher.encrypt_block(&mut block);
    block.to_vec()
}

pub fn decrypt(key: &GenericArray<u8, U32>, target_buffer: &Vec<u8>) -> Vec<u8> {
    let cipher = Aes256::new(key);
    let mut block = GenericArray::clone_from_slice(&target_buffer);
    cipher.decrypt_block(&mut block);
    block.to_vec()
}
