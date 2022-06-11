

pub struct DiamondListMax200  {
	count: Uint1,
	diamonds: Vec<DiamondName>,
}


impl DiamondListMax200 {

    pub fn new() -> DiamondListMax200 {
        DiamondListMax200 {
            count: Uint1::new(),
            diamonds: Vec::new(),
        }
    }

    // parse function
    pub_fn_field_parse_wrap_return!(DiamondListMax200, {DiamondListMax200::new()});

}


// impl Field for DiamondListMax200
impl_Field_trait_for_list!(DiamondListMax200, 
    Uint1, 
    DiamondName,
    count,
    diamonds
);





