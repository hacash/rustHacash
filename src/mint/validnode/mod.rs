use crate::interface::*;
use crate::cores::blocks;
use crate::chain::state::*;
use crate::mint::blockchain::*;

use std::sync::{ Arc, Mutex};
use std::io::{self, Write};

use easy_http_request::DefaultHttpRequest;

pub struct ValidNode {

    chainstate: ArcMutexChainStateInstance,

}


impl ValidNode {


    /*
     - 179600
     
     */
    pub fn start(&self, blockdataurl: &String) -> Result<(), String>  {


        println!("request and check block height:");
        println!("        ");

        let mut curblkhei: u64 = 1;

        loop {

            // println!("{}", curblkhei);
            let url = format!("{}{}", blockdataurl, curblkhei);
            let response = DefaultHttpRequest::get_from_url_str(url).unwrap().send().unwrap();
            let bodyhexstr = String::from_utf8(response.body).unwrap();
            let body = hex::decode(bodyhexstr);
            if let Err(_) = body {
                break
            } 
            let blkbytes = body.unwrap();
            // let (block, _) = blocks::parse(&blkbytes, 0) ? ;
            
            // parse and append
            let (substate, block, _) = append_block_fork_state(self.chainstate.clone(), &blkbytes, 0) ? ;
            // cover copy
            let mut basestat = self.chainstate.lock().unwrap();
            basestat.traversal_copy(substate.clone()) ? ;
            // set pendding
            basestat.set_pending_block(&block.copy_block_ptr());
            // clean
            basestat.remove_child(substate);
            
            // ok
            // if *block.get_transaction_count() > 1i32 {
            //     print!("\n------------------{}\n", block.get_transaction_count(), );
            // }
            print!("\r{:<8} {} {}", curblkhei, block.get_transaction_count(), block.hash());
            io::stdout().flush().unwrap();
            curblkhei += 1;
            // next block
        }


        println!("\n\nall block request and check success!");



        Ok(())
    }




}


pub fn create(datadir: &String) -> Result<ValidNode, String> {

    let state = ChainStateInstance::new();

    let stateptr = Arc::new(Mutex::new(state));
    Ok(ValidNode{
        chainstate: stateptr,
    })
}





