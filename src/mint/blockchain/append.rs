

pub fn append_block_fork_state(basestate: ArcMutexChainStateInstance, buffer: &Vec<u8>, seek: usize) -> Result<(ArcMutexChainStateInstance, Box<dyn Block>, usize), String> {
    // unbox state
    //  println!("---------------append_block_fork_state--1");
    let mut latest_block: BlockIntroItem;
    let mut needhei: BlockHeight;
    {
        let bastat = basestate.lock().unwrap();
        latest_block = bastat.get_latest_block_intro() ? ;
        needhei = bastat.pending_block_height() + 1;
    }
    //  println!("---------------append_block_fork_state--2");
    // parse block
    let (newblock, seek2) = blocks::parse(buffer, seek) ? ;
    let newblkhei = newblock.get_height().clone();
    let newblkhash = newblock.hash();
    // if first block
    if newblkhei == 1u64 {
        let gns = genesis::genesis_block();
        latest_block = BlockIntroItem{
            hash: gns.hash(),
            headmeta: gns.copy_block_headmeta()
        };
    }
    //  println!("---------------newblkhei--1");
    // error
    let reterr = |e|{ Err(format!("try insert append new block height-{}, hx-{} to chain error: {}", newblkhei, newblkhash, e)) };
    // height
    if newblkhei != needhei {
        return reterr(format!("state need height {} but got {}", needhei ,newblkhei))
    }
    // prev hash
    if latest_block.hash != *newblock.get_prev_hash() {
        return reterr(format!("block need prev hash {} but got {}", latest_block.hash ,newblock.get_prev_hash()))
    }
    //  println!("---------------newblkhei--2");
    // timestamp
    let timenow = Utc::now();
    let timenowstamp = timenow.timestamp();
    let blockstamp = newblock.get_timestamp().value() as i64;
    let prevstamp = latest_block.headmeta.head.timestamp.value() as i64;
    // now time
    if blockstamp > timenowstamp {
        let blocktime = Utc.timestamp(blockstamp, 0);
        return reterr(format!("block create timestamp cannot equal or more than now {} but got {}", timenow.to_string(), blocktime.to_string()))
    }
    // prev time
    if blockstamp <= prevstamp {
        let blocktime = Utc.timestamp(blockstamp, 0);
        let prevtime = Utc.timestamp(prevstamp, 0);
        return reterr(format!("block create timestamp cannot equal or less than prev time {} but got {}", prevtime.to_string(), blocktime.to_string()))
    }
    //  println!("---------------blockstamp--2");
    // tx count
    let txs = newblock.get_transactions();
    let txlen = txs.len();
    if txlen < 1 {
        return reterr(format!("block not included any transactions"))
    }
    let txlenck = newblock.get_transaction_count().value() as usize;
    if txlen != txlenck {
        return reterr(format!("transaction count wrong, accept {} but got {}", txlenck, txlen))
    }
    if txlen > SINGLE_BLOCK_MAX_TX_COUNT {
        return reterr(format!("transaction total count cannot overflow max {}", SINGLE_BLOCK_MAX_TX_COUNT))
    }
    //  println!("---------------tx count--2");
    // mkrl root
    let mkrlhash = newblock.mrklroot();
    //  println!("---------------mrklroot--2");
    let mkrlroot = newblock.get_mrkl_root();
    if mkrlhash != *mkrlroot {
        return reterr(format!("need block mkrl root {} but got {}", mkrlroot, mkrlhash))
    }
    //  println!("---------------mkrl root--2");
    // coinbase tx
    let coinbase_tx = &txs[0];
    if coinbase_tx.get_type() != TRANSACTION_TYPE_0_COINBASE {
        return reterr(format!("Not find coinbase tx in transactions at first"))
    }
    //  println!("---------------coinbase tx--2");
    // block reward
    let reward_amount = coinbase_tx.get_reward();
    let rwdamt = coinbase::block_coinbase_reward( newblkhei.value() );
    if *reward_amount != rwdamt {
        return reterr(format!("block coinbase reward need {} but got {}", rwdamt, rwdamt))
    }
    //  println!("---------------block reward--2");
    // difficulty
    /*****************************/
    // TODO:: CHECK HASH DIFFICULTY
    /*****************************/
    // signatures
    //  println!("---------------verify_all_signs--1");
    newblock.verify_all_signs() ? ;
    //  println!("---------------verify_all_signs--2");
    // tx type & time & size
    let mut total_tx_size: usize = 0;
    for i in 1..txlen { // over coinbase
        let tx = &txs[i];
        if tx.get_type() == TRANSACTION_TYPE_0_COINBASE {
            return reterr("tx type error: cannot use repeat coinbase tx in block".to_string())
        }
        if *tx.get_timestamp() > timenowstamp {
            let txtime = Utc.timestamp(tx.get_timestamp().value() as i64, 0);
            return reterr(format!("tx {} timestamp {} is not more than now {}", tx.hash(), txtime.to_string(), timenow.to_string()))
        }
        total_tx_size += tx.size();
    }
    // size max
	if total_tx_size > SINGLE_BLOCK_MAX_SIZE {
		return Err(format!("txs total size {} is overflow max size {}.", total_tx_size, SINGLE_BLOCK_MAX_SIZE))
	}

    // fork & write_in_chain_state
    //  println!("---------------fork_with_next_block--1");
    let newstate = ChainStateInstance::fork_with_next_block(basestate.clone(), newblock.as_ref()) ? ;
    //  println!("----------------fork_with_next_block-2");

    // block refer & bytes & status
    let usestate = newstate.clone();
    let mut usestate = usestate.lock().unwrap();
    let usestate = usestate.deref_mut();
    usestate.set_block_refer(&newblkhei, &newblkhash) ? ;
    let blkbytes = BytesMax4294967295::from_bytes(buffer[seek..seek2].to_vec()).unwrap();
    usestate.set_block_bytes(&newblkhash, &blkbytes) ? ;
    // latest
    let blkintro = BlockIntroItem {
        hash: newblkhash,
        headmeta: newblock.copy_block_headmeta(),
    };
    usestate.set_latest_block_intro(&blkintro) ? ;

    // println!("------------------------3");
    // setup
    if newblkhei == 1i32 {
        state_setup(usestate);
    }
    // insert success
    // println!("------------------------4");
    Ok((newstate, newblock, seek))
}