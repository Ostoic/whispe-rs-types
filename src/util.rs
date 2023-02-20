use core::{iter::zip, ops::Range};

#[inline]
pub const fn interp(x: u32, range1: Range<u32>, range2: Range<u32>) -> u32 {
    let offset = {
        let m = (x - range1.start) as f32 / (range1.end - range1.start) as f32;
        m * (range2.end - range2.start) as f32
    } as u32;

    range2.start + offset
}

#[test]
#[cfg(test)]
fn test_interp() {
    assert_eq!(4, interp(2, 0..4, 0..8));
    assert_eq!(2000, interp(1000, 0..4000, 0..8000));
    assert_eq!(9, interp(1, 1..6, 9..11));
    assert_eq!(3, interp(4, 1..6, 0..5));
}

#[inline(always)]
#[warn(dead_code)]
pub const fn min(x: usize, y: usize) -> usize {
    match x < y {
        true => x,
        false => y,
    }
}

#[cfg(test)]
#[test]
fn test_const_min() {
    assert_eq!(min(10, 11), 10);
    assert_eq!(min(11, 10), 10);
}

#[macro_export]
macro_rules! __stmts_strlen {
    ($($stmt:stmt,)*) => {{
        0usize $(+ stringify!($stmt).len())*
    }}
}

#[cfg(test)]
#[test]
fn test_stmts_strlen() {
    let x = crate::__stmts_strlen!("hello", "there",);

    assert_eq!(x, 16);
}

use widestring::{U16CStr, Utf16Str};
use winapi::shared::ntdef::UNICODE_STRING;

#[inline]
pub fn convert_unicode_unchecked(s: &UNICODE_STRING) -> Option<&Utf16Str> {
    unsafe { U16CStr::from_ptr_truncate(s.Buffer as _, s.Length as _) }
        .map(|s| unsafe { Utf16Str::from_ucstr_unchecked(s) })
        .ok()
}

pub struct UnicodeString(pub UNICODE_STRING);

impl From<UNICODE_STRING> for UnicodeString {
    #[inline]
    fn from(value: UNICODE_STRING) -> Self {
        Self(value)
    }
}

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

#[test]
fn test_eq_ignore_ascii_case() {
    assert!("Bongour".chars().eq_ignore_ascii_case("bongour".chars()));
    assert!(utf16str!("Testo")
        .chars()
        .eq_ignore_ascii_case("testo".chars()));
    assert!("Bongour".eq_ignore_ascii_case("bongour".chars().as_str()));
}
