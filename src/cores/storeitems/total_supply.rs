
// pub struct
pub_struct_store_item_define_common!(TotalSupplyItem, 

    block_reward                              , Uint8          , // HAC
	moved_bitcoin                             , Uint4          , // BTC
	minted_diamond                            , Uint3          , // HACD
	
    channel_interest                          , Float8         , // float64 HAC
    btc_move_subsidy_unlock_successed         , Float8         , // float64 HAC

    channel_opening_count                     , Uint4          ,
    channel_located_hac                       , Float8         , // float64 HAC
    channel_located_sat                       , Uint8          , // SAT

    burning_fee                               , Float8         , // float64 HAC                  
 
    syslend_diamond                           , Uint3          ,
    syslend_diamond_loan_hac                  , Float8         , // float64 HAC
    syslend_diamond_ransom_hac                , Float8         , // float64 HAC

    syslend_bitcoin_portion                   , Uint8          , // unit: 0.01 BTC
    syslend_bitcoin_portion_loan_hac          , Float8         ,
    syslend_bitcoin_portion_ransom_hac        , Float8         ,
    syslend_bitcoin_portion_burning_interest  , Float8         ,

    userlend_diamond                           , Uint3         ,
    userlend_bitcoin                           , Float8        , // float64 BTC
    userlend_loan_hac                          , Float8        , // float64 HAC
    userlend_burning_interest                  , Float8        , // float64 HAC

);

