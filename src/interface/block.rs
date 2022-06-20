
pub trait BlockReadOnly : Field {

    fn hash(&self) -> Option<Hash>;

    /* */

	fn get_version(&self) -> u8;
    fn get_height(&self) -> &BlockHeight;
	fn get_timestamp(&self) -> &BlockTxTimestamp;
	fn get_prev_hash(&self) -> &Hash;
	fn get_mrkl_root(&self) -> &Hash;
	fn get_difficulty(&self) -> u32;
	fn get_nonce(&self) -> &Fixedbytes4;
	fn get_nonce_num(&self) -> u32;
    fn get_witness_stage(&self) -> &Fixedbytes2;

    /* */


}



pub trait Block : BlockReadOnly {

    fn verify_all_signs(&self) -> bool {
        false
    }
    
	// change chain state
	fn write_in_chain_state(&self, _: &mut dyn ChainState, _: &mut dyn BlockStore) -> ActionStateWriteInReturnType {
        panic!("never call this!")
    }

}