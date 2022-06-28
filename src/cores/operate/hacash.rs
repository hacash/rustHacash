

macro_rules! pub_fn_hacash_operate_common{
    ($func_name: ident, $addr:ident, $amt:ident, $oldhac:ident,  $newamtblock:block) => (

pub fn $func_name(state: &mut dyn ChainState, $addr: &Address, $amt: &Amount) -> Result<Amount, String> {
    if ! $amt.is_positive() {
		return Err("amount value is not positive".to_string())
    }
    let mut userbls;
    if let Some(b) = state.get_balance( $addr ) ? {
        userbls = b;
    } else {
        userbls = BalanceItem::new(); // empty
    }
    let $oldhac = userbls.get_hacash();
    /* -------- */
    let newamt = $newamtblock;// operate
    /* -------- */
    if newamt.size() > 11 {
		return Err("amount size over 11 can not to store".to_string())
    }
    // save
    userbls.set_hacash( newamt.clone() );
    state.set_balance($addr, &userbls) ? ;
    Ok(newamt)
}

    )
}


/**************************** */

pub_fn_hacash_operate_common!(hac_add, addr, amt, oldhac, {
    // do add
    oldhac.add( amt ) ? 
});

pub_fn_hacash_operate_common!(hac_sub, addr, amt, oldhac, {  
    // check
    if oldhac < amt {
		return Err(format!("do hac_sub error: address {} balance {} not enough, need {}", 
            addr.to_readable(), oldhac.to_fin_string(), amt.to_fin_string()))
    }
    // do sub
    oldhac.sub( amt ) ?
});



/**************************** */


pub fn hac_transfer(state: &mut dyn ChainState, addr_from: &Address, addr_to: &Address, amt: &Amount) -> Result<(), String> {
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


pub fn hac_check(state: &mut dyn ChainState, addr: &Address, amt: &Amount) -> Result<Amount, String> {
    if ! amt.is_positive() {
        return Err("check amount is cannot empty".to_string())
    }
    if let Some(bls) = state.get_balance( addr ) ? {
        if bls.hacash >= *amt {
            return Ok(bls.hacash)
        }
    }
    Err("address {} balance not enough".to_string())
}







