use std::future::Future;

/// The interface for calling a function with arguments.
/// Is implemented for tuples of up to 26 items to allow for up to 26 arguments.
///
/// # Example
/// ```rs
/// async fn add(a: i32, b: i32) -> i32 {
///     a + b
/// }
///
/// add.call((1, 2)).await;
/// ```
pub trait Caller<Args, Pro, Err> {
    type Output;
    type Future: Future<Output = Self::Output>;

    fn call(&self, args: Args) -> Self::Future;
}
