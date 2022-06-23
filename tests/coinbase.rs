// use hacash::interface::*;
use hacash::cores::{coinbase::*, fields::Amount};

/*
    cargo test --test coinbase -- --nocapture
*/

#[test]
fn coinbases() {

    let bamt = Amount::new_coin(1);
    let bamt2 = Amount::from_mei_i64(12345).unwrap();

    let namt = calculate_interest( &bamt, &bamt,1, 10).unwrap();
    assert_eq!(namt.to_string(), "1001:245");

    let namt = calculate_interest( &bamt, &bamt,42, 1).unwrap();
    assert_eq!(namt.to_string(), "100420861:240");

    let namt = calculate_interest( &bamt2, &bamt2,42, 1).unwrap();
    assert_eq!(namt.to_string(), "1239695543209:240");

}
