//! Collection types.

use arceos_api::modules::axhal;
use core::hash::{BuildHasher, Hasher};

pub struct HashMap<K, V> {
    inner: hashbrown::HashMap<K, V, RandomState>,
}

impl<K, V> HashMap<K, V>
where
    K: core::hash::Hash + Eq,
{
    pub fn new() -> Self {
        Self {
            inner: hashbrown::HashMap::with_hasher(RandomState::new()),
        }
    }

    pub fn insert(&mut self, k: K, v: V) -> Option<V> {
        self.inner.insert(k, v)
    }

    pub fn iter(&self) -> hashbrown::hash_map::Iter<'_, K, V> {
        self.inner.iter()
    }
}

pub struct RandomState {
    seed: u64,
}

impl RandomState {
    pub fn new() -> Self {
        Self {
            seed: axhal::misc::random() as u64,
        }
    }
}

impl Default for RandomState {
    fn default() -> Self {
        Self::new()
    }
}

impl BuildHasher for RandomState {
    type Hasher = MyHasher;
    fn build_hasher(&self) -> Self::Hasher {
        MyHasher::new(self.seed)
    }
}

pub struct MyHasher {
    state: u64,
}

impl MyHasher {
    const OFFSET: u64 = 0xcbf29ce484222325;
    const PRIME: u64 = 0x00000100000001b3;
    pub fn new(s: u64) -> Self {
        Self {
            state: Self::OFFSET ^ s,
        }
    }
}

impl Hasher for MyHasher {
    fn finish(&self) -> u64 {
        self.state
    }
    fn write(&mut self, bytes: &[u8]) {
        for &byte in bytes {
            self.state ^= byte as u64;
            self.state = self.state.wrapping_mul(Self::PRIME);
        }
    }
}
