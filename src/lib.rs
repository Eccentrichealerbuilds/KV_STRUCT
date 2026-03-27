
/// The `macro_` module contains the definitions for generating JSON-like strings.
/// All underlying operations in this module rely heavily on the `Debug` trait to format the 
/// values being serialized into the JSON structure.
pub mod macro_;

pub use macro_::*;

