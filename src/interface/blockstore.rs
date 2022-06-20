// BlockStoreRead
pub trait BlockStoreRead {

    fn read_block_hash_by_height(&self, _: &BlockHeight) -> Result<Hash, String>;
    fn read_block_bytes_by_hash(&self, _: &Hash) -> Result<Vec<u8>, String>;
    fn read_block_bytes_by_height(&self, _: &BlockHeight) -> Result<(Hash, Vec<u8>), String>;

    fn read_diamond(&self, _: &DiamondName) -> Result<DiamondSmeltItem, String>;
    fn read_diamond_by_number(&self, _: &DiamondNumber) -> Result<DiamondSmeltItem, String>;
    fn read_diamond_name_by_number(&self, _: &DiamondNumber) -> Result<DiamondName, String>;

    fn get_btcmove_log_total_page(&self) -> Result<Uint4, String>; // Number of data pages, 100 pieces of data per page
    fn get_btcmove_log_page_data(&self, _: &Uint4) -> Result<Vec<SatoshiGenesisItem>, String>; // Get data page

    fn get_validated_satoshi_genesis(&self, _: &Uint4) -> Result<(SatoshiGenesisItem, bool), String>; // Get verified BTC transfer logs & whether verification is required

}

// BlockStore
pub trait BlockStore: BlockStoreRead {
    fn close(&mut self);

    // save
    fn save_block(&mut self, _: &BlockBag) -> Option<String>;

    // Set the block hash that the block height points to
    fn update_set_block_hash_refer_to_height(&mut self,_: &BlockHeight, _: &Hash) -> Option<String>;

    // tx
    //ReadTransactionBytesByHash(fields.Hash) (uint64, []byte, error)

    // diamond
    fn save_diamond(&mut self,_: &DiamondSmeltItem) -> Option<String>;

    // Set the diamond name pointed by the diamond number
    fn update_set_diamond_name_refer_to_number(&mut self,_: &DiamondNumber, _: &DiamondName) -> Option<String>;

    // btc move log
    // fn RunDownLoadBTCMoveLog();
    // fn SaveBTCMoveLogPageData(&mut self, int, []*SatoshiGenesis) -> Option<String>; // Save data page
}

// BlockStoreInstance
pub trait BlockStoreInstance: BlockStore {



}