use std::{error::Error, future::Future, pin::Pin};

/// Implemented on a provider, it provides a future that resolves to `Ok(T)` on success or `Err(Self::Err)` on error.
///
/// # Example
/// ```rs
/// use extractors::Provide;
/// use std::{error::Error, fmt::Display};
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
pub trait Provide<T, E: Error>: Sized {
    /// Provide a value of type `T`.
    /// This method should return a future that resolves to `Ok(T)` on success or `Err(Self::Error)` on error.
    ///
    /// # Example
    /// ```rs
    /// use extractors::Provide;
    ///
    /// // uses the `Example` struct and impl from the `Provide` trait documentation above.
    /// let example = Example(1);
    /// let provided: i32 = example.provide().await.unwrap();
    /// assert_eq!(provided, 1);
    /// ```
    fn provide<'a>(&'a self) -> Pin<Box<dyn Future<Output = Result<T, E>> + 'a>>;
}
