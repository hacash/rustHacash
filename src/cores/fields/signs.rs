

pub struct SignListMax255  {
	count: Uint1,
	signs: Vec<Sign>,
}


impl SignListMax255 {

    pub fn new() -> SignListMax255 {
        SignListMax255 {
            count: Uint1::new(),
            signs: Vec::new(),
        }
    }

    // parse function
    pub_fn_field_parse_wrap_return!(SignListMax255, {SignListMax255::new()});

}


// impl Field for DiamondListMax200
impl_Field_trait_for_list!(SignListMax255, 
    Uint1, 
    Sign,
    count,
    signs
);



/***************************************** */


pub struct SignListMax65535  {
	count: Uint2,
	signs: Vec<Sign>,
}


impl SignListMax65535 {

    pub fn new() -> SignListMax65535 {
        SignListMax65535 {
            count: Uint2::new(),
            signs: Vec::new(),
        }
    }

    pub fn get_count(&self) -> &Uint2 {
        &self.count
    }
    pub fn get_list(&self) -> &Vec<Sign> {
        &self.signs
    }

    // parse function
    pub_fn_field_parse_wrap_return!(SignListMax65535, {SignListMax65535::new()});

}


// impl Field for DiamondListMax200
impl_Field_trait_for_list!(SignListMax65535, 
    Uint2, 
    Sign,
    count,
    signs
);
