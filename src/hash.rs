#![warn(dead_code)]
use core::ops::Range;

#[macro_export]
macro_rules! nt_hash {
    ($s:expr) => {{
        const _NT_HASH: u32 = $crate::hash::nt_hash($s);
        _NT_HASH
    }};
}

#[macro_export]
macro_rules! hash_str {
    ($s:expr, $r:expr) => {{
        const _HASH: u32 = $crate::hash::hash($s, $r);
        _HASH
    }};
    ($s:expr) => {
        $crate::hash_str!($s, 0..usize::MAX)
    };
}

#[macro_export]
macro_rules! hash_bytes_fn {
    ($h:ident, $s:expr, $r:expr) => {{
        const _HASH: u32 = $h($s, $r);
        _HASH
    }};
}

#[macro_export]
macro_rules! hash_bytes {
    ($s:expr, $r:expr) => {{
        const _HASH: u32 = $crate::hash::hash_bytes_impl($s, $r);
        _HASH
    }};
}

pub fn hash_utf16str(s: &widestring::Utf16Str) -> u32 {
    let bytes: &[u8] = unsafe { core::slice::from_raw_parts(s.as_ptr() as _, s.len() * 2) };
    hash_bytes_impl(bytes, 0..usize::MAX, 2)
}

/// Modified from the obfstr library
pub const fn djb2(s: &[u8], range: Range<usize>, step: usize) -> u32 {
    let mut result = 3581u32;
    let mut i = range.start; // Starts at the 3rd character

    // TODO: Fix min(.., range.end) code when step != 1
    while i < min(s.len(), range.end) {
        result = result.wrapping_mul(33) ^ s[i] as u32;
        i += step;
    }
    result
}

#[cfg(test)]
#[test]
fn test_djb2_impl() {
    let kgs_tests: [(&[u8], u32); 2] = [(b"admin", 2814062226), (b"admins", 2669740193)];

    for test in kgs_tests.iter() {
        assert_eq!(djb2(test.0, 0..usize::MAX, 1), test.1);
    }
}

#[inline(always)]
pub const fn sdbm(s: &[u8], range: Range<usize>, step: usize) -> u32 {
    let mut result = 3581u32;
    let mut i = range.start;

    while i < min(s.len(), range.end) {
        result = result.wrapping_mul(65599) + s[i] as u32;
        i += step;
    }
    result
}

#[test]
#[cfg(test)]
fn test_sdbm_impl() {
    let kgs_tests: [(&[u8], u32); 2] = [(b"admin", 1243319410), (b"admins", 3375992961)];

    for test in kgs_tests.iter() {
        assert_eq!(sdbm(test.0, 0..usize::MAX, 1), test.1);
    }
}

/// ElfHash (32-bit variant) hash function.
#[warn(dead_code)]
pub const fn seeded_elf(s: &[u8], range: Range<usize>, step: usize) -> u32 {
    let mut result: u32 = obfstr::random!(u32)
        .wrapping_mul((range.end - range.start) as _)
        .wrapping_mul(s.len() as _);

    let mut high;

    let mut i = range.start;
    while i < min(s.len(), range.end) {
        result = result.rotate_left(4) + s[i] as u32;
        high = result & 0xF0000000;
        if high != 0 {
            result ^= high.rotate_right(24);
        }

        result &= !high;
        i += step;
    }

    result
}

use crate::util::min;
pub use djb2 as hash_bytes_impl;

#[inline(always)]
pub fn wide_hash(s: impl AsRef<[u16]>, range: Range<usize>) -> u32 {
    let (_, s, _) = unsafe { s.as_ref().align_to::<u8>() };
    hash_bytes_impl(s, range, 2)
}

#[test]
#[cfg(test)]
fn test_wide_hash() {
    let x = obfstr::obfwide!("\\Man\\Test");
    let y = "\\Man\\Test";

    assert_eq!(wide_hash(x, 0..usize::MAX), hash(y, 0..usize::MAX));
}

#[inline(always)]
pub const fn hash(s: &str, range: Range<usize>) -> u32 {
    hash_bytes_impl(s.as_bytes(), range, 1)
}

#[inline(always)]
pub const fn nt_hash(s: &str) -> u32 {
    crate::hash::hash(s, 2..s.len())
}

#[cfg(test)]
#[test]
fn test_nt_hash() {
    // Verify nt_hash ignores the first two letters
    assert_eq!(nt_hash!("ZwCreateProcess"), nt_hash!("NtCreateProcess"));
    assert_eq!(
        nt_hash!("ZwDelayExecution"),
        crate::hash_str!("NtDelayExecution", 2..usize::MAX)
    );
}
