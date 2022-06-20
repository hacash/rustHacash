
pub struct ChainState {


    // block_store: Option<Box<dyn BlockStore>>,


}


impl ChainState {
    
    pub fn new() ->ChainState {
        ChainState{
            // block_store: None,
        }
    }
    
}



impl ChainStateRead for ChainState {



    fn get_balance(&self, _addr: &Address) -> Result<BalanceItem, String> {
        Err("".to_string())
    }

    
}