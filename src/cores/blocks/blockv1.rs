

pub struct BlockV1 {

	// head
	version            : Uint1,
	height             : BlockHeight,
	timestamp          : BlockTxTimestamp,
	prev_hash          : Hash,
	mrkl_root          : Hash,
	transaction_count  : Uint4,

	// meta
	nonce              : Fixedbytes4, // Mining random value
	difficulty         : Uint4, // Target difficulty value
	witness_stage      : Fixedbytes2, // Witness quantity level

	// body
	transactions       : Vec<Box<dyn Transaction>>

}


impl_Field_for_common_block!( BlockV1,
	version            , Uint1,
	height             , BlockHeight,
	timestamp          , BlockTxTimestamp,
	prev_hash          , Hash,
	mrkl_root          , Hash,
	transaction_count  , Uint4,
	nonce              , Fixedbytes4, 
	difficulty         , Uint4, 
	witness_stage      , Fixedbytes2, 

);

impl BlockV1 {

    pub fn _serialize_head(&self) -> Vec<u8> {
        field_serialize_items_concat!(
            self.version,
            self.height,
            self.timestamp, 
            self.prev_hash,
            self.mrkl_root,
            self.transaction_count
        )
    }

    pub fn _serialize_head_meta(&self) -> Vec<u8> {
        let mut head = self._serialize_head();
        let mut meta = field_serialize_items_concat!(
            self.nonce,
            self.difficulty,
            self.witness_stage
        );
        head.append(&mut meta);
        head
    }


    // parse function
    pub_fn_field_parse_wrap_return!(BlockV1, {BlockV1::new()});
    
}



impl BlockRead for BlockV1 {

    fn hash(&self) -> Hash { 
        panic!("")
    } 

    fn copy_block_ptr(&self) -> BlockPtr {
        BlockPtr{
            height: self.get_height().clone(),
            hash:   self.hash(),
        }
    }

    fn copy_block_head(&self) -> BlockHead {
        BlockHead{
            version               : self.get_version().clone(),
            height                : self.get_height().clone(),
            timestamp             : self.get_timestamp().clone(),
            prev_hash             : self.get_prev_hash().clone(),
            mrkl_root             : self.get_mrkl_root().clone(),
            transaction_count     : self.get_transaction_count().clone(),
        }
    }

    /* */

	fn get_version(&self) -> &Uint1 {
        &self.version
    }
    fn get_height(&self) -> &BlockHeight {
        &self.height
    }
	fn get_timestamp(&self) -> &BlockTxTimestamp {
        &self.timestamp
    }
	fn get_prev_hash(&self) -> &Hash {
        &self.prev_hash
    }
	fn get_mrkl_root(&self) -> &Hash {
        &self.mrkl_root
    }
    fn get_transaction_count(&self) -> &Uint4 {
        &self.transaction_count
    }
	fn get_difficulty(&self) -> u32 {
        self.difficulty.value()
    }
	fn get_nonce(&self) -> &Fixedbytes4 {
        &self.nonce
    }
	fn get_nonce_num(&self) -> u32 {
        0
    }
    fn get_witness_stage(&self) -> &Fixedbytes2 {
        &self.witness_stage
    }
	fn get_transactions(&self) -> &Vec<Box<dyn Transaction>> {
        &self.transactions
    }

    /* */
}

impl Block for BlockV1 {

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