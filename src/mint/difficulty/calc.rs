
lazy_static! {

    static ref PREV_288_BLOCK_TIMESTAMP_CACHE: Mutex<HashMap<u64, u64>> = {
        let mut m: HashMap<u64, u64> = HashMap::new();
        Mutex::new(m)
    };
    
}



/**
 * calculate next block difficulty
 */
pub fn calculate_next_difficulty(state: &dyn ChainStateRead, curblk: &BlockHeadMeta) -> Result<(Hash, u32), String> {

    let curblkhei = curblk.head.height.value();
    if curblkhei % TARGET_DIFFICULTY_ADJUST_BLOCKS != 0 {
        
    }





    Err("".to_string())
}





fn read_prev_288_block_timestamp(state: &dyn ChainStateRead, curblkhei: u64) -> Result<u64, String> {

    if curblkhei <= TARGET_DIFFICULTY_ADJUST_BLOCKS {
        return Ok(genesis::genesis_block().get_timestamp().value())
    }
    
    let prev288hei = curblkhei / TARGET_DIFFICULTY_ADJUST_BLOCKS * TARGET_DIFFICULTY_ADJUST_BLOCKS;

    let cache = PREV_288_BLOCK_TIMESTAMP_CACHE.lock().unwrap()[&prev288hei];
    if cache > 0 {
        return Ok(cache)
    }

    let blkhash = state.get_block_refer(&BlockHeight::from(curblkhei)) ? ;
    if let None = blkhash {
        return Err(format!("block {} not find in state!", curblkhei))
    }
    let blkhash = blkhash.unwrap();
    let blkbytes = state.get_block_bytes(&blkhash) ? ;
    if let None = blkbytes {
        return Err(format!("block {} - {} not find in state!", curblkhei, blkhash))
    }
    let blkbytes = blkbytes.unwrap();
    // parse block
    let (blkhead, _) = BlockHead::parse(blkbytes.get_bytes(), 0) ? ;
    // cache
    let timestamp = blkhead.timestamp.value() as u64;
    PREV_288_BLOCK_TIMESTAMP_CACHE.lock().unwrap().insert(prev288hei, timestamp);
    return Ok(timestamp)
}
