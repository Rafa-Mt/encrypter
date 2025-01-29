use std::fmt::Error;

use env; 

#[derive(Clone, Debug)]
pub enum Action {
    CreateKeys,
    Encrypt (String),
    Decrypt (String)
}

pub struct Args {
    pub action: Action,
    pub target: String,
    // pub algorythm: Algorythm,   
}

pub fn parse_args() -> Result<Args, Error> {
    let args: Vec<String> = env::args().collect();

    let actionstring = args.get(1);
    let target = args.get(2);
    let keyfile = args.get(3);

    if actionstring == None || target == None {
        Error
    }

    let action = match &actionstring.unwrap().to_ascii_lowercase() {
        "-e" => Action::Encrypt(keyfile.unwrap()), 
        "-d" => Action::Decrypt(keyfile.unwrap()),
        "-c" => Action::CreateKeys
    };

    Ok(Args {
        action,
        target: target.unwrap()
    })
}