
macro_rules! transactions_type_define_parse_func_include{
    ( $( $trstype:ident, $typev:expr, $class:ty ),+ ) => (

        // kind define
        $(
            pub const $trstype: u8 = $typev;
        )+

        // parse func
        pub fn parse(buf: &Vec<u8>, seek: usize) -> Result<(usize, Box<dyn TransactionReadOnly>), String> {
            println!("----- transactions.parse start ------ {}", seek);
            let (_, typev) = parse_move_seek_or_return_err!("transactions.parse", Uint1, buf, seek);
            let ty = typev.value() as u8;
            println!("----- transactions. typev.value()------ {} {}", seek, typev.value());
            match ty {
            $(
                $trstype => {
                    let (mvsk, trs) = <$class>::parse(buf, seek) ? ;
                    Ok((mvsk, Box::new(trs)))
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

	ty: Uint1,

	timestamp: BlockTxTimestamp,
	address: Address,
	fee: Amount,

    actions: DynListActionMax65535,

    signs: SignListMax65535,

	multisign_mark: Uint2,

}

impl_Field_trait_for_common!( 0, $class,
    ty,
    timestamp,
    address, 
    fee,
    actions,
    signs,
    multisign_mark,
);



impl $class {

    pub fn new() -> $class {
        $class {
            ty: Uint1::new(),
            timestamp: BlockTxTimestamp::new(),
            address: Address::new(),
            fee: Amount::new(),
            actions: DynListActionMax65535::new(),
            signs: SignListMax65535::new(),
            multisign_mark: Uint2::new()
        }
    }

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

impl TransactionReadOnly for $class {

    fn get_type(&self) -> u8 {
        $tyid
    }

    fn get_address(&self) -> Option<&Address> {
        Some(&self.address)
    }

	fn get_signs(&self) -> Option<&Vec<Sign>> {
        Some(self.signs.get_list())
    }

    fn get_fee(&self) -> Option<&Amount> {
        Some(&self.fee)
    }
    fn get_fee_of_miner_real_received(&self) -> Option<Amount> {
        let mut fee = self.get_fee().unwrap().clone();
        let burn90 = self.is_burning_90_persent_tx_fee();
        if false==burn90 || 0==fee.unit() {
            return Some(fee)
        }
        // change amt to be 90%
        fee.unit_sub(1);
        Some(fee)
    }
    fn get_message(&self) -> Option<&StringTrim16> {
        None
    }
	fn get_actions(&self) -> Option<&Vec<Box<dyn Action>>> {
        let list = self.actions.get_list();
        Some(&list)
    }

    /* */

    fn hash(&mut self) -> Option<Hash> {
        // calculate hash no fee
        let stuff = self.serialize_for_sign_no_fee();
        let hx = x16rs::calculate_hash(stuff);
        let hx = Hash::from(hx);
        Some(hx)
    }
    fn hash_with_fee(&mut self) -> Option<Hash> {
        // calculate hash with fee
        let stuff = self.serialize_for_sign();
        let hx = x16rs::calculate_hash(stuff);
        let hx = Hash::from(hx);
        Some(hx)
    }

    fn fee_purity(&self) -> u64 {
        0
    }
    fn verify_all_signs(&self) -> bool {
        false
    }
    fn verify_target_signs(&self, _: &Vec<Address>) -> bool {
        false
    }

    /* */

    fn is_burning_90_persent_tx_fee(&self) -> bool {
        let alis = self.actions.get_list().iter();
        for i in alis {
            if i.is_burning_90_persent_tx_fee() {
                return true
            }
        }
        false
    }

    fn request_sign_addresses(&self) -> Option<Vec<Address>> {
        let mut addrarys = vec![vec![self.get_address().unwrap().clone()]];
        let alis = self.actions.get_list().iter();
        for i in alis {
            addrarys.push(i.request_sign_addresses())
        }
        let _addrs = addrarys.concat();
        // drop repeat
        // addrarys.sort();
        None
    }

	// change chain state
	fn write_in_chain_state(&self, _: &mut dyn ChainStateOperation) -> Option<String> {
        panic!("never call this!")
    }
}

    )
}

