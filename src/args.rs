use std::env;

#[derive(Clone, Debug, PartialEq)]
pub enum Action {
    CreateKeys,
    Encrypt(String),
    Decrypt(String, String),
}

#[derive(Clone, Debug)]
pub struct Args {
    pub action: Action,
    pub target: String,
    // pub algorythm: Algorythm,   
}

pub fn parse_args() -> Result<Args, &'static str> {
    let args: Vec<String> = env::args().collect();

    let actionstring = args.get(1);
    let target = args.get(2);
    let rsa_key_file = args.get(3);
    let aes_key_file = args.get (4);


    if actionstring == None || target == None {
        return Err("Invalid arguments");
    }


    /* 
    ENCRIPTACION ( LICITACION, RSAKEY){
    let data = fs::read(LICITACION).expect("Error al leer README.md");
    

    let key = aes::generate_key();
    aes::save_to_file(&key, "AesOut/claveRs.key");
    let mut data_to_encrypt = data.clone();

    let key_array: [u8; 32] = *key.as_ref();
    let encrypted = aes::encrypt(&key_array, &mut data_to_encrypt)
        .expect("Fallo al encriptar los datos");
    }






    


     */

    


    let action = match &actionstring.unwrap().to_ascii_lowercase()[..] {
        "-e" => Action::Encrypt(String::from(rsa_key_file.unwrap())), 
        "-d" => Action::Decrypt(String::from(rsa_key_file.unwrap()),String::from(aes_key_file.unwrap())),
        "-c" => Action::CreateKeys,
        _ => return Err("Invalid action")
    };

    Ok(Args {
        action,
        target: String::from(target.unwrap())
    })
}

pub fn get_usage_message() -> String {
    String::from(r#"
    Usage: encrypter [action] [target] [keyfile]

    actions:
        -d: Decrypt file 
        -e: Encrypt file
        -c: Generate public and private keys

    target:
        For Encryption and Decryption: file to encrypt/decrypt directory (Output goes to ./)
        For Key Generation: output directory (if same directory use ".")

    keyfile:
        Public/Private key directory
        (Only used in encryption and decryption)
    "#)
}


