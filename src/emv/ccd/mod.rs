//! The EMV Common Core Definitions (CCD) values. Not all issuers comply with
//! these, hence why they're in their own section.
//!
//! Information for this can be found in EMV Book 3, under `Part V`.

// Modules
mod cci;
mod cvr;
mod iad;

// Public Exports
pub use self::{cci::*, cvr::*, iad::*};
