

macro_rules! define_chain_state_operation_of_common{
    ($( $name: ident, $keyty: ty, $vtype: ty, )+) => (


pub trait ChainStateOperationRead {
    $(
        concat_idents!(fn_get = get_, $name {
            fn fn_get(&self, _: &$keyty) -> Result<$vtype, String>;
        });
    )+
}

pub trait ChainStateOperation : ChainStateOperationRead {
    $(
        concat_idents!(fn_set = set_, $name {
            fn fn_set(&mut self, _: &$keyty, _: &$vtype) -> Option<String>;
        });
        concat_idents!(fn_del = del_, $name {
            fn fn_del(&mut self, _: &$keyty) -> Option<String>;
        });
    )+
}

    )
}


// ChainStateOperation & ChainStateOperationRead
define_chain_state_operation_of_common!(
    balance, Address, BalanceItem,
);


// ChainState
pub trait ChainState : ChainStateOperation {

}

