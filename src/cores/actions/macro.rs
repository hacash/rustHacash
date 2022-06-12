
macro_rules! actions_kind_define_parse_func_include{
    ( $( $kindid:ident, $kindv:tt, $class:ty ),+ ) => (

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
