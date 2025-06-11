#[cfg(not(feature = "safe"))]
mod asynch;
#[cfg(not(feature = "safe"))]
mod blocking;

#[cfg(not(feature = "safe"))]
pub(crate) use asynch::AsyncTestFlash;
#[cfg(not(feature = "safe"))]
pub(crate) use blocking::BlockingTestFlash;
