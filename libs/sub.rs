// Defining doc-test

/// Substraction - substract two numbers!
///
/// This function takes two arguments and substracts them from each other.
/// Code blocks start with tripe backquotes and have implicit
/// 'fn main()' inside and 'extern crate <createname>'.
/// So, for our example that will be 'my_libs' (look at Cargo.toml)
///
/// ```
/// let result = my_libs::sub::substract(35, 34);
/// assert_eq!(result, 1);
/// ```
///
pub fn substract(a: i32, b: i32) -> i32 {
    a - b
}
