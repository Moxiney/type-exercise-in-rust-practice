use crate::{
    array_impl::{Int32Array, StringArray},
    Array, Scalar, ScalarRef,
};

macro_rules! impl_scalar_dispatch {
    ($( { $Abc:ident, $abc:ident, $AbcArray:ty, $AbcArrayBuilder:ty, $Owned:ty, $Ref:ty } ),*) => {
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
        )*
    };
}

macro_rules! for_all_impl_macros {
    ($macro:tt) => {
        $macro! {
            { Int32, int32, Int32Array, Int32ArrayBuilder, i32, i32 },
            { String, string, StringArray, StringArrayBuilder, String, &'a str }
        }
    };
}

for_all_impl_macros! {impl_scalar_dispatch}
for_all_impl_macros! {impl_scalar_ref_dispatch}
for_all_impl_macros! {impl_array_dispatch}

#[cfg(test)]
mod test {
    use crate::{array_impl::Int32ArrayBuilder, ArrayBuilder};

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
