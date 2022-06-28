
// create struct Action28SatoshiFromTransfer
action_create_struct_for_common_items!(
    ACTION_KIND_28, Action28SatoshiFromTransfer,

	from_address,   Address,
	satoshi,        Satoshi,
);


impl_Action_trait_for_common!( Action28SatoshiFromTransfer, self,
    { false }, // is_burning_90_persent_tx_fee

    {   // request_sign_addresses
        [self.from_address.clone()]
    }, 
    
    state, trs, {

        let to_address = trs.get_address();
        return operate::sat_transfer(state, &self.from_address, to_address, &self.satoshi)

    }
);

