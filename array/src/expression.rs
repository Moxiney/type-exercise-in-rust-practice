#![allow(unused)]

use crate::{
    array_impl::StringArray, dispatch::ArrayImpl, prelude::BooleanArray, Array, ArrayBuilder,
    Scalar, ScalarRef,
};

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

pub fn str_contains(s1: &str, s2: &str) -> bool {
    s1.contains(s2)
}

pub fn cmp_ge<I1: Scalar, I2: Scalar, C: Scalar + 'static>(
    i1: I1::RefType<'_>,
    i2: I2::RefType<'_>,
) -> bool
where
    for<'a> I1::RefType<'a>: Into<C::RefType<'a>>,
    for<'a> I2::RefType<'a>: Into<C::RefType<'a>>,
    for<'a> C::RefType<'a>: Ord,
{
    let i1: C::RefType<'_> = i1.into();
    let i2: C::RefType<'_> = i2.into();
    let i1: C::RefType<'_> = <C as Scalar>::upcast_to(i1);
    let i2: C::RefType<'_> = <C as Scalar>::upcast_to(i2);

    i1.cmp(&i2) == std::cmp::Ordering::Greater
}

trait Expression {
    fn eval_batch(&self, arrays: &[&ArrayImpl]) -> Result<ArrayImpl, ()>;
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

enum ExpressionType {
    CmpGe,
    ConstainsStr,
}

impl ExpressionType {
    pub fn build_expression(self) -> Box<dyn Expression> {
        match self {
            ExpressionType::CmpGe => Box::new(BinaryExpression::<bool, bool, bool, _>::new(
                cmp_ge::<bool, bool, bool>,
            )),
            ExpressionType::ConstainsStr => Box::new(
                BinaryExpression::<String, String, bool, _>::new(str_contains),
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        array_impl::{StringArray, StringArrayBuilder},
        dispatch::ScalarRefImpl,
        prelude::BooleanArray,
    };

    use super::*;

    #[test]
    fn test() {
        let mut array1 = StringArrayBuilder::with_capacity(1);
        array1.push(Some("hello"));
        let array1 = array1.finish();

        let mut array2 = StringArrayBuilder::with_capacity(1);
        array2.push(Some("hello"));
        let array2 = array2.finish();

        let expression = BinaryExpression::<String, String, bool, _>::new(str_contains);

        let result: BooleanArray = expression
            .eval(&array1.into(), &array2.into())
            .unwrap()
            .try_into()
            .unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result.get(0), Some(true));
    }

    #[test]
    fn test_build_str_contains() {
        let expr = ExpressionType::ConstainsStr.build_expression();

        for _ in 0..10 {
            let result = expr
                .eval_batch(&[
                    &StringArray::from_slice(&[Some("000"), Some("111"), None]).into(),
                    &StringArray::from_slice(&[Some("0"), Some("0"), None]).into(),
                ])
                .unwrap();
            assert_eq!(
                result.get(0).unwrap(),
                ScalarRefImpl::Boolean("000".contains("0"))
            );
            assert_eq!(
                result.get(1).unwrap(),
                ScalarRefImpl::Boolean(str_contains("111", "0"))
            );
            assert!(result.get(2).is_none());
        }
    }
}
