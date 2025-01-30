use std::env;

#[derive(Clone, Debug)]
pub enum Action {
    CreateKeys,
    Encrypt(String),
    Decrypt(String)
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
    let keyfile = args.get(3);

    if actionstring == None || target == None {
        return Err("Invalid arguments");
    }

    let action = match &actionstring.unwrap().to_ascii_lowercase()[..] {
        "-e" => Action::Encrypt(String::from(keyfile.unwrap())), 
        "-d" => Action::Decrypt(String::from(keyfile.unwrap())),
        "-c" => Action::CreateKeys,
        _ => return Err("Invalid action")
    };

    Ok(Args {
        action,
        target: String::from(target.unwrap())
    })
}