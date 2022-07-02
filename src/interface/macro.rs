
static NEVER_CALL_THIS: &str = "never call this"; 

#[macro_export] 
macro_rules! panic_never_call_this{
()=>( panic!("{}", NEVER_CALL_THIS) )
}

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
        pub fn parse(buf: &Vec<u8>, seek: usize) -> Result<($name, usize), String> {
            let mut v = $newcall;
            let res = v.parse(buf, seek);
            match res {
                Ok(seek) => Ok((v, seek)),
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
    ($classid: expr; $( $key: expr, $var: expr ),+ ) => ({
        let mut list = Vec::new();
        if $classid > 0 {
            list.push( format!("\"{}\":{}", "clid", $classid) );
        }
        $(
            list.push( format!("\"{}\":{}", $key, $var.describe()) );
        )*
        let desstr = list.join(",");
        format!("{{{}}}", desstr)
    })
}



#[macro_export] 
macro_rules! impl_Field_trait_for_common{
    ($cln: expr, $class: ident, $( $name: ident, $vtype: ty, )+) => (


impl Field for $class {

    fn new() -> $class {
        $class {
            $(
                $name: <$vtype>::new(),
            )+
        }
    }

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
        field_describe_items_wrap!( $cln; $( stringify!($name), self.$name ),*)
    }

} 
    )
}





#[macro_export] 
macro_rules! impl_Field_trait_if_exist{
    ($class: ident, $mark: ident, $value: ident, $value_type: ty) => (


impl Field for $class {

    fn new() -> $class {
        $class {
            $mark: Bool::create(false),
            $value: None,
        }
    }

    fn serialize(&self) -> Vec<u8> {
        let mut resdt = self.$mark.serialize();
        if self.$mark.check() {
            let mut vardt = self.$value.as_ref().unwrap().serialize();
            resdt.append(&mut vardt);
        }
        resdt
    }

    fn parse(&mut self, buf: &Vec<u8>, seek: usize) -> Result<usize, String> {
        let mut seek = self.$mark.parse(buf, seek) ? ;
        // println!("impl_Field_trait_if_exist -- {} {}", seek, self.$mark.check());
        if self.$mark.check() {
            let (var, mvsk) = <$value_type>::parse(buf, seek) ? ;
            self.$value = Some(var);
            seek = mvsk
        }
        // println!("impl_Field_trait_if_exist end seek {}", seek);
        Ok(seek)
    }

    fn size(&self) -> usize {
        let mut size = self.$mark.size();
        if self.$mark.check() {
            size += self.$value.as_ref().unwrap().size();
        }
        size
    }

    fn describe(&self) -> String {
        let mut valstr = "null".to_string();
        if let Some(v) = &self.$value {
            valstr = v.describe()  
        }
        format!("{{\"{}\":{},\"{}\":{}}}",
            stringify!($mark),
            self.$mark.describe(),
            stringify!($value),
            valstr
        )
    }

} 
    )
}




#[macro_export] 
macro_rules! impl_Field_trait_for_list{
    ($class: ident, $count: ident, $count_type: ty, $vec_list: ident, $value_type: ty) => (


impl Field for $class {

    fn new() -> $class {
        $class {
            $count: <$count_type>::new(),
            $vec_list: Vec::new(),
        }
    }

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
            (obj, seek) = <$value_type>::parse(buf, seek) ? ;
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


/**************************** */


macro_rules! pub_struct_field_define_for_common{
    ($class: ident, $( $value: ident, $value_type: ty,)+) => (

// 
#[derive(Clone)]
pub struct $class {
    $(
        pub $value: $value_type,
    )+
}

impl $class {

    // parse function
    pub_fn_field_parse_wrap_return!($class, {$class::new()});

}


// impl Field for Sign
impl_Field_trait_for_common!(0, $class, 
    $(
        $value, $value_type,
    )+
);


    )
}







