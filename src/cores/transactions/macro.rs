
macro_rules! transactions_type_define_parse_func_include{
    ( $( $trstype:ident, $typev:expr, $class:ty ),+ ) => (

        // kind define
        $(
            pub const $trstype: u8 = $typev;
        )+

        // parse func
        pub fn parse(buf: &Vec<u8>, seek: usize) -> Result<(Box<dyn Transaction>, usize), String> {
            // println!("----- transactions.parse start ------ {}", seek);
            let (typev, _) = parse_move_seek_or_return_err!("transactions.parse", Uint1, buf, seek);
            let ty = typev.value() as u8;
            // println!("----- transactions. typev.value()------ {} {}", seek, typev.value());
            match ty {
            $(
                $trstype => {
                    let (trs, mvsk) = <$class>::parse(buf, seek) ? ;
                    Ok((Box::new(trs), mvsk))
                },
            )+
            _ => Err(format!("Transaction Type <{}> not find.", ty))
            }
        }

    )
}



macro_rules! create_common_transaction_struct{
    ($tyid: expr, $class:ident) => (

pub struct $class {

	pub ty: Uint1,

	pub timestamp: BlockTxTimestamp,
	pub address: Address,
	pub fee: Amount,

    pub actions: DynListActionMax65535,

    pub signs: SignListMax65535,

	pub multisign_mark: Uint2,

}

impl_Field_trait_for_common!( 0, $class,
    ty, Uint1,
    timestamp, BlockTxTimestamp,
    address, Address,
    fee, Amount,
    actions, DynListActionMax65535,
    signs, SignListMax65535,
    multisign_mark, Uint2,
);



impl $class {

    fn serialize_for_sign(&self) -> Vec<u8> {
        field_serialize_items_concat!(
            self.ty,
            self.timestamp,
            self.address, 
            self.fee,
            self.actions
        )
    }

    fn serialize_for_sign_no_fee(&self) -> Vec<u8> {
        field_serialize_items_concat!(
            self.ty,
            self.timestamp,
            self.address, 
            self.actions
        )
    }
    
    // parse function
    pub_fn_field_parse_wrap_return!($class, {<$class>::new()});
}

impl TransactionRead for $class {

    fn get_type(&self) -> u8 {
        $tyid
    }

    fn get_timestamp(&self) -> &BlockTxTimestamp { 
        &self.timestamp
    }

    fn get_address(&self) -> &Address {
        &self.address
    }

	fn get_signs(&self) -> &Vec<Sign> {
        self.signs.get_list()
    }

    fn get_fee(&self) -> &Amount {
        &self.fee
    }
    fn get_fee_of_miner_real_received(&self) -> Amount {
        let mut fee = self.get_fee().clone();
        let burn90 = self.is_burning_90_persent_tx_fee();
        if false==burn90 || 0==fee.unit() {
            return fee
        }
        // change amt to be 90%
        fee.unit_sub(1);
        fee
    }

    fn get_action_count(&self) ->&Uint2 {
        self.actions.get_count()
    }

	fn get_actions(&self) -> &Vec<Box<dyn Action>> {
        self.actions.get_list()
    }

    /* */

    fn hash(&self) -> Hash {
        // calculate hash no fee
        let stuff = self.serialize_for_sign_no_fee();
        let hx = x16rs::calculate_hash(stuff);
        let hx = Hash::from(hx);
        hx
    }
    fn hash_with_fee(&self) -> Hash {
        // calculate hash with fee
        let stuff = self.serialize_for_sign();
        let hx = x16rs::calculate_hash(stuff);
        let hx = Hash::from(hx);
        hx
    }

    fn fee_purity(&self) -> u64 {
        0
    }

    /* */

    fn is_burning_90_persent_tx_fee(&self) -> bool {
        for act in self.get_actions() {
            if act.is_burning_90_persent_tx_fee() {
                return true
            }
        }
        false
    }

    fn request_sign_addresses(&self) -> HashMap<Address, ()> {
        let mut needsignaddrs = HashMap::new();
        needsignaddrs.insert(self.get_address().clone(), ());
        for act in self.get_actions() {
            for (k, _) in act.request_sign_addresses() {
                needsignaddrs.insert(k, ());
            }
        }
        needsignaddrs
    }

}

impl Transaction for $class {


    // verify_all_signs
    fn verify_all_signs(&self) -> Result<(), String> {
        let nnsigadrs = self.request_sign_addresses();
        self.verify_target_signs(&nnsigadrs)
    }

    fn verify_need_signs(&self, addrs: &Vec<Address>) -> Result<(), String> { 
        let mut sigadrs = HashMap::new();
        for adr in addrs {
            sigadrs.insert(adr.clone(), ());
        }
        self.verify_target_signs(&sigadrs)
     }

    fn verify_target_signs(&self, nnsigadrs: &HashMap<Address, ()>) -> Result<(), String> {
        let txty = self.get_type();
        let hash = self.hash();
        let main_addr = self.get_address();
        let signs = self.get_signs();
        // type 1
        if TRANSACTION_TYPE_1 == txty {
            // there have a sign problem in tx type 1 !
            // all address use tx hash
            for (addr, _) in nnsigadrs {
                if ! verify_one_sign(&hash, &addr, signs) {
                    return Err(format!("address {} verify signature fail", addr.to_readable())) // fail
                }
            }
            return Ok(())
        }
        let hash_fee = self.hash_with_fee();
        // type 2 and other
        for (addr, _) in nnsigadrs {
            let mut usehash = &hash;
            if addr == main_addr {
                usehash = &hash_fee;
            }
            if ! verify_one_sign(usehash, &addr, signs) {
                return Err(format!("address {} verify signature fail", addr.to_readable()))
            }
        }
        return Ok(())
    }

	// change chain state
	fn write_in_chain_state(&self, state: &mut dyn ChainState) -> Result<(), String> {
        let block_height = state.pending_block_height().value();
        // check bug
        if TRANSACTION_TYPE_1 == self.get_type() && block_height > 37000 {
            return Err("Transaction type<1> be discard DO_NOT_USE_WITH_BUG".to_string())
        }
        // max fee size
        if block_height > 200000 {
            if self.get_fee().size() > 2+4 {
                return Err("BlockHeight more than 20w trs.Fee.Size() must less than 6 bytes.".to_string())
            }
        }
        // call actions
        self.actions.check() ? ;
        for act in self.get_actions() {
            act.write_in_chain_state(state, self) ? ;
        }
        // deduct fee
        operate::hac_sub( state, self.get_address(), self.get_fee() ) ? ;
        // ok
        Ok(())
    }

}

    )
}

