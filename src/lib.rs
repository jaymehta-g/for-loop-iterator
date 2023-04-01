#[cfg(test)]
mod test;
/// Iterator that acts like a traditional for loop
pub struct ForLoopIterator<Item> 
{
    current_value: Item,
    while_predicate: Box<dyn Fn(&Item) -> bool>,
    next_value_function: Box<dyn Fn(&Item) -> Item>
}
impl<Item> ForLoopIterator<Item> {
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
    pub fn new(init: Item, pred: impl Fn(&Item) -> bool + 'static, next: impl Fn(&Item) -> Item + 'static) -> ForLoopIterator<Item> 
        // where
        //     PredType: Fn(Item) -> bool,/*Fn<Item, Output = bool>*/
        //     NextFnType: Fn(Item) -> Item
    {
        ForLoopIterator {
            current_value: init,
            while_predicate: Box::new(pred),
            next_value_function: Box::new(next)
        }
    }
}
impl<Item> Iterator for ForLoopIterator<Item> {
    type Item = Item;

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