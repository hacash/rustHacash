


// AddrHac
pub_struct_define_for_common!(AddrHac, 
	address, Address,
	hac, Amount,
);



// HacAndSat
pub_struct_define_for_common!(HacSat, 
	hac, Amount,
	sat, SatoshiOptional,
);


// AddrHacSat
pub_struct_define_for_common!(AddrHacSat, 
	address, Address,
	hacsat, HacSat,
);


