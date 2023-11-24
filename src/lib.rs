use std::borrow::Cow;

mod components;

#[cfg(feature = "node")]
mod napi;
// #[cfg(feature = "py")]
// compile_error!("Feature `py` is not supported yet");
// #[cfg(feature = "py")]
// mod pypi;

type Result<T> = std::result::Result<T, Cow<'static, str>>;
