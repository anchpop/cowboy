mod sheriff;
mod traits;

pub use sheriff::{SHERIFF, Sheriff};
use std::sync::{Arc, RwLock};

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub struct Cowboy<T> {
    pub inner: Arc<RwLock<T>>,
}

pub fn cowboy<T>(inner: T) -> Cowboy<T> {
    Cowboy::new(inner)
}

impl<T> Cowboy<T> {
    pub fn new(inner: T) -> Self {
        Cowboy {
            inner: Arc::new(RwLock::new(inner)),
        }
    }

    pub fn r(&self) -> std::sync::RwLockReadGuard<'_, T> {
        self.read()
    }

    pub fn read(&self) -> std::sync::RwLockReadGuard<'_, T> {
        self.inner.read().unwrap()
    }

    pub fn w(&self) -> std::sync::RwLockWriteGuard<'_, T> {
        self.write()
    }

    pub fn write(&self) -> std::sync::RwLockWriteGuard<'_, T> {
        self.inner.write().unwrap()
    }

    /// Unsoundly get a mutable reference to the value.
    /// 🚨 DANGER ZONE 🚨 This function can be trivially used to get multiple mutable references to the same value, which is instantly undefined behavior.
    /// If you can use .write(), please use that instead. On the other hand... a little undefined behavior is usually okay in practice.
    #[deprecated(
        since = "0.1.0",
        note = "🚨 DANGER ZONE 🚨 This function gives you a reference to the inner value, but won't prevent anyone else from mutating it while you have it. If anyone else uses .write() or .howdy(), that will be undefined behavior. If you can use .read(), please use that instead. On the other hand... a little undefined behavior is usually okay in practice."
    )]
    #[must_use = "If you're calling this function, at least use the returned reference"]
    #[allow(clippy::transmute_ptr_to_ref)] // To avoid clippy warnings about the transmute
    pub fn howdy(&self) -> &T {
        let guard: std::sync::RwLockReadGuard<T> = self.inner.read().expect("RwLock poisoned");
        let inner_ref: &T = &guard;

        // Unsafely extend the lifetime of the mutable reference.
        // This is where the unsoundness lies. We are telling the compiler
        // that `inner_mut_ref` (which is tied to `guard`) can actually live
        // as long as `&self` (the lifetime of the `Cowboy` reference passed to `yehaw`).
        // This is a lie because `guard` will be dropped at the end of this function,
        // releasing the lock.
        let extended_lifetime_mut_ref: &T =
            // SAFETY: None at all
            unsafe { std::mem::transmute::<&T, &T>(inner_ref) };

        extended_lifetime_mut_ref
    }

    /// Unsoundly get a mutable reference to the value.
    /// 🚨 DANGER ZONE 🚨 This function can be trivially used to get multiple mutable references to the same value, which is instantly undefined behavior.
    /// If you can use .write(), please use that instead. On the other hand... a little undefined behavior is usually okay in practice.
    #[deprecated(
        since = "0.1.0",
        note = "🚨 DANGER ZONE 🚨 This function can be trivially used to get multiple mutable references to the same value, which is undefined behavior. If you can use .write(), please use that instead. On the other hand... a little undefined behavior is usually okay in practice."
    )]
    #[must_use = "If you're calling this function, at least use the returned reference"]
    #[allow(clippy::transmute_ptr_to_ref)] // To avoid clippy warnings about the transmute
    #[allow(clippy::mut_from_ref)] // To avoid clippy warnings about the transmute
    pub fn yehaw(&self) -> &mut T {
        let mut guard: std::sync::RwLockWriteGuard<T> =
            self.inner.write().expect("RwLock poisoned");
        let inner_mut_ref: &mut T = &mut guard;

        // Unsafely extend the lifetime of the mutable reference.
        // This is where the unsoundness lies. We are telling the compiler
        // that `inner_mut_ref` (which is tied to `guard`) can actually live
        // as long as `&self` (the lifetime of the `Cowboy` reference passed to `yehaw`).
        // This is a lie because `guard` will be dropped at the end of this function,
        // releasing the lock.
        let extended_lifetime_mut_ref: &mut T =
            // SAFETY: None at all
            unsafe { std::mem::transmute::<&mut T, &mut T>(inner_mut_ref) };

        extended_lifetime_mut_ref
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
