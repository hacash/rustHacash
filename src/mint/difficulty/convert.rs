// convert

fn uint32_to_big(v: u32) -> BigUint {
    panic!();
}


fn uint32_to_hash(v: u32) -> [u8; 32] {
    let diffbytes = u32::to_be_bytes(v);
    panic!();

    



}


/* 
func bytes_to_bits(stuff: []) []byte {
	results := make([]byte, 0, 32*8)
	for _, v := range stuff {
		results = append(results, ByteToBits(v)...)
	}
	return results
}
*/

// 256 to 32
fn bits_to_byte(bits: [u8; 8]) -> u8 {
	let mut byte = 0
	    + 1   * bits[7]
	    + 2   * bits[6]
	    + 4   * bits[5]
	    + 8   * bits[4]
	    + 16  * bits[3]
	    + 32  * bits[2]
	    + 64  * bits[1]
	    + 128 * bits[0]
        ;
	return byte
}


// 32 to 256
fn byte_to_bits(b: u8) -> [u8; 8] {
	return [
	    ((b >> 7) & 0x1),
	    ((b >> 6) & 0x1),
	    ((b >> 5) & 0x1),
	    ((b >> 4) & 0x1),
	    ((b >> 3) & 0x1),
	    ((b >> 2) & 0x1),
	    ((b >> 1) & 0x1),
	    ((b >> 0) & 0x1),
    ]
}
