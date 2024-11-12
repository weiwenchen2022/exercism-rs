/// What should the type of _function be?
pub fn map<F, T, U>(input: Vec<T>, mut function: F) -> Vec<U>
where
    F: FnMut(T) -> U,
{
    // todo!("Transform input vector {input:?} using passed function");

    let mut output = Vec::with_capacity(input.len());
    for x in input {
        output.push(function(x));
    }
    output
}
