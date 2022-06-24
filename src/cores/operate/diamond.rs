

macro_rules! pub_fn_diamond_operate_common{
    ($func_name: ident, $addr:ident, $hacd:ident, $oldhacd:ident, $newhacdblock:block) => (

pub fn $func_name(state: &mut dyn ChainState, $addr: &Address, $hacd: &DiamondNumber) -> Result<DiamondNumber, String> {
    let mut userbls;
    if let Some(b) = state.get_balance( $addr ) ? {
        userbls = b;
    } else {
        userbls = BalanceItem::new(); // empty
    }
    let $oldhacd = userbls.get_diamond();
    /* -------- */
    let newhacd = $newhacdblock;// operate
    /* -------- */
    // save
    userbls.set_diamond( newhacd.clone() );
    state.set_balance($addr, &userbls) ? ;
    Ok(newhacd)
}

    )
}


/**************************** */

pub_fn_diamond_operate_common!(hacd_add, addr, hacd, oldhacd, {
    // do add
    *oldhacd + *hacd
});

pub_fn_diamond_operate_common!(hacd_sub, addr, hacd, oldhacd, {  
    // check
    if oldhacd.value() < hacd.value() {
		return Err(format!("do hacd_sub error: address {} diamond {} not enough, need {}", 
            addr.to_readable(), oldhacd.value(), hacd.value()))
    }
    // do sub
    *oldhacd - *hacd
});



/**************************** */


pub fn hacd_transfer(state: &mut dyn ChainState, addr_from: &Address, addr_to: &Address, hacd: &DiamondNumber) -> Result<(), String> {
	let is_trs_to_my_self = addr_from == addr_to;
    if is_trs_to_my_self {
		return Err("cannot trs to self".to_string())
    }
    hacd_sub(state, addr_from, hacd) ? ;
    hacd_add(state, addr_to, hacd) ? ;
    // ok
    Ok(())
}
