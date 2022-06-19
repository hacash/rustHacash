

macro_rules! pub_struct_define_for_common{
    ($class: ident, $( $value: ident, $value_type: ty,)+) => (

// 
pub struct $class {
    $(
        $value: $value_type,
    )+
}

impl $class {

    pub fn new() -> $class {
        $class {
            $(
                $value: <$value_type>::new(),
            )+
        }
    }

    // parse function
    pub_fn_field_parse_wrap_return!($class, {$class::new()});

}


impl Clone for $class {
    fn clone(&self) -> $class {
        $class {
            $(
                $value: self.$value.clone(),
            )+
        }
    }
}


// impl Field for Sign
impl_Field_trait_for_common!(0, $class, 
    $(
        $value,
    )+
);


    )
}



/******************************************* */



macro_rules! pub_struct_define_for_if_exist{
    ($class: ident, $value: ident, $value_type: ty) => (

// if have $value
pub struct $class {
	is_exist: Bool,
	$value: Option<$value_type>,
}


impl $class {

    pub fn new() -> $class {
        $class {
            is_exist: Bool::create(false),
            $value: None,
        }
    }

    // parse function
    pub_fn_field_parse_wrap_return!($class, {<$class>::new()});

}


impl Clone for $class {
    fn clone(&self) -> $class {
        $class {
            is_exist: self.is_exist.clone(),
            $value: self.$value.clone(),
        }
    }
}


// impl Field for $class
impl_Field_trait_if_exist!($class, 
    is_exist,
    $value, $value_type
);




    )
}




/******************************************* */



macro_rules! pub_struct_define_for_list{
    ($class: ident, $count: ident, $count_type: ty, $value: ident, $value_type: ty) => (


pub struct $class  {
	$count: $count_type,
	$value: Vec<$value_type>,
}


impl $class {

    pub fn new() -> $class {
        $class {
            $count: <$count_type>::new(),
            $value: Vec::new(),
        }
    }

    pub fn get_count(&self) -> &$count_type {
        &self.$count
    }

    pub fn get_list(&self) -> &Vec<$value_type> {
        &self.$value
    }

    // parse function
    pub_fn_field_parse_wrap_return!($class, {$class::new()});

}


impl Clone for $class {
    fn clone(&self) -> $class {
        $class {
            $count: self.$count.clone(),
            $value: self.$value.clone(),
        }
    }
}



// impl Field for $class
impl_Field_trait_for_list!($class, 
    $count,
    $count_type, 
    $value,
    $value_type
);



    )
}


