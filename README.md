# For Loop Iterators
Allows you to create Rust iterators that act like traditional for loops!

Iterators will produce values just like for loops do
# Examples
```java
// A for loop in Java
for (int i = 0; i < 10; i++>) {
	out.println(i);
}
```
```rust
// A Rust Iterator that performs the same function
ForLoopIterator::new(
        0,					// Initial Value
        |i| i < &10,		// Predicate that returns true if a value should be returned
        |i| i + 1		// Function that, given a value from the iterator, returns the next one
    )
	.for_each( |i| {
		println!("{i}");
	});
```