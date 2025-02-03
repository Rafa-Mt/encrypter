use std::fmt::{self, Debug, Formatter};

use::rsa::{
    pkcs1::{DecodeRsaPublicKey, DecodeRsaPrivateKey, EncodeRsaPrivateKey, EncodeRsaPublicKey}, 
    pkcs8::LineEnding, 
    Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey 
};
use rand;

pub struct RsaKeys {
    public: RsaPublicKey,
    private: RsaPrivateKey
}

impl RsaKeys {
    pub fn new() -> RsaKeys {
        let mut rng = rand::thread_rng();
        let private = RsaPrivateKey::new(&mut rng, 2048)
            .expect("Failed to generate private key");
    
        let public = private.to_public_key();
        RsaKeys {
            public,
            private
        }
    }

    pub fn save_to_file(&self, target: &str) {
        self.public.write_pkcs1_pem_file(&format!("{}/public.pem", target), LineEnding::LF)
            .expect("Failed to write public key");
        self.private.write_pkcs1_pem_file(&format!("{}/private.pem", target), LineEnding::LF)
            .expect("Failed to write private key");
    }

    pub fn read_pubkey(target: &str) -> RsaPublicKey {
        RsaPublicKey::read_pkcs1_pem_file(target)
            .expect("Failed to read public key")
    }

    pub fn read_privkey(target: &str) -> RsaPrivateKey {
        RsaPrivateKey::read_pkcs1_pem_file(target)
            .expect("Failed to read private key")
    }
}

impl Debug for RsaKeys {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Keys {{ \npublic: {:?}, \nprivate: {:?} \n}}", self.public.to_pkcs1_pem(LineEnding::LF), self.private.to_pkcs1_pem(LineEnding::LF))
    }
}

pub fn encrypt(public_key: &RsaPublicKey, buffer: &[u8]) -> Vec<u8> {
    let mut rng = rand::thread_rng();

    public_key.encrypt(&mut rng, Pkcs1v15Encrypt, buffer)
        .expect("Failed to encrypt")
}

pub fn decrypt(private_key: &RsaPrivateKey, buffer: &[u8]) -> Vec<u8> {
    private_key.decrypt(Pkcs1v15Encrypt, buffer)
        .expect("Failed to decrypt")
}