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
