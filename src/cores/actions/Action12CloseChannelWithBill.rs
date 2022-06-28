
// create struct Action12CloseChannelWithBill
action_create_struct_for_common_items!(
    ACTION_KIND_12, Action12CloseChannelWithBill,

	channel_id      , ChannelId, 
    left_bill       , AddrHacSat,
    right_bill      , AddrHacSat,
);



impl_Action_trait_for_common!( Action12CloseChannelWithBill, self,
    { false }, // is_burning_90_persent_tx_fee

    {   // request_sign_addresses
        [self.left_bill.address.clone(), 
        self.right_bill.address.clone()]
    }, 
    
    state, _trs, {

        if ! state.is_debug_test_mode() { 
            return Err("mainnet not yet".to_string()) 
        }
        let cid = &self.channel_id;
        check_vaild_store_item_key("lockbls", cid.value(), CHANNEL_ID_SIZE) ? ;
        // query
        let chan = state.get_channel(cid) ? ;
        let chan = must_find!(chan, "channel") ? ;
        // check
        if chan.left_bill.address != self.left_bill.address ||
        chan.right_bill.address != self.right_bill.address {
            return Err(format!("channel {} address not match", cid.to_hex())) 
        }
        // sat
        let left_sat = self.left_bill.hacsat.satoshi.to_real();
        let right_sat = self.right_bill.hacsat.satoshi.to_real();
        // close
        let is_final_closed = false;
        operate::close_channel_with_distribution(state, &cid, &chan,
            &self.left_bill.hacsat.amount, &self.right_bill.hacsat.amount, 
            &left_sat, &right_sat, is_final_closed
        )
    }
);

