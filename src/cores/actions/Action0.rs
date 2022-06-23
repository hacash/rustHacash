
// create struct ________
action_create_struct_for_common_items!(
    ACTION_KIND_2, ________,

	channel_id    , ChannelId, 
    leftbill      , AddrHac,
);


impl_Action_trait_for_common_single!( Action2OpenPaymentChannel, self, state, trs, {
   


    
    Ok(())
});

