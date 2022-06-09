use std::fmt;
use std::cmp::Ordering::{Less,Greater};
// use std::ops::{Add, Sub};

use crate::interface::Field;
use num_bigint::BigInt;
use num_bigint::Sign::{Minus, Plus};
use num_traits::{FromPrimitive, ToPrimitive, Num};


include!("./macro.rs");

include!("./fixedbytes.rs");
include!("./varint.rs");
include!("./stringtrim.rs");
include!("./string.rs");
include!("./amount.rs");
include!("./address.rs");


