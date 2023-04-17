// Trait definition
mod array;
mod data_type;
mod scalar;

// Trait implementaion
mod array_impl;
mod scalar_impl;

// Dispatch implementation
mod dispatch;

/// Expression definition and implementation
mod expression;
mod expression_impl;

mod test;

pub use array::*;
pub use data_type::DataType;
pub use expression::*;
pub use scalar::*;

pub mod prelude {
    use crate::array_impl::{PrimitiveArray, PrimitiveArrayBuilder};

    pub type Int16Array = PrimitiveArray<i16>;
    pub type Int32Array = PrimitiveArray<i32>;
    pub type Int64Array = PrimitiveArray<i64>;
    pub type Float32Array = PrimitiveArray<f32>;
    pub type Float64Array = PrimitiveArray<f64>;
    pub type BooleanArray = PrimitiveArray<bool>;

    pub type Int16ArrayBuilder = PrimitiveArrayBuilder<i16>;
    pub type Int32ArrayBuilder = PrimitiveArrayBuilder<i32>;
    pub type Int64ArrayBuilder = PrimitiveArrayBuilder<i64>;
    pub type Float32ArrayBuilder = PrimitiveArrayBuilder<f32>;
    pub type Float64ArrayBuilder = PrimitiveArrayBuilder<f64>;
    pub type BooleanArrayBuilder = PrimitiveArrayBuilder<bool>;
}
