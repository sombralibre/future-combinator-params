use futures::Future;
use std::{boxed::Box, pin::Pin};

pub fn combine_param<R: Sized + 'static, T: Sized + 'static>(
    t: T,
) -> impl FnOnce(Result<R, ()>) -> Pin<Box<dyn Future<Output = Result<(R, T), ()>>>> {
    |r: Result<R, ()>| Box::pin(async { Ok((r.unwrap(), t)) })
}

pub fn combine_flat_param<R: Sized + 'static, T: Sized + 'static>(
    t: T,
) -> impl FnOnce(R) -> Pin<Box<dyn Future<Output = Result<(R, T), ()>>>> {
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

