

// create String macro
macro_rules! create_message_struct_and_impl{
    ($tip:expr, $name:ident, $lenty:ty, $size_max:expr) => (

pub struct $name {
    len: $lenty,
    msg: Vec<u8>,
}


impl Field for $name {

    fn serialize(&self) -> Vec<u8> {
        let lv = self.size();
        let mut res = Vec::with_capacity(lv);
        if self.len.value() as usize != self.msg.len() {
            panic!("{} size not match.", $tip)
        }
        res.append(&mut self.len.serialize());
        res.append(&mut self.msg.clone());
        res
    }

    fn parse(&mut self, buf: &Vec<u8>, seek: usize) -> Result<usize, String> {
        let (seek, obj) = parse_move_seek_or_return_err!($tip, $lenty, buf, seek);
        self.len = obj;
        let strlen = self.len.value() as usize;
        let seek2 = parse_move_seek_or_buf_too_short!($tip, seek, strlen, buf);
        let sv = &buf[seek..seek2];
        self.msg = sv.to_vec();
        Ok(seek2)
    }

    fn size(&self) -> usize {
        self.len.size() + self.len.value() as usize
    }

    fn describe(&self) -> String {
        format!("{{\"len\":{},\"hex\":{},\"string\":{}}}", 
            self.size(), 
            hex::encode(&self.msg), 
            self.to_string()
        )
    }

} 



impl $name {

    pub fn new() -> $name {
        $name{
            len: <$lenty>::new(),
            msg: vec![],
        }
    }

    pub fn from(v: impl AsRef<[u8]>) -> Result<$name, String> {
        let bytes = v.as_ref();
        let lv = bytes.len();
        if lv == 0 {
            return Err(format!("{}.new len cannot be 0.", $tip))
        }
        if lv > $size_max {
            return Err(format!("{}.new len cannot more than {}.", $tip, $size_max))
        }
        Ok($name{
            len: <$lenty>::from_u64(lv as u64),
            msg: bytes.to_vec(),
        })
    }

    pub fn to_string(&self) -> String {
        String::from_utf8(self.msg.clone()).unwrap()
    }
    
    pub fn from_string(stuff: &String) -> Result<$name, String> {
        <$name>::from(stuff)
    }

    pub fn clone(&self) -> $name {
        $name{
            len: self.len.clone(),
            msg: self.msg.clone(),
        }
    }

    // parse function
    pub_fn_field_parse_wrap_return!($name, {<$name>::new()});

}



    )
}



// create
create_message_struct_and_impl!("MessageMax255", MessageMax255, Uint1, 255);
create_message_struct_and_impl!("MessageMax65535", MessageMax65535, Uint2, 65535);


