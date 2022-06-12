
pub struct CoinbaseExtend {
	is_exist: Bool,
    datas_v1: Option<CoinbaseExtendDataV1>,
}


impl CoinbaseExtend {

    pub fn new() -> CoinbaseExtend {
        CoinbaseExtend {
            is_exist: Bool::create(false),
            datas_v1: None,
        }
    }

    // parse function
    pub_fn_field_parse_wrap_return!(CoinbaseExtend, {CoinbaseExtend::new()});

}


// impl Field for OptionalAddress
impl_Field_trait_if_exist!(CoinbaseExtend, 
    is_exist,
    datas_v1, CoinbaseExtendDataV1
);





/***************************** */

pub struct CoinbaseExtendDataV1 {
	miner_nonce: Fixedbytes32,
	witness_count: Uint1, // Number of voting witnesses
}


impl CoinbaseExtendDataV1 {

    pub fn new() -> CoinbaseExtendDataV1 {
        CoinbaseExtendDataV1 {
            miner_nonce: Fixedbytes32::new(),
            witness_count: Uint1::new(),
        }
    }

    // parse function
    pub_fn_field_parse_wrap_return!(CoinbaseExtendDataV1, {CoinbaseExtendDataV1::new()});

}


// impl Field for Sign
impl_Field_trait_for_common!(0, CoinbaseExtendDataV1, 
    miner_nonce,
    witness_count
);
