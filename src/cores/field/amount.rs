

pub const AMOUNT_MIN_SIZE: usize = 2;

static _BIG_MEI_DIP_DIV: u64 = 1_00000000_00000000;
static _BIG_MEI_DIP_UNIT: u8 = 248 - 8 - 8; // 232

macro_rules! amount_check_data_len{
    ($self:expr, $tip:expr) => (
        if $self.dist.abs() as usize != $self.byte.len() {
            panic!("Amount.{}() dist abs is not match byte len.", $tip)
        }
    )
}

pub struct Amount {
	unit: u8,
	dist: i8,
	byte: Vec<u8>,
}

impl Amount {
    pub fn new() -> Amount {
        Amount{
            unit: 0,
            dist: 0,
            byte: Vec::new(),
        }
    }

    // parse function
    pub_fn_field_parse_wrap_return!(Amount, {Amount::new()});

}


impl fmt::Debug for Amount {

    fn fmt(&self,f: &mut fmt::Formatter) -> fmt::Result{
        write!(f,"[unit:{}, dist:{}, byte: {:?}]", self.unit, self.dist, self.byte)
    }

}


impl Clone for Amount {
    fn clone(&self) -> Amount {
        Amount{
            unit: self.unit,
            dist: self.dist,
            byte: self.byte.clone(),
        }
    }
}

// impl Copy for Amount {}

impl Field for Amount {

    fn serialize(&self) -> Vec<u8> {
        let mut resv = vec!(self.unit, self.dist as u8);
        resv.append( &mut self.byte.clone() );
        resv
    }

    fn parse(&mut self, buf: &Vec<u8>, seek: usize) -> Result<usize, String> {
        let seek1 = parse_move_seek_or_buf_too_short!("Amount", seek, 1, buf);
        self.unit = buf[seek1];
        let seek2 = parse_move_seek_or_buf_too_short!("Amount", seek1, 1, buf);
        self.dist = buf[seek2] as i8;
        let seek3 = parse_move_seek_or_buf_too_short!("Amount", seek2, self.dist.abs() as usize, buf);
        self.byte = buf[seek2..seek3].to_vec();
        amount_check_data_len!(self, "parse");
        Ok(seek3)
    }

    fn size(&self) -> usize {
        // amount_check_data_len!(self, "size");
        1 + 1 + self.dist.abs() as usize
    }

} 

// new or from
impl Amount {

    pub fn new_coin(num: u8) -> Amount {
        if num % 10 == 0 {
            panic!("{} is not support.", num)
        }
        let amt = Amount{
            unit: 248,
            dist: 1,
            byte: vec!(num),
        };
        amount_check_data_len!(amt, "new_coin");
        amt
    }

    pub fn new_small(num: i8, unit: u8) -> Amount {
        if num % 10 == 0 {
            panic!("{} is not support.", num)
        }
        let amt = Amount{
            unit,
            dist: match num < 0 { true => -1, false => 1 },
            byte: vec!(num.abs() as u8),
        };
        amount_check_data_len!(amt, "new_small");
        amt
    }

    pub fn from_i64(mei: i64, unit: u8) -> Result<Amount, String> {
        let mut amt = Amount::new();
        if mei == 0 {
            return Ok(amt);
        }
        if mei % 10 == 0 {
            return Err(format!("Amount.from_mei_i64 `{}` format error.", mei))
        }
        // parse
        let big: BigInt = FromPrimitive::from_i64(mei).unwrap();
        let (sign, bigbts) = big.to_bytes_be();
        let dlen = bigbts.len();
        if dlen > 127 {
            return Err("amount is too big".to_string());
        }
        amt.unit = unit;
        let dlen = dlen as i8;
        amt.dist = match sign == Minus { true => -dlen, false => dlen };
        // byte
        amt.byte = bigbts;
        amount_check_data_len!(amt, "from_i64");
        // ok
        return Ok(amt);
    }

    pub fn from_mei_i64(mei: i64) -> Result<Amount, String> {
        // ok
        return Amount::from_i64(mei, 248);
    }

    pub fn from_mei_string_unsafe(v: &String) -> Result<Amount, String> {
        let mayerr = ||{
            Err(format!("Amount.from_mei_string_unsafe `{}` format error.", v))
        };
        // let mut amt = Amount::new();
        let nums: Vec<&str> = v.as_str().trim().split(".").collect();
        if 1 == nums.len() {
            // int
            let ii = match v.parse::<i64>() {
                Err(_) => return mayerr(),
                Ok(i) => i,
            };
            return Amount::from_mei_i64(ii);

        }else if 2 == nums.len() {
            let ff = match v.parse::<f64>() {
                Err(_) => return mayerr(),
                Ok(f) => f,
            };
            // float
            let fdl = nums[1].trim_end_matches("0").len();
            if fdl > 8 + 8 {
                return Err("amount is too big".to_string());
            }
            let base = 10i32.pow(fdl as u32) as f64;
            let ii = (ff * base) as i64;
            return Amount::from_i64(ii, 248 - (fdl as u8));

        }else{
            return mayerr();
        }
        
    }

    pub fn from_fin_string(v: &String) -> Result<Amount, String> {
        let v = v.to_uppercase().replace("ㄜ", " ").replace("HAC", " ");
        let v = v.as_str().trim().to_string();
        let vs:Vec<&str> = v.split(":").collect();
        let mayerr = ||{
            format!("amount fin string `{}` format error.", v)
        };
        if 2 != vs.len() {
            return Err(mayerr());
        }
        let unit_v = vs[1].to_string();
        let num_v = vs[0].to_string();
        // create amt
        let mut amt = Amount::new();
        let unit = match unit_v.parse::<u32>() {
            Ok(uv) => uv,
            Err(_) => return Err(mayerr()),
        };
        amt.unit = match unit<=255 {
            true => unit as u8,
            false => return Err(mayerr()),
        };
        // num
        // let zorebigint: BigInt = FromPrimitive::from_i8(0).unwrap();
        let bignum = match BigInt::from_str_radix(&num_v, 10) {
            Ok(n) => n,
            Err(_) => return Err(mayerr()),
        };
        let (sign, nunbts) = bignum.to_bytes_be(); // bigend
        if nunbts.len() > 127 {
            return Err(mayerr());
        }
        amt.dist = nunbts.len() as i8;
        if sign == Minus {
            amt.dist = -amt.dist; // - Minus
        }
        // println!("{} {}", bignum.to_string(), sign==Minus);
        amt.byte = nunbts;
        // check
        amount_check_data_len!(amt, "to_fin_string");
        Ok(amt)
    }


}

// from / to bigint 
impl Amount {

    pub fn from_bigint( bignum: &BigInt ) -> Result<Amount, String> {
        let numstr = bignum.to_string();
        let numuse = numstr.as_str().trim_end_matches('0');
        let unit = numstr.len() - numuse.len();
        if unit > 255 {
            return Err("Amount is too wide.".to_string())
        }
        let biguse = BigInt::from_str_radix(&numuse, 10).unwrap();
        let (sign, byte) = biguse.to_bytes_be();
        let dist = byte.len();
        if dist > 127 {
            return Err("Amount is too wide.".to_string())
        }
        let mut dist = dist as i8;
        if sign == Minus {
            dist *= -1;
        }
        // amt
        let mut amt = Amount::new();
        amt.unit = unit as u8;
        amt.dist = dist;
        amt.byte = byte;
        // check
        amount_check_data_len!(amt, "from_i64");
        // ok
        Ok(amt)
    }

    pub fn to_bigint(&self) -> BigInt {
        if self.is_empty() {
            return FromPrimitive::from_u64(0).unwrap();
        }
        let mut bignum = BigInt::from_bytes_be(Plus, &self.byte[..]);
        bignum = match self.dist < 0 {
            true => bignum * -1,
            false => bignum,
        };
        let base: BigInt = FromPrimitive::from_u64(10).unwrap();
        let powv = base.pow(self.unit as u32);
        bignum * powv
    }

}

// to string 
impl Amount {

    pub fn to_fin_string(&self) -> String {
        ("ㄜ".to_owned() + self.to_string().as_str()).to_string()
    }
    pub fn to_string(&self) -> String {
        let (s1, s2, s3) = self.to_strings();
        format!("{}{}:{}", s1, s2, s3)
        
    }

    pub fn to_strings(&self) -> (String, String, String) {
        let bignum = BigInt::from_bytes_be(Plus, &self.byte[..]);
        let s1 = match self.dist < 0 {
            true => "-".to_string(),
            false => "".to_string(),
        };
        let s2 = bignum.to_string();
        let s3 = format!("{}", self.unit);
        (s1, s2, s3)
    }

    pub fn to_mei_string_unsafe(&self) -> String {
        // let mut amt = Amount::new();
        if self.is_empty() {
            return "0".to_string()
        }
        let chax = (248 - (self.unit as i32)).abs() as u32;
        if chax > 8 + 8 {
            return "0".to_string()
        }
        // num
        let num = BigInt::from_bytes_be(Plus, &self.byte[..]).to_f64().unwrap();
        // unit
        let base = 10i32.pow(chax) as f64;
        let resv = match self.unit > 248 {
            true => num * base,
            false => num / base,
        };
        // sign
        let resv = resv.to_string();
        let resv = resv.as_str();
        match self.dist < 0 {
            true => ("-".to_owned() + resv).to_string(),
            false => resv.to_string(),
        }
    }

    pub fn to_mei_or_fin_string(&self, usemei: bool) -> String {
        match usemei {
            true => self.to_mei_string_unsafe(),
            false => self.to_fin_string(),
        }
    }

}

// compute
impl Amount {

    pub fn add(&self, amt: &Amount) -> Result<Amount, String> {
        let var1 = self.to_bigint();
        let var2 = amt.to_bigint();
        let varres = var1 + var2;
        Amount::from_bigint(&varres)
    }

    pub fn sub(&self, amt: &Amount) -> Result<Amount, String> {
        let var1 = self.to_bigint();
        let var2 = amt.to_bigint();
        let varres = var1 - var2;
        Amount::from_bigint(&varres)
    }

    pub fn compress(&self, nummaxlen: usize, upper: bool) -> Result<Amount, String> {
        let mut useamt = self.clone();
        loop {
            let (_, numstr, _) = useamt.to_strings();
            if numstr.len() <= nummaxlen {
                break; // ok
            }
            let mut nnn = numstr.parse::<u64>().unwrap();
            nnn = nnn / 10;
            let unit_n = useamt.unit as u64 + 1;
            if unit_n > 255 {
                return Err(format!("`{}` compress failed.", self.to_fin_string()));
            }
            useamt.unit = unit_n as u8;
            if upper {
                nnn += 1;
            }
            let big_n: BigInt = FromPrimitive::from_u64(nnn).unwrap();
            (_, useamt.byte) = big_n.to_bytes_be();
            // next
        };
        Ok(useamt)
    }
}

// compare 
impl Amount {

    pub fn equal(&self, amt: &Amount) -> bool {
        if self.byte == amt.byte 
        && self.dist == amt.dist 
        && self.unit == amt.unit {
            return true
        }
        return false
    }

    pub fn not_equal(&self, amt: &Amount) -> bool {
        return self.equal(amt) == false;
    }

    pub fn more_than(&self, amt: &Amount) -> bool {
        if self.equal(amt) {
            return false
        }
        let var1 = self.to_bigint();
        let var2 = amt.to_bigint();
        match var1.cmp( &var2 ) {
            Greater => true,
            _ => false,
        }
    }

    pub fn less_than(&self, amt: &Amount) -> bool {
        if self.equal(amt) {
            return false
        }
        let var1 = self.to_bigint();
        let var2 = amt.to_bigint();
        match var1.cmp( &var2 ) {
            Less => true,
            _ => false,
        }
    }

}

// check
impl Amount {

    pub fn is_empty(&self) -> bool {
        if self.unit == 0 {
            return true;
        }
        if self.dist == 0 {
            return true;
        }
        return false;
    }
    pub fn is_not_empty(&self) -> bool {
        return self.is_empty() == false;
    }

    // check must be negative and cannot be zero
    pub fn is_positive(&self) -> bool {
        if self.unit == 0 {
            return false
        }
        if self.dist >= 0 {
            return false
        }
        // yes
        return true
    }   

    // check must be negative and cannot be zero
    pub fn is_negative(&self) -> bool {
        if self.unit == 0 {
            return false
        }
        if self.dist >= 0 {
            return false
        }
        // yes
        return true
    }

}