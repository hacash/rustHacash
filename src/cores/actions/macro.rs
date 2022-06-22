
macro_rules! actions_kind_define_parse_func_include{
    ( $( $kindid:ident, $kindv:expr, $class:ty, )+ ) => (

        // kind define
        $(
            pub const $kindid: u16 = $kindv;
        )+

        // include mod files
        $(
            include!( stringify!($class.rs) );
        )+

        // parse func
        pub fn parse(buf: &Vec<u8>, seek: usize) -> Result<(usize, Box<dyn Action>), String> {
            let (_, kindv) = parse_move_seek_or_return_err!("actions.parse", Uint2, buf, seek);
            let kdv = kindv.value() as u16;
            match kdv {
            $(
                $kindid => {
                    let (mvsk, act) = <$class>::parse(buf, seek) ? ;
                    Ok((mvsk, Box::new(act)))
                },
            )+
            _ => Err(format!("Action kind <{}> not find.", kdv))
            }
        }

    )
}





/*************** actions ****************/


#[macro_export] 
macro_rules! action_create_struct_for_common_items{
    ($kindid: expr, $class: ident, $( $k: ident, $ty:ty, )+) => (

pub struct $class {
    kind: Uint2,
    $(
        $k: $ty
    ),*
}


impl $class {

    pub fn new() -> $class {
        $class {
            kind: Uint2::new(),
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
impl_Field_trait_for_common!(0, $class, 
    $(
        $k,
    )+
);



    )
}



macro_rules! impl_Action_trait_for_common{
    ($class: ty, $codeblock1: block, $codeblock2: block, $param_state: ident, $param_trs: ident, $param_self: tt, $codeblock3: block) => (

impl Action for $class {

    fn kind(&self) -> u16 {
        <$class>::get_kind()
    }

    fn is_burning_90_persent_tx_fee(&self) -> bool {
        $codeblock1
    }

    fn request_sign_addresses(&self) -> HashMap<Address, bool> {
        $codeblock2
    }

	fn write_in_chain_state(&$param_self, $param_state: &mut dyn ChainState, $param_trs: & dyn Transaction) -> Result<bool, String> {
        $codeblock3
    }


}

    )
}


macro_rules! impl_Action_trait_for_common_single{
    ($class: ty, $param_state: ident, $param_trs: ident, $param_self: tt, $codeblock3: block) => (

    impl_Action_trait_for_common!( $class, 

        // is_burning_90_persent_tx_fee
        { false },
    
        // request_sign_addresses
        { HashMap::new() },
    
        // write_in_chain_state
        // _state &mut dyn ChainStateOperation -> Result<bool, String>
        $param_state, $param_trs, $param_self, $codeblock3
    
    );
    
        

    )
}



