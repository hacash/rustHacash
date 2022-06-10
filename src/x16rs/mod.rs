use sha3::{Digest, Sha3_256};
use sha2::{Sha256};
use ripemd::{Ripemd160};


extern "C" {
    fn x16rs_hash(a: i32, b: *const u8, c: *const u8) -> ();
}


pub fn x16rs_hash_wrap(loopnum: i32, indata: &[u8; 32]) -> [u8; 32] {

    let outdata = [0u8; 32];
    unsafe {
        // input hash
        let input: *const u8 = indata.as_ptr();

        // output hash
        let output: *const u8 = outdata.as_ptr();

        // do call
        x16rs_hash(loopnum, input, output);
        // println!("{:?}", outdata);
    }
    return outdata;
}


// sha3
pub fn sha3(stuff: impl AsRef<[u8]>) -> [u8; 32] {

    let mut hasher = Sha3_256::new();
    hasher.update(stuff);
    let result = hasher.finalize();
    let result: [u8; 32] = result[..].try_into().unwrap();
    result

}


// sha2
pub fn sha2(stuff: impl AsRef<[u8]>) -> [u8; 32] {

    let mut hasher = Sha256::new();
    hasher.update(stuff);
    let result = hasher.finalize();
    let result: [u8; 32] = result[..].try_into().unwrap();
    result

}


pub fn ripemd160(stuff: impl AsRef<[u8]>) -> [u8; 20] {

    let mut hasher = Ripemd160::new();
    hasher.update(stuff);
    // to [u8; 20]
    let result = hasher.finalize();
    let result: [u8; 20] = result[..].try_into().unwrap();
    result
}


