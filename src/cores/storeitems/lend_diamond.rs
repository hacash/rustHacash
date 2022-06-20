


// pub struct
pub_struct_store_item_define_common!(DiamondSystemLendingItem, 

	is_ransomed                           ,  Bool                  ,    // Whether it has been redeemed (redeemed)
	create_block_height                   ,  BlockHeight           ,    // lend start height
	main_address                          ,  Address               ,    // Address of Borrower
	mortgage_diamond_list                 ,  DiamondListMax200     ,    // Mortgage diamond list
	loan_total_amount_mei                 ,  Uint4                 ,    // loan HAC mei
	borrow_period                         ,  Uint1                 ,    // Borrowing cycle: one cycle represents 0.5% interest and 10000 blocks for about 35 days, with a minimum of 1 and a maximum of 20

	// data if redeemed
    if_redeemed                           , LendingRedeemedDataOptional ,

);


