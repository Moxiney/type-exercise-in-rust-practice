use crate::{
    array_impl::{PrimitiveArray, StringArray},
    PrimitiveType, Scalar, ScalarRef,
};

impl PrimitiveType for i32 {}

impl Scalar for i32 {
    type RefType<'a> = i32;
    type ArrayTpye = PrimitiveArray<i32>;

    fn as_scalar_ref<'a>(&'a self) -> Self::RefType<'a> {
        todo!()
    }
}

impl<'a> ScalarRef<'a> for i32 {
    type ScalarType = i32;

    type ArrayType = PrimitiveArray<i32>;

    fn to_scalar_owned(&self) -> Self::ScalarType {
        todo!()
    }
}

impl Scalar for String {
    type RefType<'a> = &'a str;

    type ArrayTpye = StringArray;

    fn as_scalar_ref<'a>(&'a self) -> Self::RefType<'a> {
        todo!()
    }
}

impl<'a> ScalarRef<'a> for &'a str {
    type ScalarType = String;

    type ArrayType = StringArray;

    fn to_scalar_owned(&self) -> Self::ScalarType {
        self.to_string()
    }
}
