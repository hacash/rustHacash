

#[macro_export] 
macro_rules! pub_type_prefix_value_check_impl_Field_trait{
    ($class: ident, $num_name: ident, $num_type: ty, $value_type: ty, $back_offset: expr, $over_check: block) => (

pub struct $class<T:Clone + PartialOrd + FieldNumber, V: Clone + Field> {
    $num_name: T,
    field_val: Option<V>,
}

impl<T: Clone + PartialOrd + FieldNumber, V: Clone + Field> $class<T, V> {
    pub fn to_real(&self) -> Option<V> {
        match &self.field_val {
            Some(d) => Some(d.clone()),
            _ => None,
        }
    }
}

impl<T: Clone + PartialOrd + FieldNumber, V: Clone + Field> Field for $class<T, V> {

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
        // println!("pub_type_prefix_value_check_impl_Field_trait: {}", $num_name.get_value());
        self.$num_name = $num_name.clone();
        if $over_check {
            let mut val: V = V::new();
            let newseek = val.parse(buf, seek) ? ;
            self.field_val = Some(val);
            // println!("$over_check newseek: {}", newseek);
            Ok(newseek)
        }else{
            // println!("oldseek: {}", seek);
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

