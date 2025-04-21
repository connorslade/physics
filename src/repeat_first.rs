use std::mem;

pub struct RepeatFirst<T: Clone, I: Iterator<Item = T>> {
    first: Option<T>,
    iter: I,
}

pub trait IteratorRepeatFirst: Iterator {
    fn repeat_first(self) -> RepeatFirst<Self::Item, Self>
    where
        Self: Sized,
        Self::Item: Clone,
    {
        RepeatFirst::new(self)
    }
}

impl<T: Iterator> IteratorRepeatFirst for T {}

impl<T: Clone, I: Iterator<Item = T>> RepeatFirst<T, I> {
    pub fn new(iter: I) -> Self {
        Self { first: None, iter }
    }
}

impl<T: Clone, I: Iterator<Item = T>> Iterator for RepeatFirst<T, I> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let no_first = self.first.is_none();
        let next = self.iter.next().or_else(|| mem::take(&mut self.first));
        if next.is_some() && no_first {
            self.first = next.clone();
        }

        next
    }
}
