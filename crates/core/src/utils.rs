pub trait IterExt: Iterator {
    fn reduce_with<B, F, I>(mut self, init: I, f: F) -> Option<B>
    where
        Self: Sized,
        I: FnOnce(Self::Item) -> B,
        F: FnMut(B, Self::Item) -> B,
    {
        let first = init(self.next()?);
        Some(self.fold(first, f))
    }
}
impl<I: Iterator> IterExt for I {}
