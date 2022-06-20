

pub struct TxBag {
    hash: Hash,
    hash_with_fee: Hash,
    body: Vec<u8>,
    obj: Box<dyn Transaction>
}


impl TxBag {
    
    pub fn parse(buf: &Vec<u8>, seek: usize) -> Result<TxBag, String> {
        let (mvsk, txobj) = transactions::parse(buf, seek) ? ;
        Ok(TxBag {
            hash: txobj.hash().unwrap(),
            hash_with_fee: txobj.hash_with_fee().unwrap(),
            body: buf[seek..mvsk].to_vec(),
            obj: txobj,
        })
    }

    pub fn from_bytes(body: Vec<u8>) -> Result<TxBag, String> {
        let (mvsk, txobj) = transactions::parse(&body, 0) ? ;
        if mvsk != body.len() {
            return Err("body.len error".to_string())
        }
        Ok(TxBag {
            hash: txobj.hash().unwrap(),
            hash_with_fee: txobj.hash_with_fee().unwrap(),
            body: body,
            obj: txobj,
        })
    }



}




