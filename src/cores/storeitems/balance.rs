

// pub struct
pub_struct_store_item_define_common!(BalanceItem, 
	diamond  , DiamondNumber    ,
	satoshi  , Satoshi  ,
	hacash   , Amount   ,
);

impl BalanceItem {
	pub fn from_hacash(amt: Amount) -> BalanceItem {
		BalanceItem{
			diamond: DiamondNumber::from(0),
			satoshi: Satoshi::from(0),
			hacash: amt,
		}
	}
}
