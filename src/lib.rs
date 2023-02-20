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

use core::{ffi::c_void, ptr::NonNull, time::Duration};

pub type Handle = *mut c_void;
pub type NonNullHandle = NonNull<c_void>;

pub const CURRENT_PROCESS: NonNullHandle =
unsafe { NonNullHandle::new(-1isize as _).unwrap_unchecked() };

pub const CURRENT_THREAD: NonNullHandle =
    unsafe { NonNullHandle::new(-2isize as _).unwrap_unchecked() };

pub trait AsRawHandle {
    fn as_raw_handle(&self) -> Handle;
}

impl<H> AsRawHandle for NonNull<H>
where
    *mut H: AsRawHandle,
{
    #[inline(always)]
    fn as_raw_handle(&self) -> Handle {
        self.as_ptr().as_raw_handle()
    }
}

impl<H: AsRawHandle> AsRawHandle for &H {
    #[inline(always)]
    fn as_raw_handle(&self) -> Handle {
        (*self).as_raw_handle()
    }
}

macro_rules! impl_as_raw_handle {
    ($handle:ty) => {
        impl AsRawHandle for $handle {
            #[inline]
            fn as_raw_handle(&self) -> Handle {
                (*self) as _
            }
        }
    };
}

impl_as_raw_handle!(Handle);
// impl_as_raw_handle!(*mut ::core::ffi::c_void);

// #[cfg(feature = "std")]
// impl_as_raw_handle!(*mut ::std::ffi::c_void);

/// Construct I/O objects from raw handles.
pub trait FromRawHandle {
    /// # Safety
    /// It is inherently unsafe to use an implementation-specific handle to create encapsulated IO objects
    unsafe fn from_raw_handle(handle: NonNullHandle) -> Self;
}

#[derive(Clone, Copy)]
pub struct NativeDuration(LARGE_INTEGER);

impl NativeDuration {
    #[inline]
    pub fn as_ptr(&self) -> *const LARGE_INTEGER {
        &self.0 as _
    }
}

impl AsRef<LARGE_INTEGER> for NativeDuration {
    #[inline]
    fn as_ref(&self) -> &LARGE_INTEGER {
        &self.0
    }
}

impl From<Duration> for NativeDuration {
    #[inline]
    fn from(x: Duration) -> Self {
        let mut interval = LARGE_INTEGER::default();
        unsafe { *interval.QuadPart_mut() = -(x.as_millis() as i64) * 10000 };
        Self(interval)
    }
}

pub mod ntapi_ext;
pub mod ntstatus;
pub use ntstatus::NtStatus;
use winapi::um::winnt::LARGE_INTEGER;

pub mod types {
    pub use ntapi::*;
}

pub mod util;
