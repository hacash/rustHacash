
// create struct Action8SatoshiTransfer
action_create_struct_for_common_items!(
    ACTION_KIND_8, Action8SatoshiTransfer,

	to_address,     Address,
	satoshi,        Satoshi,
);


impl_Action_trait_for_common_single!( Action8SatoshiTransfer, self, state, trs, {
    let from_address = trs.get_address();
    return operate::sat_transfer(state, from_address, &self.to_address, &self.satoshi)
});

