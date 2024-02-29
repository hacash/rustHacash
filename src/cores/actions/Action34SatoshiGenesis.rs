
// create struct Action34SatoshiGenesis
action_create_struct_for_common_items!(
    ACTION_KIND_34, Action34SatoshiGenesis,

	transfer_no                      , Uint4                  , // Transfer serial number
	bitcoin_block_height             , Uint4                  , // Height of bitcoin block transferred
	bitcoin_block_timestamp          , BlockTxTimestamp       , // Bitcoin block timestamp of transfer
	bitcoin_effective_genesis        , Uint4                  , // The number of bitcoins successfully transferred before this
	bitcoin_quantity                 , Uint4                  , // Number of bitcoins transferred in this transaction (unit: piece)
	release_total_hac_mei            , Uint4                  , // The total amount of hac that should be release in this transfer (unit: mei)
	origin_address                   , Address                , // Bitcoin source address transferred out
	bitcoin_transfer_hash            , Hash                   , // Bitcoin transfer transaction hash
);


impl_Action_trait_for_common_single!( Action34SatoshiGenesis, self, state, trs, {
   
    let txactnum = trs.get_action_count().get_value();
    if txactnum > 1 {
        return Err(format!("Satoshi Genesis tx need only one action but got {} actions.", txactnum))
    }
	// Check the hac release that has been recorded (to avoid repeated execution of the completed hac release)
    let btcmvhx = state.get_btcmove_txhash(&self.transfer_no) ? ;
    if let Some(_) = btcmvhx {
		return Err(format!("Satoshi Genesis transfer number {} has been executed.", self.transfer_no))
    }
    // check data
    if state.is_check_btcmove() {
        let movelog = state.get_satoshi_genesis(&self.transfer_no) ? ;
        if let None = movelog {
            return Err(format!("Satoshi Genesis transfer number {} not find.", self.transfer_no))
        }
        let movelog = movelog.unwrap();
        // compare
        if movelog.transfer_no != self.transfer_no || 
            movelog.bitcoin_block_height != self.bitcoin_block_height || 
            movelog.bitcoin_block_timestamp != self.bitcoin_block_timestamp || 
            movelog.bitcoin_effective_genesis != self.bitcoin_effective_genesis || 
            movelog.bitcoin_quantity != self.bitcoin_quantity || 
            movelog.release_total_hac_mei != self.release_total_hac_mei || 
            movelog.origin_address != self.origin_address || 
            movelog.bitcoin_transfer_hash != self.bitcoin_transfer_hash
         {
			return Err("Action_34_SatoshiGenesis act and state.log() check act is mismatch.".to_string())
        }
        // check prev log
        if self.transfer_no > 1 {
            let prevno = self.transfer_no - 1;
            let prevlog =  state.get_satoshi_genesis(&prevno) ? ;
            if let None = prevlog {
                return Err(format!("Satoshi Genesis transfer number {} not find.", self.transfer_no))
            }
            let prevlog = prevlog.unwrap();
            if prevlog.transfer_no != prevno ||
                prevlog.bitcoin_block_height > self.bitcoin_block_height ||
                prevlog.bitcoin_block_timestamp > self.bitcoin_block_timestamp ||
                prevlog.bitcoin_effective_genesis + prevlog.bitcoin_quantity != self.bitcoin_effective_genesis
            {
                return Err(format!("Satoshi Genesis prev log mismatch."))
            }
        }else{
            if self.bitcoin_effective_genesis != 0 {
                return Err(format!("Satoshi Genesis effective number mismatch."))
            }
        }
        // quantity
		if self.bitcoin_quantity < 1 && self.bitcoin_quantity > 10000 {
			return Err("SatoshiGenesis act BitcoinQuantity number is error (right is 1 ~ 10000).".to_string())
		}
        // hac mei
        let rlsmei = coinbase::calculate_hac_mei_release_by_btc_move(&self.bitcoin_effective_genesis, &self.bitcoin_quantity);
        if self.release_total_hac_mei != rlsmei {
			return Err(format!("SatoshiGenesis release_total_hac_mei need {} but got {}.", rlsmei, self.release_total_hac_mei))
            
        }
		// inspection time (28 days later)
        let btcmovetime = Utc.timestamp(self.bitcoin_block_timestamp.value() as i64, 0);
        let day28 = Duration::days(28);
        let livestarttime = btcmovetime + day28;
        if Utc::now() < livestarttime {
            return Err(format!("SatoshiGenesis submit tx time must over {}", livestarttime.to_string()))
        }
    }
    // total supply
    let mut ttsupply = state.get_total_supply() ? ;
    // release hac
    let ttrlsamt = Amount::from_mei_i64( self.release_total_hac_mei.value() as i64 ) ? ;
    let calc_lock_week_idx = self.bitcoin_effective_genesis.value() + 1;
    let (mut weekhei, weeklocks) = coinbase::calculate_lock_week_by_btc_move_one_idx( calc_lock_week_idx );
	// develop mode
	if state.is_debug_test_mode() {
		weekhei = 10 // develop mode, release ten blocks
	}
    if weeklocks > 0 {
        // create lock amount
		let wklkhacamt = Amount::from_mei_i64( self.release_total_hac_mei.value() as i64 / weeklocks as i64 ) ? ;
        let lockblsid = coinbase::gain_lockbls_id_for_btc_move( &self.transfer_no );
        let lockbls = LockblsItem{
            master_address: self.origin_address.clone(),
            effect_block_height: state.pending_block_height(),
            linear_block_number: Uint3::from(weekhei),
            linear_release_amount: wklkhacamt,
            total_lock_amount: ttrlsamt.clone(),
            balance_amount: ttrlsamt.clone()
        };
        state.set_lockbls(&lockblsid, &lockbls) ? ; // set lock amt
    } else {
        // release
        operate::hac_add(state, &self.origin_address, &ttrlsamt) ? ;
        ttsupply.btc_move_subsidy_unlock_successed += ttrlsamt.to_mei_unsafe();
    }
    // move btc unit: SAT     1BTC = 100000000SAT
    let satnum = self.bitcoin_quantity.value() as u64 * 1_0000_0000;
    operate::sat_add(state, &self.origin_address, &Uint8::from(satnum)) ? ;
    // set supply
    state.set_total_supply(&ttsupply)
});

