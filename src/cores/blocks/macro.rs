
macro_rules! block_version_define_parse_func_include{
    ( $( $version:ident, $vernum:tt, $class:ty ),+ ) => (

        // kind define
        $(
            pub const $version: u8 = $vernum;
        )+

        // parse func
        pub fn parse(buf: &Vec<u8>, seek: usize) -> Result<(usize, Box<dyn BlockReadOnly>), String> {
            println!("--------------------------------{}", seek);
            let (_, typev) = parse_move_seek_or_return_err!("blocks.parse", Uint1, buf, seek);
            let ty = typev.value() as u8;
            println!("-----{} {:?} {} {}", seek, buf[0], typev.value(), ty);
            match ty {
            $(
                $version => {
                    let (mvsk, block) = <$class>::parse(buf, seek) ? ;
                    Ok((mvsk, Box::new(block)))
                },
            )+
            _ => Err(format!("Block version <{}> not find.", ty))
            }
        }

    )
}







macro_rules! impl_Field_for_common_block {
    ( $blockname: ident, $( $item:ident ),+ ) => (



impl Field for BlockV1 {

    fn serialize(&self) -> Vec<u8> {
        let mut bts = vec![];
        $(
            bts.push( self.$item.serialize() );
        )+
        // bts.push( self.version.serialize() );
        // bts.push( self.height.serialize() );
        // bts.push( self.timestamp.serialize() );
        // bts.push( self.prev_hash.serialize() );
        // bts.push( self.mrkl_root.serialize() );
        // bts.push( self.transaction_count.serialize() );
        // bts.push( self.nonce.serialize() );
        // bts.push( self.difficulty.serialize() );
        // bts.push( self.witness_stage.serialize() );
        for i in 0..self.transaction_count.get_value() {
            let bt = self.transactions[i as usize].as_ref().serialize();
            bts.push(bt);
        }
        bts.concat()
    }

    fn parse(&mut self, buf: &Vec<u8>, seek: usize) -> Result<usize, String> {
        let mut sk: usize = seek;
        $(
            println!("----------- {} {} {}", stringify!($item), sk, buf[sk]);
            sk = self.$item.parse(buf, sk) ? ;
            println!("-------next v  =  {} {}", buf[sk], buf[sk+1]);
        )+
        // sk = self.version.parse(buf, sk) ? ;
        // sk = self.height.parse(buf, sk) ? ;
        // sk = self.timestamp.parse(buf, sk) ? ;
        // sk = self.prev_hash.parse(buf, sk) ? ;
        // sk = self.mrkl_root.parse(buf, sk) ? ;
        // sk = self.transaction_count.parse(buf, sk) ? ;
        // sk = self.nonce.parse(buf, sk) ? ;
        // sk = self.difficulty.parse(buf, sk) ? ;
        // sk = self.witness_stage.parse(buf, sk) ? ;
        self.transactions = Vec::new();
        let trsnum = self.transaction_count.get_value();
        println!("--- self.transaction_count.get_value() {}", trsnum);
        for i in 0..trsnum {
            println!("--- +++ for _ in 0..trsnum iiiiiiiiiiiiiiiiiiiiiiiii {} {}", sk, i);
            let(mvsk, obj) = transactions::parse(buf, sk) ? ;
            println!("--- +++ for _ in 0..trsnum xxxxxxxxxxxxxxxxxxxxxxxxx {}", mvsk);
            sk = mvsk;
            self.transactions.push(obj);
        }
        println!("Ok(sk)Ok(sk)Ok(sk)Ok(sk)Ok(sk)Ok(sk)Ok(sk) {} ", sk);
        Ok(sk)
    }

    fn size(&self) -> usize {
        let mut sznum = 0;
        $(
            sznum += self.$item.size();
        )+
        // sznum += self.version.size();
        // sznum += self.height.size();
        // sznum += self.timestamp.size();
        // sznum += self.prev_hash.size();
        // sznum += self.mrkl_root.size();
        // sznum += self.transaction_count.size();
        // sznum += self.nonce.size();
        // sznum += self.difficulty.size();
        // sznum += self.witness_stage.size();
        for i in 0..self.transaction_count.get_value() {
            sznum += self.transactions[i as usize].as_ref().size();
        }
        sznum
    }

    fn describe(&self) -> String {
        let mut items = Vec::new();
        $(
            items.push(format!("\"{}\":{}", stringify!($item), self.$item.describe()));
        )+
        // items.push(format!("\"version\":{}", self.version.describe()));
        // items.push(format!("\"height\":{}", self.height.describe()));
        // items.push(format!("\"timestamp\":{}", self.timestamp.describe()));
        // items.push(format!("\"prev_hash\":{}", self.prev_hash.describe()));
        // items.push(format!("\"mrkl_root\":{}", self.mrkl_root.describe()));
        // items.push(format!("\"transaction_count\":{}", self.transaction_count.describe()));
        // items.push(format!("\"nonce\":{}", self.nonce.describe()));
        // items.push(format!("\"difficulty\":{}", self.difficulty.describe()));
        // items.push(format!("\"witness_stage\":{}", self.witness_stage.describe()));
        let mut trs = Vec::new();
        let count = self.transaction_count.value() as usize;
        for i in 0..count {
            trs.push( self.transactions[i].describe() );
        }
        items.push(format!("\"transactions\":[{}]", trs.join(",")));
        format!("{{{}}}", items.join(","))

    }

} 





    )

}