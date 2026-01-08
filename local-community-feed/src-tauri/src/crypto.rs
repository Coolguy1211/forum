use ed25519_compact::{KeyPair, PublicKey, Signature};
use names::Generator;

pub struct Identity {
    pub keypair: KeyPair,
    pub username: String,
}

impl Identity {
    pub fn new() -> Self {
        let keypair = KeyPair::from_seed(Default::default());
        let mut generator = Generator::default();
        let username = generator.next().unwrap();
        Identity { keypair, username }
    }

    pub fn sign(&self, message: &str) -> Signature {
        self.keypair.sk.sign(message, None)
    }
}

pub fn verify(message: &str, signature: &Signature, public_key: &PublicKey) -> bool {
    public_key.verify(message, signature).is_ok()
}
