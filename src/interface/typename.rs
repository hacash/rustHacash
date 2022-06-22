
pub type ArcMutexDynChainState = Arc<Mutex<dyn ChainState>>;
pub type WeakArcMutexDynChainState = ArcWeak<Mutex<dyn ChainState>>;

