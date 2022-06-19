
pub const FLOAT4_SIZE: usize = 4;
pub const FLOAT8_SIZE: usize = 8;


macro_rules! create_varfloat_struct_and_impl{
    ($tip:expr, $name:ident, $vty:ty, $size:expr) => (


#[derive(PartialEq)]
pub struct $name {
    bytes: [u8; $size],
}

impl Field for $name {

    fn serialize(&self) -> Vec<u8> {
        self.bytes.to_vec() // clone
    }

    fn parse(&mut self, buf: &Vec<u8>, seek: usize) -> Result<usize, String> {
        let mvseek = parse_move_seek_or_buf_too_short!($tip, seek, $size, buf);
        self.bytes = buf[seek..mvseek].try_into().unwrap();
        // println!("{}  {}  {}  +=+++=+++===", seek, mvseek, self.bytes[0]);
        Ok(mvseek)
    }

    fn size(&self) -> usize {
        <$name>::size()
    }

    fn describe(&self) -> String {
        format!("{}", self.value() as f64)
    }

}

impl Clone for $name {
    fn clone(&self) -> $name {
        $name{
            bytes: self.bytes.clone(),
        }
    }
}

impl $name {

    const fn size() -> usize {
        $size
    }

    pub fn new() -> $name {
        $name{
            bytes: [0u8; $size],
        }
    }

    pub fn from(v: $vty) -> $name {
        let bts = <$vty>::to_be_bytes(v);
        $name{
            bytes: bts,
        }
    }

    pub const fn from_bytes(v: [u8; $size]) -> $name {
        $name{
            bytes: v,
        }
    }

    pub fn value(&self) -> $vty {
        <$vty>::from_be_bytes(self.bytes)
    }

    // parse function
    pub_fn_field_parse_wrap_return!($name, {<$name>::new()});
}


    )
}


// create struct
create_varfloat_struct_and_impl!("Float4", Float4, f32, FLOAT4_SIZE);
create_varfloat_struct_and_impl!("Float8", Float8, f64, FLOAT8_SIZE);

