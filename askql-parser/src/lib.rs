#[macro_use]
extern crate serde;
#[macro_use]
extern crate lazy_static;
pub mod askcode;
pub use askcode::*;
pub mod parse;
pub use parse::*;
pub mod reduce;
pub use reduce::*;
pub mod value;
pub use value::*;
