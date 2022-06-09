

macro_rules! parse_move_seek_or_buf_too_short{
    ($name:expr, $seek:expr, $sk:expr, $buf:expr) => ( {
        let mvseek = $seek + $sk;
        let buflen = $buf.len();
        match mvseek < buflen {
            true => mvseek,
            false => return Err(format!("field::{}.parse() buf too short, need {} but got {}.", $name, mvseek, buflen)),
        }
    })
}


macro_rules! parse_move_seek_or_return_err{
    ($tip:expr, $type:ty, $buf:expr, $seek:expr) => ({
        let res = <$type>::parse($buf, $seek);
        match res {
            Err(e) => return Err(format!("{}.prase error: {}", $tip, e)),
            Ok(res) => res,
        }
    })
}


macro_rules! pub_fn_parse_wrap_return{
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
