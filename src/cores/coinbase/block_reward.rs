


/**
 * Currency release algorithm: 22 million in the first 66 years
 */
pub fn block_coinbase_reward_number(block_height: u64) -> u8 {
    let part1 = [1u8, 1, 2, 3, 5, 8];
    let part2 = [8u8, 5, 3, 2, 1, 1];
    let part3 = 1u8;
    let tbn1: u64 =  10_0000;
    let tbn2: u64 = 100_0000;
    let spx1: u64 = part1.len() as u64 * tbn1;
    let spx2: u64 = part2.len() as u64 * tbn2 + spx1;
    let mut basenum = block_height;
    if block_height <= spx1 {
        return part1[(basenum/tbn1) as usize]
    }
    if block_height <= spx2 {
        basenum -= spx1;
        return part2[(basenum/tbn2) as usize]
    }
    return part3
}

pub fn block_coinbase_reward(block_height: u64) -> Amount {
	let num = block_coinbase_reward_number(block_height);
	return Amount::new_coin(num)
}
