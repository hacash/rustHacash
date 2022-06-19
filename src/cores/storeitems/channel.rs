

pub const CHANNEL_STATUS_OPENING                  : Uint1 = Uint1::from_bytes([0u8]); // Normal opening
pub const CHANNEL_STATUS_CHALLENGING              : Uint1 = Uint1::from_bytes([1u8]); // Challenging period
pub const CHANNEL_STATUS_AGREEMENT_CLOSED         : Uint1 = Uint1::from_bytes([2u8]); // After negotiation is closed, reuse can be enabled again
pub const CHANNEL_STATUS_FINAL_ARBITRATION_CLOSED : Uint1 = Uint1::from_bytes([3u8]); // Final arbitration closed, never reusable

// pub struct
pub_struct_store_item_define_common!(ChannelItem, 

	status                        , Uint1          , // Closed and settled
	reuse_version                 , Uint4          , // Reuse version number from 1

	belong_height                 , BlockHeight    , // Block height when channel is opened
	arbitration_lock_block        , Uint2          , // Number of blocks to be locked for unilateral end channel
	interest_attribution          , Uint1          , // Interest attribution of 1% annualized: 0 Press end to assign 1 All to left 2 Give it all right
	
    left_bill                     , HacSatAddr     ,
    right_bill                    , HacSatAddr     ,

    // status = 1
    if_challenging                , ChallengePeriodDataOptional ,

    // status = 2 or 3
    if_distribution               , ClosedDistributionDataOptional ,

);

