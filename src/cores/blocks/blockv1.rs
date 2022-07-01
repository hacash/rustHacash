

pub struct BlockV1 {

	// head meta
    pub headmeta           : BlockHeadMeta,

	// body
	pub transactions       : Vec<Box<dyn Transaction>>

}


impl_Field_for_common_block!( BlockV1,
	headmeta            , BlockHeadMeta,
);

impl BlockV1 {

    // parse function
    pub_fn_field_parse_wrap_return!(BlockV1, {BlockV1::new()});
    
}


fn mrkl_merge(list: Vec<Hash>) -> Vec<Hash> {
    let num = list.len();
    let mut res = vec![];
    for i in 0..num{
        let lh = list[i].to_vec();
        let rh = match i+1 < num {
            true => list[i+1].to_vec(),
            false => lh.clone(),
        };
        let hx = x16rs::calculate_hash(vec![lh, rh].concat());
        res.push(Hash::from(hx));
    }
    res
}


impl BlockRead for BlockV1 {

    fn hash(&self) -> Hash { 
        let stuff = self.headmeta.serialize();
        let hx = x16rs::block_hash(self.headmeta.head.height.value(), stuff);
        Hash::from(hx)
    } 

    fn mrklroot(&self) -> Hash {
        let mut list = vec![];
        for t in &self.transactions {
            list.push(t.hash_with_fee())
        }
        loop {
            if list.len() <= 1 {
                return list[0].clone()
            }
            list = mrkl_merge(list);
        }
    }

    fn copy_block_ptr(&self) -> BlockPtr {
        BlockPtr{
            height: self.get_height().clone(),
            hash:   self.hash(),
        }
    }

    fn copy_block_head(&self) -> BlockHead {
        self.headmeta.head.clone()
    }

    fn copy_block_headmeta(&self) -> BlockHeadMeta {
        self.headmeta.clone()
    }

    /* */

	fn get_version(&self) -> &Uint1 {
        &self.headmeta.head.version
    }
    fn get_height(&self) -> &BlockHeight {
        &self.headmeta.head.height
    }
	fn get_timestamp(&self) -> &BlockTxTimestamp {
        &self.headmeta.head.timestamp
    }
	fn get_prev_hash(&self) -> &Hash {
        &self.headmeta.head.prev_hash
    }
	fn get_mrkl_root(&self) -> &Hash {
        &self.headmeta.head.mrkl_root
    }
    fn get_transaction_count(&self) -> &Uint4 {
        self.headmeta.get_transaction_count()
    }
	fn get_difficulty(&self) -> u32 {
        self.headmeta.meta.difficulty.value()
    }
	fn get_nonce(&self) -> &Fixedbytes4 {
        &self.headmeta.meta.nonce
    }
	fn get_nonce_num(&self) -> u32 {
        0
    }
    fn get_witness_stage(&self) -> &Fixedbytes2 {
        &self.headmeta.meta.witness_stage
    }
	fn get_transactions(&self) -> &Vec<Box<dyn Transaction>> {
        &self.transactions
    }

    /* */
}

impl Block for BlockV1 {


    fn set_mrkl_root(&mut self, root: &Hash) {
        self.headmeta.head.mrkl_root = root.clone();
    }

    fn verify_all_signs(&self) -> Result<(), String> {
        for act in self.get_transactions() {
            act.verify_all_signs() ? ;
        }
        Ok(())
    }

	// change chain state
	fn write_in_chain_state(&self, state: &mut dyn ChainState) -> Result<(), String> {
        let txlist = self.get_transactions();
        let txlen = txlist.len();
        if txlen < 1 {
            return Err("coinbase tx not find".to_string())
        }
        let blkhei = self.get_height().value();
        let mut fee_total = Amount::new();
        let mut fee_total_received = Amount::new();
        // txs 
        for ti in 1..txlen {
            let tx = &txlist[ti];
            let txhx = tx.hash();
            if transactions::TRANSACTION_TYPE_0_COINBASE == tx.get_type() {
                return Err("tx type error".to_string())
            }
            // check tx exist
            let cctx = state.get_tx_contain(&txhx) ? ;
            if let Some(hei) = cctx {
                // problem repair: block 63448 contains the same transaction twice
                if blkhei != 63448 {
                    return Err(format!("tx {} is exist in block {}.", txhx.to_hex(), hei.get_height().value()))
                }
            }
            // save tx contain
            let mut cctxobj = ContainTxItem::new();
            cctxobj.set_height(BlockHeight::from_u64(blkhei));
            state.set_tx_contain(&txhx, &cctxobj) ? ;
            // tx call write_in_chain_state
            tx.write_in_chain_state(state) ? ;
            // fee statistics
            let feegot = tx.get_fee_of_miner_real_received();
            fee_total_received = fee_total_received.add(&feegot) ? ; // add up
            fee_total = fee_total.add( &tx.get_fee() ) ? ;
        }
        // coinbase call write_in_chain_state
        let cbtx = &txlist[0];
        if cbtx.get_type() != transactions::TRANSACTION_TYPE_0_COINBASE {
            return Err("coinbase tx type error".to_string())
        }
        cbtx.write_in_chain_state(state) ? ;
        let cbaddr = cbtx.get_address();
        // send block fee
        operate::hac_add(state, cbaddr, &fee_total_received) ? ;
        // update total supply
        if fee_total.not_equal(&fee_total_received) {
            let burnfee_blk = fee_total.sub(&fee_total_received) ? ;
            let mut ttcount = state.get_total_supply() ? ;
            let rlbfe = ttcount.total_burning_fee.value() + burnfee_blk.to_mei_unsafe(); // total burn fee
            ttcount.total_burning_fee = Float8::from(rlbfe);
            state.set_total_supply( &ttcount ) ? ;
        }
        Ok(())
    }

}