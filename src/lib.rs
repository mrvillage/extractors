//! # extractors
//!
//! A library for using a provider to extract values as arguments to a function.
//! This is useful for calling a function with only a provider and enabling it to take a variety of arguments extracted from the provider, such as a database connection or a web request.

mod caller;
mod extract;
mod tuple;

use std::error::Error;

pub use caller::Caller;
pub use extract::Extract;

/// Use a provider to call a function
///
/// # Example
/// ```rs
/// use extractors::{call, Extract};
///
/// async fn add(a: i32, b: i32) -> i32 {
///     a + b
/// }
///
/// struct Example(i32);
///
/// #[derive(Debug)]
/// struct Err;
///
/// impl Display for Err {
///     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
///         write!(f, "Err")
///     }
/// }
///
/// impl Error for Err {}
///
/// impl Extract<i32, Err> for Example {
///     fn extract<'a>(&'a self) -> Pin<Box<dyn Future<Output = Result<i32, Err>> + 'a>> {
///         Box::pin(std::future::ready(Ok(self.0)))
///     }
/// }
///
/// let example = Example(1);
/// let result = call(&example, add).await;
/// ```
pub async fn call<P, F, Args, Err>(provider: &P, func: F) -> Result<F::Output, Err>
where
    P: Extract<Args, Err>,
    F: Caller<Args, P, Err>,
    Err: Error,
{
    Ok(func.call(provider.extract().await?).await)
}
