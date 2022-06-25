// "0WTYUIAHXVMEKBSZN"
pub static DIAMOND_HASH_BASE_CHARS: [u8; 17] = *b"0WTYUIAHXVMEKBSZN";

fn search_dhxchar(c: u8) -> u8 {
    for i in 1..17 {
        if c == DIAMOND_HASH_BASE_CHARS[i] {
            return c;
        }
    }
    0
}

pub fn check_diamond_hash_result(stuff: impl AsRef<[u8]>) -> (bool, Option<[u8; 6]>) {
    let hxval = stuff.as_ref().to_vec();
    if hxval.len() != 16 {
        return (false, None)
    }
    for i in 0..10 {
        if hxval[i] != b'0' {
            return (false, None)
        }
    }
    for i in 10..16 {
        if hxval[i] == b'0' {
            return (false, None)
        }
        if search_dhxchar(hxval[i]) <= 0 {
            return (false, None)
        }
    }
    // ok
    (true, Some(hxval[10..16].try_into().unwrap()))
}

pub fn check_diamond_difficulty(number: u32, sha3hx: &[u8; 32], diareshx: &[u8; 32]) -> bool {
    const MODIFFBITS: [u8; 32] = [// difficulty requirements
        128, 132, 136, 140, 144, 148, 152, 156, // step +4
        160, 164, 168, 172, 176, 180, 184, 188,
        192, 196, 200, 204, 208, 212, 216, 220,
        224, 228, 232, 236, 240, 244, 248, 252,
    ];
    // check step 1
	// referring to Moore's law, the excavation difficulty of every 42000 diamonds will double in about 2 years,
	// and the difficulty increment will tend to decrease to zero in 64 years
    // and increase the difficulty by 32-bit hash every 65536 diamonds
	let shnumlp = number as usize / 42000; // 32step max to 64 years
    let shmaxit = 255 - ((number / 65536) as u8);
    for i in 0..32 {
        if i < shnumlp && sha3hx[i] >= MODIFFBITS[i] {
            return false // fail
        }
        if sha3hx[i] > shmaxit {
            return false // fail
        }
    }
    // check step 2
	// every 3277 diamonds is about 56 days. Adjust the difficulty 3277 = 16 ^ 6 / 256 / 20
	// when the difficulty is the highest, the first 20 bits of the hash are 0, not all 32 bits are 0.
    let mut diffnum = number as usize / 3277;
    for a in diareshx {
        if diffnum < 255 {
			if (*a as usize) + diffnum > 255 {
				return false // difficulty check failed
			} else {
				return true // check success
			}
        }else if diffnum >= 255 {
            if *a != 0 {
                return false
            }
			// to do next round check
			diffnum -= 255;
        }
    }


    // over loop
    false
}

pub fn mine_diamond_hash_repeat(number: u32) -> i32 {
	let mut repeat = number/8192 + 1; // adjust the hashing times every 8192 diamonds (about 140 days and half a year)
	if repeat > 16 {
		repeat = 16; // atmost 16 round due to x16rs algorithm
	}
	return repeat as i32
}


pub fn diamond_hash(bshash: &[u8; 32]) -> [u8; 16] {
    let mut reshx = [0u8; 16];
    let mut mgcidx: u32 = 13; // index number and magic num
    for i in 0..16 {
        let num = mgcidx * (bshash[i*2] as u32) * (bshash[i*2 + 1] as u32);
        mgcidx = num % 17;
        reshx[i] = DIAMOND_HASH_BASE_CHARS[mgcidx as usize];
        if mgcidx == 0 {
            mgcidx = 13;
        }
    } 
    // ok
    reshx
}


pub fn mine_diamond(number: u32, prevblockhash: &[u8; 32], nonce: &[u8; 8], address: &[u8; 21], 
    custom_message: impl AsRef<[u8]>) -> ([u8; 32], [u8; 32], [u8; 16]) {
    /* hash stuff */
    let stuff =  [ 
        prevblockhash.to_vec(),
        nonce.to_vec(),
        address.to_vec(),
        custom_message.as_ref().to_vec()
    ].concat();
    let repeat = mine_diamond_hash_repeat(number);
	/* get ssshash by sha3 algrotithm */
    let ssshash = sha3(stuff);
	/* get diamond hash value by HashX16RS algorithm */
    let reshash = x16rs_hash_wrap(repeat, &ssshash);
	/* get diamond name by DiamondHash function */
    let diastr = diamond_hash(&reshash);
    // ok
    (ssshash, reshash, diastr)
}






