
// create struct Action10LockblsRelease
action_create_struct_for_common_items!(
    ACTION_KIND_10, Action10LockblsRelease,

	lockbls_id             , LockblsId, 
    release_amount         , Amount,
);


impl_Action_trait_for_common_single!( Action10LockblsRelease, self, state, _trs, {
   
    // is exists
    let hav = state.get_lockbls(&self.lockbls_id) ? ;
    if let None = hav {
        return Err(format!("lockbls {} is not exists.", self.lockbls_id.to_hex()))
    }
    let mut lockbls = hav.unwrap();
    // check
    let pendding_hei = state.pending_block_height();
    if pendding_hei < lockbls.effect_block_height {
        return Err(format!("effect_block_height be set {}", lockbls.effect_block_height))
    }
    let canrlsnum = (pendding_hei - lockbls.effect_block_height).value() as u64 / lockbls.linear_block_number.value() as u64;
    if canrlsnum == 0 {
        return Err(format!("first release Block Height is {}", lockbls.effect_block_height.value() as u64 + lockbls.linear_block_number.value() as u64))
    }
    let lock_balance = lockbls.balance_amount.clone();
    if lock_balance < self.release_amount {
        return Err(format!("balance {} not enough.", lock_balance.to_fin_string()))
    }
    // can release
    let mut canrlshac = lockbls.linear_release_amount.mul( canrlsnum ) ? ;
    let leftoverhac = lockbls.total_lock_amount.sub(&lock_balance) ? ;
    if leftoverhac < canrlshac {
        canrlshac = leftoverhac; // min amt is ok
    }
    if self.release_amount > canrlshac {
        return Err(format!("current can release max amount {} not enough.", canrlshac.to_fin_string()))
    }
    // update
    operate::hac_add(state, &lockbls.master_address, &self.release_amount) ? ;
    lockbls.balance_amount = lock_balance.sub(&self.release_amount) ? ;
    if lockbls.balance_amount.is_empty() {
        state.del_lockbls(&self.lockbls_id) ? ;
    }else{
        state.set_lockbls(&self.lockbls_id, &lockbls) ? ;
    }
    // btc move unlocked
    let is_btcmove_lock = self.lockbls_id[0] == 0;
    if is_btcmove_lock {
        let mut ttsupply = state.get_total_supply() ? ;
        ttsupply.btc_move_subsidy_unlock_successed += self.release_amount.to_mei_unsafe();
        state.set_total_supply(&ttsupply) ? ;
    }
    // ok
    Ok(())
});

