type Bool = Uint1;

impl Copy for Bool {}

impl Bool {
    pub fn set(&mut self, v: bool) {
        self.bytes = [match v { true => 1u8, false => 0u8 }];
    }
    pub fn create(v: bool) -> Bool {
        let mut var = Bool::new();
        var.set(v);
        var
    }
    pub fn check(&self) -> bool {
        match self.value() {
            0u8 => false,
            _ => true,
        }
    }

    fn describe(&self) -> String {
        format!("{}", match self.check() {
            true => "true",
            false => "false"
        } )
    }
    
}