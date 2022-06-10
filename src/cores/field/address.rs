use base58check::ToBase58Check;

pub const ADDRESS_SIZE: usize = 21;

pub struct Address {
    addr: [u8; ADDRESS_SIZE],
}



impl Address {
    pub fn from(a: [u8; ADDRESS_SIZE]) -> Address {
        Address{addr: a}
    }
    // parse function
    pub_fn_parse_wrap_return!(Address, {Address::from([0u8; ADDRESS_SIZE])});

}

impl Clone for Address {
    fn clone(&self) -> Address {
        Address::from(self.addr.clone())
    }
}

// impl Copy for Address {}

impl Field for Address {

    fn serialize(&self) -> Vec<u8> {
        Vec::from(self.addr)
    }

    fn parse(&mut self, buf: &Vec<u8>, seek: usize) -> Result<usize, String> {
        let mvseek = parse_move_seek_or_buf_too_short!("Address", seek, ADDRESS_SIZE, buf);
        // println!("00000000000000000000");
        self.addr =  (&buf[seek..mvseek]).try_into().unwrap();
        Ok(mvseek)
    }

    fn size(&self) -> usize {
        ADDRESS_SIZE
    }

} 


// format
impl Address {
    pub fn readable(&self) -> String {
        let version = self.addr[0];
        self.addr[1..].to_base58check(version)
    }
}