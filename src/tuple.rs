use std::{error::Error, future::Future, pin::Pin};

use crate::{caller::Caller, provide::Provide};

macro_rules! tuple {
    ($($param:ident)*) => {
        impl<Func, Fut, Pro, Err, $($param,)*> Caller<($($param,)*), Pro, Err> for Func
        where
            Func: Fn($($param),*) -> Fut + Clone + 'static,
            Fut: Future,
            Pro: $(Provide<$param, Err> +)*,
            Err: Error,
        {
            type Output = Fut::Output;
            type Future = Fut;

            #[inline]
            #[allow(non_snake_case)]
            fn call(&self, ($($param,)*): ($($param,)*)) -> Self::Future {
                (self)($($param),*)
            }
        }

        impl<Pro, Err, $($param,)*> Provide<($($param,)*), Err> for Pro
        where
            Pro: $(Provide<$param, Err> +)*,
            Err: Error,
        {
            #[inline]
            #[allow(non_snake_case)]
            fn provide<'a>(&'a self) -> Pin<Box<dyn Future<Output = Result<($($param,)*), Err>> + 'a>> {
                let fut = async move {
                    $(
                        let $param = <Pro as Provide<$param, Err>>::provide(self).await?;
                    )*
                    Ok(($($param,)*))
                };
                Box::pin(fut)
            }
        }
    };
}

// I'm sorry
tuple! { A }
tuple! { A B }
tuple! { A B C }
tuple! { A B C D }
tuple! { A B C D E }
tuple! { A B C D E F }
tuple! { A B C D E F G }
tuple! { A B C D E F G H }
tuple! { A B C D E F G H I }
tuple! { A B C D E F G H I J }
tuple! { A B C D E F G H I J K }
tuple! { A B C D E F G H I J K L }
tuple! { A B C D E F G H I J K L M }
tuple! { A B C D E F G H I J K L M N }
tuple! { A B C D E F G H I J K L M N O }
tuple! { A B C D E F G H I J K L M N O P }
tuple! { A B C D E F G H I J K L M N O P Q }
tuple! { A B C D E F G H I J K L M N O P Q R }
tuple! { A B C D E F G H I J K L M N O P Q R S }
tuple! { A B C D E F G H I J K L M N O P Q R S T }
tuple! { A B C D E F G H I J K L M N O P Q R S T U }
tuple! { A B C D E F G H I J K L M N O P Q R S T U V }
tuple! { A B C D E F G H I J K L M N O P Q R S T U V W }
tuple! { A B C D E F G H I J K L M N O P Q R S T U V W X }
tuple! { A B C D E F G H I J K L M N O P Q R S T U V W X Y }
tuple! { A B C D E F G H I J K L M N O P Q R S T U V W X Y Z }
