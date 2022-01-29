#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

pub use anyhow::*;
pub use tracked_impl::tracked;

/// Provides the `t` ("track") method for `Option` and `Result`.
pub trait Track<T, E>: private::Sealed {
 fn t(self) -> Result<T, anyhow::Error>;
}

impl<T> Track<T, core::convert::Infallible> for Option<T> {
 #[track_caller]
 fn t(self) -> Result<T, anyhow::Error> {
  match self {
   Some(t) => Ok(t),
   None => {
    let l = std::panic::Location::caller();
    Err(anyhow::Error::msg(format!("NoneError at {}:{}:{}", l.file(), l.line(), l.column())))
   }
  }
 }
}

impl<T, E> Track<T, E> for Result<T, E>
where
 E: std::error::Error + Send + Sync + 'static,
{
 #[track_caller]
 fn t(self) -> Result<T, anyhow::Error>
 where
  E: std::error::Error + Send + Sync + 'static,
 {
  match self {
   std::result::Result::Ok(t) => Ok(t),
   Err(e) => {
    let l = std::panic::Location::caller();
    let msg = e.to_string();
    Err(anyhow::Error::new(e).context(format!(
     "{} at {}:{}:{}",
     msg,
     l.file(),
     l.line(),
     l.column()
    )))
   }
  }
 }
}

impl<T> Track<T, core::convert::Infallible> for Result<T> {
 #[track_caller]
 fn t(self) -> Result<T, anyhow::Error> {
  match self {
   std::result::Result::Ok(t) => Ok(t),
   Err(e) => {
    let l = std::panic::Location::caller();
    let msg = e.to_string();
    Err(e.context(format!("{} at {}:{}:{}", msg, l.file(), l.line(), l.column())))
   }
  }
 }
}

pub(crate) mod private {
 pub trait Sealed {}

 impl<T> Sealed for Option<T> {}
 impl<T, E> Sealed for Result<T, E> {}
}
