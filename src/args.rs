use std::env;

#[derive(Clone, Debug, PartialEq)]
pub enum Action {
    CreateKeys,
    Encrypt(String),
    Decrypt(String, String),
    ClonePubKey(String),
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

    let action = match &actionstring.unwrap().to_ascii_lowercase()[..] {
        "-e" => Action::Encrypt(String::from(rsa_key_file.unwrap())), 
        "-d" => Action::Decrypt(String::from(rsa_key_file.unwrap()),String::from(aes_key_file.unwrap())),
        "-c" => Action::CreateKeys,
        "-k" => Action::ClonePubKey(String::from(rsa_key_file.unwrap())),
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


