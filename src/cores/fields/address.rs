use base58check::ToBase58Check;


pub type Address = Fixedbytes21;

pub const ADDRESS_SIZE: usize = Address::size();


impl Address {
    // parse function
    // pub_fn_field_parse_wrap_return!(Address, {Address::from([0u8; ADDRESS_SIZE])});

}

/*
impl Clone for Address {
    fn clone(&self) -> Address {
        Address::clone
    }
}
*/

// impl Copy for Address {}





// format
impl Address {
    
    pub fn readable(&self) -> String {
        let version = self.bytes[0];
        self.bytes[1..].to_base58check(version)
    }

    pub fn describe(&self) -> String {
        format!("\"{}\"", self.readable() )
    }
    
}