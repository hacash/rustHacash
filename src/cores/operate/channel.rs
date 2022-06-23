
// close default
pub fn close_channel_of_default(state: &mut dyn ChainState, channel_id: &ChannelId, paychan: &ChannelItem) -> Result<(), String> {
    close_channel_with_distribution(state, channel_id, paychan, 
    &paychan.left_bill.hacsat.amount,
    &paychan.right_bill.hacsat.amount,
    &paychan.left_bill.hacsat.satoshi.to_real(),
    &paychan.right_bill.hacsat.satoshi.to_real(),
    false)
}


// close
pub fn close_channel_with_distribution(state: &mut dyn ChainState, channel_id: &ChannelId, 
    paychan: &ChannelItem, 
    left_amt: &Amount,  right_amt: &Amount,
    left_sat: &Satoshi, right_sat: &Satoshi,
    is_final_closed: bool) -> Result<(), String> {
    // check
    let left_addr = &paychan.left_bill.address;
    let right_addr = &paychan.right_bill.address;
	if left_amt.is_negative() || right_amt.is_negative() {
		return Err("channel distribution amount cannot be negative.".to_string())
	}
    let ttamt = paychan.left_bill.hacsat.amount.add(&paychan.right_bill.hacsat.amount)?;
    if  left_amt.add(right_amt)?.not_equal(&ttamt) {
        return Err("HAC distribution amount must equal with lock in.".to_string())
    }
    let ttsat = paychan.left_bill.hacsat.satoshi.to_real() + paychan.right_bill.hacsat.satoshi.to_real();
    if *left_sat + *right_sat != ttsat {
        return Err("BTC distribution amount must equal with lock in.".to_string())
    }
    // total supply
    let mut ttsupply = state.get_total_supply() ? ;
    ttsupply.channel_opening_count -= 1u32;
    // do close
    if ttamt.is_positive() {
        // calculate_interest
        let (newamt1, newamt2) = coinbase::calculate_interest_of_height(
            &state.pending_block_height(),  &paychan.belong_height, 
            paychan.interest_attribution, left_amt, right_amt
        ) ? ;
        let ttnewhac = newamt1.add(&newamt2) ?;
        if ttnewhac.less_than( &ttamt ) {
            return Err("interest calculate error!".to_string())
        }
        let ttiesthac =  ttnewhac.sub(&ttamt) ? .to_mei_unsafe();
        ttsupply.channel_interest += ttiesthac;
        ttsupply.channel_located_hac -= ttamt.to_mei_unsafe();
        if newamt1.is_positive() {
            hac_add(state, left_addr, &newamt1) ? ;
        }
        if newamt2.is_positive() {
            hac_add(state, right_addr, &newamt2) ? ;
        }
    }
    if ttsat.value() > 0 {
        ttsupply.channel_located_sat -= ttsat;
        if left_sat.value() > 0 {
            sat_add(state, left_addr, left_sat) ? ;
        }
        if right_sat.value() > 0 {
            sat_add(state, right_addr, right_sat) ? ;
        }
    }
    // save channel
    let distribution = ClosedDistributionDataOptional::must(ClosedDistributionData{
        left_bill: HacSat{
            amount: left_amt.clone(),
            satoshi: SatoshiOptional::must(left_sat.clone()),
        }
    });
    let mut savechan = paychan.clone();
    savechan.status = match is_final_closed {
        true => CHANNEL_STATUS_FINAL_ARBITRATION_CLOSED,
        false => CHANNEL_STATUS_AGREEMENT_CLOSED,
    };
    savechan.if_distribution = distribution;
    // save
    state.set_channel(channel_id, &savechan) ? ;
    // ok
    state.set_total_supply(&ttsupply)
}
