

#[macro_export] 
macro_rules! pub_type_prefix_value_check_impl_Field_trait{
    ($class: ident, $num_name: ident, $num_type: ty, $value_type: ty, $back_offset: expr, $over_check: block) => (

pub struct $class<T:Clone + PartialOrd + FieldNumber, V:Field> {
    $num_name: T,
    field_val: Option<V>,
}

impl<T: Clone + PartialOrd + FieldNumber, V:Field> Field for $class<T, V> {

    fn new() -> $class<T, V> {
        $class::<T, V> { 
            $num_name: T::new(),
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
        let mut $num_name: T = T::new();
        $num_name.parse(buf, ofstsk) ? ;
        self.$num_name = $num_name.clone();
        if $over_check {
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

