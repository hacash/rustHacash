
// create struct Action5DiamondTransfer
action_create_struct_for_common_items!(
    ACTION_KIND_2, Action5DiamondTransfer,

	diamond          , DiamondName   , 
    to_address       , Address       ,
);


impl_Action_trait_for_common_single!( Action5DiamondTransfer, self, state, trs, {
    let main_addr = trs.get_address();
    // move
    operate::hacd_move_one_diamond(state, main_addr, &self.to_address, &self.diamond) ? ;
    // update count
    operate::hacd_transfer(state, main_addr, &self.to_address, &DiamondNumber::from(1)) ? ;
    // ok
    Ok(())
});

