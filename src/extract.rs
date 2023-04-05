use std::{error::Error, future::Future, pin::Pin};

/// Implemented on a provider, it extracts a future that resolves to `Ok(T)` on success or `Err(Self::Err)` on error.
///
/// # Example
/// ```rs
/// use extractors::Extract;
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
/// impl Extract<i32, Err> for Example {
///     fn extract<'a>(&'a self) -> Pin<Box<dyn Future<Output = Result<i32, Err>> + 'a>> {
///         Box::pin(std::future::ready(Ok(self.0)))
///     }
/// }
pub trait Extract<T, E: Error>: Sized {
    /// Extracts a value of type `T`.
    /// This method should return a future that resolves to `Ok(T)` on success or `Err(Self::Error)` on error.
    ///
    /// # Example
    /// ```rs
    /// use extractors::Extract;
    ///
    /// // uses the `Example` struct and impl from the `Extract` trait documentation above.
    /// let example = Example(1);
    /// let extracted: i32 = example.extract().await.unwrap();
    /// assert_eq!(extracted, 1);
    /// ```
    fn extract<'a>(&'a self) -> Pin<Box<dyn Future<Output = Result<T, E>> + 'a>>;
}

impl<P, T, E> Extract<Option<T>, E> for P
where
    P: Extract<T, E>,
    E: Error,
{
    fn extract<'a>(&'a self) -> Pin<Box<dyn Future<Output = Result<Option<T>, E>> + 'a>> {
        Box::pin(async move {
            match self.extract().await {
                Ok(value) => Ok(Some(value)),
                Err(_) => Ok(None),
            }
        })
    }
}
