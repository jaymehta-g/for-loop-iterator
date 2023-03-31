#[cfg(test)]
mod test;

/// Iterator that acts like a traditional for loop
pub struct ForLoopIterator<T> 
{
    current_value: T,
    while_predicate: fn(&T) -> bool,
    next_value_function: fn(&T) -> T
}
impl<T> ForLoopIterator<T> {
    /// Creates an iterator that behaves similarly to a traditional for loop
    /// * `init`: initial value
    /// * `pred`: predicate that returns true if the value should be outputted
    /// * `next`: function that creates the next value given the previous
    /// # Examples
    /// ```ignore
    /// let mut iter = ForLoopIterator::new(
    ///     10,
    ///     |i| i > &0,
    ///     |i| i - &2
    /// );
    /// assert_eq!(iter.next(), Some(10));
    /// assert_eq!(iter.next(), Some(8));
    /// let mut iter = iter.skip(2);
    /// assert_eq!(iter.next(), Some(2));
    /// assert_eq!(iter.next(), None);
    /// ```
    /// The values can be of any type.
    /// ```ignore
    /// struct MyStruct(i32);
    /// let mut it = ForLoopIterator::new(
	/// 	MyStruct(0),
	/// 	|i| i.0 < 10i32,
	/// 	|i| MyStruct(i.0+1)
	/// );
	/// assert_eq!(it.next(), Some(MyStruct(0)));
	/// assert_eq!(it.next(), Some(MyStruct(1)));
	/// assert_eq!(it.next(), Some(MyStruct(2)));
    /// // etc
    /// ```
    pub fn new(init: T, pred: fn(&T) -> bool, next: fn(&T) -> T) -> ForLoopIterator<T> {
        ForLoopIterator {
            current_value: init,
            while_predicate: pred,
            next_value_function: next
        }
    }
}
impl<T> Iterator for ForLoopIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        use std::mem;

        if !(self.while_predicate)(&self.current_value) {
            return None;
        }
        let mut swap_with = (self.next_value_function)(&self.current_value);
        mem::swap(&mut self.current_value, &mut swap_with);
        Some(swap_with)
    }
}