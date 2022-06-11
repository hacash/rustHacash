pub const UINT1_SIZE: usize = 1;
pub const UINT2_SIZE: usize = 2;
pub const UINT3_SIZE: usize = 3;
pub const UINT4_SIZE: usize = 4;
pub const UINT5_SIZE: usize = 5;
pub const UINT6_SIZE: usize = 6;
pub const UINT7_SIZE: usize = 7;
pub const UINT8_SIZE: usize = 8;

pub const UINT1_SIZE_VL: usize = 1;
pub const UINT2_SIZE_VL: usize = 2;
pub const UINT3_SIZE_VL: usize = 4;
pub const UINT4_SIZE_VL: usize = 4;
pub const UINT5_SIZE_VL: usize = 8;
pub const UINT6_SIZE_VL: usize = 8;
pub const UINT7_SIZE_VL: usize = 8;
pub const UINT8_SIZE_VL: usize = 8;

/////////////////////////////////




macro_rules! create_varint_struct_and_impl{
    ($tip:expr, $name:ident, $vty:ty, $size:expr, $size_vl:expr) => (


pub struct $name {
    bytes: [u8; $size],
}

impl Field for $name {

    fn serialize(&self) -> Vec<u8> {
        self.bytes.to_vec() // clone
    }

    fn parse(&mut self, buf: &Vec<u8>, seek: usize) -> Result<usize, String> {
        let seek = parse_move_seek_or_buf_too_short!($tip, seek, $size, buf);
        self.bytes = buf[seek..seek + $size].try_into().unwrap();
        self.bytes = self.bytes.clone(); // clone
        Ok(seek)
    }

    fn size(&self) -> usize {
        <$name>::size()
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
        let drop_zore = $size_vl - $size;
        $name{
            // drop left zore
            bytes: bts[drop_zore..].try_into().unwrap(),
        }
    }

    pub fn from_u64(v: u64) -> $name {
        <$name>::from(v as $vty)
    }

    pub fn from_string(strv: &String) -> Result<$name, String> {
        match strv.parse::<u64>() {
            Err(e) => Err(e.to_string()),
            Ok(v) => Ok(<$name>::from(v as $vty)),
        }
    }

    pub fn clone(&self) -> $name {
        $name{
            bytes: self.bytes.clone(),
        }
    }

    pub fn value(&self) -> $vty {
        // add left zore
        let drop_zore = $size_vl - $size;
        let mut rv = Vec::with_capacity($size_vl);
        rv.resize(drop_zore, 0u8);
        let appbts = &mut self.bytes.to_vec();
        rv.append(appbts);
        <$vty>::from_be_bytes(rv.try_into().unwrap())
    }

    // parse function
    pub_fn_field_parse_wrap_return!($name, {<$name>::new()});
}


    )
}


// create struct
create_varint_struct_and_impl!("UInt1", Uint1,  u8, UINT1_SIZE, UINT1_SIZE_VL);
create_varint_struct_and_impl!("UInt2", Uint2, u16, UINT2_SIZE, UINT2_SIZE_VL);
create_varint_struct_and_impl!("UInt3", Uint3, u32, UINT3_SIZE, UINT3_SIZE_VL);
create_varint_struct_and_impl!("UInt4", Uint4, u32, UINT4_SIZE, UINT4_SIZE_VL);
create_varint_struct_and_impl!("UInt5", Uint5, u64, UINT5_SIZE, UINT5_SIZE_VL);
create_varint_struct_and_impl!("UInt6", Uint6, u64, UINT6_SIZE, UINT6_SIZE_VL);
create_varint_struct_and_impl!("UInt7", Uint7, u64, UINT7_SIZE, UINT7_SIZE_VL);
create_varint_struct_and_impl!("UInt8", Uint8, u64, UINT8_SIZE, UINT8_SIZE_VL);

