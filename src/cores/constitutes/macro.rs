



/******************************************* */



macro_rules! pub_struct_define_for_if_exist{
    ($class: ident, $value: ident, $value_type: ty) => (

#[derive(Clone)]
pub struct $class {
	exists: Bool,
	$value: Option<$value_type>,
}


impl $class {

    pub fn must(v: $value_type) -> $class {
        $class {
            exists: Bool::create(true),
            $value: Some(v),
        }
    }

    pub fn is_exist(&self) -> bool {
        self.exists.check()
    }

    pub fn get_value(&self) -> Option<& $value_type> {
        match &self.$value {
            Some(v) => Some(&v),
            None => None,
        }
    }

    // parse function
    pub_fn_field_parse_wrap_return!($class, {<$class>::new()});

}

// impl Field for $class
impl_Field_trait_if_exist!($class, 
    exists,
    $value, $value_type
);




    )
}




/******************************************* */



macro_rules! pub_struct_define_for_list{
    ($class: ident, $count: ident, $count_type: ty, $value: ident, $value_type: ty) => (


#[derive(Clone)]
pub struct $class  {
	$count: $count_type,
	$value: Vec<$value_type>,
}


impl $class {

    pub fn get_count(&self) -> &$count_type {
        &self.$count
    }

    pub fn get_list(&self) -> &Vec<$value_type> {
        &self.$value
    }

    // parse function
    pub_fn_field_parse_wrap_return!($class, {$class::new()});

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


