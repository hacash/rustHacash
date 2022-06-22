
// create struct Action1HacTrs
action_create_struct_for_common_items!(
    ACTION_KIND_1, Action1HacTrs,

	to_address, Address,
	amount, Amount,
);


impl_Action_trait_for_common_single!( Action1HacTrs, state, trs, self, {
    let from_address = trs.get_address();
    return operate::hac_transfer(state, from_address, &self.to_address, &self.amount)
});

