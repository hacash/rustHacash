use crate::interface::*;
use crate::cores::fields::*;
use crate::cores::blocks;
use crate::cores::constitutes::*;
use crate::cores::storeitems::*;
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
    pub fn start_request_all_blocks(&self, blockdataurl: &String) -> Result<(), String>  {


        println!("request and check block height:");
        println!("        ");

        let mut curblkhei: u64 = 1;
        {
            let mut state = self.chainstate.lock().unwrap();
            let last_blk = state.get_latest_block_intro() ? ;
            let curhei = last_blk.headmeta.head.height.value();
            if curhei > 1 {
                curblkhei = curhei + 1; // next block
                // set pendding
                state.set_pending_block(&BlockPtr{
                    height: last_blk.headmeta.head.height.clone(),
                    hash: last_blk.hash.clone(),
                });
            }
        }

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

    pub fn read_some_data(&self) -> Result<(), String> {

        let mut basestat = self.chainstate.lock().unwrap();

        let addresses = [
            "18K88nMJgB3pbA1auPtCsxZXAfeZDQpKhf",
            "188255WMwyMQrHEFEsFTxCUvtnCJuNGGay",
            "18Yt6UbnDKaXaBaMPnBdEHomRYVKwcGgyH",
            "13RnDii79ypWayV8XkrBFFci29cHtzmq3Z",
            "1KdapWe5nHnzNsTPXDD2HbiCDqEmpz2vm7",
            "1Q1SYthS3dMWp68DFRvUVPBpLPynr7EuMk",
            "1K6UYiwNfdPwu1XjmDVK5SAfAjmbYXoLmH",
            "1FT26nEJaf3EgZEUXb8YUnB6zfjbJv8ubG",
            "12pbJdDmvZcRs3BhJnJ1RKuiHE48mVJS71"
        ];

        for a in addresses {
            // println!("{}", a);
            let addr = Address::form_readable(&a.to_string()).unwrap();
            let bls = basestat.get_balance(&addr) ? ;
            if let Some(bls) = bls {
                println!("{}: {}", a, bls.describe());
            }
        }
        
        // ok
        Ok(())
    }



}


pub fn create(datadir: &String) -> Result<ValidNode, String> {

    let state = ChainStateInstance::create_from_disk(datadir);

    let stateptr = Arc::new(Mutex::new(state));
    Ok(ValidNode{
        chainstate: stateptr,
    })
}





