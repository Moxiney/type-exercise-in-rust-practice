use crate::{Array, ArrayBuilder, Scalar, ScalarRef};

pub struct Experssion<I1, I2, O, F> {
    f: F,
    _marker: std::marker::PhantomData<fn(I1, I2) -> O>,
}

impl<I1, I2, O, F> Experssion<I1, I2, O, F>
where
    I1: Array,
    I2: Array,
    O: Array,
    F: for<'a> Fn(I1::RefItem<'a>, I2::RefItem<'a>) -> O::OwnedItem,
{
    pub fn new(f: F) -> Self {
        Self {
            f,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn eval(&self, i1: &I1, i2: &I2) -> Result<O, ()> {
        assert!(i1.len() == i2.len(), "size mismatch");

        let mut builder = O::Builder::with_capacity(i1.len());
        for (i1, i2) in i1.iter().zip(i2.iter()) {
            match (i1, i2) {
                (Some(i1), Some(i2)) => {
                    let result = (self.f)(i1, i2);
                    builder.push(Some(result.as_scalar_ref()));
                }
                _ => builder.push(None),
            }
        }

        Ok(builder.finish())
    }
}

pub fn str_contains(s1: &str, s2: &str) -> bool {
    s1.contains(s2)
}

// guideline
// ```
// let expr = BinaryExpression::<StringArray, StringArray, BoolArray, _>::new(str_contains);
// // We only need to pass `ArrayImpl` to the expression, and it will do everything for us,
// // including type checks, loopping, etc.
// let result = expr.eval(/* &ArrayImpl,  &ArrayImpl */).unwrap();
// ```

#[cfg(test)]
mod tests {
    use crate::{
        array_impl::{StringArray, StringArrayBuilder},
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

        let expression = Experssion::<StringArray, StringArray, BooleanArray, _>::new(str_contains);

        let result = expression.eval(&array1, &array2).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result.get(0), Some(true));
    }
}
