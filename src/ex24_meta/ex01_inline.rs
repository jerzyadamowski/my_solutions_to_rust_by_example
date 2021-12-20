#[doc(inline)]
pub use bar::_Bar;

/// bar docs
mod bar {
  /// the docs for Bar
  pub struct _Bar;
}

// Example from libcore/prelude
#[doc(no_inline)]
pub use std::mem::drop;

// Example from the futures-rs library
// #[doc(hidden)]
// pub use self::async_await::*;
