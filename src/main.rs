
use std::{fs, io::{Read, Write}, path::Path};

mod algorithms;
mod args;
use algorithms::{aes, rsa::{self, save_to_file, RsaKeys}};
use args::Action;

fn main() {
    let args = args::parse_args()
        .expect(&args::get_usage_message());



    let buffer = if args.action != Action::CreateKeys && !matches!(args.action, Action::ClonePubKey(_)) {
        Some(read_file_to_bytes(&args.target))
    } else {
        None
    };





    let data = match args.action.clone() {
        Action::Encrypt (rsa_key_path) =>{
            let public_key = RsaKeys::read_pubkey(&rsa_key_path);
            let key = aes::generate_key();

            let key_aes: [u8; 32] = *key.as_ref();
            let encrypted_file = aes::encrypt(&key_aes, &mut buffer.unwrap())
                .expect("Fallo al encriptar los datos");

            let aeskey_encrypted = rsa::encrypt(&public_key, &key_aes.as_slice());
            fs::write("./aeskey_encrypted.bin", &aeskey_encrypted)
            .expect("Error al escribir el archivo encriptado");
        
            Some(encrypted_file)


        },
        Action::Decrypt (rsa_key_path, aes_key_path) =>{
            let private_key = RsaKeys::read_privkey(&rsa_key_path);
            let aes_key_encrypted = fs::read(aes_key_path).expect("Error al leer la llave");
            
            let aes_key = rsa::decrypt(&private_key, &aes_key_encrypted);
            let key_array: [u8; 32] = aes_key.try_into().expect("expected vec of differnet lengjht");
            let decrypted = aes::decrypt(&key_array, (buffer.unwrap()).as_slice())
            .expect("Fallo al desencriptar los datos");
            
            

    
            Some(decrypted)
        } 
        Action::CreateKeys => {
            RsaKeys::new().save_to_file(&args.target);
            None
        },
        Action::ClonePubKey (rsa_key_path) => {
            let private_key = RsaKeys::read_privkey(&rsa_key_path);
            let public_key = private_key.to_public_key();
            save_to_file(&public_key, &args.target);
            None

        }
    };

    match data {
        None => {},
        Some(data) => {
            let file_extension = match args.action.clone() {
                Action::Encrypt(_) => "bin",
                Action::Decrypt(_,_) => "txt",
                Action::CreateKeys => panic!("How."),
                Action::ClonePubKey(_) =>  panic!("How.x2")
            };

            fs::File::create(format!("./out.{file_extension}"))
                .expect("Failed to create output file")
                .write_all(&data)
                .expect("Failed to write output file");
        }
    }
}

fn read_file_to_bytes(file_path: &str) -> Vec<u8> {
    let path = Path::new(file_path);

    let mut file = fs::File::open(path)
        .expect("Failed to open file");

    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer)
        .expect("Failed to read file");

    buffer
}

#[cfg(test)]
mod tests;