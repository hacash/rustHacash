
pub fn gain_lockbls_id_for_btc_move(tno: &Uint4) -> LockblsId {
    let mut id = LockblsId::new();
    let data = tno.serialize();
    for i in 1..=4usize {
        let a = LOCKBLS_ID_SIZE - i;
        let b = data.len() - i;
        id[a] = data[b]; // copy
    }
    id
}

pub fn calculate_hac_mei_release_by_btc_move(bitcoin_effective_genesis: &Uint4, bitcoin_quantity: &Uint4) -> Uint4 {
    
    let mut tt_tac: u32 = 0;
    let start = bitcoin_effective_genesis.value() + 1;
    let end = start + bitcoin_quantity.value();
    for idx in  start .. end {
        tt_tac += calculate_hac_mei_release_by_btc_move_one_idx(idx);
    }
    Uint4::from(tt_tac)
}


pub fn calculate_hac_mei_release_by_btc_move_one_idx(idx: u32) -> u32 {
    let lvn = 21;
    if idx == 1 {
        return powf2(lvn - 1)
    }
    if idx > powf2(lvn)-1 {
		return 1 // Finally, always issue an additional one
    }
    let mut tarlvn = 0;
    for i in 0..lvn {
        let l = powf2(i) - 1;
        let r = powf2(i+1) - 1;
        if idx > l && idx <= r {
            tarlvn = i + 1;
            break
        }
    }
    return powf2(lvn - tarlvn)
}


pub fn calculate_lock_week_by_btc_move_one_idx(idx: u32) -> (u32, u32) {

	let oneweekhei: u32 = 2000;   // 2000 / 288 = 6.9444 day
	let mostlockweek: u32 = 1024; // 1024 weeks is about 20 years
	if idx == 1 {
		return (mostlockweek, oneweekhei)
	}
    // loop calc
    let lvn = 21;
	let mut lockweek = mostlockweek;
    for i in 0..lvn {
        let l = powf2(i) - 1;
        let r = powf2(i+1) - 1;
        if idx > l && idx <= r {
            break
        }
		lockweek /= 2;
		if lockweek == 0 {
			return (0, oneweekhei)
		}
    }
    return (lockweek, oneweekhei)
}

fn powf2(n: u32) -> u32 {
	2u32.pow(n)
}
