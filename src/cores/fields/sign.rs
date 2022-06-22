

pub struct Sign {
    pub publickey: Fixedbytes33,   
    pub signature: Fixedbytes64,
}

impl Sign {

    pub fn new() -> Sign {
        Sign {
            publickey: Fixedbytes33::new(),
            signature: Fixedbytes64::new(),
        }
    }

    // parse function
    pub_fn_field_parse_wrap_return!(Sign, {Sign::new()});

}

impl Clone for Sign {
    fn clone(&self) -> Sign {
        Sign{
            publickey: self.publickey.clone(),
            signature: self.signature.clone(),
        }
    }
}


// impl Field for Sign
impl_Field_trait_for_common!(0, Sign, 
    publickey,
    signature,
);


