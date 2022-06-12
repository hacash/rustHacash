
pub trait TransactionReadOnly : Field {

    fn hash(&mut self) -> Option<Hash> { None }
    fn hash_with_fee(&mut self) -> Option<Hash>{ None }

    /* */

    fn get_type(&self) -> u8;
	fn get_address(&self) -> Option<&Address> { None }
	fn get_signs(&self) -> Option<&Vec<Sign>> { None }
    fn get_reward(&self) -> Option<&Amount> { None }
    fn get_fee(&self) -> Option<&Amount> { None }
    fn get_fee_of_miner_real_received(&self) -> Option<Amount> { None }
    fn get_message(&self) -> Option<&StringTrim16> { None }
	fn get_actions(&self) -> Option<&Vec<Box<dyn Action>>> { None }

    /* */

    fn fee_purity(&self) -> u64 { 0 }
    fn verify_all_signs(&self) -> bool { false }
    fn verify_target_signs(&self, _: &Vec<Address>) -> bool { false }

    /* */

    fn is_burning_90_persent_tx_fee(&self) -> bool { false }

    fn request_sign_addresses(&self) -> Option<Vec<Address>> { None }

	// change chain state
	fn write_in_chain_state(&self, _: &mut dyn ChainStateOperation) -> Option<String> {
        panic!("never call this!")
    }

}
