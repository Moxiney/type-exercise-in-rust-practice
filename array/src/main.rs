use bitvec::vec::BitVec;

/// [`Array`] is a container of the same type.
/// Each item in the array can be null or not.
pub trait Array: Send + Sync + Sized + 'static {
    type Item<'a>: Clone + Copy + std::fmt::Debug;
    type Builder: ArrayBuilder<Array = Self>;

    fn len(&self) -> usize;
    fn get(&self, index: usize) -> Option<Self::Item<'_>>;
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
    fn push(&mut self, item: Option<<<Self as ArrayBuilder>::Array as Array>::Item<'_>>);

    /// Finish building and return array
    fn finish(self) -> Self::Array;

    // Extend array?
    // fn extend(&mut self, array_iter: impl Array)
}

pub struct PrimitiveArray<T> {
    /// The actual data of this array
    data: Vec<T>,

    /// The null bitmap for this array, which indicates whether an element at
    /// `i` is null
    bitmap: BitVec,
}

impl<T> Array for PrimitiveArray<T>
where
    T: Default + Clone + Copy + Send + Sync + std::fmt::Debug + 'static,
{
    type Item<'a> = T;
    type Builder = PrimitiveArrayBuilder<T>;

    fn len(&self) -> usize {
        self.bitmap.len()
    }

    fn get(&self, index: usize) -> Option<Self::Item<'_>> {
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
    T: Default + Clone + Copy + Send + Sync + std::fmt::Debug + 'static,
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
}

impl Array for StringArray {
    type Item<'a> = &'a str;
    type Builder = StringArrayBuilder;

    fn get<'b>(&'b self, index: usize) -> Option<Self::Item<'b>> {
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
    cur_offset: usize,
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
            cur_offset: 0,
        }
    }

    fn push(&mut self, item: Option<&str>) {
        match item {
            Some(item) => {
                let bytes = item.to_string().into_bytes();
                let len = bytes.len();

                self.bitmap.push(true);
                self.data.extend(bytes);
                self.offsets.push(self.cur_offset);

                self.cur_offset += len;
            }
            None => {
                self.bitmap.push(false);
                self.offsets.push(self.cur_offset);
            }
        }
    }

    fn finish(mut self) -> Self::Array {
        self.offsets.push(self.cur_offset);

        assert_eq!(self.bitmap.len() + 1, self.offsets.len());

        StringArray {
            data: self.data,
            offset: self.offsets,
            bitmap: self.bitmap,
        }
    }
}

pub struct ArrayIterator<'a, A: Array> {
    array: &'a A,
    pos: usize,
}

impl<'a, A: Array> Iterator for ArrayIterator<'a, A> {
    type Item = A::Item<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.array.len() {
            None
        } else {
            let item = self.array.get(self.pos);
            self.pos += 1;
            item
        }
    }
}

impl<'a, A: Array> ArrayIterator<'a, A> {
    pub fn new(array: &'a A) -> Self {
        Self { array, pos: 0 }
    }
}

fn eval_binary<I: Array, O: Array>(i1: I, i2: I) -> O {
    assert_eq!(i1.len(), i2.len(), "size mismatch");
    let mut builder = O::Builder::with_capacity(i1.len());
    for (i1, i2) in i1.iter().zip(i2.iter()) {
        builder.push(sql_func(i1, i2));
    }
    builder.finish()
}

pub fn sql_func<Item1, Item2, OutputItem>(i1: Item1, i2: Item2) -> OutputItem {
    todo!()
}

fn main() {
    println!("Hello, world!");
}
