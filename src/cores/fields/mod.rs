use std::fmt;
use std::cmp::Ordering::{Less,Greater};
// use std::ops::{Add, Sub};

use num_bigint::BigInt;
use num_bigint::Sign::{Minus, Plus};
use num_traits::{FromPrimitive, ToPrimitive, Num};

use crate::interface::{Field, FieldNumber};

include!("./macro.rs");

include!("./fixedbytes.rs");
include!("./bytes.rs");
include!("./varint.rs");
include!("./varfloat.rs");
include!("./bool.rs");
include!("./stringtrim.rs");
include!("./string.rs");
include!("./message.rs");
include!("./amount.rs");
include!("./address.rs");
include!("./diamond.rs");
include!("./hash.rs");
include!("./height.rs");
include!("./sign.rs");

include!("./satoshi.rs");

include!("./channel.rs");
include!("./lending.rs");
include!("./lockbls.rs");

