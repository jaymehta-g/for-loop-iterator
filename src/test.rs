use std::{ops::Add, process::Output};

use crate::*;
#[test]
fn for_iter() {
	let mut it = ForLoopIterator::new(
		30,
		|i| i < &40,
		|i| i+1
	);
	assert_eq!(it.next(), Some(30));
	assert_eq!(it.next(), Some(31));
	assert_eq!(it.next(), Some(32));
	assert_eq!(it.next(), Some(33));
	assert_eq!(it.next(), Some(34));
	let mut it=it.skip(5);
	assert_eq!(it.next(), None);
	let mut it = ForLoopIterator::new(
		100,
		|i| i > &0,
		|i| i-2
	);
	assert_eq!(it.next(), Some(100));
	assert_eq!(it.next(), Some(98));
	assert_eq!(it.next(), Some(96));
	assert_eq!(it.next(), Some(94));
	let mut it=it.skip(45);
	assert_eq!(it.next(), Some(2));
	assert_eq!(it.next(), None);
}
#[test]
fn for_f32() {
	let mut it = ForLoopIterator::new(
		10f32,
		|i| i > &0f32,
		|i| i-0.25
	);
	assert_eq!(it.next(), Some(10.0));
	assert_eq!(it.next(), Some(9.75));
	assert_eq!(it.next(), Some(9.50));
	let mut it = it.skip(36);
	assert_eq!(it.next(), Some(0.25));
	assert_eq!(it.next(), None);
}
#[derive(Clone, Copy, PartialEq, Debug)]
struct MyStruct(i32);
#[test]
fn for_whatever() {
	let mut it = ForLoopIterator::new(
		MyStruct(0),
		|i| i.0 < 10i32,
		|i| MyStruct(i.0+1)
	);
	assert_eq!(it.next(), Some(MyStruct(0)));
	assert_eq!(it.next(), Some(MyStruct(1)));
	assert_eq!(it.next(), Some(MyStruct(2)));
	
}
#[test]
fn from_docs() {
	let mut iter = ForLoopIterator::new(
        10,
        |i| i > &0,
        |i| i - &2
    );
    assert_eq!(iter.next(), Some(10));
    assert_eq!(iter.next(), Some(8));
    let mut iter = iter.skip(2);
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), None);
}