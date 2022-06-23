

pub const CHANNEL_STATUS_OPENING                  : Uint1 = Uint1::from(0); // Normal opening
pub const CHANNEL_STATUS_CHALLENGING              : Uint1 = Uint1::from(1); // Challenging period
pub const CHANNEL_STATUS_AGREEMENT_CLOSED         : Uint1 = Uint1::from(2); // After negotiation is closed, reuse can be enabled again
pub const CHANNEL_STATUS_FINAL_ARBITRATION_CLOSED : Uint1 = Uint1::from(3); // Final arbitration closed, never reusable

// Interest attribution of 1% annualized: 0 Press end to assign 1 All to left 2 Give it all right
pub const CHANNEL_INTEREST_ATTRIBUTION_TYPE_DEFAULT          : Uint1 = Uint1::from(0); // default 
pub const CHANNEL_INTEREST_ATTRIBUTION_TYPE_ALL_TO_LEFT      : Uint1 = Uint1::from(1); // give all to left 
pub const CHANNEL_INTEREST_ATTRIBUTION_TYPE_ALL_TO_RIGHT     : Uint1 = Uint1::from(2); // give all to right  

// pub struct
pub_struct_store_item_define_common!(ChannelItem, 

	status                        , Uint1          , // Closed and settled
	reuse_version                 , Uint4          , // Reuse version number from 1

	belong_height                 , BlockHeight    , // Block height when channel is opened
	arbitration_lock_block        , Uint2          , // Number of blocks to be locked for unilateral end channel
	interest_attribution          , Uint1          , // Interest attribution of 1% annualized: 0 Press end to assign 1 All to left 2 Give it all right
	
    left_bill                     , AddrHacSat     ,
    right_bill                    , AddrHacSat     ,

    // status = 1
    if_challenging                , ChallengePeriodDataOptional ,

    // status = 2 or 3
    if_distribution               , ClosedDistributionDataOptional ,

);


