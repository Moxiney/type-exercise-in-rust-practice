use bitvec::vec::BitVec;

use crate::{
    array::{Array, ArrayBuilder},
    PrimitiveType, Scalar, ScalarRef,
};

pub struct PrimitiveArray<T> {
    /// The actual data of this array
    data: Vec<T>,

    /// The bitmap for this array
    /// If the bit is set, the corresponding value is not null.
    bitmap: BitVec,
}

impl<T> Array for PrimitiveArray<T>
where
    T: PrimitiveType,
    T: for<'a> Scalar<RefType<'a> = T, ArrayTpye = Self>,
    T: for<'a> ScalarRef<'a, ScalarType = T, ArrayType = Self>,
{
    type RefItem<'a> = T;
    type OwnedItem = T;
    type Builder = PrimitiveArrayBuilder<T>;

    fn len(&self) -> usize {
        self.bitmap.len()
    }

    fn get(&self, index: usize) -> Option<Self::RefItem<'_>> {
        match self.bitmap.get(index).as_deref() {
            Some(&true) => self.data.get(index).copied(),
            _ => None,
        }
    }
}

pub struct PrimitiveArrayBuilder<T> {
    data: Vec<T>,
    bitmap: BitVec,
}

impl<T> ArrayBuilder for PrimitiveArrayBuilder<T>
where
    T: PrimitiveType,
    T: for<'a> Scalar<RefType<'a> = T, ArrayTpye = PrimitiveArray<T>>,
    T: for<'a> ScalarRef<'a, ScalarType = T, ArrayType = PrimitiveArray<T>>,
{
    type Array = PrimitiveArray<T>;

    fn with_capacity(capacity: usize) -> Self {
        let data = Vec::with_capacity(capacity);
        let bitmap = BitVec::with_capacity(capacity);
        Self { data, bitmap }
    }

    fn push(&mut self, item: Option<T>) {
        match item {
            Some(item) => {
                self.data.push(item);
                self.bitmap.push(true);
            }
            None => {
                self.data.push(T::default());
                self.bitmap.push(false);
            }
        }
    }

    fn finish(self) -> Self::Array {
        Self::Array {
            data: self.data,
            bitmap: self.bitmap,
        }
    }
}

#[derive(Debug)]
pub struct StringArray {
    /// The fallten data of strings.
    data: Vec<u8>,
    /// The offset of each string.
    /// The length of offset is len + 1.
    offset: Vec<usize>,
    /// The null bitmap for this array, which indicates whether an element at
    /// `i` is null
    bitmap: BitVec,
}

impl StringArray {
    pub fn get(&self, index: usize) -> Option<&str> {
        if self.bitmap.get(index).as_deref() == Some(&true) {
            let start = self.offset[index];
            let end = self.offset[index + 1];
            Some(std::str::from_utf8(&self.data[start..end]).unwrap())
        } else {
            None
        }
    }

    pub fn from_slice(data: &[Option<&str>]) -> Self {
        let mut builder = <Self as Array>::Builder::with_capacity(data.len());
        for val in data {
            builder.push(*val);
        }
        builder.finish()
    }
}

impl Array for StringArray {
    type RefItem<'a> = &'a str;
    type OwnedItem = String;
    type Builder = StringArrayBuilder;

    fn get<'b>(&'b self, index: usize) -> Option<Self::RefItem<'b>> {
        self.get(index)
    }

    fn len(&self) -> usize {
        self.bitmap.len()
    }
}

pub struct StringArrayBuilder {
    data: Vec<u8>,
    bitmap: BitVec,
    offsets: Vec<usize>,
}

impl ArrayBuilder for StringArrayBuilder {
    type Array = StringArray;

    fn with_capacity(capacity: usize) -> Self {
        let mut offsets = Vec::with_capacity(capacity + 1);
        offsets.push(0);

        Self {
            data: Vec::new(),
            bitmap: BitVec::with_capacity(capacity),
            offsets,
        }
    }

    fn push(&mut self, item: Option<&str>) {
        match item {
            Some(item) => {
                let bytes = item.to_string().into_bytes();

                self.bitmap.push(true);
                self.data.extend(bytes);
                self.offsets.push(self.data.len());
            }
            None => {
                self.bitmap.push(false);
                self.offsets.push(self.data.len());
            }
        }
    }

    fn finish(self) -> Self::Array {
        assert_eq!(self.bitmap.len() + 1, self.offsets.len());

        StringArray {
            data: self.data,
            offset: self.offsets,
            bitmap: self.bitmap,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string_array_test() {
        let str_array = StringArray::from_slice(&[Some("1"), Some("2"), None]);

        dbg!(&str_array);

        assert_eq!(str_array.get(0), Some("1"));
        assert_eq!(str_array.get(1), Some("2"));
        assert_eq!(str_array.get(2), None);

        let mut iter = str_array.iter();
        assert_eq!(iter.next(), Some(Some("1")));
        assert_eq!(iter.next(), Some(Some("2")));
        assert_eq!(iter.next(), Some(None));
        assert_eq!(iter.next(), None);
    }
}
