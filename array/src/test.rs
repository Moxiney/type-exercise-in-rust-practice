use crate::{dispatch::ArrayImpl, Array, ArrayBuilder, Scalar};

fn sql_func<'a, I: Array, O: Array>(_i1: I::RefItem<'a>, _i2: I::RefItem<'a>) -> O::OwnedItem {
    todo!()
}

fn eval_binary<I: Array, O: Array>(i1: I, i2: I) -> O {
    assert_eq!(i1.len(), i2.len(), "size mismatch");
    let mut builder = O::Builder::with_capacity(i1.len());
    for (i1, i2) in i1.iter().zip(i2.iter()) {
        match (i1, i2) {
            (Some(i1), Some(i2)) => builder.push(Some(sql_func::<I, O>(i1, i2).as_scalar_ref())),
            _ => builder.push(None),
        }
    }
    builder.finish()
}

fn eval_binary_impl<'a, I1, I2>(i1: &'a ArrayImpl, i2: &'a ArrayImpl) -> Result<ArrayImpl, ()>
where
    &'a I1: TryFrom<&'a ArrayImpl, Error = ()> + Array,
    &'a I2: TryFrom<&'a ArrayImpl, Error = ()> + Array,
{
    let ia1: &I1 = i1.try_into()?;
    let ia2: &I2 = i2.try_into()?;

    todo!()
}
