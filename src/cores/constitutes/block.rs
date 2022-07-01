

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

impl BlockHead {
	pub fn get_transaction_count(&self) -> &Uint4 {
		&self.transaction_count
	}
}

// BlockMeta
pub_struct_field_define_for_common!(BlockMeta, 
	// meta
	nonce                 , Fixedbytes4,  // Mining random value
	difficulty            , Uint4,        // Target difficulty value
	witness_stage         , Fixedbytes2,  // Witness quantity level
);


// BlockHead
pub_struct_field_define_for_common!(BlockHeadMeta, 
	// head                   
	head                  , BlockHead,
	// metas                      
	meta                  , BlockMeta,
);

impl BlockHeadMeta {
	pub fn get_transaction_count(&self) -> &Uint4 {
		&self.head.get_transaction_count()
	}
}
