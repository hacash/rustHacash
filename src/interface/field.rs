
pub trait Field {
    fn serialize(&self) -> Vec<u8>;
    fn parse(&mut self, _: &Vec<u8>, _: usize) -> Result<usize, String>;
    fn size(&self) -> usize;
    fn describe(&self) -> String; // json value style
}

