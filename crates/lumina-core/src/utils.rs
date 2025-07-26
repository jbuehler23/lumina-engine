use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Id(u64);

impl Id {
    pub fn new() -> Self {
        Self(uuid::Uuid::new_v4().as_u128() as u64)
    }

    pub fn from_u64(value: u64) -> Self {
        Self(value)
    }

    pub fn as_u64(&self) -> u64 {
        self.0
    }
}

impl Default for Id {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub fn hash_string(s: &str) -> u64 {
    use std::collections::hash_map::DefaultHasher;
    let mut hasher = DefaultHasher::new();
    s.hash(&mut hasher);
    hasher.finish()
}

pub fn next_power_of_two(mut n: u32) -> u32 {
    if n == 0 {
        return 1;
    }
    
    n -= 1;
    n |= n >> 1;
    n |= n >> 2;
    n |= n >> 4;
    n |= n >> 8;
    n |= n >> 16;
    n + 1
}

pub fn align_up(value: usize, alignment: usize) -> usize {
    (value + alignment - 1) & !(alignment - 1)
}

pub fn align_down(value: usize, alignment: usize) -> usize {
    value & !(alignment - 1)
}

#[derive(Debug, Clone)]
pub struct BitSet {
    data: Vec<u64>,
    len: usize,
}

impl BitSet {
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            len: 0,
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        let words = (capacity + 63) / 64;
        Self {
            data: vec![0; words],
            len: capacity,
        }
    }

    pub fn set(&mut self, index: usize) {
        if index >= self.len {
            self.resize(index + 1);
        }
        
        let word_index = index / 64;
        let bit_index = index % 64;
        self.data[word_index] |= 1u64 << bit_index;
    }

    pub fn clear(&mut self, index: usize) {
        if index < self.len {
            let word_index = index / 64;
            let bit_index = index % 64;
            self.data[word_index] &= !(1u64 << bit_index);
        }
    }

    pub fn get(&self, index: usize) -> bool {
        if index >= self.len {
            return false;
        }
        
        let word_index = index / 64;
        let bit_index = index % 64;
        (self.data[word_index] & (1u64 << bit_index)) != 0
    }

    pub fn toggle(&mut self, index: usize) {
        if index >= self.len {
            self.resize(index + 1);
        }
        
        let word_index = index / 64;
        let bit_index = index % 64;
        self.data[word_index] ^= 1u64 << bit_index;
    }

    pub fn resize(&mut self, new_len: usize) {
        let new_words = (new_len + 63) / 64;
        self.data.resize(new_words, 0);
        self.len = new_len;
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn clear_all(&mut self) {
        for word in &mut self.data {
            *word = 0;
        }
    }

    pub fn iter_set_bits(&self) -> impl Iterator<Item = usize> + '_ {
        let len = self.len;
        self.data.iter().enumerate().flat_map(move |(word_idx, &word)| {
            (0..64).filter_map(move |bit_idx| {
                let index = word_idx * 64 + bit_idx;
                if index < len && (word & (1u64 << bit_idx)) != 0 {
                    Some(index)
                } else {
                    None
                }
            })
        })
    }
}

impl Default for BitSet {
    fn default() -> Self {
        Self::new()
    }
}

#[macro_export]
macro_rules! define_handle {
    ($name:ident) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct $name(pub(crate) u32);

        impl $name {
            pub fn new(index: u32) -> Self {
                Self(index)
            }

            pub fn index(&self) -> u32 {
                self.0
            }
        }

        impl From<u32> for $name {
            fn from(index: u32) -> Self {
                Self(index)
            }
        }

        impl From<$name> for u32 {
            fn from(handle: $name) -> Self {
                handle.0
            }
        }
    };
}

pub use define_handle;