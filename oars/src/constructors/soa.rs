//! Experimental constructors for strong orthogonal arrays

use crate::soa::{SOAConstructor, SOAResult};
use oars_proc_macro::Checked;

#[derive(Checked)]
pub struct HeTang {}

impl SOAConstructor for HeTang {
    fn gen(&self) -> SOAResult {
        unimplemented!();
    }
}
