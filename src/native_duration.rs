#![cfg(windows)]

use core::time::Duration;

use winapi::um::winnt::LARGE_INTEGER;

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct NativeDuration(LARGE_INTEGER);

impl PartialEq for NativeDuration {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        unsafe { self.0.QuadPart() == other.0.QuadPart() }
    }
}

#[cfg(not(feature = "nosym"))]
impl core::fmt::Debug for NativeDuration {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let duration = Duration::from(*self);
        f.debug_tuple("NativeDuration").field(&duration).finish()
    }
}

#[cfg(not(feature = "nosym"))]
impl core::fmt::Display for NativeDuration {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        <Self as core::fmt::Debug>::fmt(self, f)
    }
}

impl PartialOrd for NativeDuration {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        let duration: Duration = (*self).into();
        let other_duration: Duration = (*other).into();
        duration.partial_cmp(&other_duration)
    }
}

#[cfg(feature = "bincode")]
impl bincode::Encode for NativeDuration {
    fn encode<E: bincode::enc::Encoder>(
        &self,
        encoder: &mut E,
    ) -> Result<(), bincode::error::EncodeError> {
        bincode::Encode::encode(unsafe { self.0.QuadPart() }, encoder)
    }
}

#[cfg(feature = "bincode")]
impl bincode::Decode for NativeDuration {
    fn decode<D: bincode::de::Decoder>(
        decoder: &mut D,
    ) -> Result<Self, bincode::error::DecodeError> {
        let large_int: LARGE_INTEGER = unsafe { core::mem::transmute(i64::decode(decoder)?) };
        Ok(Self(large_int))
    }
}

#[cfg(feature = "bincode")]
bincode::impl_borrow_decode!(NativeDuration);

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

impl From<NativeDuration> for Duration {
    fn from(val: NativeDuration) -> Self {
        let large_int = unsafe { val.0.QuadPart() };
        let millis: u128 = (-large_int / 10000) as _;
        Duration::from_millis(millis as _)
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

#[cfg(test)]
mod tests {
    use core::time::Duration;

    use crate::NativeDuration;

    #[test]
    #[cfg(feature = "std")]
    #[cfg(feature = "bincode")]
    fn test_duration_bincode() -> anyhow::Result<()> {
        let bincode_config = bincode::config::standard();

        let duration = NativeDuration::from(Duration::from_secs(2));
        let duration_bytes = bincode::encode_to_vec(&duration, bincode_config)?;
        let (decoded_duration, _): (NativeDuration, usize) =
            bincode::decode_from_slice(&duration_bytes, bincode_config)?;

        assert_eq!(decoded_duration, duration);
        Ok(())
    }

    #[test]
    fn test_duration_conversions() {
        let ms300 = Duration::from_millis(300);
        let native_duration = NativeDuration::from(ms300);
        assert_eq!(
            <NativeDuration as Into<Duration>>::into(native_duration),
            ms300
        );

        let native_100ms: NativeDuration = unsafe { core::mem::transmute([-(100 as i64) * 10000]) };
        let ms100 = NativeDuration::from(Duration::from_millis(100));
        assert!(native_100ms == ms100);
    }
}
