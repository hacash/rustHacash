
pub trait Field {
    fn new() -> Self where Self: Sized;
    fn serialize(&self) -> Vec<u8>;
    fn parse(&mut self, _: &Vec<u8>, _: usize) -> Result<usize, String>;
    fn size(&self) -> usize;
    fn describe(&self) -> String; // json value style

}

pub trait FieldNumber : Field {
    fn get_value(&self) -> u64;
    fn set_value(&mut self, _: u64);
}

pub trait ToHex {
    fn to_hex(&self) -> String;
}

pub trait ToVecU8 {
    fn to_vec(&self) -> Vec<u8>;
}

