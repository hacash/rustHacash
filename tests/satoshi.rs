use hacash::cores::{coinbase::*, fields::Uint4};

/*
    cargo test --test satoshi -- --nocapture
*/

#[test]
fn t82374528374() {
    let lid = gain_lockbls_id_for_btc_move(&Uint4::from(256));
    println!("{}", lid);
}

#[test]
fn satoshis() {

    // let top_release_mei = 1048576u32;
    let mut x: u32 = 1;
    loop {
        let mei = calculate_hac_mei_release_by_btc_move_one_idx(x);
        println!("{}: {}", x, mei);
        x = x * 2;
        if x > 2 {
            x = x - 1;
        }
        if x > 2000000 {
            break;
        }
        // assert_eq!(mei, top_release_mei/x);
    }

    assert_eq!(calculate_hac_mei_release_by_btc_move(&Uint4::from(0), &Uint4::from(7)), 3145728);
    

}


