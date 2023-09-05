#![cfg(nightly)]
#![cfg_attr(not(feature = "std"), no_std)]
#![feature(
    min_specialization,
    const_fn_floating_point_arithmetic,
    const_nonnull_new,
    const_option_ext
)]

#[cfg(feature = "alloc")]
#[no_link]
#[macro_use]
#[allow(unused_imports)]
extern crate alloc;

#[cfg(windows)]
pub mod ntapi_ext;

pub mod handle;

pub mod ntstatus;
pub use ntstatus::NtStatus;

#[cfg(windows)]
pub mod native_duration;

#[cfg(windows)]
pub use native_duration::NativeDuration;

pub mod types {
    #[cfg(windows)]
    pub use ntapi::*;
}

pub mod util;

pub mod hash;
