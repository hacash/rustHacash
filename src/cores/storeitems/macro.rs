


macro_rules! pub_struct_store_item_define_common{
    ($class: ident, $( $name: ident, $type: ty, )+) => (



pub struct $class {

    $(
        pub $name: $type
    ),+

}


impl $class {

    pub fn new() -> $class {
        $class{
            $(
                $name: <$type>::new()
            ),+
        }
    }

    // parse function
    pub_fn_field_parse_wrap_return!($class, {<$class>::new()});

    // get & set
    $(
        concat_idents!(fn_name = get_, $name {
            pub fn fn_name(&self) -> &$type {
                &self.$name
            }
        });
        concat_idents!(fn_name = set_, $name {
            pub fn fn_name(&mut self, v: $type) {
                self.$name = v;
            }
        });
    )+

}


impl Clone for $class {
    fn clone(&self) -> $class {
        $class{
            $(
                $name: self.$name.clone()
            ),+
        }
    }
}


// impl Field for $class
impl_Field_trait_for_common!(0, $class, 
    $(
        $name,
    )+
);


    )

}

