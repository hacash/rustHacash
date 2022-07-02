

impl ChainState for ChainStateInstance {

    fn id(&self) -> usize {
        self.id_key
    }

	// fn fork_with_next_block(&mut self, block: & dyn Block) -> Result<ArcMutexDynChainState, String> {
    //     let base = Arc::new(Mutex::new(self));
    //     ChainStateInstance::fork_with_next_block(base, block)
    // }


    fn set_pending_block(&mut self, ptr: &BlockPtr) {
        self.basis_block = PenddingBasisBlock::Blkptr(ptr.clone());
    }

    fn set_pending_block_height(&mut self, hei: &BlockHeight) {
        self.basis_block = PenddingBasisBlock::Height(hei.clone());
    }




}