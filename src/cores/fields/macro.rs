

macro_rules! parse_move_seek_or_buf_too_short{
    ($name:expr, $seek:expr, $sk:expr, $buf:expr) => ( {
        let mvseek = $seek + $sk;
        let buflen = $buf.len();
        match mvseek <= buflen {
            true => mvseek,
            false => return Err(format!("field::{}.parse() buf too short, need {} but got {}.", $name, mvseek, buflen)),
        }
    })
}

