

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
    if oldsat < sat {
		return Err(format!("do sat_sub error: address {} balance {} not enough, need {}", 
            addr.to_readable(), oldsat, sat))
    }
    // do sub
    *oldsat - *sat
});



/**************************** */


pub fn sat_transfer(state: &mut dyn ChainState, addr_from: &Address, addr_to: &Address, sat: &Satoshi) -> Result<(), String> {
    if addr_from == addr_to {
		return Err("cannot trs to self".to_string())
    }
	// after 200000 height, the amount transferred to self is not allowed to be greater than the available balance!
    sat_sub(state, addr_from, sat) ? ;
    sat_add(state, addr_to, sat) ? ;
    // ok
    Ok(())
}


pub fn set_check(state: &mut dyn ChainState, addr: &Address, sat: &Satoshi) -> Result<Satoshi, String> {
    if 0 == sat.value() {
        return Err("check satoshi is cannot empty".to_string())
    }
    if let Some(bls) = state.get_balance( addr ) ? {
        if bls.satoshi >= *sat {
            return Ok(bls.satoshi)
        }
    }
    Err("address {} satoshi not enough".to_string())
}





