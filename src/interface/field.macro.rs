

#[macro_export] 
macro_rules! pub_fn_field_parse_wrap_return{
    ($name:ty, $newcall:expr) => (
        pub fn parse(buf: &Vec<u8>, seek: usize) -> Result<(usize, $name), String> {
            let mut v = $newcall;
            let res = v.parse(buf, seek);
            match res {
                Ok(seek) => Ok((seek, v)),
                Err(e) => return Err(e),
            }
        }
    )
}


#[macro_export] 
macro_rules! field_serialize_items_concat{
    ($( $name: expr ),+) => (
        vec![
            $(
                $name.serialize(),
            )*
        ].concat()
    )
}


#[macro_export] 
macro_rules! field_parse_items_or_return_error{
    ($buf: expr, $seek: expr, $( $name: expr ),+) => ({
        let mut sk: usize = $seek ;
        $(
            sk = $name.parse($buf, sk) ? ;
        )*
        Ok(sk)
    })
}


#[macro_export] 
macro_rules! field_size_items_add_return{
    ($( $name: expr ),+) => ({
        let mut size: usize = 0;
        $(
            size += $name.size();
        )*
        size
    })
}



#[macro_export] 
macro_rules! impl_Field_trait_for_common{
    ($class: ident, $( $name: ident ),+) => (


impl Field for $class {

    fn serialize(&self) -> Vec<u8> {
        field_serialize_items_concat!($( self.$name ),*)
    }

    fn parse(&mut self, buf: &Vec<u8>, seek: usize) -> Result<usize, String> {
        field_parse_items_or_return_error!(buf, seek, $( self.$name ),*)
    }

    fn size(&self) -> usize {
        field_size_items_add_return!($( self.$name ),*)
    }

} 
    )
}