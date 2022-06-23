
pub trait TransactionRead : Field {

    fn hash(&self) -> Hash { panic_never_call_this!() }
    fn hash_with_fee(&self) -> Hash{ panic_never_call_this!() }

    /* */

    fn get_type(&self) -> u8;
	fn get_address(&self) -> &Address { panic_never_call_this!() }
    fn get_reward(&self) -> &Amount { panic_never_call_this!() }
	fn get_signs(&self) -> &Vec<Sign> { panic_never_call_this!() }
    fn get_fee(&self) -> &Amount { panic_never_call_this!() }
    fn get_fee_of_miner_real_received(&self) -> Amount { panic_never_call_this!() }
    fn get_message(&self) -> &StringTrim16 { panic_never_call_this!() }
	fn get_actions(&self) -> &Vec<Box<dyn Action>> { panic_never_call_this!() }

    /* */

    fn fee_purity(&self) -> u64 { 0 }

    /* */

    fn is_burning_90_persent_tx_fee(&self) -> bool { false }

    fn request_sign_addresses(&self) -> HashMap<Address, ()> { HashMap::new() }


}


pub trait Transaction : TransactionRead {

    fn verify_all_signs(&self) -> Result<(), String> { Err("".to_string()) }
    fn verify_target_signs(&self, _: &HashMap<Address, ()>) -> Result<(), String> { Err("".to_string()) }
    fn verify_need_signs(&self, _: &Vec<Address>) -> Result<(), String> { Err("".to_string()) }

	// change chain state
	fn write_in_chain_state(&self, _: &mut dyn ChainState) -> Result<(), String> {
        panic_never_call_this!()
    }
    
}