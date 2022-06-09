

pub const ADDRESS_SIZE: usize = 21;

pub struct Address {
    addr: [u8; ADDRESS_SIZE],
}

impl Address {
    pub fn new(a: [u8; ADDRESS_SIZE]) -> Address {
        Address{addr: a}
    }
    pub fn clone(&self) -> Address {
        Address::new(self.addr.clone())
    }
    // parse function
    pub_fn_parse_wrap_return!(Address, {Address::new([0u8; ADDRESS_SIZE])});

}

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

