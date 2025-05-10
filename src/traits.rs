use crate::Cowboy;

impl<T: PartialEq> PartialEq for Cowboy<T> {
    fn eq(&self, other: &Self) -> bool {
        *self.inner.read().unwrap() == *other.inner.read().unwrap()
    }
}

impl<T: Eq> Eq for Cowboy<T> {}

impl<T: PartialOrd> PartialOrd for Cowboy<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.inner
            .read()
            .unwrap()
            .partial_cmp(&other.inner.read().unwrap())
    }
}

impl<T: Ord> Ord for Cowboy<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.inner.read().unwrap().cmp(&other.inner.read().unwrap())
    }
}

impl<T: std::hash::Hash> std::hash::Hash for Cowboy<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.inner.read().unwrap().hash(state);
    }
}

impl<T: std::fmt::Display> std::fmt::Display for Cowboy<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner.read().unwrap())
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for Cowboy<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.inner.read().unwrap())
    }
}

impl<T> From<T> for Cowboy<T> {
    fn from(value: T) -> Self {
        Cowboy::new(value)
    }
}

impl<T: std::ops::Add<Output = T> + Clone> std::ops::Add for Cowboy<T> {
    type Output = Cowboy<T>;

    fn add(self, rhs: Self) -> Self::Output {
        let lhs_val = self.read().clone();
        let rhs_val = rhs.read().clone();
        Cowboy::new(lhs_val + rhs_val)
    }
}

impl<T: std::ops::Sub<Output = T> + Clone> std::ops::Sub for Cowboy<T> {
    type Output = Cowboy<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        let lhs_val = self.read().clone();
        let rhs_val = rhs.read().clone();
        Cowboy::new(lhs_val - rhs_val)
    }
}

impl<T: std::ops::Mul<Output = T> + Clone> std::ops::Mul for Cowboy<T> {
    type Output = Cowboy<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        let lhs_val = self.read().clone();
        let rhs_val = rhs.read().clone();
        Cowboy::new(lhs_val * rhs_val)
    }
}

impl<T: std::ops::Div<Output = T> + Clone> std::ops::Div for Cowboy<T> {
    type Output = Cowboy<T>;

    fn div(self, rhs: Self) -> Self::Output {
        let lhs_val = self.read().clone();
        let rhs_val = rhs.read().clone();
        Cowboy::new(lhs_val / rhs_val)
    }
}

impl<T: Default> Default for Cowboy<T> {
    fn default() -> Self {
        Cowboy::new(T::default())
    }
}

impl<T> Clone for Cowboy<T> {
    fn clone(&self) -> Self {
        Cowboy {
            inner: self.inner.clone(),
        }
    }
}

impl<T> Cowboy<Vec<T>> {
    /// Safely push to a vector
    pub fn push(&self, item: T) {
        self.write().push(item);
    }

    /// Get the length of the vector
    pub fn len(&self) -> usize {
        self.read().len()
    }

    /// Check if the vector is empty
    pub fn is_empty(&self) -> bool {
        self.read().is_empty()
    }
}

#[cfg(feature = "serde")]
impl<T: serde::Serialize> serde::Serialize for Cowboy<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.inner.read().unwrap().serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de, T: serde::Deserialize<'de>> serde::Deserialize<'de> for Cowboy<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = T::deserialize(deserializer)?;
        Ok(Cowboy::new(value))
    }
}

#[cfg(feature = "serde")]
impl<T: serde::Serialize> Cowboy<T> {
    #[track_caller]
    pub fn save(&self, path: &str) {
        use std::fs::File;
        use std::io::BufWriter;
        let file = File::create(path).unwrap_or_else(|e| {
            panic!("Failed to create file: {e}");
        });
        let writer = BufWriter::new(file);
        let s = self.read();
        serde_json::to_writer(writer, &*s).unwrap_or_else(|e| {
            panic!("Failed to serialize: {e}");
        });
    }
}

#[cfg(feature = "serde")]
impl<T: serde::de::DeserializeOwned> Cowboy<T> {
    #[track_caller]
    pub fn load(path: &str) -> Self {
        use std::fs::File;
        use std::io::BufReader;
        let file = File::open(path).unwrap_or_else(|e| {
            panic!("Failed to open file: {e}");
        });
        let reader = BufReader::new(file);
        let s = serde_json::from_reader(reader).unwrap_or_else(|e| {
            panic!("Failed to deserialize: {e}");
        });
        Cowboy::new(s)
    }
}
