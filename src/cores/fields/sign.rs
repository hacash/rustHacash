

pub struct Sign {
    publickey: Fixedbytes33,   
    signature: Fixedbytes64,
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


// impl Field for Sign
impl_Field_trait_for_common!(Sign, 
    publickey,
    signature
);


