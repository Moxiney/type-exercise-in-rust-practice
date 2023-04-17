use crate::prelude::*;
use crate::{array_impl::StringArray, Array, Scalar, ScalarRef};

macro_rules! impl_scalar_dispatch {
    ($( { $Abc:ident, $abc:ident, $AbcArray:ty, $AbcArrayBuilder:ty, $Owned:ty, $Ref:ty } ),*) => {
        #[derive(Debug, PartialEq)]
        pub enum ScalarImpl {
            $( $Abc($Owned) ),*
        }

        impl ScalarImpl {
            fn as_scalar_ref(&self) -> ScalarRefImpl<'_> {
                match self {
                    $( Self::$Abc(scalar_ref) => scalar_ref.as_scalar_ref().into() ),*
                }
            }
        }

        $(
            impl From<$Owned> for ScalarImpl {
                fn from(scalar: $Owned) -> Self {
                    ScalarImpl::$Abc(scalar)
                }
            }

            impl TryFrom<ScalarImpl> for $Owned {
                type Error = ();

                fn try_from(scalar: ScalarImpl) -> Result<Self, Self::Error> {
                    match scalar {
                        ScalarImpl::$Abc(scalar) => Ok(scalar),
                        _ => Err(())
                    }
                }
            }
        )*
    };
}

macro_rules! impl_scalar_ref_dispatch {
    ($( { $Abc:ident, $abc:ident, $AbcArray:ty, $AbcArrayBuilder:ty, $Owned:ty, $Ref:ty } ),*) => {
        #[derive(Debug, PartialEq)]
        pub enum ScalarRefImpl<'a> {
            $( $Abc($Ref) ),*
        }

        impl<'a> ScalarRefImpl<'a> {
            fn to_scalar_owned(&self) -> ScalarImpl {
                match self {
                    $( ScalarRefImpl::$Abc(scalar_ref) => scalar_ref.to_scalar_owned().into() ),*
                }
            }
        }

        $(
            impl<'a> From<$Ref> for ScalarRefImpl<'a> {
                fn from(scalar_ref: $Ref) -> Self {
                    ScalarRefImpl::$Abc(scalar_ref)
                }
            }

            impl<'a> TryFrom<ScalarRefImpl<'a>> for $Ref {
                type Error = ();

                fn try_from(ref_impl: ScalarRefImpl<'a>) -> Result<Self, Self::Error> {
                    match ref_impl {
                        ScalarRefImpl::$Abc(scalar_ref) => Ok(scalar_ref),
                        _ => Err(())
                    }
                }
            }
        )*
    };
}

macro_rules! impl_array_dispatch {
    ($( { $Abc:ident, $abc:ident, $AbcArray:ty, $AbcArrayBuilder:ty, $Owned:ty, $Ref:ty } ),*) => {
        pub enum ArrayImpl {
            $( $Abc($AbcArray) ),*
        }

        impl ArrayImpl {
            pub fn len(&self) -> usize {
                match self {
                    $(ArrayImpl::$Abc(array) => array.len()),*
                }
            }

            pub fn get(&self, index: usize) -> Option<ScalarRefImpl<'_>> {
                match self {
                    $(ArrayImpl::$Abc(array) => array.get(index).map(ScalarRefImpl::$Abc)),*
                }
            }
        }

        $(
            impl From<$AbcArray> for ArrayImpl {
                fn from(array: $AbcArray) -> Self {
                    ArrayImpl::$Abc(array)
                }
            }

            impl TryFrom<ArrayImpl> for $AbcArray {
                type Error = ();

                fn try_from(value: ArrayImpl) -> Result<Self, Self::Error> {
                    match value {
                        ArrayImpl::$Abc(array) => Ok(array),
                        _ => Err(()),
                    }
                }
            }

            impl<'a> TryFrom<&'a ArrayImpl> for &'a $AbcArray {
                type Error = ();

                fn try_from(array: &'a ArrayImpl) -> Result<&'a $AbcArray, Self::Error> {
                    match array {
                        ArrayImpl::$Abc(array) => Ok(array),
                        _ => Err(()),
                    }
                }
            }
        )*
    };
}

macro_rules! impl_for_all {
    ($macro:tt) => {
        $macro! {
            { Int16, int16, Int16Array, Int16ArrayBuilder, i16, i16 },
            { Int32, int32, Int32Array, Int32ArrayBuilder, i32, i32 },
            { Int64, int64, Int64Array, Int64ArrayBuilder, i64, i64 },
            { Float32, float32, Float32Array, Float32ArrayBuilder, f32, f32 },
            { Float64, float64, Float64Array, Float64ArrayBuilder, f64, f64 },
            { Boolean, boolean, BooleanArray, BooleanArrayBuilder, bool, bool },
            { String, string, StringArray, StringArrayBuilder, String, &'a str }
        }
    };
}

impl_for_all! {impl_scalar_dispatch}
impl_for_all! {impl_scalar_ref_dispatch}
impl_for_all! {impl_array_dispatch}

#[cfg(test)]
mod test {
    use crate::ArrayBuilder;

    use super::*;

    #[test]
    fn test() {
        let mut array_builder = Int32ArrayBuilder::with_capacity(10);
        for i in 0..10 {
            array_builder.push(Some(i));
        }
        let array: ArrayImpl = array_builder.finish().into();

        match array.get(3) {
            Some(ScalarRefImpl::Int32(value)) => println!("get {}", value),
            _ => panic!(""),
        }
    }
}
