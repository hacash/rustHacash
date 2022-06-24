
// start with the 20001st diamond and enable the 32-bit MSG byte
pub const DIAMOND_ABOVE_NUMBER_OF_CREATE_BY_CUSTOM_MESSAGE: DiamondNumber = DiamondNumber::from(2_0000);

// define special field
pub_type_prefix_value_check_impl_Field_trait!(
    DiamondCreateCustomerMsg, 
    number, DiamondNumber, Fixedbytes32,
    3+32+8+21, { number.get_value() as u64 > DIAMOND_ABOVE_NUMBER_OF_CREATE_BY_CUSTOM_MESSAGE.value() as u64 }
);


// create struct Action4DiamondCreate
action_create_struct_for_common_items!(
    ACTION_KIND_4, Action4DiamondCreate,

	diamond              , DiamondName                 ,   // diamond literal wtyuiahxvmekbszn
	number               , DiamondNumber               ,   // diamond serial number for difficulty check
	prev_hash            , Hash                        ,   // previous block hash containing diamond
	nonce                , Fixedbytes8                 ,   // random number
	address              , Address                     ,   // account
	// customer message                                   
	custom_message       , DiamondCreateCustomerMsg<DiamondNumber, Fixedbytes32>      ,

);


impl_Action_trait_for_common_single!( Action4DiamondCreate, self, state, trs, {
   
    let act_number = self.number.value();
    let act_prev_hash = self.prev_hash.value();
    let act_nonce = self.nonce.value();
    let act_address = self.address.value();
    let act_custom_message = self.custom_message.serialize();

    // mine
    let (sha3hx, mediumhx, diahx) = x16rs::mine_diamond(act_number as u32, &act_prev_hash, &act_nonce, &act_address, &act_custom_message);

    // check
    let mut is_do_complete_check = true;
    if state.is_debug_test_mode() || state.is_database_rebuild_mode() {
        is_do_complete_check = false;
    }
    if is_do_complete_check {

        let belongactionnum = trs.get_action_count().value();
        if belongactionnum != 1 {
            return Err(format!("diamond create tx need only one action but got {} actions", belongactionnum).to_string())
        }

        let pdblkhei = state.pending_block_height().value();
        if pdblkhei % 5 != 0 {
            return Err(format!("[#BACKTOPOOL] diamond must be in block height multiple of 5").to_string())
        }

        let prevdia = state.get_latest_diamond() ? ;
        let prevdianum = prevdia.number.value();
        if prevdianum + 1 != act_number {
            return Err(format!("diamond nember error, need {} but got {}", prevdianum + 1, act_number).to_string())
        }
        if prevdianum > 0 {
            if prevdia.contain_block_hash != self.prev_hash {
                return Err(format!("diamond prev_hash error, need {} but got {}", 
                    prevdia.contain_block_hash.to_hex(), self.prev_hash.to_hex(), ).to_string())
            }
        }
        // diamond str
        let (diaok, dianame) = x16rs::check_diamond_hash_result(diahx);
        if ! diaok {
            return Err(format!("diamond hash result {} not a valid diamond name", String::from_utf8(diahx.to_vec()).unwrap()).to_string())
        }
        let dianame = dianame.unwrap();
        let dn = DiamondName::from(dianame);
        if dn != self.diamond {
            return Err(format!("diamond name need {} but got {}", dn.to_string(), self.diamond.to_string() ).to_string())
        }




    }





    
    Ok(())
});

