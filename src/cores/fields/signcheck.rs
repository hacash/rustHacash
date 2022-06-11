
// signature verification data
pub struct SignCheckData {
	signdata: Sign,
	stuffstr: StringMax65535,
}

impl SignCheckData {

    pub fn new() -> SignCheckData {
        SignCheckData {
            signdata: Sign::new(),
            stuffstr: StringMax65535::new(),
        }
    }

    // parse function
    pub_fn_field_parse_wrap_return!(SignCheckData, {SignCheckData::new()});

}


// impl Field for Sign
impl_Field_trait_for_common!(SignCheckData, 
    signdata,
    stuffstr
);
