use concat_idents::concat_idents;

use crate::cores::fields::*;
use crate::cores::constitutes::*;

use crate::interface::*;


include!("macro.rs");

include!("balance.rs");
include!("diamond.rs");
include!("diamond_smelt.rs");
include!("lockbls.rs");
include!("channel.rs");
include!("satoshi_genesis.rs");
include!("total_supply.rs");
include!("transaction.rs");

include!("lend_diamond.rs");
include!("lend_bitcoin.rs");
include!("lend_users.rs");




