
// create struct Action2OpenPaymentChannel
action_create_struct_for_common_items!(
    ACTION_KIND_2, Action2OpenPaymentChannel,

	channel_id    , ChannelId, 
    left_bill      , AddrHac,
    right_bill     , AddrHac,
);


impl_Action_trait_for_common!( Action2OpenPaymentChannel, self,
    { false }, // is_burning_90_persent_tx_fee

    {   // request_sign_addresses
        [self.left_bill.address.clone(), 
        self.right_bill.address.clone()]
    }, 
    
    state, _trs, {
   
    let (cid, left_addr, left_amt, right_addr, right_amt ) = (
        &self.channel_id,
        &self.left_bill.address,
        &self.left_bill.amount,
        &self.right_bill.address,
        &self.right_bill.amount
    );

	// channel ID validity
    if cid.len() != CHANNEL_ID_SIZE || cid[0] == 0u8 || cid[CHANNEL_ID_SIZE-1] == 0u8 {
        return Err(format!("channel id {} format error.", cid.to_hex()))
    }
    // check format
    if left_addr == right_addr {
        return Err("left address cannot equal with right address".to_string())
    }
    if left_amt.size() > 6 || right_amt.size() > 6 {
        return Err("left or right amount bytes too long".to_string())
    }
    if left_amt.is_negative() || right_amt.is_negative() ||
        (left_amt.is_empty() && right_amt.is_empty()) {
        return Err("left or right amount is not positive or two both is empty".to_string())
    }

    // check exist
    let mut reuse_version = Uint4::from(1);
	// channel ID with the same left and right addresses and closed by consensus can be reused
    let havcha = state.get_channel(cid) ? ;
    if let Some(chan) = havcha {
        let chan_stat = *chan.get_status();
        let samebothaddr = ( left_addr ==  &chan.left_bill.address) 
                        && (right_addr == &chan.right_bill.address);
        if CHANNEL_STATUS_AGREEMENT_CLOSED != chan_stat || !samebothaddr {
            // exist or cannot reuse
            return Err(format!("channel {} is openning or cannot reuse.", cid.to_hex()))
        }
        reuse_version = chan.reuse_version.add(1);
    }
    // sub balance
    if left_amt.is_not_empty() {
        operate::hac_sub(state, left_addr,  left_amt) ? ;
    }
    if right_amt.is_not_empty() {
        operate::hac_sub(state, right_addr, right_amt) ? ;
    }
    // total supply
    let mut ttsply = state.get_total_supply() ? ;
    ttsply.channel_opening_count += 1;
    ttsply.channel_located_hac +=  left_amt.add(right_amt) ? .to_mei_unsafe() ;
    state.set_total_supply(&ttsply) ? ;
    // create channel
    let mut channel = ChannelItem::new();
    channel.status = CHANNEL_STATUS_OPENING;
    channel.reuse_version = reuse_version;
    channel.belong_height = state.pending_block_height();
    channel.arbitration_lock_block = Uint2::from(5000); // lock period is about 17 days
    channel.interest_attribution = CHANNEL_INTEREST_ATTRIBUTION_TYPE_DEFAULT; // 
    channel.left_bill = AddrHacSat{
        address: left_addr.clone(),
        hacsat: HacSat{amount: left_amt.clone(), satoshi: SatoshiOptional::new()},
    };
    channel.right_bill = AddrHacSat{
        address: right_addr.clone(),
        hacsat: HacSat{amount: right_amt.clone(), satoshi: SatoshiOptional::new()},
    };
    // save channel
    state.set_channel(cid, &channel) ? ;
    // ok
    Ok(())
});

