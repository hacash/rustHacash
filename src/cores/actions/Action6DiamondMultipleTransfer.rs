
// create struct Action6DiamondMultipleTransfer
action_create_struct_for_common_items!(
    ACTION_KIND_6, Action6DiamondMultipleTransfer,

    from_address       , Address                 ,
    to_address         , Address                 ,
    diamond_list       , DiamondListMax200       ,
);


impl_Action_trait_for_common_single!( Action6DiamondMultipleTransfer, self, state, _trs, {
   
    // check
    self.diamond_list.check() ? ;
    let dianum = self.diamond_list.get_count().get_value() as u32;
	if dianum > 200 {
		return Err("diamonds quantity cannot over 200".to_string())
	}
    //transfer
    for dianame in self.diamond_list.get_list() {
        // move
        operate::hacd_move_one_diamond(state, &self.from_address, &self.to_address, &dianame) ? ;
    }
    // update count
    operate::hacd_transfer(state, &self.from_address, &self.to_address, &DiamondNumber::from(dianum)) ? ;
    // ok
    Ok(())
});

