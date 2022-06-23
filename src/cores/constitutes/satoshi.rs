

pub_struct_define_for_if_exist!(SatoshiOptional, satoshi, Satoshi);


impl SatoshiOptional {

    pub fn to_real(&self) -> Satoshi {
        if self.is_exist() {
            if let Some(d) = &self.satoshi {
                return d.clone()
            }
        }
        // create new
        Satoshi::from(0)
    }
}