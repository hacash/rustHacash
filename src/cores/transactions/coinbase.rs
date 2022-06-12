
pub struct TransactionCoinbase {
    ty: Uint1,
	address: Address,
	reward: Amount,
	message: StringTrim16,

    extend: CoinbaseExtend,

}

impl TransactionCoinbase {

    pub fn new() -> TransactionCoinbase {
        TransactionCoinbase {
            ty: Uint1::new(),
            address: Address::new(),
            reward: Amount::new(),
            message: StringTrim16::new(),
            extend: CoinbaseExtend::new()
        }
    }

    // parse function
    pub_fn_field_parse_wrap_return!(TransactionCoinbase, {TransactionCoinbase::new()});
}

impl_Field_trait_for_common!( 0, TransactionCoinbase,
    ty,
    address,
    reward, 
    message,
    extend
);

impl TransactionReadOnly for TransactionCoinbase {
    
    fn get_type(&self) -> u8 {
        self.ty.value()
    }

    fn get_address(&self) -> Option<&Address> {
        Some(&self.address)
    }

    fn get_reward(&self) -> Option<&Amount> {
        Some(&self.reward)
    }

    fn get_message(&self) -> Option<&StringTrim16> {
        Some(&self.message)
    }
}




