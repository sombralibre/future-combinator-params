use futures::Future;
use std::fmt::Debug;
use std::{boxed::Box, pin::Pin};

pub fn combine_param<R, T, E>(
    t: T,
) -> impl FnOnce(Result<R, E>) -> Pin<Box<dyn Future<Output = Result<(R, T), E>> + Sync + Send>>
where
    R: Sized + 'static + Sync + Send,
    T: Sized + 'static + Sync + Send,
    E: Sized + 'static + Debug + Sync + Send,
{
    |r: Result<R, E>| Box::pin(async { Ok((r?, t)) })
}

pub fn combine_flat_param<R, T, E>(
    t: T,
) -> impl FnOnce(R) -> Pin<Box<dyn Future<Output = Result<(R, T), E>> + Sync + Send>>
where
    R: Sized + 'static + Sync + Send,
    T: Sized + 'static + Sync + Send,
    E: Sized + 'static + Debug + Sync + Send,
{
    |r: R| Box::pin(async { Ok((r, t)) })
}
