use crate::Array;

pub trait PrimitiveType: Default + std::fmt::Debug + Clone + Copy + Send + Sync + 'static {}

/// A trait for types that can be used as scalars in arrays.
/// primitive types, strings, and other arrays are scalars.
pub trait Scalar: std::fmt::Debug + Clone + Send + Sync + 'static {
    type RefType<'a>: ScalarRef<'a, ScalarType = Self, ArrayType = Self::ArrayTpye>
    where
        Self: 'a;

    type ArrayTpye: for<'a> Array<RefItem<'a> = Self::RefType<'a>, OwnedItem = Self>;

    fn as_scalar_ref<'a>(&'a self) -> Self::RefType<'a>;
}

/// A trait for types that is a reference of a scalar.
/// The ref of primitive types is itself.
/// The ref of strings is `&str`.
pub trait ScalarRef<'a>: std::fmt::Debug + Clone + Copy + Send + Sync {
    type ScalarType: Scalar<RefType<'a> = Self, ArrayTpye = Self::ArrayType>;
    type ArrayType: Array<RefItem<'a> = Self, OwnedItem = Self::ScalarType>;

    fn to_scalar_owned(&self) -> Self::ScalarType;
}
