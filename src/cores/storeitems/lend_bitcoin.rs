
// pub struct
pub_struct_store_item_define_common!(BitcoinSystemLendingItem, 

	is_ransomed                             , Bool            ,   // Whether it has been redeemed (redeemed)
	create_block_height                     , BlockHeight     ,   // lend start height
	main_address                            , Address         ,   // Address of Borrower
	mortgage_bitcoin_portion                , Uint2           ,   // Number of mortgage bitcoin copies (each = 0.01btc)
	loan_total_amount                       , Amount          ,   // The total lending HAC quantity must be less than or equal to the lendable quantity
	pre_burning_interest_amount             , Amount          ,   // Interest for pre destruction must be greater than or equal to the destroyed quantity
	realtime_total_mortgage_ratio           , Uint2           ,   // Value: 0~10000, unit: 10000

	// data if redeemed
    if_redeemed                             , LendingRedeemedDataOptional ,

);

