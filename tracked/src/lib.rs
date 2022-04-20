#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

pub use tracked_impl::tracked;

use once_cell::sync::Lazy;
use std::sync::Mutex;

#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StringError(String);

impl std::fmt::Display for StringError {
 fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
  write!(f, "{}", self.0)
 }
}

impl std::fmt::Debug for StringError {
 fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
  write!(f, "{}", self.0)
 }
}

impl From<String> for StringError {
 fn from(s: String) -> Self {
  Self(s)
 }
}

impl From<StringError> for String {
 fn from(e: StringError) -> Self {
  e.0
 }
}

impl std::error::Error for StringError {}

static BUILD_ID: Lazy<Mutex<String>> = Lazy::new(Default::default);

/// Call this once at startup to include an identifying string in reported errors.
pub fn set_build_id(build_id: impl Into<String>) {
 let mut s = build_id.into();
 s.push('/');
 *BUILD_ID.lock().unwrap() = s;
}

/// Provides the `t` ("track") method for `Option` and `Result`.
pub trait Track<T, E>: private::Sealed {
 fn t(self) -> Result<T, StringError>;
}

impl<T> Track<T, core::convert::Infallible> for Option<T> {
 #[track_caller]
 fn t(self) -> Result<T, StringError> {
  match self {
   Some(t) => Ok(t),
   None => Err(
    format!("NoneError at {}{}", BUILD_ID.lock().unwrap(), std::panic::Location::caller()).into(),
   ),
  }
 }
}

impl<T, E> Track<T, E> for Result<T, E>
where
 E: ToString,
{
 #[track_caller]
 fn t(self) -> Result<T, StringError> {
  match self {
   Ok(t) => Ok(t),
   Err(e) => Err(
    format!("{} at {}{}", e.to_string(), BUILD_ID.lock().unwrap(), std::panic::Location::caller())
     .into(),
   ),
  }
 }
}

// impl<T> Track<T, core::convert::Infallible> for Result<T> {
//  #[track_caller]
//  fn t(self) -> Result<T, StringError> {
//   match self {
//    std::result::Result::Ok(t) => std::result::Result::Ok(t),
//    Err(e) => {
//     let l = std::panic::Location::caller();
//     let msg = e.to_string();
//     Err(
//      format!("{} at {}{}:{}:{}", msg, BUILD_ID.lock().unwrap(), l.file(), l.line(), l.column())
//       .into(),
//     )
//    }
//   }
//  }
// }

pub(crate) mod private {
 pub trait Sealed {}

 impl<T> Sealed for Option<T> {}
 impl<T, E> Sealed for Result<T, E> {}
}
