use base58check::ToBase58Check;


pub type Address = Fixedbytes21;

pub const ADDRESS_SIZE: usize = Address::size();



// format
impl Address {
    
    pub fn to_readable(&self) -> String {
        let version = self.bytes[0];
        self.bytes[1..].to_base58check(version)
    }

    pub fn describe(&self) -> String {
        format!("\"{}\"", self.to_readable() )
    }
    
}