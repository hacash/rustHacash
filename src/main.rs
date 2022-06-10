// use hacash::cores::field::*;
// use hacash::interface::Field;

use hacash::x16rs::x16rs_hash_wrap;

fn main() {


    // let mut addr = Address::new([1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1]);
    // let bytes = addr.serialize();

    // let bt2 = vec![0u8];
    // let res = addr.parse(&bt2, 0);
    // res.unwrap();

    let indata = [0u8; 32];
    loop{
        let _ = x16rs_hash_wrap(100, &indata);
    }



}
