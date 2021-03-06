


macro_rules! define_chain_state_operation_instance_of_common{
    (
        ($( $kfix1:tt, $name1:ident, $vtype1:ty )+),
        ($( $kfix2:tt, $name2:ident, $keyty2:ty, $vtype2:ty )+)
    ) => (


impl ChainStateRead for ChainStateInstance {

    fn is_debug_test_mode(&self) -> bool {
        self.config.mode_debug_test
    }
    fn is_database_rebuild_mode(&self) -> bool {
        self.config.mode_database_rebuild
    }
    fn is_check_btcmove(&self) -> bool {
        self.config.mode_check_btcmove
    }

    fn pending_block_height(&self) -> BlockHeight { 
        match &self.basis_block {
            PenddingBasisBlock::Height(hei) => *hei,
            PenddingBasisBlock::Blkptr(ptr) => ptr.height,
        }
    }
    fn pending_block_hash(&self) -> Option<Hash> { 
        match &self.basis_block {
            PenddingBasisBlock::Blkptr(ptr) => Some(ptr.hash.clone()),
            _ => None,
        }
    }


    $(
    concat_idents!(fn_get_1 = get_, $name1 {
    fn fn_get_1(&self) -> Result<$vtype1, String> {
        let mkey = vec![$kfix1];
        let has = self.memdb.get(&mkey);
        if let Some(dts) = has {
            // found
            let (item, _) = $vtype1::parse(&dts, 0) ? ;
            return Ok(item)
        };
        let upstat = self.get_parent();
        if let Some(p) = upstat {
            if let Some(p) = p.upgrade() {
                return p.lock().unwrap().fn_get_1()
            }
        }
        // just not find, create one
        Ok($vtype1::new())
    }
    });
    )+

    $(
    concat_idents!(fn_get = get_, $name2 {
    fn fn_get(&self, $name2: &$keyty2) -> Result<Option<$vtype2>, String> {
        let mkey = ChainStateInstance::makey($kfix2, $name2.serialize());
        let has = self.memdb.get(&mkey);
        if let Some(dts) = has {
            // found
            let (item, _) = $vtype2::parse(&dts, 0) ? ;
            return Ok(Some(item))
        };
        let upstat = self.get_parent();
        if let Some(p) = upstat {
            if let Some(p) = p.upgrade() {
                return p.lock().unwrap().fn_get($name2)
            }
        }
        // just not find
        Ok(None)
    }
    });
    )+

    
}


impl ChainStateOperate for ChainStateInstance {

    $(
    concat_idents!(fn_set = set_, $name1 {
    fn fn_set(&mut self, item: &$vtype1) -> Result<(), String> {
        let mkey = vec![$kfix1];
        let vdts = item.serialize();
        let putres = self.memdb.put(mkey, vdts);
        match putres {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }
    });
    concat_idents!(fn_del = del_, $name1 {
    fn fn_del(&mut self) -> Result<(), String> {
        let mkey = vec![$kfix1];
        let delres = self.memdb.delete(&mkey);
        self.mark_del_key(mkey); // mark del
        match delres {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }
    });
    )+
    
    $(
    concat_idents!(fn_set = set_, $name2 {
    fn fn_set(&mut self, $name2: &$keyty2, item: &$vtype2) -> Result<(), String> {
        let mkey = ChainStateInstance::makey($kfix2, $name2.serialize());
        let vdts = item.serialize();
        let putres = self.memdb.put(mkey, vdts);
        match putres {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }
    });
    concat_idents!(fn_del = del_, $name2 {
    fn fn_del(&mut self, $name2: &$keyty2) -> Result<(), String> {
        let mkey = ChainStateInstance::makey($kfix2, $name2.serialize());
        let delres = self.memdb.delete(&mkey);
        self.mark_del_key(mkey); // mark del
        match delres {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }
    });
    )+

    
}


    )
}



define_chain_state_operation_instance_of_common!(
    (
        1u8   , total_supply                               , TotalSupplyItem
        2u8   , latest_block_intro                         , BlockIntroItem
        3u8   , latest_diamond                             , DiamondSmeltItem
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
