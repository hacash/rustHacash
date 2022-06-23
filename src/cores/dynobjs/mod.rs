use crate::cores::fields::{ Uint2 };
use crate::interface::{ Action, Field, FieldNumber };
use crate::cores::actions;



// dyn obj list

macro_rules! create_dyn_obj_list_struct_by_call_new{
    ($class: ident, $county: ty, $traity: ident, $parseobjfunc: path) => (

pub struct $class {
    count: $county,
    vlist: Vec<Box<dyn $traity>>
}

impl $class {

    pub fn new() -> $class {
        $class {
            count: <$county>::new(),
            vlist: Vec::new(),
        }
    }

    pub fn parse(buf: &Vec<u8>, seek: usize) -> Result<(usize, $class), String> {
        let mut v = $class::new();
        let res = v.parse(buf, seek);
        match res {
            Ok(seek) => Ok((seek, v)),
            Err(e) => return Err(e),
        }
    }

    pub fn get_count(&self) -> &$county {
        &self.count
    }

    pub fn get_list(&self) -> &Vec<Box<dyn $traity>> {
        &self.vlist
    }

}


impl Field for $class {

    fn new() -> $class {
        $class {
            count: <$county>::from(0),
            vlist: Vec::new(),
        }
    }

    fn serialize(&self) -> Vec<u8> {
        let mut bts = vec![];
        let bt1 = self.count.serialize();
        bts.push(bt1);
        for i in 0..self.count.get_value() {
            let bt = self.vlist[i as usize].as_ref().serialize();
            bts.push(bt);
        }
        bts.concat()
    }

    fn parse(&mut self, buf: &Vec<u8>, seek: usize) -> Result<usize, String> {
        let mut sk: usize = seek;
        sk = self.count.parse(buf, sk) ? ;
        self.vlist = Vec::new();
        for _ in 0..self.count.get_value() {
            let(mvsk, obj) = $parseobjfunc(buf, sk) ? ;
            sk = mvsk;
            self.vlist.push(obj);
        }
        Ok(sk)
    }

    fn size(&self) -> usize {
        let mut sznum = self.count.size();
        for i in 0..self.count.get_value() {
            sznum += self.vlist[i as usize].as_ref().size();
        }
        sznum
    }

    fn describe(&self) -> String {
        let mut items = Vec::new();
        let count = self.count.value() as usize;
        for i in 0..count {
            items.push( self.vlist[i].describe() );
        }
        format!("{{\"count\":{},\"vlist\":[{}]}}",
            self.count.describe(),
            items.join(",")
        )
    }

} 

    )
}



/********************************************* */




create_dyn_obj_list_struct_by_call_new!(DynListActionMax65535, Uint2, Action, actions::parse);





