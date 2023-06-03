mod abi;
mod db;
#[rustfmt::skip]
pub mod pb;
#[rustfmt::skip]
pub mod utils;
mod modules;
pub use modules::*;
substreams_ethereum::init!();