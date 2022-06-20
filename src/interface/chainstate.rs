



macro_rules! define_chain_state_operation_of_common{
    ($( $name: ident, $keyty: ty, $vtype: ty, )+) => (


pub trait ChainStateRead {

    // fn get_block_store_read(&self) -> Box<dyn BlockStoreRead>;

    $(
        concat_idents!(fn_get = get_, $name {
            fn fn_get (&self, _: &$keyty) -> Result<$vtype, String> { Err("".to_string()) }
        });  
    )+
}

pub trait ChainState : ChainStateRead {

    // fn get_block_store(&mut self) -> Box<dyn BlockStore>;

    $(
        concat_idents!(fn_set = set_, $name {
            fn fn_set(&mut self, _: &$keyty, _: &$vtype) -> Option<String> { None }
        });
        concat_idents!(fn_del = del_, $name {
            fn fn_del(&mut self, _: &$keyty) -> Option<String>  { None }
        });
    )+
}

    )
}



/**************************************** */



// ChainStateOperation & ChainStateOperationRead
define_chain_state_operation_of_common!(

    balance, Address, BalanceItem,
    // channel, ChannelId, BalanceItem,
    // diamond, DiamondName, DiamondItem,

    // lockbls, LockblsId, LockblsItem,
    // btcmove_txhash, Uint4, BTCMoveTxHashItem,
    // total_supply, Uint1, TotalSupplyItem,
    
    // bitcoin_syslend, BitcoinSyslendId, BitcoinSystemLendingItem,
    // diamond_syslend, DiamondSyslendId, BitcoinSystemLendingItem,
    // user_lend, UserLendingId, UserLendingItem,

    diamond_refer, DiamondNumber, DiamondName,
    diamond_smelt, DiamondName, DiamondSmeltItem,

    block_refer, BlockHeight, Hash,
    block_bytes, Hash, Hash,

    
);


// ChainState
pub trait ChainStateInstance : ChainState {

}

