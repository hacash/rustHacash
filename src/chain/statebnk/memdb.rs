
struct MemDB {
    datamaps: HashMap<Vec<u8>, Vec<u8>>,
}

impl MemDB {

    pub fn get(&mut self, k: &Vec<u8>) -> Result<Option<Vec<u8>>, String> {
        Ok(datamaps[k])
    }

    pub fn put(&mut self, k: Vec<u8>, v: Vec<u8>) -> Result<(), String> {
        datamaps.insert(k, v);
        Ok(())
    }

    pub fn delete(&mut self, k: &Vec<u8>) -> Result<(), String> {
        datamaps.remove(k);
        Ok(())
    }

    pub fn flush(&mut self) -> Result<(), String> {
        Ok(())
    }
}