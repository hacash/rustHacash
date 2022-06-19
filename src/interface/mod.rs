use concat_idents::concat_idents;

use crate::cores::fields::*;
use crate::cores::storeitems::*;

include!("./macro.rs");

include!("./field.rs");
include!("./action.rs");
include!("./transaction.rs");
include!("./block.rs");

include!("./state.rs");