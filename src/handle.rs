use core::{ffi::c_void, ptr::NonNull};

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
