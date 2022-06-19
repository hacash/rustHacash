

// pub struct
pub_struct_store_item_define_common!(DiamondSmeltItem, 
	diamond                   , DiamondName       ,
	number                    , DiamondNumber     ,
	contain_block_height      , BlockHeight       ,
	contain_block_hash        , Hash              ,
	prev_contain_block_hash   , Hash              ,
	miner_address             , Address           ,
	approx_fee_offer          , Amount            ,
	nonce                     , Fixedbytes8       ,
	custom_message            , Fixedbytes32      ,
	average_bid_burn_price    , Uint2             ,
	visual_gene               , Fixedbytes10      ,
);




