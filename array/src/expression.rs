#![allow(unused)]

use crate::{
    array_impl::StringArray, data_type::DataType, dispatch::ArrayImpl, prelude::BooleanArray,
    Array, ArrayBuilder, Scalar, ScalarRef,
};

pub trait Expression {
    fn eval_batch(&self, arrays: &[&ArrayImpl]) -> Result<ArrayImpl, ()>;
}

pub struct BinaryExpression<I1, I2, O, F> {
    f: F,
    _marker: std::marker::PhantomData<fn(I1, I2) -> O>,
}

impl<I1, I2, O, F> BinaryExpression<I1, I2, O, F>
where
    I1: Scalar,
    I2: Scalar,
    O: Scalar,
    for<'a> &'a I1::ArrayTpye: TryFrom<&'a ArrayImpl, Error = ()>,
    for<'a> &'a I2::ArrayTpye: TryFrom<&'a ArrayImpl, Error = ()>,
    O::ArrayTpye: Into<ArrayImpl>,
    F: Fn(I1::RefType<'_>, I2::RefType<'_>) -> O,
{
    pub fn new(f: F) -> Self {
        Self {
            f,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn eval(&self, i1: &ArrayImpl, i2: &ArrayImpl) -> Result<ArrayImpl, ()> {
        assert!(i1.len() == i2.len(), "size mismatch");

        let i1: &I1::ArrayTpye = i1.try_into()?;
        let i2: &I2::ArrayTpye = i2.try_into()?;

        let mut builder = <O::ArrayTpye as Array>::Builder::with_capacity(i1.len());
        for (i1, i2) in i1.iter().zip(i2.iter()) {
            match (i1, i2) {
                (Some(i1), Some(i2)) => {
                    let result = (self.f)(i1, i2);
                    builder.push(Some(result.as_scalar_ref()));
                }
                _ => builder.push(None),
            }
        }

        Ok(builder.finish().into())
    }
}

impl<I1, I2, O, F> Expression for BinaryExpression<I1, I2, O, F>
where
    I1: Scalar,
    I2: Scalar,
    O: Scalar,
    for<'a> &'a I1::ArrayTpye: TryFrom<&'a ArrayImpl, Error = ()>,
    for<'a> &'a I2::ArrayTpye: TryFrom<&'a ArrayImpl, Error = ()>,
    O::ArrayTpye: Into<ArrayImpl>,
    F: Fn(I1::RefType<'_>, I2::RefType<'_>) -> O,
{
    fn eval_batch(&self, arrays: &[&ArrayImpl]) -> Result<ArrayImpl, ()> {
        if arrays.len() != 2 {
            return Err(());
        }
        self.eval(&arrays[0], &arrays[1])
    }
}
