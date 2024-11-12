// the PhantomData instances in this file are just to stop compiler complaints
// about missing generics; feel free to remove them

/// A Matcher is a single rule of fizzbuzz: given a function on T, should
/// a word be substituted in? If yes, which word?
pub struct Matcher<'a, T>(pub Box<dyn Fn(T) -> &'a str + 'a>);

impl<'a, T> Matcher<'a, T> {
    pub fn new<F, S>(matcher: F, subs: S) -> Matcher<'a, T>
    where
        F: Fn(T) -> bool + 'a,
        S: Into<&'a str> + Copy + 'a,
    {
        Self(Box::new(
            move |n: T| if matcher(n) { subs.into() } else { "" },
        ))
    }
}

/// A Fizzy is a set of matchers, which may be applied to an iterator.
///
/// Strictly speaking, it's usually more idiomatic to use `iter.map()` than to
/// consume an iterator with an `apply` method. Given a Fizzy instance, it's
/// pretty straightforward to construct a closure which applies it to all
/// elements of the iterator. However, we're using the `apply` pattern
/// here because it's a simpler interface for students to implement.
///
/// Also, it's a good excuse to try out using impl trait.
pub struct Fizzy<'a, T>(pub Vec<Matcher<'a, T>>);

impl<'a, T> Fizzy<'a, T> {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    // feel free to change the signature to `mut self` if you like
    #[must_use]
    pub fn add_matcher(mut self, matcher: Matcher<'a, T>) -> Self {
        self.0.push(matcher);
        self
    }

    /// map this fizzy onto every element of an iterator, returning a new iterator
    pub fn apply<I>(self, iter: I) -> impl Iterator<Item = String> + 'a
    where
        I: IntoIterator<Item = T>,
        <I as IntoIterator>::IntoIter: 'a,
        <I as IntoIterator>::Item: ToString + Copy + 'a,
    {
        // todo!() doesn't actually work, here; () is not an Iterator
        // that said, this is probably not the actual implementation you desire
        // std::iter::empty()

        iter.into_iter().map(move |n| {
            let s = self
                .0
                .iter()
                .filter_map(|matcher| {
                    let s = matcher.0(n);
                    if !s.is_empty() {
                        Some(s)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
                .concat();
            if !s.is_empty() {
                s
            } else {
                n.to_string()
            }
        })
    }
}

/// convenience function: return a Fizzy which applies the standard fizz-buzz rules
pub fn fizz_buzz<'a, T: ToString + Copy + std::ops::Rem<Output = T> + PartialEq + From<u8> + 'a>(
) -> Fizzy<'a, T> {
    Fizzy::new()
        .add_matcher(Matcher::new(|n| n % 3_u8.into() == 0_u8.into(), "fizz"))
        .add_matcher(Matcher::new(|n| n % 5_u8.into() == 0_u8.into(), "buzz"))
}
