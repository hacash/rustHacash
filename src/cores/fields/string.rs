

// create String macro
macro_rules! create_string_struct_and_impl{
    ($tip:expr, $name:ident, $lenty:ty, $size_max:expr) => (

pub struct $name {
    len: $lenty,
    str: Vec<u8>,
}


impl Field for $name {

    fn serialize(&self) -> Vec<u8> {
        let lv = self.size();
        let mut res = Vec::with_capacity(lv);
        if self.len.value() as usize != self.str.len() {
            panic!("{} size not match.", $tip)
        }
        res.append(&mut self.len.serialize());
        res.append(&mut self.str.clone());
        res
    }

    fn parse(&mut self, buf: &Vec<u8>, seek: usize) -> Result<usize, String> {
        let (seek, obj) = parse_move_seek_or_return_err!($tip, $lenty, buf, seek);
        self.len = obj;
        let strlen = self.len.value() as usize;
        let seek2 = parse_move_seek_or_buf_too_short!($tip, seek, strlen, buf);
        let sv = &buf[seek..seek2];
        self.str = sv.to_vec();
        Ok(seek2)
    }

    fn size(&self) -> usize {
        self.len.size() + self.len.value() as usize
    }
    
    fn describe(&self) -> String {
        let se = String::from_utf8(self.str.clone());
        let sss = match se {
            Err(_) => hex::encode(&self.str),
            Ok(s) => s,
        };
        format!("\"{}\"", sss)
    }

} 



impl $name {

    pub fn new() -> $name {
        $name{
            len: <$lenty>::new(),
            str: vec![],
        }
    }

    pub fn to_string(&self) -> String {
        String::from_utf8(self.str.clone()).unwrap()
    }
    
    pub fn from_string(stuff: &String) -> Result<$name, String> {
        let bytes = stuff.clone().into_bytes();
        let lv = bytes.len();
        if lv == 0 {
            return Err(format!("{}.new len cannot be 0.", $tip))
        }
        if lv > $size_max {
            return Err(format!("{}.new len cannot more than {}.", $tip, $size_max))
        }
        Ok($name{
            len: <$lenty>::from_u64(lv as u64),
            str: bytes,
        })
    }


    // parse function
    pub_fn_field_parse_wrap_return!($name, {<$name>::new()});

}


impl Clone for $name {
    fn clone(&self) -> $name {
        $name{
            len: self.len.clone(),
            str: self.str.clone(),
        }
    }
}

    )
}





// create
create_string_struct_and_impl!("StringMax255", StringMax255, Uint1, 255);
create_string_struct_and_impl!("StringMax65535", StringMax65535, Uint2, 65535);


