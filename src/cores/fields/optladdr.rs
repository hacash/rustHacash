
// if have address
pub struct OptionalAddress {
	is_exist: Bool,
	address: Address,
}


impl OptionalAddress {

    pub fn new() -> OptionalAddress {
        OptionalAddress {
            is_exist: Bool::create(false),
            address: Address::new(),
        }
    }

    // parse function
    pub_fn_field_parse_wrap_return!(OptionalAddress, {OptionalAddress::new()});

}


// impl Field for OptionalAddress
impl_Field_trait_if_exist!(OptionalAddress, 
    is_exist,
    address
);


