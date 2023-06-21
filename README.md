# learnRust

This Repo includes my notes and exercises on Rust the resources used are mentioned below:
* [Rust Book](https://doc.rust-lang.org/book)
* [Rust By Example](https://doc.rust-lang.org/rust-by-example/index.html)
* [Rustlings](https://rustlings.cool/)
* [Comprehensive Rust](https://github.com/google/comprehensive-rust) By Google

# Notes

Regularly usable functions:
* **iter()** - iter function is a method available for collections (such as vectors, arrays, and iterators) that returns an iterator over the elements of the collection. It provides an immutable view of the collection's elements for iteration purposes.

* **fold()** - fold function is a method available for iterators that performs a folding operation on the elements of the iterator. To use the fold() function, you start with an initial value, which is like a starting point. Then, you go through each number in the list, one by one. For each number, you perform an operation and update the initial value.

For example, let's say your initial value is 0, and you have a list of numbers [1, 2, 3, 4, 5]. You can use the fold() function to add each number to the initial value. So, you start with 0 and add 1, then add 2 to that, then add 3, and so on, until you've added all the numbers in the list.
