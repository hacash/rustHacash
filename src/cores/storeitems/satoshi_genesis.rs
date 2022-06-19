

// pub struct
pub_struct_store_item_define_common!(SatoshiGenesisItem, 

	transfer_no                 , Uint4            ,         
	bitcoin_block_height        , Uint4            ,         
	bitcoin_block_timestamp     , BlockTxTimestamp , 
	bitcoin_effective_genesis   , Uint4            ,         
	bitcoin_quantity            , Uint4            ,         
	additional_total_hac_amount , Uint4            ,         
	origin_address              , Address          ,          
	bitcoin_transfer_hash       , Hash             ,             

);

