use crate::{BinaryExpression, DataType, Expression, Scalar};

macro_rules! int16 {
    ($macro: tt) => {
        $macro! { DataType::SmallInt, i16, Int16Array }
    };
}

macro_rules! int32 {
    ($macro: tt) => {
        $macro! { DataType::Integer, i32, Int32Array }
    };
}

macro_rules! int64 {
    ($macro: tt) => {
        $macro! { DataType::BigInt, i64, Int64Array }
    };
}

macro_rules! boolean {
    ($macro: tt) => {
        $macro! { DataType::Boolean, bool, BooleanArray }
    };
}

macro_rules! varchar {
    ($macro: tt) => {
        $macro! { DataType::Varchar, String, StringArray }
    };
}

macro_rules! char {
    ($macro: tt) => {
        $macro! { DataType::Char{..}, String, StringArray }
    };
}

macro_rules! data_type_pattern {
    ($data_type_pattern: pat, $scalar_type: ty, $array_type: ty) => {
        $data_type_pattern
    };
}
macro_rules! scalar_type {
    ($data_type_pattern: pat, $scalar_type: ty, $array_type: ty) => {
        $scalar_type
    };
}
macro_rules! array_type {
    ($data_type_pattern: pat, $scalar_type: ty, $array_type: ty) => {
        $array_type
    };
}

macro_rules! impl_cmp_for {
    ($cmp_func: ident, $l: ident, $r: ident, $({$ty1: tt, $ty2: tt, $convert_ty: tt}),*) => {
        match ($l, $r) {
            $(
                ($ty1!{ data_type_pattern }, $ty2!{ data_type_pattern }) => {
                    Ok(Box::new(BinaryExpression::<
                            $ty1!{ scalar_type },
                            $ty2!{ scalar_type },
                            bool,
                            _
                        >::new(
                        $cmp_func::<
                            $ty1!{ scalar_type },
                            $ty2!{ scalar_type },
                            $convert_ty!{ scalar_type }
                        >
                    )))
                }
            )*
            _ => {
                Err(Unsupported)
            }
        }
    };
}

macro_rules! for_all_cmp {
    ($macro: tt, $($param:ident),*) => {
        $macro! {
            $($param),*,
            {boolean, boolean, boolean},
            {int16, int32, int32},
            {int32, int32, int32},
            {varchar, varchar, varchar},
            {char, char, char}
        }
    };
}

#[derive(Debug)]
pub struct Unsupported;

pub fn build_expression(
    expr_type: ExpressionType,
    i1: DataType,
    i2: DataType,
) -> Result<Box<dyn Expression>, Unsupported> {
    match expr_type {
        ExpressionType::CmpGe => for_all_cmp! { impl_cmp_for, cmp_ge, i1, i2 },
        ExpressionType::CmpLe => for_all_cmp! { impl_cmp_for, cmp_le, i1, i2 },
        ExpressionType::ConstainsStr => Ok(Box::new(
            BinaryExpression::<String, String, bool, _>::new(str_contains),
        )),
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
    let i1: C::RefType<'_> = I1::upcast_to(i1).into();
    let i2: C::RefType<'_> = I2::upcast_to(i2).into();

    i1.cmp(&i2) == std::cmp::Ordering::Greater
}

pub fn cmp_le<I1: Scalar, I2: Scalar, C: Scalar + 'static>(
    i1: I1::RefType<'_>,
    i2: I2::RefType<'_>,
) -> bool
where
    for<'a> I1::RefType<'a>: Into<C::RefType<'a>>,
    for<'a> I2::RefType<'a>: Into<C::RefType<'a>>,
    for<'a> C::RefType<'a>: Ord,
{
    let i1: C::RefType<'_> = I1::upcast_to(i1).into();
    let i2: C::RefType<'_> = I2::upcast_to(i2).into();

    i1.cmp(&i2) == std::cmp::Ordering::Less
}

pub enum ExpressionType {
    CmpGe,
    CmpLe,
    ConstainsStr,
}

#[cfg(test)]
mod tests {
    use crate::{
        array_impl::{StringArray, StringArrayBuilder},
        dispatch::ScalarRefImpl,
        prelude::BooleanArray,
        Array, ArrayBuilder,
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
        let expr = build_expression(
            ExpressionType::ConstainsStr,
            DataType::Varchar,
            DataType::Varchar,
        )
        .unwrap();

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

    #[test]
    fn test_build_cmp() {
        let expr =
            build_expression(ExpressionType::CmpGe, DataType::Varchar, DataType::Varchar).unwrap();

        let result = expr
            .eval_batch(&[
                &StringArray::from_slice(&[Some("1"), Some("0"), None]).into(),
                &StringArray::from_slice(&[Some("0"), Some("1"), None]).into(),
            ])
            .unwrap();
        assert_eq!(
            result.get(0).unwrap(),
            ScalarRefImpl::Boolean(cmp_ge::<String, String, String>("1", "0"))
        );
        assert_eq!(
            result.get(1).unwrap(),
            ScalarRefImpl::Boolean(cmp_ge::<String, String, String>("0", "1"))
        );
        assert!(result.get(2).is_none());

        let expr =
            build_expression(ExpressionType::CmpLe, DataType::Varchar, DataType::Varchar).unwrap();

        let result = expr
            .eval_batch(&[
                &StringArray::from_slice(&[Some("1"), Some("0"), None]).into(),
                &StringArray::from_slice(&[Some("0"), Some("1"), None]).into(),
            ])
            .unwrap();
        assert_eq!(
            result.get(0).unwrap(),
            ScalarRefImpl::Boolean(cmp_le::<String, String, String>("1", "0"))
        );
        assert_eq!(
            result.get(1).unwrap(),
            ScalarRefImpl::Boolean(cmp_le::<String, String, String>("0", "1"))
        );
        assert!(result.get(2).is_none());
    }
}
