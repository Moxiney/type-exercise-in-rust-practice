pub struct Experssion<I1, I2, O, F> {
    f: F,
    _marker: std::marker::PhantomData<fn(I1, I2) -> O>,
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
