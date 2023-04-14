// Trait definition
mod array;
mod scalar;
mod test;

// Trait implementaion
mod array_impl;
mod scalar_impl;

// Dispatch implementation
mod dispatch;

pub use array::*;
pub use scalar::*;
