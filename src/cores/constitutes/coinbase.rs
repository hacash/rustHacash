
// CoinbaseExtendDataV1
pub_struct_field_define_for_common!(CoinbaseExtendDataV1, 
	miner_nonce, Fixedbytes32,
	witness_count, Uint1, // Number of voting witnesses
);


// CoinbaseExtend
pub_struct_define_for_if_exist!(CoinbaseExtend, datas_v1, CoinbaseExtendDataV1);


