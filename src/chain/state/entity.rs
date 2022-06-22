

impl ChainState for ChainStateInstance {

    fn id(&self) -> usize {
        self.id_key
    }

	fn get_parent(&self) -> Option<WeakArcMutexDynChainState> {
        self.parent.clone()
    }

    fn append_child(&mut self, child: ArcMutexDynChainState) {
        self.childs.insert(child.lock().unwrap().id(), child.clone());
    }



}