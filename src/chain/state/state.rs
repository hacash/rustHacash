

static CHAIN_STATE_ID_KEY_AUTO: AtomicUsize = AtomicUsize::new(0);

#[derive(Clone)]
pub struct ChainStateConfig {
    mode_debug_test: bool,
    mode_database_rebuild: bool,
    mode_check_btcmove: bool, 
}

impl ChainStateConfig {
    pub fn new() -> ChainStateConfig {
        ChainStateConfig{
            mode_debug_test: false,
            mode_database_rebuild: false,
            mode_check_btcmove: false, 
        }
    }
}



enum PenddingBasisBlock {
    Height(BlockHeight),
    Blkptr(BlockPtr),
}

pub type ArcMutexChainStateInstance = Arc<Mutex<ChainStateInstance>>;
pub type WeakArcMutexChainStateInstance = ArcWeak<Mutex<ChainStateInstance>>;


pub struct ChainStateInstance {

    id_key: usize,

    config: ChainStateConfig,

    basis_block: PenddingBasisBlock,

    leveldb: RefCell<DB>,
    delkeys: HashMap<Vec<u8>, ()>,

    parent: Option<WeakArcMutexChainStateInstance>,
    childs: HashMap<usize, ArcMutexChainStateInstance>,

}

impl ChainStateInstance {

    pub fn generate_id() -> usize {
        // rand::random::<u64>()
        CHAIN_STATE_ID_KEY_AUTO.fetch_add(1, Ordering::Relaxed);
        match CHAIN_STATE_ID_KEY_AUTO.compare_exchange(usize::MAX, 1, Ordering::Acquire, Ordering::Relaxed) {
            Ok(_) => 1,
            _ => CHAIN_STATE_ID_KEY_AUTO.load(Ordering::SeqCst),
        }
    }

    fn from_leveldb(db: DB) -> ChainStateInstance {
        ChainStateInstance{
            id_key: ChainStateInstance::generate_id(),
            config: ChainStateConfig::new(),
            basis_block: PenddingBasisBlock::Height(BlockHeight::from(0)),
            parent: None,
            childs: HashMap::new(),
            leveldb: RefCell::new(db),
            delkeys: HashMap::new(),
        }
    }

    pub fn new() -> ChainStateInstance {

        let opt = rusty_leveldb::in_memory();
        let tempdb = DB::open("state", opt).unwrap();

        ChainStateInstance::from_leveldb(tempdb)
    }


    pub fn create_from_disk(datadir: &String) -> ChainStateInstance {

        let opt = Options::default();
        let diskdb = DB::open(datadir, opt).unwrap();

        ChainStateInstance::from_leveldb(diskdb)
    }


    pub fn destory(&mut self) {

    }


    // fork


	pub fn fork(base: ArcMutexChainStateInstance) -> ChainStateInstance { 
        let opt = rusty_leveldb::in_memory();
        let tempdb = DB::open("childstate", opt).unwrap();
        let bsstat = base.lock().unwrap();
        let basis_height = bsstat.pending_block_height();
        let basis_block = match bsstat.pending_block_hash() {
            None => PenddingBasisBlock::Height(basis_height),
            Some(hx) => PenddingBasisBlock::Blkptr(BlockPtr{
                height: basis_height,
                hash: hx.clone()
            }),
        };
        ChainStateInstance{
            id_key: ChainStateInstance::generate_id(),
            config: ChainStateConfig{
                mode_debug_test: bsstat.is_debug_test_mode(),
                mode_database_rebuild: bsstat.is_database_rebuild_mode(),
                mode_check_btcmove: bsstat.is_check_btcmove(),
            },
            basis_block: basis_block,
            leveldb: RefCell::new(tempdb),
            delkeys: HashMap::new(),
            parent: Some( Arc::downgrade(&base) ),
            childs: HashMap::new(),
        }
    }

    // fork
	// pub fn fork_sub_child(base: ArcMutexDynChainState) -> ArcMutexDynChainState { 
    //     Arc::new(Mutex::new(ChainStateInstance::fork(base)))
    // }

	pub fn fork_with_next_block(base: ArcMutexChainStateInstance, newblock: & dyn Block) -> Result<ArcMutexChainStateInstance, String> { 
        // println!("------------------------fork_with_next_block");
        let mut child = ChainStateInstance::fork(base.clone());
        child.basis_block = PenddingBasisBlock::Blkptr(newblock.copy_block_ptr());
        // write state
        // println!("------------------------write_in_chain_state");
        let _ = newblock.write_in_chain_state(&mut child) ? ;
        // println!("------------------------writ end");
        // set 
        let child_ptr = Arc::new(Mutex::new(child));
        let ptr2 = child_ptr.clone();
        base.lock().unwrap().append_child( child_ptr );
        // ok
        Ok(ptr2)
    }

    fn append_child(&mut self, child: ArcMutexChainStateInstance) {
        self.childs.insert(child.lock().unwrap().id(), child.clone());
    }

    pub fn remove_child(&mut self, child: ArcMutexChainStateInstance) {
        let cid = child.lock().unwrap().id();
        self.childs.remove(&cid);
    }

	fn get_parent(&self) -> Option<WeakArcMutexChainStateInstance> {
        self.parent.clone()
    }

    // copy cover
	pub fn traversal_copy(&mut self, sub: ArcMutexChainStateInstance) -> Result<(), String> {
        let sub = sub.lock().unwrap();
        // iter copy
        let mut items = sub.leveldb.borrow_mut().new_iter().unwrap();
        let mut basedb = self.leveldb.borrow_mut();
        loop {
            let item = items.next();
            if let None = item {
                break // end
            }
            let kv = item.unwrap();
            // println!("traversal_copy - {}, {}", hex::encode(&kv.0), hex::encode(&kv.1));
            basedb.put(&kv.0, &kv.1).unwrap();
        }
        basedb.flush().unwrap();
        // delete
        for (i, _) in &sub.delkeys {
            basedb.delete(&i).unwrap();
        }
        // copy del mark
        if let Some(_) = self.parent {
            for (i, _) in &sub.delkeys {
                self.delkeys.insert(i.clone(), ());
            }
        }
        // ok
        Ok(())
    }

    
}


// static func
impl ChainStateInstance {


    fn mark_del_key(&mut self, stuff: Vec<u8>) {
        self.delkeys.insert(stuff, ());
    }

    fn makey(prefix: u8, stuff: Vec<u8>) -> Vec<u8> {
        let rl = 1 + stuff.len();
        let mut reskey = Vec::<u8>::with_capacity(rl);
        reskey.push(prefix);
        reskey.extend(stuff.iter().copied()); 
        reskey
    } 


}



