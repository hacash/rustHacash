
pub const DIAMOND_STATUS_NORMAL: Uint1 = Uint1::from_bytes([0u8]);
pub const DIAMOND_STATUS_LENDING_TO_SYSTEM: Uint1 = Uint1::from_bytes([1u8]);
pub const DIAMOND_STATUS_LENDING_TO_USER: Uint1 = Uint1::from_bytes([2u8]);

// pub struct
pub_struct_store_item_define_common!(DiamondItem, 
	status    , Uint1   ,
	address   , Address ,
);



