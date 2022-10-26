use futures::Future;
use std::fmt::Debug;
use std::{boxed::Box, pin::Pin};

pub fn combine_param<R: Sized + 'static, T: Sized + 'static, E: Sized + 'static + Debug>(
    t: T,
) -> impl FnOnce(Result<R, E>) -> Pin<Box<dyn Future<Output = Result<(R, T), E>>>> {
    |r: Result<R, E>| Box::pin(async { Ok((r.unwrap(), t)) })
}

pub fn combine_flat_param<R: Sized + 'static, T: Sized + 'static, E: Sized + 'static + Debug>(
    t: T,
) -> impl FnOnce(R) -> Pin<Box<dyn Future<Output = Result<(R, T), E>>>> {
    |r: R| Box::pin(async { Ok((r, t)) })
}

/* #[cfg(test)]
mod tests {
    use super::{combine_flat_param, combine_param};

    #[test]
    fn it_works() {

        //assert_eq!(result, 4);
    }
} */
