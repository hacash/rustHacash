// use hacash::cores::field::Address;
use hacash::cores::account::{ Account };

/*
    cargo test --test account -- --nocapture
*/

#[test]
fn account() {

    let acc = Account::create_by_password("123456".to_string()).unwrap();
    assert_eq!(acc.readable(), "1MzNY1oA3kfgYi75zquj3SRUPYztzXHzK9");

    let acc = Account::create_by_password("123456789".to_string()).unwrap();
    assert_eq!(acc.readable(), "1P6DHQYjP6WygqTCzwXpwo7TxWqhA1SgVY");

    let acc = Account::create_by_password("123456789abcdefg".to_string()).unwrap();
    assert_eq!(acc.readable(), "1Jfea4js1xovV5WB7JrYia3Yt6UXVtv9We");

}
