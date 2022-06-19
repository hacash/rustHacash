

// ChallengePeriodData
pub_struct_define_for_common!(ChallengePeriodData, 
	// Status = 1 challenge period save data
	is_have_challenge_log             , Bool             , // Record challenge data log
	challenge_launch_height           , BlockHeight      , // Block height at the beginning of the challenge
	assert_bill_auto_number           , Uint8            , // Statement serial number provided by the proposer
	assert_address_is_left_or_right   , Bool             , // Whether the proposer is the left address or the right true left false right
	assert_bill                       , HacAndSat        , // The amount claimed by the proponent
);

pub_struct_define_for_if_exist!(ChallengePeriodDataOptional, challenge, ChallengePeriodData);



/******************************* */



// FinalDistributionData
pub_struct_define_for_common!(ClosedDistributionData, 
	// Status = 2 or 3 
	distribution                       , HacAndSat        , 
);

pub_struct_define_for_if_exist!(ClosedDistributionDataOptional, closed_distribution, ClosedDistributionData);




