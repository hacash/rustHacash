
pub const FLOAT4_SIZE: usize = 4;
pub const FLOAT8_SIZE: usize = 8;


macro_rules! create_varfloat_struct_and_impl{
    ($tip:expr, $name:ident, $vty:ty, $size:expr) => (


#[derive(Copy, Clone, PartialEq)]
pub struct $name {
    value: $vty,
}


impl Add for $name {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {value: self.value + other.value}
    }
}

impl Sub for $name {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {value: self.value - other.value}
    }
}

impl Mul for $name {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self {value: self.value * other.value}
    }
}

impl Div for $name {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        Self {value: self.value / other.value}
    }
}

impl AddAssign for $name {
    fn add_assign(&mut self, other: Self) {
        self.value += other.value;
    }
}

impl SubAssign for $name {
    fn sub_assign(&mut self, other: Self) {
        self.value -= other.value;
    }
}

impl MulAssign for $name {
    fn mul_assign(&mut self, other: Self) {
        self.value *= other.value;
    }
}

impl DivAssign for $name {
    fn div_assign(&mut self, other: Self) {
        self.value /= other.value;
    }
}


impl AddAssign<f64> for $name {
    #[inline]
    fn add_assign(&mut self, other: f64) {
        self.value += other as $vty;
    }
}

impl AddAssign<f32> for $name {
    #[inline]
    fn add_assign(&mut self, other: f32) {
        self.value += other as $vty;
    }
}

impl SubAssign<f64> for $name {
    #[inline]
    fn sub_assign(&mut self, other: f64) {
        self.value -= other as $vty;
    }
}

impl SubAssign<f32> for $name {
    #[inline]
    fn sub_assign(&mut self, other: f32) {
        self.value -= other as $vty;
    }
}




impl Field for $name {

    fn serialize(&self) -> Vec<u8> {
        <$vty>::to_be_bytes(self.value).to_vec()
    }

    fn parse(&mut self, buf: &Vec<u8>, seek: usize) -> Result<usize, String> {
        let mvseek = parse_move_seek_or_buf_too_short!($tip, seek, $size, buf);
        let bytes = buf[seek..mvseek].try_into().unwrap();
        self.value = <$vty>::from_be_bytes(bytes);
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


impl $name {

    const fn size() -> usize {
        $size
    }

    pub fn new() -> $name {
        $name{
            value: 0 as $vty,
        }
    }

    pub fn from(v: $vty) -> $name {
        $name{
            value: v,
        }
    }

    pub fn from_bytes(v: [u8; $size]) -> $name {
        $name{
            value: <$vty>::from_be_bytes(v),
        }
    }

    pub fn value(&self) -> $vty {
        self.value
    }

    // parse function
    pub_fn_field_parse_wrap_return!($name, {<$name>::new()});
}


    )
}


// create struct
create_varfloat_struct_and_impl!("Float4", Float4, f32, FLOAT4_SIZE);
create_varfloat_struct_and_impl!("Float8", Float8, f64, FLOAT8_SIZE);

