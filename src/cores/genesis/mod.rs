use crate::interface::*;
use crate::cores::fields::*;
use crate::cores::constitutes::*;
use crate::cores::transactions::*;
use crate::cores::blocks::*;

use std::mem::MaybeUninit;
use std::sync::{Mutex};

use lazy_static::lazy_static;

lazy_static! {
    static ref GENESIS_BLOCK_LOCK: Mutex<i32> = {
        let mut m = Mutex::new(0);
        m
    };
}
static mut GENESIS_BLOCK_CACHE: MaybeUninit<BlockV1> = MaybeUninit::uninit();


pub fn genesis_block() -> &'static BlockV1 {
    let mut hav = GENESIS_BLOCK_LOCK.lock().unwrap();
    unsafe{
        if *hav == 1 {
            return GENESIS_BLOCK_CACHE.assume_init_ref()
        }
    }
    // create
    let blktime = BlockTxTimestamp::from(1549250700);
    let blknoncenum = Uint4::from(160117829);
    let reward_addr = Address::form_readable(&"1271438866CSDpJUqrnchoJAiGGBFSQhjd".to_string()).unwrap();
    let mut genesis_block = BlockV1 {
        headmeta: BlockHeadMeta { 
            head: BlockHead { 
                version: Uint1::from(1), 
                height: BlockHeight::from(0), 
                timestamp: blktime, 
                prev_hash: Hash::new(), 
                mrkl_root: Hash::new(), 
                transaction_count: Uint4::from(1)
            }, 
            meta: BlockMeta { 
                nonce: Fixedbytes4::from(blknoncenum.to_bytes()), 
                difficulty: Uint4::from(0), 
                witness_stage: Fixedbytes2::new() 
            },
        },
        transactions: vec![Box::new(TransactionCoinbase{
            ty: Uint1::from(0),
            address: reward_addr,
            reward: Amount::new_coin(1),
            message: StringTrim16::from(&"hardertodobetter".to_string()),
            extend: CoinbaseExtend::new()
        })]
    };
    // edit
    genesis_block.set_mrkl_root( &genesis_block.mrklroot() );
    /*
    //ad557702fc70afaf70a855e7b8a4400159643cb5a7fc8a89ba2bce6f818a9b01
    //00000c1aaa4e6007cc58cfb932052ac0ec25ca356183f80101686172646572746f646f62657474657200
    //01 0000000000 005c57b08c 0000000000000000000000000000000000000000000000000000000000000000 ad557702fc70afaf70a855e7b8a4400159643cb5a7fc8a89ba2bce6f818a9b0100000001098b344500000000000000000 c1aaa4e6007cc58cfb932052ac0ec25ca356183f80101686172646572746f646f62657474657200
    println!("{}", genesis_block.mrklroot());
    println!("{}", hex::encode(genesis_block.transactions[0].serialize()));
    println!("{}", hex::encode(genesis_block.serialize()));
    */
    // check
    let blkhx = genesis_block.hash();
    let checkhx = Hash::from_hex(&"000000077790ba2fcdeaef4a4299d9b667135bac577ce204dee8388f1b97f7e6".to_string()).unwrap();
    if blkhx != checkhx {
        panic!("{}", format!("Genesis Block Hash Error: need {} but give {}", checkhx, blkhx))
    }
    *hav = 1; // mark
    unsafe{
        // save
        GENESIS_BLOCK_CACHE.as_mut_ptr().write(genesis_block);
        // return
        return GENESIS_BLOCK_CACHE.assume_init_ref()
    }

}

