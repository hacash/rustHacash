pub const DIAMOND_NAME_VALID_CHARS: &[u8; 16]  = b"WTYUIAHXVMEKBSZN";

pub type DiamondName = Fixedbytes6;
pub type DiamondNumber = Uint3;

impl DiamondName {
    pub fn name(&self) -> String {
        self.to_string()
    }

    #[allow(dead_code)]
    fn describe(&self) -> String {
        format!("\"{}\"", self.name())
    }

    pub fn is_diamond_name_string(stuff: impl AsRef<[u8]>) -> bool {
        let v = stuff.as_ref();
        if v.len() != 6 {
            return false
        }
        // check in array
        for i in v {
            let mut ok = false;
            for a in DIAMOND_NAME_VALID_CHARS {
                if i == a {
                    ok = true;
                    break
                }
            }
            if !ok {
                return false
            }
        }
        true
    }
}

