mod caller;
mod provide;
mod store;
mod tuple;

use std::error::Error;

pub use caller::Caller;
pub use provide::Provide;
pub use store::TypeStore;

/// Use a provider to call a function
///
/// # Example
/// ```rs
/// use extractors::{call, Provide};
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
/// impl Provide<i32, Err> for Example {
///     fn provide<'a>(&'a self) -> Pin<Box<dyn Future<Output = Result<i32, Err>> + 'a>> {
///         Box::pin(std::future::ready(Ok(self.0)))
///     }
/// }
///
/// let example = Example(1);
/// let result = call(&example, add).await;
/// ```
pub async fn call<P, F, Args, Err>(provider: &P, func: F) -> Result<F::Output, Err>
where
    P: Provide<Args, Err>,
    F: Caller<Args, P, Err>,
    Err: Error,
{
    Ok(func.call(provider.provide().await?).await)
}
