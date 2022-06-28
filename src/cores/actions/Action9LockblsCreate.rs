
// create struct Action9LockblsCreate
action_create_struct_for_common_items!(
    ACTION_KIND_9, Action9LockblsCreate,

	lockbls_id                  , LockblsId    ,  // Linear lock ID
	payment_address             , Address      ,  // Payment address
	master_address              , Address      ,  // Main address (claim)
	effect_block_height         , BlockHeight  ,  // Effective (start) block
	linear_block_number         , Uint3        ,  // Number of stepping blocks < 17000000 about 160 years
	total_stock_amount          , Amount       ,  // Total deposit limit
	linear_release_amount       , Amount       ,  // Limit released each time
);


impl_Action_trait_for_common!( Action9LockblsCreate, self,
    { false }, // is_burning_90_persent_tx_fee

    {   // request_sign_addresses
        [self.payment_address.clone()]
    }, 
    
    state, _trs, {

        // check
        if self.lockbls_id.size() != LOCKBLS_ID_SIZE || self.lockbls_id[0] == 0 || self.lockbls_id[LOCKBLS_ID_SIZE-1] == 0 {
            return Err("lockbls_id format error.".to_string())
        }
        // is exists
        let hav = state.get_lockbls(&self.lockbls_id) ? ;
        if let Some(_) = hav {
            return Err(format!("lockbls {} already exists.", self.lockbls_id.to_hex()))
        }
        if self.linear_block_number < 288 {
            return Err("linear_block_number cannot less 288.".to_string())
        }
        if !self.total_stock_amount.is_positive() || !self.linear_release_amount.is_positive() {
            return Err("TotalStockAmount or LinearReleaseAmount error.".to_string())
        }
        if self.total_stock_amount < self.linear_release_amount {
            return Err("LinearReleaseAmount cannot more than TotalStockAmount.".to_string())
        }
        // sub hac
        operate::hac_sub(state, &self.payment_address, &self.total_stock_amount) ? ;
        // save
        let lockbls = LockblsItem{
            master_address: self.master_address.clone(),
            effect_block_height: state.pending_block_height(),
            linear_block_number: self.linear_block_number,
            linear_release_amount: self.linear_release_amount.clone(),
            total_lock_amount: self.total_stock_amount.clone(),
            balance_amount: self.total_stock_amount.clone()
        };
        state.set_lockbls(&self.lockbls_id, &lockbls) // set lock amt
});

