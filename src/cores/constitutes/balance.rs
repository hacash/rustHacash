


// AddrHac
pub_struct_field_define_for_common!(AddrHac, 
	address, Address,
	amount, Amount,
);



// HacAndSat
pub_struct_field_define_for_common!(HacSat, 
	amount, Amount,
	satoshi, SatoshiOptional,
);


// AddrHacSat
pub_struct_field_define_for_common!(AddrHacSat, 
	address, Address,
	hacsat, HacSat,
);


