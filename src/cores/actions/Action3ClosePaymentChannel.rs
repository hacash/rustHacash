
// create struct Action3ClosePaymentChannel
action_create_struct_for_common_items!(
    ACTION_KIND_3, Action3ClosePaymentChannel,

	channel_id    , ChannelId, 
);


impl_Action_trait_for_common_single!( Action3ClosePaymentChannel, self, state, trs, {
    // check status
    let cid = &self.channel_id;
    let chan = state.get_channel(cid) ? ;
    let chan = must_find!(chan, "channel") ? ;
    if chan.status != CHANNEL_STATUS_OPENING {
        return Err("channel status is not opening".to_string())
    }
	// verify two address sign
    trs.verify_need_signs(&vec![
        chan.left_bill.address,
        chan.right_bill.address
    ]) ? ;


    
    Ok(())
});

