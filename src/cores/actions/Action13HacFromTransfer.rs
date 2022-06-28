
// create struct Action13HacFromTransfer
action_create_struct_for_common_items!(
    ACTION_KIND_13, Action13HacFromTransfer,

	from_address,   Address,
	amount,         Amount,
);


impl_Action_trait_for_common!( Action13HacFromTransfer, self,
    { false }, // is_burning_90_persent_tx_fee

    {   // request_sign_addresses
        [self.from_address.clone()]
    }, 
    
    state, trs, {

        let to_address = trs.get_address();
        operate::hac_transfer(state, &self.from_address, to_address, &self.amount)

    }
);

