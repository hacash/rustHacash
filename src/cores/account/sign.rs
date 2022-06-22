

// signature
impl Account {

    pub fn verify_signature(msg: &[u8; 32], publickey: &[u8; 33], signature: &[u8; 64]) -> bool {
        if let Ok(pubkey) = PublicKey::parse_compressed(publickey) {
            if let Ok(sigobj) = Signature::parse_standard(signature) {
                return libsecp256k1::verify(
                    &Message::parse(msg),
                    &sigobj,
                    &pubkey,
                )
            }
        }
        false
    }

}
        