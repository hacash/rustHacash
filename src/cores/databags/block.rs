

pub struct BlockBag {
    hash: Hash,
    body: Vec<u8>,
    obj: Box<dyn Block>
}


impl BlockBag {
    
    pub fn from_bytes(body: Vec<u8>) -> Result<BlockBag, String> {
        let (mvsk, blkobj) = blocks::parse(&body, 0) ? ;
        if mvsk != body.len() {
            return Err("body.len error".to_string())
        }
        Ok(BlockBag {
            hash: blkobj.hash().unwrap(),
            body: body,
            obj: blkobj,
        })
    }

    pub fn parse(buf: &Vec<u8>, seek: usize) -> Result<BlockBag, String> {
        let (mvsk, blkobj) = blocks::parse(buf, seek) ? ;
        Ok(BlockBag {
            hash: blkobj.hash().unwrap(),
            body: buf[seek..mvsk].to_vec(),
            obj: blkobj,
        })
    }



}
