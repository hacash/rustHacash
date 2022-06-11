pub const STRINGTRIM16_SIZE: usize = 16;
pub const STRINGTRIM34_SIZE: usize = 34;
pub const STRINGTRIM64_SIZE: usize = 64;

// create String macro
macro_rules! create_stringtrim_struct_and_impl{
    ($tip:expr, $name:ident, $size:expr) => (

pub struct $name {
    bytes: [u8; $size],
}


impl Field for $name {

    fn serialize(&self) -> Vec<u8> {
        self.bytes.to_vec()
    }

    fn parse(&mut self, buf: &Vec<u8>, seek: usize) -> Result<usize, String> {
        let seek2 = parse_move_seek_or_buf_too_short!($tip, seek, $size, buf);
        let sv = &buf[seek..seek2];
        self.bytes = sv.to_vec().try_into().unwrap();
        Ok(seek2)
    }

    fn size(&self) -> usize {
        $size
    }

} 



impl $name {

    pub fn to_string(&self) -> String {
        let v = String::from_utf8(self.bytes.to_vec()).unwrap();
        v.as_str().trim().to_string()
    }
    
    pub fn new(stuff: &String) -> $name {
        let bytes = stuff.clone().into_bytes();
        let lv = bytes.len();
        let mut resv = Vec::new();
        resv.append(&mut stuff.clone().into_bytes());
        resv.resize($size - lv, ' ' as u8);
        $name{
            bytes: bytes.try_into().unwrap(),
        }
    }

    pub fn clone(&self) -> $name {
        $name{
            bytes: self.bytes.clone(),
        }
    }

    // parse function
    pub_fn_field_parse_wrap_return!($name, {<$name>::new(&String::from(" "))});

}



    )
}



// create
create_stringtrim_struct_and_impl!("StringTrim16", StringTrim16, STRINGTRIM16_SIZE);
create_stringtrim_struct_and_impl!("StringTrim34", StringTrim34, STRINGTRIM34_SIZE);
create_stringtrim_struct_and_impl!("StringTrim64", StringTrim64, STRINGTRIM64_SIZE);


