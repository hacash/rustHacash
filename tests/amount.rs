use hacash::cores::fields::*;

/*
    cargo test --test amount -- --nocapture
*/

#[test]
fn amounts() {

    let mut amt = Amount::new();
    assert_eq!("0", amt.to_mei_string_unsafe());
    
    amt = amt.add(&Amount::new_coin(3)).unwrap();
    assert_eq!("3:248", amt.to_string());

    amt = amt.add(&Amount::new_coin(7)).unwrap();
    assert_eq!("1:249", amt.to_string());

    amt = amt.sub(&Amount::new_coin(5)).unwrap();
    assert_eq!("5:248", amt.to_string());

    amt = amt.sub(&Amount::new_small(5, 246)).unwrap();
    assert_eq!("495:246", amt.to_string());




}
