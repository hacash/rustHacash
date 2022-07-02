// use hacash::cores::field::*;
// use hacash::interface::Field;

// use hacash::x16rs::x16rs_hash_wrap;


use hacash::mint::validnode::*;

fn main() {


    // let mut addr = Address::new([1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1]);
    // let bytes = addr.serialize();

    // let bt2 = vec![0u8];
    // let res = addr.parse(&bt2, 0);
    // res.unwrap();

    // let indata = [0u8; 32];
    // loop{
    //     let _ = x16rs_hash_wrap(100, &indata);
    // }


    let datadir = "/home/jji/hacash/rust/rustHacash/test_data".to_string();
    let blkurl = "http://127.0.0.1:33381/query?action=blockdatahex&body=1&id=".to_string();

    let ins = create(&datadir);
    if let Err(e) = ins {
        panic!("{}", e)
    }
    let node = ins.unwrap();
    // start
    let res = node.start(&blkurl);
    if let Err(e) = res {
        panic!("{}", e)
    }





}
