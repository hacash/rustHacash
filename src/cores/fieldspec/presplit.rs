

macro_rules! pub_type_prefix_value_check_impl_Field_trait{
    ($class: ident, $num_type: ty, $value_type: ty, $back_offset: expr, $over_check: expr) => (

pub struct $class<T:FieldNumber, V:Field> {
    check_num: T,
    field_val: Option<V>,
}

impl<T:FieldNumber, V:Field> Field for $class<T, V> {

    fn new() -> $class<T, V> {
        $class::<T, V> { 
            check_num: T::new(),
            field_val: None,
        }
    }

    fn serialize(&self) -> Vec<u8> {
        match &self.field_val {
            Some(d) => d.serialize(),
            _ => vec![],
        }
    }

    fn parse(&mut self, buf: &Vec<u8>, seek: usize) -> Result<usize, String> {
        let ofst = $back_offset as usize;
        if ofst  > seek {
            return Err("cannot parse prefix_value_check_field of buf seek too small".to_string())
        }
        let ofstsk = seek - ofst;
        let mut num: T = T::new();
        num.parse(buf, ofstsk) ? ;
        if num.get_value() as u64 > $over_check as u64 {
            let mut val: V = V::new();
            let newseek = val.parse(buf, ofstsk) ? ;
            self.field_val = Some(val);
            Ok(newseek)
        }else{
            Ok(seek)
        }
    }

    fn size(&self) -> usize {
        match &self.field_val {
            Some(d) => d.size(),
            _ => 0,
        }
    }

    fn describe(&self) -> String {
        match &self.field_val {
            Some(d) => d.describe(),
            _ => "\"\"".to_string(),
        }
    }

} 
    )
}



/******************************* */


/*

// Dig out diamonds
type Action_4_DiamondCreate struct {
	Diamond  fields.DiamondName   // Diamond literal wtyuiahxvmekbszn
	Number   fields.DiamondNumber // Diamond serial number for difficulty check
	PrevHash fields.Hash          // Previous block hash containing diamond
	Nonce    fields.Bytes8        // random number
	Address  fields.Address       // Account
	// Customer message
	CustomMessage fields.Bytes32
}
*/

pub_type_prefix_value_check_impl_Field_trait!(
    DiamondCreateExtendMsg, 
    DiamondNumber, Fixedbytes32,
    3+32+8+21, 20000
);