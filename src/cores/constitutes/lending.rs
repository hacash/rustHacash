

// LendingRedeemedData
pub_struct_define_for_common!(LendingRedeemedData, 
	// data if redeemed
	block_height                  , BlockHeight     , // Block height at redemption
	amount                        , Amount          , // Redemption amount
	address_if_other_redemption  , AddressOptional , // If it is a third party redemption, record the third party address
);

pub_struct_define_for_if_exist!(LendingRedeemedDataOptional, redeemed, LendingRedeemedData);
