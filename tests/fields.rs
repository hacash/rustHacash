use hacash::cores::field::Amount;

/*
    cargo test --test fields -- --nocapture
*/

#[test]
fn amount() {


    let amt1 = Amount::new_small(123, 248);
    let amt2 = Amount::from_mei_string_unsafe(&"123".to_string()).unwrap();
    let amt3 = Amount::from_fin_string(&"HAC-1234567812345678:240".to_string()).unwrap();
    let amt4 = Amount::from_mei_string_unsafe(&"-12345678.12345678".to_string()).unwrap();
    let amt5 = Amount::from_fin_string(&"ㄜ1:240".to_string()).unwrap();
    let amt6 = Amount::from_mei_string_unsafe(&"0.00000001".to_string()).unwrap();


    assert_eq!(amt1.to_mei_string_unsafe(), amt2.to_mei_string_unsafe());
    assert_eq!(amt1.to_fin_string(),        amt2.to_fin_string());

    assert_eq!(amt3.to_mei_string_unsafe(), amt4.to_mei_string_unsafe());
    assert_eq!(amt3.to_fin_string(),        amt4.to_fin_string());

    assert_eq!(amt5.to_mei_string_unsafe(), amt6.to_mei_string_unsafe());
    assert_eq!(amt5.to_fin_string(),        amt6.to_fin_string());
    
    assert_eq!(amt1.to_fin_string(), "ㄜ123:248");
    assert_eq!(amt2.to_mei_string_unsafe(), "123");
    assert_eq!(amt3.to_mei_string_unsafe(), "-12345678.12345678");
    assert_eq!(amt4.to_fin_string(), "ㄜ-1234567812345678:240");
    assert_eq!(amt5.to_mei_string_unsafe(), "0.00000001");
    assert_eq!(amt6.to_fin_string(), "ㄜ1:240");

    // compress
    assert_eq!(amt1.compress(1, false).unwrap().to_fin_string(), "ㄜ1:250");
    assert_eq!(amt1.compress(2, true).unwrap().to_fin_string(), "ㄜ13:249");
    assert_eq!(amt3.compress(8, false).unwrap().to_fin_string(), "ㄜ-12345678:248");
    assert_eq!(amt3.compress(4, true).unwrap().to_fin_string(), "ㄜ-1235:252");



    let sss = Amount::new_small(127, 255).to_bigint().to_string();
    assert_eq!(258, sss.len());


    println!("{}, {}", sss.len(), sss)





}


