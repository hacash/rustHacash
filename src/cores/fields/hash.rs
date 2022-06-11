
pub type Hash = Fixedbytes32;
pub type HashHalfChecker = Fixedbytes16;
pub type HashNonceChecker = Fixedbytes8;

const HASH_SIZE: usize = Hash::size();
const HASH_HALF_CHECKER_SIZE: usize = HashHalfChecker::size();
const HASH_NONCE_CHECKER_SIZE: usize = HashNonceChecker::size();


impl Hash {

    pub fn create(v: impl AsRef<[u8]>) -> Result<Hash, String> {
        let v = v.as_ref();
        if v.len() != HASH_SIZE {
            return Err("Hash.from() size error.".to_string())
        }
        let v: [u8; HASH_SIZE] = v.try_into().unwrap();
        Ok(Hash{
            bytes: v,
        })
    }

    pub fn half_checker(&self) -> HashHalfChecker {
        let pt: [u8; HASH_HALF_CHECKER_SIZE] = self.value()[0..HASH_HALF_CHECKER_SIZE].try_into().unwrap();
        HashHalfChecker::from(pt)
    }

    pub fn nonce_checker(&self) -> HashNonceChecker {
        let pt: [u8; HASH_NONCE_CHECKER_SIZE] = self.value()[0..HASH_NONCE_CHECKER_SIZE].try_into().unwrap();
        HashNonceChecker::from(pt)
    }

}