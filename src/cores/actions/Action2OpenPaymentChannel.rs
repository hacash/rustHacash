
// create struct Action2OpenPaymentChannel
action_create_struct_for_common_items!(
    ACTION_KIND_2, Action2OpenPaymentChannel,

	channel_id    , ChannelId, 
    left_bill      , AddrHac,
    right_bill     , AddrHac,
);


impl_Action_trait_for_common_single!( Action2OpenPaymentChannel, state, _trs, self, {
   
    let cid = &self.channel_id;
    let mut reuse_version = Uint4::from(1);
    // check exist
	// channel ID with the same left and right addresses and closed by consensus can be reused
    let retexisterr = ||{ Err(format!("channel {} openning or cannot reuse.", cid.to_hex())) };
    let havcha = state.get_channel(cid) ? ;
    if let Some(chan) = havcha {
        let chan_stat = *chan.get_status();
        let samebothaddr = ( self.left_bill.address ==  chan.left_bill.address) 
                        && (self.right_bill.address == chan.right_bill.address);
        if CHANNEL_STATUS_AGREEMENT_CLOSED != chan_stat || !samebothaddr {
            // exist or cannot reuse
            return retexisterr()
        }
        reuse_version = chan.reuse_version.add(1);
    }
    // open or reuse
	// channel ID validity
    if cid.len() != CHANNEL_ID_SIZE || cid[0] == 0u8 || cid[CHANNEL_ID_SIZE-1] == 0u8 {
        return Err(format!("channel id {} format error.", cid.to_hex()))
    }

    // todo todo todo todo todo





    Ok(false)
});

