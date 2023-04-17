use std::fmt::Debug;

use crate::{
    array_impl::{PrimitiveArray, StringArray},
    PrimitiveType, Scalar, ScalarRef,
};

impl<T: PrimitiveType> Scalar for T {
    type RefType<'a> = T;
    type ArrayTpye = PrimitiveArray<T>;

    fn as_scalar_ref<'a>(&'a self) -> Self::RefType<'a> {
        *self
    }
    fn upcast_to<'a, 'b: 'a>(ref_item: Self::RefType<'b>) -> Self::RefType<'a> {
        ref_item
    }
}

impl<'a, T: PrimitiveType> ScalarRef<'a> for T {
    type ScalarType = T;
    type ArrayType = PrimitiveArray<T>;

    fn to_scalar_owned(&self) -> Self::ScalarType {
        *self
    }
}

impl PrimitiveType for i16 {}
impl PrimitiveType for i32 {}
impl PrimitiveType for i64 {}
impl PrimitiveType for f32 {}
impl PrimitiveType for f64 {}
impl PrimitiveType for bool {}

impl Scalar for String {
    type RefType<'a> = &'a str;

    type ArrayTpye = StringArray;

    fn as_scalar_ref<'a>(&'a self) -> Self::RefType<'a> {
        &self
    }
    fn upcast_to<'a, 'b: 'a>(ref_item: Self::RefType<'b>) -> Self::RefType<'a> {
        ref_item
    }
}

impl<'a> ScalarRef<'a> for &'a str {
    type ScalarType = String;

    type ArrayType = StringArray;

    fn to_scalar_owned(&self) -> Self::ScalarType {
        self.to_string()
    }
}
