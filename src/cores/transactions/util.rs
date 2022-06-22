

fn verify_one_sign(hash: &Hash, addr: &Address, signs: &Vec<Sign>) -> bool {
    for sig in signs {
        let curpubkey = sig.publickey.value();
        let curaddr = Account::get_address_by_public_key(curpubkey);
        if *addr == curaddr {
            return Account::verify_signature(&hash.value(), &curpubkey, &sig.signature.value())
        }
    }
    false
}