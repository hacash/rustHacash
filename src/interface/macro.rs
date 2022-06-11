

#[macro_export] 
macro_rules! parse_move_seek_or_return_err{
    ($tip:expr, $type:ty, $buf:expr, $seek:expr) => ({
        let res = <$type>::parse($buf, $seek);
        match res {
            Err(e) => return Err(format!("{}.prase error: {}", $tip, e)),
            Ok(res) => res,
        }
    })
}


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
macro_rules! field_describe_items_wrap{
    ($( $key: expr; $var: expr ),+) => ({
        let mut list = Vec::new();
        $(
            list.push( format!("\"{}\":{}", $key, $var.describe()) );
        )*
        let desstr = list.join(",");
        format!("{{{}}}", desstr)
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

    fn describe(&self) -> String {
        field_describe_items_wrap!($( stringify!($name); self.$name ),*)
    }

} 
    )
}





#[macro_export] 
macro_rules! impl_Field_trait_if_exist{
    ($class: ident, $mark: ident, $value: ident) => (


impl Field for $class {

    fn serialize(&self) -> Vec<u8> {
        let mut resdt = self.$mark.serialize();
        if self.$mark.check() {
            let mut vardt = self.$value.serialize();
            resdt.append(&mut vardt);
        }
        resdt
    }

    fn parse(&mut self, buf: &Vec<u8>, seek: usize) -> Result<usize, String> {
        let mut seek = self.$mark.parse(buf, seek) ? ;
        if self.$mark.check() {
            seek = self.$value.parse(buf, seek) ? ;
        }
        Ok(seek)
    }

    fn size(&self) -> usize {
        let mut size = self.$mark.size();
        if self.$mark.check() {
            size += self.$value.size();
        }
        size
    }

    fn describe(&self) -> String {
        format!("{{\"{}\":{},\"{}\":{}}}",
            stringify!($mark),
            self.$mark.describe(),
            stringify!($value),
            self.$value.describe()
        )
    }

} 
    )
}




#[macro_export] 
macro_rules! impl_Field_trait_for_list{
    ($class: ident, $count_type: ident, $value_type: ident, $count: ident, $vec_list: ident) => (


impl Field for $class {

    fn serialize(&self) -> Vec<u8> {
        let mut resdt = self.$count.serialize();
        let count = self.$count.value() as usize;
        for i in 0..count {
            let mut vardt = self.$vec_list[i].serialize();
            resdt.append(&mut vardt);
        }
        resdt
    }

    fn parse(&mut self, buf: &Vec<u8>, seek: usize) -> Result<usize, String> {
        let mut seek = self.$count.parse(buf, seek) ? ;
        let count = self.$count.value() as usize;
        self.$vec_list = Vec::new();
        for _ in 0..count {
            let obj: $value_type;
            (seek, obj) = <$value_type>::parse(buf, seek) ? ;
            self.$vec_list.push(obj);
        }
        Ok(seek)
    }

    fn size(&self) -> usize {
        let mut size = self.$count.size();
        let count = self.$count.value() as usize;
        for i in 0..count {
            size += self.$vec_list[i].size();
        }
        size
    }

    fn describe(&self) -> String {
        let mut items = Vec::new();
        let count = self.$count.value() as usize;
        for i in 0..count {
            items.push( self.$vec_list[i].describe() );
        }
        format!("{{\"{}\":{},\"{}\":[{}]}}",
            stringify!($count),
            self.$count.describe(),
            stringify!($vec_list),
            items.join(",")
        )
    }

} 
    )
}



/*************** actions ****************/


#[macro_export] 
macro_rules! action_create_struct_for_common_items{
    ($kindid: expr, $class: ident, $( $k: ident, $ty:ty),+) => (

pub struct $class {
    $(
        $k: $ty
    ),*
}


impl $class {

    pub fn new() -> $class {
        $class {
            $(
                $k: <$ty>::new()
            ),*
        }
    }

    const fn get_kind() -> u16 {
        $kindid
    }

    // parse function
    pub_fn_field_parse_wrap_return!($class, {$class::new()});

}


// impl Field for $class
impl_Field_trait_for_common!($class, 
    $(
        $k
    ),*
);



    )
}





#[macro_export] 
macro_rules! impl_Action_trait_for_common{
    ($class: ty, $codeblock1: block, $codeblock2: block, $codeblock3: block) => (



impl Action for $class {

    fn kind(&self) -> u16 {
        <$class>::get_kind()
    }

    fn is_burning_90_persent_tx_fee(&self) -> bool {
        $codeblock1
    }

    fn request_sign_addresses(&self) -> Vec<Address> {
        $codeblock2
    }

	fn write_in_chain_state(&self, _state: &mut dyn ChainStateOperation) -> ActionStateWriteInReturnType {
        $codeblock3
    }


}

    )
}







