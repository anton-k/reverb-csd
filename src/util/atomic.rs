use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};

/// A small helper to atomically load and store an `f32` value.
pub struct AtomicF32(AtomicU32);

impl AtomicF32 {
    /// Stores the given `value` using the given `order`ing.
    #[inline]
    pub fn store(&self, value: f32) {
        self.0.store(f32_to_u32_bytes(value), Ordering::Relaxed)
    }

    /// Loads the contained `value` using the given `order`ing.
    #[inline]
    pub fn load(&self) -> f32 {
        f32_from_u32_bytes(self.0.load(Ordering::Relaxed))
    }

    /// Stores the given `value`, and returns the previously stored one.
    #[inline]
    pub fn swap(&self, value: f32) -> f32 {
        f32::from_bits(self.0.swap(value.to_bits(), Ordering::Relaxed))
    }
}

/// Creates a new atomic `f32`.
impl From<f32> for AtomicF32 {
    #[inline]
    fn from(value: f32) -> Self {
        Self(AtomicU32::new(f32_to_u32_bytes(value)))
    }
}

/// Packs a `f32` into the bytes of an `u32`.
///
/// The resulting value is meaningless and should not be used directly,
/// except for unpacking with [`f32_from_u32_bytes`].
///
/// This is an internal helper used by [`AtomicF32`].
#[inline]
fn f32_to_u32_bytes(value: f32) -> u32 {
    u32::from_ne_bytes(value.to_ne_bytes())
}

/// The counterpart to [`f32_to_u32_bytes`].
#[inline]
fn f32_from_u32_bytes(bytes: u32) -> f32 {
    f32::from_ne_bytes(bytes.to_ne_bytes())
}

// WARNING: not completley atomic as values can be updated in between
pub struct AtomicValue {
    is_updated: AtomicBool,
    value: AtomicF32,
}

impl AtomicValue {
    #[inline]
    pub fn swap(&self, new_value: f32) -> f32 {
        let res = self.value.swap(new_value);
        if res != new_value {
            let _ = self
                .is_updated
                .swap(true, std::sync::atomic::Ordering::Relaxed);
        }
        res
    }

    #[inline]
    pub fn load(&self) -> f32 {
        self.value.load()
    }

    #[inline]
    pub fn store(&self, new_value: f32) {
        self.value.store(new_value);
        self.is_updated
            .store(true, std::sync::atomic::Ordering::Relaxed);
    }

    #[inline]
    fn is_updated(&self) -> bool {
        self.is_updated.load(std::sync::atomic::Ordering::Relaxed)
    }

    #[inline]
    fn updated_processed(&self) {
        self.is_updated
            .store(false, std::sync::atomic::Ordering::Relaxed);
    }

    #[inline]
    pub fn call_on_update<F: FnOnce()>(&self, callback: F) -> bool {
        if self.is_updated() {
            callback();
            self.updated_processed();
            true
        } else {
            false
        }
    }
}

impl From<f32> for AtomicValue {
    #[inline]
    fn from(value: f32) -> Self {
        Self {
            is_updated: AtomicBool::new(true),
            value: AtomicF32::from(value),
        }
    }
}
