use std::any::Any;
use std::hash::{Hash, Hasher};
use std::sync::LazyLock;

use dashmap::DashMap;

use crate::Cowboy;

/// A wrapper type for keys that provides type-erased equality and hashing
struct KeyBox {
    // The actual key value
    value: Box<dyn Any + Send + Sync>,
    // Type ID for runtime type checking
    type_id: std::any::TypeId,
    // Functions for equality and hashing
    eq_fn: fn(&Box<dyn Any + Send + Sync>, &Box<dyn Any + Send + Sync>) -> bool,
    hash_fn: fn(&Box<dyn Any + Send + Sync>, &mut dyn Hasher),
}

impl KeyBox {
    /// Create a new KeyBox from any type that implements Eq, Hash, and Clone
    fn new<K: Eq + Hash + Send + Sync + 'static>(key: K) -> Self {
        // Type-specific equality function
        fn eq_impl<T: Eq + 'static>(
            a: &Box<dyn Any + Send + Sync>,
            b: &Box<dyn Any + Send + Sync>,
        ) -> bool {
            if let (Some(a), Some(b)) = (a.downcast_ref::<T>(), b.downcast_ref::<T>()) {
                a == b
            } else {
                false
            }
        }

        // Type-specific hash function
        fn hash_impl<T: Hash + 'static>(
            value: &Box<dyn Any + Send + Sync>,
            mut state: &mut dyn Hasher,
        ) {
            if let Some(value) = value.downcast_ref::<T>() {
                value.hash(&mut state);
            }
        }

        KeyBox {
            value: Box::new(key),
            type_id: std::any::TypeId::of::<K>(),
            eq_fn: eq_impl::<K>,
            hash_fn: hash_impl::<K>,
        }
    }
}

impl PartialEq for KeyBox {
    fn eq(&self, other: &Self) -> bool {
        // Only compare if the types match
        if self.type_id == other.type_id {
            (self.eq_fn)(&self.value, &other.value)
        } else {
            false
        }
    }
}

impl Eq for KeyBox {}

impl Hash for KeyBox {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Hash the type ID first
        self.type_id.hash(state);
        // Then use the type-specific hash function
        (self.hash_fn)(&self.value, state);
    }
}

/// A global registry for Cowboy instances
pub struct Sheriff {
    registry: DashMap<KeyBox, Box<dyn Any + Send + Sync>>,
}

impl Sheriff {
    /// Create a new Sheriff instance
    fn new() -> Self {
        Self {
            registry: DashMap::new(),
        }
    }

    /// Register a Cowboy instance with a key
    ///
    /// ```rust
    /// use cowboy::*;
    ///
    /// let player = "Player".to_string().cowboy();
    /// SHERIFF.register("player1", player);
    /// ```
    pub fn register<K, T>(&self, key: K, cowboy: Cowboy<T>)
    where
        K: Eq + Hash + Clone + Send + Sync + 'static,
        T: 'static + Send + Sync,
    {
        self.registry.insert(KeyBox::new(key), Box::new(cowboy));
    }

    /// Get a Cowboy instance by key
    ///
    /// ```rust
    /// use cowboy::*;
    ///
    /// // First register a value
    /// let player = "Player".to_string().cowboy();
    /// SHERIFF.register("player1", player);
    ///
    /// // Then retrieve it
    /// let retrieved: Cowboy<String> = SHERIFF.get("player1");
    /// assert_eq!(*retrieved.read(), "Player");
    /// ```
    #[track_caller]
    pub fn get<K, T>(&self, key: K) -> Cowboy<T>
    where
        K: Eq + Hash + Send + Sync + 'static,
        T: 'static + Send + Sync,
    {
        let key_box = KeyBox::new(key);

        self.registry
            .get(&key_box)
            .and_then(|boxed| boxed.downcast_ref::<Cowboy<T>>().cloned())
            .expect("No Cowboy found with that key")
    }

    /// Check if a key is registered
    ///
    /// ```rust
    /// use cowboy::*;
    ///
    /// // Register a value
    /// let player = "Player".to_string().cowboy();
    /// SHERIFF.register("player1", player);
    ///
    /// // Check if keys exist
    /// assert!(SHERIFF.contains(&"player1"));
    /// assert!(!SHERIFF.contains(&"player2"));
    /// ```
    pub fn contains<K>(&self, key: &K) -> bool
    where
        K: Eq + Hash + Clone + Send + Sync + 'static,
    {
        self.registry.contains_key(&KeyBox::new(key.clone()))
    }

    /// Remove a registered Cowboy instance
    ///
    /// ```rust
    /// use cowboy::*;
    ///
    /// // Register a value
    /// let player = "Player".to_string().cowboy();
    /// SHERIFF.register("player1", player);
    ///
    /// // Remove it
    /// assert!(SHERIFF.remove(&"player1"));
    /// assert!(!SHERIFF.contains(&"player1"));
    ///
    /// // Trying to remove a non-existent key returns false
    /// assert!(!SHERIFF.remove(&"player2"));
    /// ```
    pub fn remove<K>(&self, key: &K) -> bool
    where
        K: Eq + Hash + Clone + Send + Sync + 'static,
    {
        self.registry.remove(&KeyBox::new(key.clone())).is_some()
    }
}

/// Global Sheriff instance
pub static SHERIFF: LazyLock<Sheriff> = LazyLock::new(|| {
    if !cfg!(debug_assertions) {
        eprintln!("Use of SHERIFF in production is not recommended!");
    }
    Sheriff::new()
});
