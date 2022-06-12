

pub struct BlockV1 {

	// head
	version: Uint1,
	height: BlockHeight,
	timestamp: BlockTxTimestamp,
	prev_hash: Hash,
	mrkl_root: Hash,
	transaction_count: Uint4,

	// meta
	nonce: Fixedbytes4, // Mining random value
	difficulty: Uint4, // Target difficulty value
	witness_stage: Fixedbytes2, // Witness quantity level

	// body
	transactions: Vec<Box<dyn TransactionReadOnly>>

}


impl_Field_for_common_block!( BlockV1,
    version,
    height,
    timestamp, 
    prev_hash,
    mrkl_root,
    transaction_count,
    nonce,
    difficulty,
    witness_stage
);

impl BlockV1 {

    pub fn new() -> BlockV1 {
        BlockV1 {
            version: Uint1::new(),
            height: BlockHeight::new(),
            timestamp: BlockTxTimestamp::new(),
            prev_hash: Hash::new(),
            mrkl_root: Hash::new(),
            transaction_count: Uint4::new(),
            // meta
            nonce: Fixedbytes4::new(),
            difficulty: Uint4::new(),
            witness_stage: Fixedbytes2::new(),
            // body
            transactions: Vec::new(),
        }
    }

    pub fn _serialize_head(&self) -> Vec<u8> {
        field_serialize_items_concat!(
            self.version,
            self.height,
            self.timestamp, 
            self.prev_hash,
            self.mrkl_root,
            self.transaction_count
        )
    }

    pub fn _serialize_head_meta(&self) -> Vec<u8> {
        let mut head = self._serialize_head();
        let mut meta = field_serialize_items_concat!(
            self.nonce,
            self.difficulty,
            self.witness_stage
        );
        head.append(&mut meta);
        head
    }


    // parse function
    pub_fn_field_parse_wrap_return!(BlockV1, {BlockV1::new()});
    
}



impl BlockReadOnly for BlockV1 {

    fn hash(&self) -> Option<Hash> { None } 

    /* */

	fn get_version(&self) -> u8 {
        self.version.value()
    }
    fn get_height(&self) -> &BlockHeight {
        &self.height
    }
	fn get_timestamp(&self) -> &BlockTxTimestamp {
        &self.timestamp
    }
	fn get_prev_hash(&self) -> &Hash {
        &self.prev_hash
    }
	fn get_mrkl_root(&self) -> &Hash {
        &self.mrkl_root
    }
	fn get_difficulty(&self) -> u32 {
        self.difficulty.value()
    }
	fn get_nonce(&self) -> &Fixedbytes4 {
        &self.nonce
    }
	fn get_nonce_num(&self) -> u32 {
        0
    }
    fn get_witness_stage(&self) -> &Fixedbytes2 {
        &self.witness_stage
    }

    /* */

    fn verify_all_signs(&self) -> bool {
        false
    }

	// change chain state
	fn write_in_chain_state(&self, _: &mut dyn ChainStateOperation) -> ActionStateWriteInReturnType {
        panic!("never call this!")
    }
}