use core::{iter::zip, ops::Range};
use widestring::{U16CStr, Utf16Str};

#[cfg(target_os = "windows")]
use winapi::shared::ntdef::UNICODE_STRING;

#[inline]
pub const fn interp(x: u32, range1: Range<u32>, range2: Range<u32>) -> u32 {
    let offset = {
        let m = (x - range1.start) as f32 / (range1.end - range1.start) as f32;
        m * (range2.end - range2.start) as f32
    } as u32;

    range2.start + offset
}

#[inline(always)]
#[warn(dead_code)]
pub const fn min(x: usize, y: usize) -> usize {
    match x < y {
        true => x,
        false => y,
    }
}

#[cfg(target_os = "windows")]
#[inline]
pub fn convert_unicode_unchecked(s: &UNICODE_STRING) -> Option<&Utf16Str> {
    unsafe { U16CStr::from_ptr_truncate(s.Buffer as _, s.Length as _) }
        .map(|s| unsafe { Utf16Str::from_ucstr_unchecked(s) })
        .ok()
}

#[cfg(target_os = "windows")]
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct UnicodeString(UNICODE_STRING);

#[cfg(not(feature = "nosym"))]
#[cfg(target_os = "windows")]
impl core::fmt::Debug for UnicodeString {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let c_str = unsafe { U16CStr::from_ptr(self.0.Buffer, self.0.Length as _) };

        c_str
            .map(Utf16Str::from_ucstr)
            .map_err(|_| core::fmt::Error)?
            .fmt(f)
    }
}

#[cfg(not(feature = "nosym"))]
#[cfg(target_os = "windows")]
impl core::fmt::Display for UnicodeString {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        <Self as core::fmt::Debug>::fmt(self, f)
    }
}

#[cfg(target_os = "windows")]
impl<'a> TryFrom<UnicodeString> for &'a Utf16Str {
    type Error = widestring::error::Utf16Error;

    fn try_from(value: UnicodeString) -> Result<Self, Self::Error> {
        let c_str = unsafe { U16CStr::from_ptr_unchecked(value.0.Buffer, value.0.Length as _) };
        Utf16Str::from_ucstr(c_str)
    }
}

#[cfg(target_os = "windows")]
impl From<UNICODE_STRING> for UnicodeString {
    #[inline]
    fn from(value: UNICODE_STRING) -> Self {
        Self(value)
    }
}

#[cfg(target_os = "windows")]
impl From<UnicodeString> for UNICODE_STRING {
    #[inline]
    fn from(value: UnicodeString) -> Self {
        value.0
    }
}

pub trait EqIgnoreAsciiCase<Other = Self> {
    fn eq_ignore_ascii_case(self, other: Other) -> bool;
}

impl<T, Other> EqIgnoreAsciiCase<Other> for T
where
    T: IntoIterator<Item = char>,
    Other: IntoIterator<Item = char>,
{
    #[inline]
    default fn eq_ignore_ascii_case(self, other: Other) -> bool {
        zip(self, other).all(|(c1, c2)| c1.to_lowercase().eq(c2.to_lowercase()))
    }
}

#[cfg(target_os = "windows")]
impl<Other: IntoIterator<Item = char>> EqIgnoreAsciiCase<Other> for UnicodeString {
    #[inline]
    fn eq_ignore_ascii_case(self, other: Other) -> bool {
        let c_str = unsafe { U16CStr::from_ptr_unchecked(self.0.Buffer, self.0.Length as _) };
        Utf16Str::from_ucstr(c_str)
            .map(Utf16Str::chars)
            .map(|chars| chars.eq_ignore_ascii_case(other))
            .unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use crate::util::{interp, EqIgnoreAsciiCase};

    #[test]
    #[cfg(target_os = "windows")]
    fn test_eq_ignore_ascii_case() {
        assert!("Bongour".chars().eq_ignore_ascii_case("bongour".chars()));
        assert!(widestring::utf16str!("Testo")
            .chars()
            .eq_ignore_ascii_case("testo".chars()));

        assert!("Bongour".eq_ignore_ascii_case("bongour".chars().as_str()));
    }

    #[test]
    fn test_interp() {
        assert_eq!(4, interp(2, 0..4, 0..8));
        assert_eq!(2000, interp(1000, 0..4000, 0..8000));
        assert_eq!(9, interp(1, 1..6, 9..11));
        assert_eq!(3, interp(4, 1..6, 0..5));
    }

    #[cfg(test)]
    #[test]
    fn test_const_min() {
        use crate::util::min;
        assert_eq!(min(10, 11), 10);
        assert_eq!(min(11, 10), 10);
    }
}
