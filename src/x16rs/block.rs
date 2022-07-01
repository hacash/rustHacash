

pub fn block_height_hash_repeat(height: u64) -> i32 {
	let mut repeat = height/50000 + 1; 
	if repeat > 16 {
		repeat = 16;
	}
	return repeat as i32
}


pub fn block_hash(height: u64, stuff: impl AsRef<[u8]>) -> [u8; 32] {
    let repeat = block_height_hash_repeat(height);
    let reshash = calculate_hash(stuff);
    x16rs_hash_wrap(repeat, &reshash)
}

