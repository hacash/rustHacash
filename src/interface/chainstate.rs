



macro_rules! define_chain_state_operation_of_common{
    (
        ($( $kfix1:tt, $name1:ident, $vtype1:ty )+),
        ($( $kfix2:tt, $name2:ident, $keyty2:ty, $vtype2:ty )+)
    ) => (


pub trait ChainStateRead {

    fn is_debug_test_mode(&self) -> bool { false } // is debug mode
    fn pending_block_height(&self) -> BlockHeight { BlockHeight::from_u64(0) }
    fn pending_block_hash(&self) -> Option<Hash> { None }

    $(
        concat_idents!(fn_get_1 = get_, $name1 {
            fn fn_get_1 (&self) -> Result<$vtype1, String> { Err("".to_string()) }
        });  
    )+
    $(
        concat_idents!(fn_get_2 = get_, $name2 {
            fn fn_get_2 (&self, _: &$keyty2) -> Result<Option<$vtype2>, String> { Ok(None) }
        });  
    )+
}

pub trait ChainStateOperate : ChainStateRead {

    $(
        concat_idents!(fn_set_1 = set_, $name1 {
            fn fn_set_1(&mut self, _: &$vtype1) -> Result<(), String> { Ok(()) }
        });
        concat_idents!(fn_del_1 = del_, $name1 {
            fn fn_del_1(&mut self) -> Result<(), String> { Ok(()) }
        });
    )+
    $(
        concat_idents!(fn_set_2 = set_, $name2 {
            fn fn_set_2(&mut self, _: &$keyty2, _: &$vtype2) -> Result<(), String> { Ok(()) }
        });
        concat_idents!(fn_del_2 = del_, $name2 {
            fn fn_del_2(&mut self, _: &$keyty2) -> Result<(), String> { Ok(()) }
        });
    )+
}

    )
}



/**************************************** */



// ChainStateOperation & ChainStateOperationRead
define_chain_state_operation_of_common!(
    (
        1u8   , total_supply                               , TotalSupplyItem
    ),(
        21u8  , tx_contain          , Hash                 , ContainTxItem
                                                                        
        22u8  , balance             , Address              , BalanceItem
        23u8  , channel             , ChannelId            , ChannelItem
        24u8  , diamond             , DiamondName          , DiamondItem
        25u8  , lockbls             , LockblsId            , LockblsItem
                                                                                          
        26u8  , satoshi_genesis     , Uint4                , SatoshiGenesisItem
        27u8  , btcmove_txhash      , Uint4                , BTCMoveTxHashItem
                                                                            
        28u8  , bitcoin_syslend     , BitcoinSyslendId     , BitcoinSystemLendingItem
        29u8  , diamond_syslend     , DiamondSyslendId     , BitcoinSystemLendingItem
        30u8  , user_lend           , UserLendingId        , UserLendingItem
                                                                                      
        31u8  , diamond_refer       , DiamondNumber        , DiamondName
        32u8  , diamond_smelt       , DiamondName          , DiamondSmeltItem
                                                                                      
        33u8  , block_refer         , BlockHeight          , Hash
        34u8  , block_bytes         , Hash                 , BytesMax4294967295
    )
);


// ChainState
pub trait ChainState : ChainStateOperate {

    fn id(&self) -> usize;

	// Destruction
	fn close(&mut self) {} // close datadir etc
	fn destory(&mut self) {} // Destroy, including deleting all sub States, caches, status data, etc

	// Judgment type
	fn is_immutable(&self) -> bool { false }
	// Save on disk
	fn immutable_write_to_disk(&mut self) -> Result<(), String> { Ok(()) }
	// Get parent status
	fn get_parent(&self) -> Option<WeakArcMutexDynChainState> { None }
	// Get all child States
	// fn get_childs(&self) -> Vec<ArcMutexDynChainState> { vec![] }
    fn append_child(&mut self, _: ArcMutexDynChainState) {}

	// Start a sub state
	// fn fork_with_next_block(&self, _: & dyn Block) -> Result<ArcMutexDynChainState, String> { Err(String::new()) }
	// fn fork_sub_child(&self) -> ArcMutexDynChainState;
    // fn fork_sub_child(_: ArcMutexDynChainState) -> ArcMutexDynChainState;

    // copy cover
	fn traversal_copy(&mut self, _: &ArcMutexDynChainState) -> Result<(), String> { Ok(()) }

	//GetReferBlock() (uint64, fields.Hash)
	fn search_state_by_block_hash(&self, _: &Hash) -> Result<Option<ArcMutexDynChainState>, String> { Ok(None) }

}

