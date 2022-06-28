
// create struct Action11SatoshiFromToTransfer
action_create_struct_for_common_items!(
    ACTION_KIND_11, Action11SatoshiFromToTransfer,

	from_address,     Address,
	to_address,       Address,
	satoshi,          Satoshi,
);


impl_Action_trait_for_common!( Action11SatoshiFromToTransfer, self,
    { false }, // is_burning_90_persent_tx_fee

    {   // request_sign_addresses
        [self.from_address.clone()]
    }, 
    
    state, _trs, {

        operate::sat_transfer(state, &self.from_address, &self.to_address, &self.satoshi)
    }
);

