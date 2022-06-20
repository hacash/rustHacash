

// create Bytes macro
macro_rules! create_bytes_struct_and_impl{
    ($tip:expr, $name:ident, $lenty:ty, $size_max:expr) => (

pub struct $name {
    len: $lenty,
    bytes: Vec<u8>,
}


impl Field for $name {

    fn serialize(&self) -> Vec<u8> {
        let lv = self.size();
        let mut res = Vec::with_capacity(lv);
        if self.len.value() as usize != self.bytes.len() {
            panic!("{} size not match.", $tip)
        }
        res.append(&mut self.len.serialize());
        res.append(&mut self.bytes.clone());
        res
    }

    fn parse(&mut self, buf: &Vec<u8>, seek: usize) -> Result<usize, String> {
        let (seek, obj) = parse_move_seek_or_return_err!($tip, $lenty, buf, seek);
        self.len = obj;
        let strlen = self.len.value() as usize;
        let seek2 = parse_move_seek_or_buf_too_short!($tip, seek, strlen, buf);
        let sv = &buf[seek..seek2];
        self.bytes = sv.to_vec();
        Ok(seek2)
    }

    fn size(&self) -> usize {
        self.len.size() + self.len.value() as usize
    }
    
    fn describe(&self) -> String {
        let sss: String;
        if self.bytes.len() > 200 {
            let s1 = hex::encode(&self.bytes[0..200]);
            sss = (s1.to_owned() + "...").to_string();
        }else{
            sss = hex::encode(&self.bytes);
        };
        format!("\"{}\"", sss)
    }

} 



impl $name {

    pub fn new() -> $name {
        $name{
            len: <$lenty>::new(),
            bytes: vec![],
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.bytes.clone()
    }
    
    pub fn from_bytes(bytes: Vec<u8>) -> Result<$name, String> {
        let lv = bytes.len();
        if lv == 0 {
            return Err(format!("{}.new len cannot be 0.", $tip))
        }
        if lv > $size_max {
            return Err(format!("{}.new len cannot more than {}.", $tip, $size_max))
        }
        Ok($name{
            len: <$lenty>::from_u64(lv as u64),
            bytes: bytes,
        })
    }


    // parse function
    pub_fn_field_parse_wrap_return!($name, {<$name>::new()});

}


impl Clone for $name {
    fn clone(&self) -> $name {
        $name{
            len: self.len.clone(),
            bytes: self.bytes.clone(),
        }
    }
}

    )
}





// create
create_bytes_struct_and_impl!("BytesMax255",        BytesMax255,        Uint1, 255usize);
create_bytes_struct_and_impl!("BytesMax65535",      BytesMax65535,      Uint2, 65535usize);
create_bytes_struct_and_impl!("BytesMax4294967295", BytesMax4294967295, Uint4, 4294967295usize);


