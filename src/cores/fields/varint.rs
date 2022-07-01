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

#[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
pub struct $name {
    value: $vty,
}

impl fmt::Display for $name{
    fn fmt(&self,f: &mut fmt::Formatter) -> fmt::Result{
        write!(f,"{}",self.value)
    }
}

impl Deref for $name {
    type Target = $vty;
    fn deref(&self) -> &$vty {
        &self.value
    }
}

impl PartialEq<i32>  for $name {
    #[inline]
    fn eq(&self, other: &i32) -> bool {
        self.value == *other as $vty
    }
}

impl PartialOrd for $name {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialOrd<i32> for $name {
    #[inline]
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        let v = *other as $vty;
        Some(self.value.cmp(&v))
    }
}

impl Ord for $name {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}


impl Add for $name {
    type Output = Self;
    #[inline]
    fn add(self, other: Self) -> Self {
        Self {value: self.value + other.value}
    }
}

impl Add<i32> for $name {
    type Output = Self;
    #[inline]
    fn add(self, other: i32) -> Self {
        Self {value: self.value + (other as $vty)}
    }
}

impl Sub for $name {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        Self {value: self.value - other.value}
    }
}

impl Sub<i32> for $name {
    type Output = Self;
    #[inline]
    fn sub(self, other: i32) -> Self {
        Self {value: self.value - (other as $vty)}
    }
}


impl Mul for $name {
    type Output = Self;
    #[inline]
    fn mul(self, other: Self) -> Self {
        Self {value: self.value * other.value}
    }
}

impl Mul<i32> for $name {
    type Output = Self;
    #[inline]
    fn mul(self, other: i32) -> Self {
        Self {value: self.value * other as $vty}
    }
}

impl Div for $name {
    type Output = Self;
    #[inline]
    fn div(self, other: Self) -> Self {
        Self {value: self.value / other.value}
    }
}

impl Div<i32> for $name {
    type Output = Self;
    #[inline]
    fn div(self, other: i32) -> Self {
        Self {value: self.value / other as $vty}
    }
}

impl AddAssign for $name {
    #[inline]
    fn add_assign(&mut self, other: Self) {
        self.value += other.value;
    }
}

impl SubAssign for $name {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        self.value -= other.value;
    }
}

impl MulAssign for $name {
    #[inline]
    fn mul_assign(&mut self, other: Self) {
        self.value *= other.value;
    }
}

impl DivAssign for $name {
    #[inline]
    fn div_assign(&mut self, other: Self) {
        self.value /= other.value;
    }
}

impl AddAssign<u64> for $name {
    #[inline]
    fn add_assign(&mut self, other: u64) {
        self.value += other as $vty;
    }
}

impl AddAssign<i64> for $name {
    #[inline]
    fn add_assign(&mut self, other: i64) {
        self.value += other as $vty;
    }
}

impl AddAssign<u32> for $name {
    #[inline]
    fn add_assign(&mut self, other: u32) {
        self.value += other as $vty;
    }
}

impl AddAssign<i32> for $name {
    #[inline]
    fn add_assign(&mut self, other: i32) {
        self.value += other as $vty;
    }
}

impl SubAssign<u64> for $name {
    #[inline]
    fn sub_assign(&mut self, other: u64) {
        self.value -= other as $vty;
    }
}

impl SubAssign<i64> for $name {
    #[inline]
    fn sub_assign(&mut self, other: i64) {
        self.value -= other as $vty;
    }
}

impl SubAssign<u32> for $name {
    #[inline]
    fn sub_assign(&mut self, other: u32) {
        self.value -= other as $vty;
    }
}

impl SubAssign<i32> for $name {
    #[inline]
    fn sub_assign(&mut self, other: i32) {
        self.value -= other as $vty;
    }
}



impl Field for $name {

    fn new() -> $name {
        $name {
            value: 0,
        }
    }

    fn serialize(&self) -> Vec<u8> {
        self.to_bytes().to_vec()
    }

    fn parse(&mut self, buf: &Vec<u8>, seek: usize) -> Result<usize, String> {
        let mvseek = parse_move_seek_or_buf_too_short!($tip, seek, $size, buf);
        self.value = <$name>::from_bytes_value(buf[seek..mvseek].try_into().unwrap());
        // println!("{}  {}  {}  +=+++=+++===", seek, mvseek, self.bytes[0]);
        Ok(mvseek)
    }

    fn size(&self) -> usize {
        <$name>::size()
    }

    fn describe(&self) -> String {
        format!("{}", self.value() as u64)
    }

}


impl FieldNumber for $name {

    fn get_value(&self) -> u64 {
        self.value() as u64
    }

    fn set_value(&mut self, v: u64) {
        self.value = v as $vty;
    }
}

impl $name {

    const fn size() -> usize {
        $size
    }

    pub const fn from(v: $vty) -> $name {
        $name{
            value: v,
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

    pub fn from_bytes_value(v: [u8; $size]) -> $vty {
        // add left zore
        let drop_zore = $size_vl - $size;
        let mut rv = Vec::with_capacity($size_vl);
        rv.resize(drop_zore, 0u8);
        let appbts = &mut v.to_vec();
        rv.append(appbts);
        <$vty>::from_be_bytes(rv.try_into().unwrap())
    }

    pub fn from_bytes(v: [u8; $size]) -> $name {
        $name{
            value: <$name>::from_bytes_value(v),
        }
    }

    pub fn to_bytes(&self) -> [u8; $size] {
        let mut real = [0u8; $size];
        let bts = <$vty>::to_be_bytes(self.value);
        for x in 1..=$size {
            real[$size-x] = bts[$size_vl-x];
        }
        real
    }
    
    pub fn value(&self) -> $vty {
        self.value
    }

    // parse function
    pub_fn_field_parse_wrap_return!($name, {<$name>::new()});

    // add sub mul div
    pub fn add(&self, v: $vty) -> $name {
        $name {
            value: self.value + v,
        }
    }
    pub fn sub(&self, v: $vty) -> $name {
        $name {
            value: self.value - v,
        }
    }
    pub fn mul(&self, v: $vty) -> $name {
        $name {
            value: self.value * v,
        }
    }
    pub fn div(&self, v: $vty) -> $name {
        $name {
            value: self.value / v,
        }
    }
}




    )
}




// create struct
create_varint_struct_and_impl!("Uint1", Uint1,  u8, UINT1_SIZE, UINT1_SIZE_VL);
create_varint_struct_and_impl!("Uint2", Uint2, u16, UINT2_SIZE, UINT2_SIZE_VL);
create_varint_struct_and_impl!("Uint3", Uint3, u32, UINT3_SIZE, UINT3_SIZE_VL);
create_varint_struct_and_impl!("Uint4", Uint4, u32, UINT4_SIZE, UINT4_SIZE_VL);
create_varint_struct_and_impl!("Uint5", Uint5, u64, UINT5_SIZE, UINT5_SIZE_VL);
create_varint_struct_and_impl!("Uint6", Uint6, u64, UINT6_SIZE, UINT6_SIZE_VL);
create_varint_struct_and_impl!("Uint7", Uint7, u64, UINT7_SIZE, UINT7_SIZE_VL);
create_varint_struct_and_impl!("Uint8", Uint8, u64, UINT8_SIZE, UINT8_SIZE_VL);

