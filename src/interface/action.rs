

// pub type Option<String> = Option<String>;

pub trait Action : Field {

    fn kind(&self) -> u16;

    fn is_burning_90_persent_tx_fee(&self) -> bool;

    fn request_sign_addresses(&self) -> HashMap<Address, ()>;

	// change chain state
	fn write_in_chain_state(&self, _: &mut dyn ChainState, _: & dyn Transaction) -> Result<(), String>;

}

