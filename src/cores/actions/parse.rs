
macro_rules! actions_parse_func{
    ( $( $kindid:tt, $class:ty ),+ ) => (

        pub fn parse(buf: &Vec<u8>, seek: usize) -> Result<(usize, Box<dyn Action>), String> {
            let (mvseek, kindv) = parse_move_seek_or_return_err!("actions.parse", Uint2, buf, seek);
            let kdv = kindv.value() as u16;
            match kdv {
            $(
                $kindid => {
                    let (mvsk, atc) = <$class>::parse(buf, mvseek) ? ;
                    Ok((mvsk, Box::new(atc)))
                },
            )*
            _ => Err(format!("Action kind <{}> not find.", kdv))
            }
        }

    )
}

// parse func
actions_parse_func!(
    ACTION_KIND_1_HAC_TRS, Action1HacTrs
);

