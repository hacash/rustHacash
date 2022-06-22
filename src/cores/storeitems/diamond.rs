
pub const DIAMOND_STATUS_NORMAL: Uint1               = Uint1::from(0);
pub const DIAMOND_STATUS_LENDING_TO_SYSTEM: Uint1    = Uint1::from(1);
pub const DIAMOND_STATUS_LENDING_TO_USER: Uint1      = Uint1::from(2);

// pub struct
pub_struct_store_item_define_common!(DiamondItem, 
	status    , Uint1   ,
	address   , Address ,
);



