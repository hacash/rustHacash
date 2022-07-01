
// create struct Action29SubmitTimeLimit
action_create_struct_for_common_items!(
    ACTION_KIND_29, Action29SubmitTimeLimit,

	start_height    , BlockHeight, 
    end_height      , BlockHeight,
);


impl_Action_trait_for_common_single!( Action29SubmitTimeLimit, self, state, _trs, {
   
	let curblockhei = state.pending_block_height();

	// start check
	if self.start_height > 0 {
		if curblockhei < self.start_height {
			return Err(format!("The transaction cannot be submitted if the current block height is less than {}", self.start_height))
		}
	}

	// end check
	if self.end_height > 0 {
		if curblockhei > self.end_height {
			return Err(format!("The transaction cannot be submitted if the current block height is more than {}", self.end_height))
		}
	}

	// submit time check ok pass
    Ok(())
});

