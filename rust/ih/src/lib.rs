#![feature(doc_auto_cfg)]
#![feature(doc_cfg)]
// #![feature(fn_traits)]
// #![feature(unboxed_closures)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(irrefutable_let_patterns)]
#![allow(rustdoc::broken_intra_doc_links)]
// Modules are generated based on the naming conventions of protobuf, which might cause "module inception"
#![allow(clippy::module_inception)]
// This is all generated code, so "manually" implementing derivable impls is okay
#![allow(clippy::derivable_impls)]
// For enums with many variants, the matches!(...) macro isn't obviously better
#![allow(clippy::match_like_matches_macro)]
// Used by extension codegen
#![allow(clippy::redundant_closure)]
// TODO: Ideally we don't allow this
#![allow(clippy::option_as_ref_deref)]
// TODO: Ideally we don't allow this
#![allow(clippy::match_single_binding)]

mod proto;
pub use proto::*;

#[cfg(feature = "err")]
pub type CodeBody = (State, Vec<u8>);

#[cfg(feature = "err")]
pub mod err;

#[cfg(feature = "err")]
pub use err::state::captcha;
