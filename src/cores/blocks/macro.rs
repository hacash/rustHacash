
macro_rules! block_version_define_parse_func_include{
    ( $( $version:ident, $vernum:tt, $class:ty ),+ ) => (

        // kind define
        $(
            pub const $version: u8 = $vernum;
        )+

        // parse func
        pub fn parse(buf: &Vec<u8>, seek: usize) -> Result<(Box<dyn Block>, usize), String> {
            println!("--------------------------------{}", seek);
            let (typev, _) = parse_move_seek_or_return_err!("blocks.parse", Uint1, buf, seek);
            let ty = typev.value() as u8;
            println!("-----{} {:?} {} {}", seek, buf[0], typev.value(), ty);
            match ty {
            $(
                $version => {
                    let (block, mvsk) = <$class>::parse(buf, seek) ? ;
                    Ok((Box::new(block), mvsk))
                },
            )+
            _ => Err(format!("Block version <{}> not find.", ty))
            }
        }

    )
}

#[allow(unused_macros)]
macro_rules! impl_Action_trait_for_common{
    ($class: ty, $codeblock1: block, $codeblock2: block, $param_state: ident, $param_store: ident, $codeblock3: block) => (



impl Action for $class {

    fn kind(&self) -> u16 {
        <$class>::get_kind()
    }

    fn is_burning_90_persent_tx_fee(&self) -> bool {
        $codeblock1
    }

    fn request_sign_addresses(&self) -> Vec<Address> {
        $codeblock2
    }

	fn write_in_chain_state(&self, $param_state: &mut dyn ChainState) -> Option<String> {
        $codeblock3
    }


}

    )
}








macro_rules! impl_Field_for_common_block {
    ( $blockname: ident, $( $item:ident, $vtype:ty, )+ ) => (



impl Field for $blockname {

    fn new() -> $blockname {
        $blockname {
            $(
                $item: <$vtype>::new(),
            )+
            // body
            transactions: Vec::new(),
        }
    }


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
        for i in 0..self.get_transaction_count().value() {
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
        let trsnum = self.get_transaction_count().value();
        println!("--- self.transaction_count.get_value() {}", trsnum);
        for i in 0..trsnum {
            println!("--- +++ for _ in 0..trsnum iiiiiiiiiiiiiiiiiiiiiiiii {} {}", sk, i);
            let(obj, mvsk) = transactions::parse(buf, sk) ? ;
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
        for i in 0..self.get_transaction_count().value() {
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
        let count = self.get_transaction_count().value() as usize;
        for i in 0..count {
            trs.push( self.transactions[i].describe() );
        }
        items.push(format!("\"transactions\":[{}]", trs.join(",")));
        format!("{{{}}}", items.join(","))

    }

} 





    )

}