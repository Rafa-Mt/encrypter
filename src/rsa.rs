use rand::rngs::ThreadRng;

use rsa;

#[derive(Debug)]
pub struct Keys {
    pub private: rsa::RsaPrivateKey,
    pub public: rsa::RsaPublicKey,
    pub rng: ThreadRng
}

pub fn generate_keys() -> Keys {
    let mut rng = rand::thread_rng();
    let bits = 2048;
    let private_key = rsa::RsaPrivateKey::new(&mut rng, bits)
        .expect("Failed to generate private key");

    let public_key = rsa::RsaPublicKey::from(&private_key);
    Keys {
        private: private_key,
        public: public_key,
        rng: rng
    }

}

pub fn encrypt(data: &[u8], keys: &mut Keys) -> Vec<u8>{
    keys.public.encrypt(&mut keys.rng, rsa::Pkcs1v15Encrypt, data)
    .expect("Failed to encrypt data")
}
