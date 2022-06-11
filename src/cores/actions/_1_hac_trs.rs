

// create struct Action1HacTrs
action_create_struct_for_common_items!(
    ACTION_KIND_1_HAC_TRS,
    Action1HacTrs,
	to_address, Address,
	amount, Amount
);


impl_Action_trait_for_common!( Action1HacTrs, 

    // is_burning_90_persent_tx_fee
    { false },

    // request_sign_addresses
    { vec![] },

    // write_in_chain_state
    // _state &mut dyn ChainStateOperation -> Result<Option<Box<dyn ChainState>>, String>
    _state, {
        println!("{}", "---++++++++++----------- impl Action1HacTrs pub fn write_in_chain_state");
        Ok(None) // do nothing
    }

);

