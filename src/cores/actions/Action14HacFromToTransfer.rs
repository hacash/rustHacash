
// create struct Action14HacFromToTransfer
action_create_struct_for_common_items!(
    ACTION_KIND_14, Action14HacFromToTransfer,

	from_address,   Address,
	to_address,     Address,
	amount,         Amount,
);



impl_Action_trait_for_common!( Action14HacFromToTransfer, self,
    { false }, // is_burning_90_persent_tx_fee

    {   // request_sign_addresses
        [self.from_address.clone()]
    }, 
    
    state, _trs, {

        operate::hac_transfer(state, &self.from_address, &self.to_address, &self.amount)
    }
);

