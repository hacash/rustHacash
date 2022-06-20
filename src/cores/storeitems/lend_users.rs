


// pub struct
pub_struct_store_item_define_common!(UserLendingItem, 
    
	is_ransomed                        , Bool                  , // Whether it has been redeemed (redeemed)
	is_redemption_overtime             , Bool                  , // Whether it can be redeemed after expiration (automatic extension)
	is_public_redeemable               , Bool                  , // Public redeemable after maturity

	create_block_height                , BlockHeight           , // lend start height
	expire_block_height                , BlockHeight           , // Agreed expiration block height

	mortgagor_address                  , Address               , // Address of mortgagor
	lender_address                     , Address               , // Lender address

	mortgage_bitcoin                   , SatoshiOptional      , // Mortgage bitcoin quantity unit: SAT
	mortgage_diamond_list              , DiamondListMax200  , // Mortgage diamond table

	loan_total_amount                  , Amount                , // The total lending HAC quantity must be less than or equal to the lendable quantity
	agreed_redemption_amount           , Amount                , // Agreed redemption amount

	pre_burning_interest_amount        , Amount                , // Interest for pre destruction must be greater than or equal to 1% of the lending amount

	// data if redeemed
	if_redeemed                        , LendingRedeemedDataOptional   , // redemption

);