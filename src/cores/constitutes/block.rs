

// BlockPtr
pub_struct_field_define_for_common!(BlockPtr, 
	// ptr
	height                , BlockHeight,
	hash                  , Hash,
);


// BlockHead
pub_struct_field_define_for_common!(BlockHead, 
	// head
	version               , Uint1,
	height                , BlockHeight,
	timestamp             , BlockTxTimestamp,
	prev_hash             , Hash,
	mrkl_root             , Hash,
	transaction_count     , Uint4,
);
