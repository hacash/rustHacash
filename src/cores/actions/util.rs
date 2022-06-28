

fn check_vaild_store_item_key(tip: &str, kobj: impl AsRef<[u8]>, ksize: usize) -> Result<(), String> {
    let key = kobj.as_ref();
    if key.len() != ksize {
        return Err(format!("{} check key {} size fail.", tip, hex::encode(key)))
    }
    if key[0] == 0 || key[ksize-1] == 0 {
        return Err(format!("{} check key {} format fail.", tip, hex::encode(key)))
    }
    Ok(())
}