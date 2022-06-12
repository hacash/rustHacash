pub type Satoshi = Uint8;

impl Satoshi {

    pub fn to_satoshi_variation(&self) -> SatoshiVariation {
        SatoshiVariation {
            is_exist: Bool::create(true),
            value_sat: Some(self.clone()),
        }
    }

}


// if satoshi
pub struct SatoshiVariation {
	is_exist: Bool,
	value_sat: Option<Satoshi>,
}


impl SatoshiVariation {

    pub fn new() -> SatoshiVariation {
        SatoshiVariation {
            is_exist: Bool::create(false),
            value_sat: None,
        }
    }

    // parse function
    pub_fn_field_parse_wrap_return!(SatoshiVariation, {SatoshiVariation::new()});

}


// impl Field for OptionalAddress
impl_Field_trait_if_exist!(SatoshiVariation, 
    is_exist,
    value_sat, Satoshi
);


