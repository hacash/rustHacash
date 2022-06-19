



// HacAndSat
pub_struct_define_for_common!(HacAndSat, 
	hac, Amount,
	sat, SatoshiOptional,
);


// HacSatAddr
pub_struct_define_for_common!(HacSatAddr, 
	address, Address,
	hacsat, HacAndSat,
);


