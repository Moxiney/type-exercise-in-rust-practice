use crate::{Scalar, ScalarRef};

/// [`Array`] is a container of the same type.
/// Each item in the array can be null or not.
pub trait Array: Send + Sync + Sized + 'static
where
    for<'a> Self::OwnedItem: Scalar<RefType<'a> = Self::RefItem<'a>>,
{
    type Builder: ArrayBuilder<Array = Self>;

    type RefItem<'a>: ScalarRef<'a, ArrayType = Self, ScalarType = Self::OwnedItem>;
    type OwnedItem: Scalar<ArrayTpye = Self>;

    fn len(&self) -> usize;
    fn get(&self, index: usize) -> Option<Self::RefItem<'_>>;
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
    fn iter(&self) -> ArrayIterator<'_, Self> {
        ArrayIterator::new(self)
    }
}

/// Array Builder is to build certain Array
pub trait ArrayBuilder {
    type Array: Array;

    /// Create a new builder with `capacity`.
    fn with_capacity(capacity: usize) -> Self;

    /// Append a value to builder.
    fn push(&mut self, item: Option<<<Self as ArrayBuilder>::Array as Array>::RefItem<'_>>);

    /// Finish building and return array
    fn finish(self) -> Self::Array;

    // Extend array?
    // fn extend(&mut self, array_iter: impl Array)
}

pub struct ArrayIterator<'a, A: Array> {
    array: &'a A,
    pos: usize,
}

impl<'a, A: Array> Iterator for ArrayIterator<'a, A> {
    type Item = Option<A::RefItem<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.array.len() {
            None
        } else {
            let item = self.array.get(self.pos);
            self.pos += 1;
            Some(item)
        }
    }
}

impl<'a, A: Array> ArrayIterator<'a, A> {
    pub fn new(array: &'a A) -> Self {
        Self { array, pos: 0 }
    }
}

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
