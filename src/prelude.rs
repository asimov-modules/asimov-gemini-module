// This is free and unencumbered software released into the public domain.

#[cfg(not(feature = "std"))]
extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

#[allow(unused)]
pub use alloc::{
    string::{String, ToString},
    vec,
    vec::Vec,
};
#[allow(unused)]
pub use core::{fmt, result::Result};
#[cfg(feature = "std")]
use std as alloc;

#[cfg(feature = "tracing")]
#[allow(unused)]
pub use tracing::{debug, error, info, Level, span, warn};

