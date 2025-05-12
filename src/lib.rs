//! # Cowboy
//!
//! A safe and convenient container for quick prototyping. (Essentially a wrapper around `Arc<RwLock<T>>`)
//!
//! You should probably not use this crate in production, but it's a fun way to prototype something.
//!
//! ```rust
//! use cowboy::*;
//!
//! let cowboy = 42.cowboy();
//! *cowboy.w() = 84;
//! assert_eq!(*cowboy.r(), 84);
//! ```
//!
//! ## Sheriff
//!
//! ```rust
//! use cowboy::*;
//!
//! SHERIFF.register("player1_score", 0_i32.cowboy()); // String key
//! let score = SHERIFF.get::<_, i32>("player1_score");
//! *score.w() += 1;
//! assert_eq!(*score.r(), 1);
//! ```
//!
//! (The Sheriff should really not be used in production.)

mod sheriff;
mod traits;

pub use sheriff::{SHERIFF, Sheriff};
use std::sync::{Arc, RwLock};

pub struct Cowboy<T> {
    pub inner: Arc<RwLock<T>>,
}

impl<T> Cowboy<T> {
    /// Create a new `Cowboy` wrapping the provided value
    ///
    /// ```rust
    /// use cowboy::*;
    ///
    /// let cowboy = Cowboy::new(42);
    /// assert_eq!(*cowboy.read(), 42);
    /// ```
    pub fn new(inner: T) -> Self {
        Cowboy {
            inner: Arc::new(RwLock::new(inner)),
        }
    }

    /// Get a read guard to the inner value.
    /// Shorthand for [`Cowboy::read()`]
    ///
    /// ```rust
    /// use cowboy::*;
    ///
    /// let cowboy = 42.cowboy();
    /// assert_eq!(*cowboy.r(), 42);
    /// ```
    pub fn r(&self) -> std::sync::RwLockReadGuard<'_, T> {
        self.read()
    }

    /// Get a read guard to the inner value.
    ///
    /// ```rust
    /// use cowboy::*;
    ///
    /// let cowboy = 42.cowboy();
    /// assert_eq!(*cowboy.read(), 42);
    /// ```
    pub fn read(&self) -> std::sync::RwLockReadGuard<'_, T> {
        self.inner.read().unwrap()
    }

    /// Get a write guard to the inner value.
    /// Shorthand for [`Cowboy::write()`]
    ///
    /// ```rust
    /// use cowboy::*;
    ///
    /// let cowboy = 42.cowboy();
    /// *cowboy.w() = 84;
    /// assert_eq!(*cowboy.read(), 84);
    /// ```
    pub fn w(&self) -> std::sync::RwLockWriteGuard<'_, T> {
        self.write()
    }

    /// Get a write guard to the inner value.
    ///
    /// ```rust
    /// use cowboy::*;
    ///
    /// let cowboy = 42.cowboy();
    /// *cowboy.write() = 84;
    /// assert_eq!(*cowboy.read(), 84);
    /// ```
    pub fn write(&self) -> std::sync::RwLockWriteGuard<'_, T> {
        self.inner.write().unwrap()
    }

    /// Modify the inner value using a function.
    ///
    /// ```rust
    /// use cowboy::*;
    ///
    /// let cowboy = 42.cowboy();
    /// cowboy.modify(|value| *value *= 2);
    /// assert_eq!(*cowboy.read(), 84);
    /// ```
    pub fn modify<F>(&self, f: F)
    where
        F: FnOnce(&mut T),
    {
        let mut guard = self.write();
        f(&mut *guard);
    }

    /// Set the inner value.
    ///
    /// ```rust
    /// use cowboy::*;
    ///
    /// let cowboy = 42.cowboy();
    /// cowboy.set(84);
    /// assert_eq!(*cowboy.read(), 84);
    /// ```
    pub fn set(&self, value: T) {
        let mut guard = self.write();
        *guard = value;
    }

    /// Replace the inner value (returning the old value).
    ///
    /// ```rust
    /// use cowboy::*;
    ///
    /// let cowboy = 42.cowboy();
    /// let old_value = cowboy.replace(84);
    /// assert_eq!(old_value, 42);
    /// assert_eq!(*cowboy.read(), 84);
    /// ```
    pub fn replace(&self, value: T) -> T {
        let mut guard = self.write();
        std::mem::replace(&mut *guard, value)
    }

    /// Unsoundly get a mutable reference to the value.
    /// 🚨 DANGER ZONE 🚨 This function can be trivially used to get multiple mutable references to the same value, which is instantly undefined behavior.
    /// If you can use .write(), please use that instead. On the other hand... a little undefined behavior is usually okay in practice.
    ///
    /// This function is only available if the `evil` feature is enabled.
    #[cfg(feature = "evil")]
    #[deprecated(
        since = "0.1.0",
        note = "🚨 DANGER ZONE 🚨 This function gives you a reference to the inner value, but won't prevent anyone else from mutating it while you have it. If anyone else uses .write() or .howdy(), that will be undefined behavior. If you can use .read(), please use that instead. On the other hand... a little undefined behavior is usually okay in practice."
    )]
    #[must_use = "If you're calling this function, at least use the returned reference"]
    #[allow(clippy::transmute_ptr_to_ref)] // To avoid clippy warnings about the transmute
    pub unsafe fn howdy(&self) -> &T {
        let guard: std::sync::RwLockReadGuard<T> = self.inner.read().expect("RwLock poisoned");
        let inner_ref: &T = &guard;

        // Unsafely extend the lifetime of the mutable reference.
        // This is where the unsoundness lies. We are telling the compiler
        // that `inner_mut_ref` (which is tied to `guard`) can actually live
        // as long as `&self` (the lifetime of the `Cowboy` reference passed to `yeehaw`).
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
    ///
    /// This function is only available if the `evil` feature is enabled.
    #[cfg(feature = "evil")]
    #[deprecated(
        since = "0.1.0",
        note = "🚨 DANGER ZONE 🚨 This function can be trivially used to get multiple mutable references to the same value, which is undefined behavior. If you can use .write(), please use that instead. On the other hand... a little undefined behavior is usually okay in practice."
    )]
    #[must_use = "If you're calling this function, at least use the returned reference"]
    #[allow(clippy::transmute_ptr_to_ref)] // To avoid clippy warnings about the transmute
    #[allow(clippy::mut_from_ref)] // To avoid clippy warnings about the transmute
    pub unsafe fn yeehaw(&self) -> &mut T {
        let mut guard: std::sync::RwLockWriteGuard<T> =
            self.inner.write().expect("RwLock poisoned");
        let inner_mut_ref: &mut T = &mut guard;

        // Unsafely extend the lifetime of the mutable reference.
        // This is where the unsoundness lies. We are telling the compiler
        // that `inner_mut_ref` (which is tied to `guard`) can actually live
        // as long as `&self` (the lifetime of the `Cowboy` reference passed to `yeehaw`).
        // This is a lie because `guard` will be dropped at the end of this function,
        // releasing the lock.
        let extended_lifetime_mut_ref: &mut T =
            // SAFETY: None at all
            unsafe { std::mem::transmute::<&mut T, &mut T>(inner_mut_ref) };

        extended_lifetime_mut_ref
    }
}

impl<T: Clone> Cowboy<T> {
    /// Clone the contents of the `Cowboy`
    ///
    /// ```rust
    /// use cowboy::*;
    ///
    /// let cowboy = Cowboy::new(42);
    /// let cloned = cowboy.get_cloned();
    /// assert_eq!(cloned, 42);
    /// ```
    pub fn get_cloned(&self) -> T {
        self.read().clone()
    }
}

pub trait IntoCowboy: Sized {
    fn cowboy(self) -> Cowboy<Self>;
}

impl<T> IntoCowboy for T {
    fn cowboy(self) -> Cowboy<Self> {
        Cowboy::new(self)
    }
}
