

// define special field
pub_type_prefix_value_check_impl_Field_trait!(
    DiamondCreateCustomerMsg, 
    number, DiamondNumber, Fixedbytes32,
    3+32+8+21, { number.get_value() > coinbase::DIAMOND_ABOVE_NUMBER_OF_CREATE_BY_CUSTOM_MESSAGE as u64 }
);


// create struct Action4DiamondCreate
action_create_struct_for_common_items!(
    ACTION_KIND_4, Action4DiamondCreate,

	diamond              , DiamondName                 ,   // diamond literal wtyuiahxvmekbszn
	number               , DiamondNumber               ,   // diamond serial number for difficulty check
	prev_hash            , Hash                        ,   // previous block hash containing diamond
	nonce                , Fixedbytes8                 ,   // random number
	address              , Address                     ,   // account
	// customer message                                   
	custom_message       , DiamondCreateCustomerMsg<DiamondNumber, Fixedbytes32>      ,

);


impl_Action_trait_for_common!( Action4DiamondCreate, self, 
    
    // is_burning_90_persent_tx_fee
    {
        // Starting from the 30001st diamond, destroy 90% of the bidding cost
        self.number.value() > coinbase::DIAMOND_ABOVE_NUMBER_OF_BURNING90_PERCENT_TX_FEES
    },

    // request_sign_addresses
    { [] },
    
    state, trs, {
   
    let act_number = self.number.value();
    let act_prev_hash = self.prev_hash.value();
    let act_nonce = self.nonce.value();
    let act_address = self.address.value();
    let act_custom_message = self.custom_message.serialize();

    let pending_block_height = state.pending_block_height();
    let pending_block_hash = match state.pending_block_hash() {
        Some(hx) => hx,
        _ => Hash::new(),
    };

    // mine
    let (_, mediumhx, diahx) = x16rs::mine_diamond(act_number as u32, &act_prev_hash, &act_nonce, &act_address, &act_custom_message);

    // check
    let mut is_do_complete_check = true;
    if state.is_debug_test_mode() || state.is_database_rebuild_mode() {
        is_do_complete_check = false;
    }
    if is_do_complete_check {
        // do complete check
        let belongactionnum = trs.get_action_count().value();
        if belongactionnum != 1 {
            return Err(format!("diamond create tx need only one action but got {} actions", belongactionnum).to_string())
        }

        let pdblkhei = state.pending_block_height().value();
        if pdblkhei % 5 != 0 {
            return Err(format!("[#BACKTOPOOL] diamond must be in block height multiple of 5").to_string())
        }

        let prevdia = state.get_latest_diamond() ? ;
        let prevdianum = prevdia.number.value();
        if prevdianum + 1 != act_number {
            return Err(format!("diamond nember error, need {} but got {}", prevdianum + 1, act_number).to_string())
        }
        if prevdianum > 0 {
            if prevdia.contain_block_hash != self.prev_hash {
                return Err(format!("diamond prev_hash error, need {} but got {}", 
                    prevdia.contain_block_hash.to_hex(), self.prev_hash.to_hex(), ).to_string())
            }
        }
        // diamond str
        let (diaok, dianame) = x16rs::check_diamond_hash_result(diahx);
        if ! diaok {
            return Err(format!("diamond hash result {} not a valid diamond name", String::from_utf8(diahx.to_vec()).unwrap()).to_string())
        }
        let dianame = dianame.unwrap();
        let dn = DiamondName::from(dianame);
        if dn != self.diamond {
            return Err(format!("diamond name need {} but got {}", dn.to_string(), self.diamond.to_string() ).to_string())
        }
        // exist
        let existdia = state.get_diamond( &dn ) ? ;
        if let Some(_) = existdia {
            return Err(format!("diamond {} already exist", dn.to_string()).to_string())
        }
    }
    // read total supply
    let mut ttsupply = state.get_total_supply() ? ;
    // add hacd to user
    operate::hacd_add(state, &self.address, &DiamondNumber::from(1)) ? ;
    // save diamond
    let diaitem = DiamondItem{
        status: DIAMOND_STATUS_NORMAL,
        address: self.address.clone(),
    };
    state.set_diamond(&self.diamond, &diaitem) ? ;
    // save diamond refer
    state.set_diamond_refer(&self.number, &self.diamond) ? ;
    // gene
    let txbidfee = trs.get_fee();
    let visual_gene = coinbase::calculate_diamond_visual_gene(&self.number, &mediumhx, &diahx, &pending_block_hash, &txbidfee);
    // save diamond smelt
    let average_bid_burn = coinbase::calculate_diamond_average_bid_burn(&self.number, ttsupply.diamond_bid_burning_fee.value());
    let diasmelt = DiamondSmeltItem{
        diamond: self.diamond.clone(),
        number: self.number.clone(),
        contain_block_height: pending_block_height.clone(),
        contain_block_hash: pending_block_hash.clone(),
        prev_contain_block_hash: self.prev_hash.clone(),
        miner_address: self.address.clone(),
        bid_fee: txbidfee.clone(),
        nonce: self.nonce.clone(),
        custom_message: CustomMessageOptional::from(self.custom_message.to_real()),
        average_bid_burn: average_bid_burn,
        visual_gene: visual_gene
    };
    state.set_diamond_smelt(&self.diamond, &diasmelt) ? ;
    // update total supply
    // burn fee
    if self.is_burning_90_persent_tx_fee() {
        let mut burnfeeof90per = txbidfee.clone();
        if burnfeeof90per.unit() > 1 {
            burnfeeof90per.unit_sub(1);
        }
        ttsupply.diamond_bid_burning_fee += burnfeeof90per.to_mei_unsafe();
    }
    // minted diamond
    ttsupply.minted_diamond = self.number.clone();
    // update
    state.set_total_supply(&ttsupply)
});

