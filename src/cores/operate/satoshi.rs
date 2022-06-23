

macro_rules! pub_fn_satoshi_operate_common{
    ($func_name: ident, $addr:ident, $sat:ident, $oldsat:ident,  $newsatblock:block) => (

pub fn $func_name(state: &mut dyn ChainState, $addr: &Address, $sat: &Satoshi) -> Result<Satoshi, String> {
    if ! $sat.value() == 0 {
		return Err("satoshi value cannot zore".to_string())
    }
    let mut userbls;
    if let Some(b) = state.get_balance( $addr ) ? {
        userbls = b;
    } else {
        userbls = BalanceItem::new(); // empty
    }
    let $oldsat = userbls.get_satoshi();
    /* -------- */
    let newsat = $newsatblock;// operate
    /* -------- */
    // save
    userbls.set_satoshi( newsat.clone() );
    state.set_balance($addr, &userbls) ? ;
    Ok(newsat)
}

    )
}


/**************************** */

pub_fn_satoshi_operate_common!(sat_add, addr, sat, oldsat, {
    // do add
    *oldsat + *sat 
});

pub_fn_satoshi_operate_common!(sat_sub, addr, sat, oldsat, {  
    // check
    if oldsat.value() < sat.value() {
		return Err(format!("do sat_sub error: address {} balance {} not enough, need {}", 
            addr.to_readable(), oldsat.value(), sat.value()))
    }
    // do sub
    *oldsat - *sat
});



/**************************** */


pub fn sat_transfer(state: &mut dyn ChainState, addr_from: &Address, addr_to: &Address, amt: &Amount) -> Result<(), String> {
	let is_trs_to_my_self = addr_from == addr_to;
    if is_trs_to_my_self && state.pending_block_height().value() < 20_0000 {
        // you can transfer it to yourself without changing the status, which is a waste of service fees
		return Ok(()) 
    }
	// after 200000 height, the amount transferred to self is not allowed to be greater than the available balance!
    hac_sub(state, addr_from, amt) ? ;
    hac_add(state, addr_to, amt) ? ;
    // ok
    Ok(())
}


pub fn set_check(state: &mut dyn ChainState, addr: &Address, sat: &Satoshi) -> Result<Satoshi, String> {
    if 0 == sat.value() {
        return Err("check satoshi is cannot empty".to_string())
    }
    if let Some(bls) = state.get_balance( addr ) ? {
        if bls.satoshi.value() >= sat.value() {
            return Ok(bls.satoshi)
        }
    }
    Err("address {} satoshi not enough".to_string())
}





