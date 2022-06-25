

// pub struct
pub_struct_store_item_define_common!(DiamondSmeltItem, 
	diamond                   , DiamondName       ,
	number                    , DiamondNumber     ,
	contain_block_height      , BlockHeight       ,
	contain_block_hash        , Hash              ,
	prev_contain_block_hash   , Hash              ,
	miner_address             , Address           ,
	bid_fee                   , Amount            ,
	nonce                     , Fixedbytes8       ,
	custom_message            , CustomMessageOptional      ,
	average_bid_burn          , Uint2             ,
	visual_gene               , Fixedbytes10      ,
);




