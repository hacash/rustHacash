use std::cell::{RefCell};
// use std::rc::Rc;
use std::sync::{ Arc, Mutex};
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};

use concat_idents::concat_idents;
// use rand;
use rusty_leveldb::{DB, Options};

use crate::interface::*;
use crate::cores::fields::*;
use crate::cores::constitutes::*;
use crate::cores::storeitems::*;


include!("state.rs");
include!("entity.rs");
include!("operate.rs");

