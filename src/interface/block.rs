
pub trait BlockRead : Field {

    fn hash(&self) -> Hash;

    fn copy_block_ptr(&self) -> BlockPtr;
    fn copy_block_head(&self) -> BlockHead;

    /* */

	fn get_version(&self) -> &Uint1;
    fn get_height(&self) -> &BlockHeight;
	fn get_timestamp(&self) -> &BlockTxTimestamp;
	fn get_prev_hash(&self) -> &Hash;
	fn get_mrkl_root(&self) -> &Hash;
    fn get_transaction_count(&self) -> &Uint4;
	fn get_difficulty(&self) -> u32;
	fn get_nonce(&self) -> &Fixedbytes4;
	fn get_nonce_num(&self) -> u32;
    fn get_witness_stage(&self) -> &Fixedbytes2;
	fn get_transactions(&self) -> &Vec<Box<dyn Transaction>>;

    /* */


}



pub trait Block : BlockRead {

    fn verify_all_signs(&self) -> Result<(), String> {
        Err("".to_string())
    }
    
	// change chain state
	fn write_in_chain_state(&self, _: &mut dyn ChainState) -> Result<(), String> {
        panic!("never call this!")
    }

}