#![feature(bigint_helper_methods)]
#![feature(const_option)]
#![feature(const_trait_impl)]
#![feature(effects)]
#![feature(iter_collect_into)]
#![allow(clippy::unusual_byte_groupings)]

// TODO: Temporary
#![allow(incomplete_features)]

extern crate num_traits;
extern crate strum;
extern crate strum_macros;
extern crate num_enum;

pub mod number;
pub mod utility;
pub mod programming;
pub mod processor;
pub mod instruction;