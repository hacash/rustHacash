
pub struct TransactionCoinbase {
    pub ty: Uint1,
	pub address: Address,
	pub reward: Amount,
	pub message: StringTrim16,

    pub extend: CoinbaseExtend,

}

impl TransactionCoinbase {

    // parse function
    pub_fn_field_parse_wrap_return!(TransactionCoinbase, {TransactionCoinbase::new()});
}

impl_Field_trait_for_common!( 0, TransactionCoinbase,
    ty, Uint1,
    address, Address,
    reward, Amount,
    message, StringTrim16,
    extend, CoinbaseExtend,
);

impl TransactionRead for TransactionCoinbase {

    fn hash(&self) -> Hash { 
        self.hash_with_fee()
    }
    
    fn hash_with_fee(&self) -> Hash {
        let stuff = self.serialize();
        let hx = x16rs::calculate_hash(stuff);
        Hash::from(hx)
    }

    
    fn get_type(&self) -> u8 {
        self.ty.value()
    }

    fn get_address(&self) -> &Address {
        &self.address
    }

    fn get_reward(&self) -> &Amount {
        &self.reward
    }

    fn get_message(&self) -> &StringTrim16 {
        &self.message
    }
}

impl Transaction for TransactionCoinbase {

	// change chain state
	fn write_in_chain_state(&self, state: &mut dyn ChainState) -> Result<(), String> {
        let rwdcoin = self.get_reward();
        // send coinbase reward
        operate::hac_add(state, self.get_address(), rwdcoin) ? ;
        // update total supply
        let mut ttcount = state.get_total_supply() ? ;
        let ttrwd = ttcount.get_block_reward().value() + rwdcoin.to_mei_unsafe() as u64; // total
        ttcount.set_block_reward( Uint8::from(ttrwd) );
        state.set_total_supply( &ttcount ) ? ;
        // ok
        Ok(())
    }

}



