
struct MemDB {
    pub datamaps: HashMap<Vec<u8>, Vec<u8>>,
}

impl MemDB {

    pub fn new() -> MemDB {
        MemDB{
            datamaps: HashMap::new(),
        }
    }

    pub fn get(&self, k: &Vec<u8>) -> Option<Vec<u8>> {
        let res = self.datamaps.get(k);
        match res {
            Some(d) => Some(d.clone()),
            _ => None,
        }
    }

    pub fn put(&mut self, k: Vec<u8>, v: Vec<u8>) -> Result<(), String> {
        self.datamaps.insert(k, v);
        Ok(())
    }

    pub fn delete(&mut self, k: &Vec<u8>) -> Result<(), String> {
        self.datamaps.remove(k);
        Ok(())
    }

    pub fn flush(&mut self) -> Result<(), String> {
        Ok(())
    }
}