

fn state_setup(state: &mut dyn ChainState) {

    let addr1 = Address::form_readable(&"12vi7DEZjh6KrK5PVmmqSgvuJPCsZMmpfi".to_string()).unwrap();
    let addr2 = Address::form_readable(&"1LsQLqkd8FQDh3R7ZhxC5fndNf92WfhM19".to_string()).unwrap();
    let addr3 = Address::form_readable(&"1NUgKsTgM6vQ5nxFHGz1C4METaYTPgiihh".to_string()).unwrap();
    let amt1 = Amount::new_small(1, 244);
    let amt2 = Amount::new_small(12, 244);
    state.set_balance(&addr1, &BalanceItem::from_hacash(amt2)).unwrap();
    state.set_balance(&addr2, &BalanceItem::from_hacash(amt1.clone())).unwrap();
    state.set_balance(&addr3, &BalanceItem::from_hacash(amt1)).unwrap();

}