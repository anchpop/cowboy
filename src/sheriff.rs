use std::collections::HashMap;
use std::sync::{LazyLock, RwLock};

use crate::Cowboy;

/// A global registry for Cowboy instances
pub struct Sheriff {
    registry: RwLock<HashMap<String, Box<dyn std::any::Any + Send + Sync>>>,
}

impl Sheriff {
    /// Create a new Sheriff instance
    fn new() -> Self {
        Self {
            registry: RwLock::new(HashMap::new()),
        }
    }

    /// Register a Cowboy instance with a name
    pub fn register<T: 'static + Send + Sync>(&self, name: &str, cowboy: Cowboy<T>) {
        let mut registry = self.registry.write().unwrap();
        registry.insert(name.to_string(), Box::new(cowboy));
    }

    /// Get a Cowboy instance by name
    pub fn get<T: 'static + Send + Sync>(&self, name: &str) -> Cowboy<T> {
        let registry = self.registry.read().unwrap();
        registry
            .get(name)
            .and_then(|boxed| boxed.downcast_ref::<Cowboy<T>>())
            .cloned()
            .expect("No Cowboy found with that name")
    }

    /// Check if a name is registered
    pub fn contains(&self, name: &str) -> bool {
        let registry = self.registry.read().unwrap();
        registry.contains_key(name)
    }

    /// Remove a registered Cowboy instance
    pub fn remove(&self, name: &str) -> bool {
        let mut registry = self.registry.write().unwrap();
        registry.remove(name).is_some()
    }
}

/// Global Sheriff instance
pub static SHERIFF: LazyLock<Sheriff> = LazyLock::new(Sheriff::new);
