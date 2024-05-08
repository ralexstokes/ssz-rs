use crate::lib::*;
use bitvec::vec::BitVec;

pub type ChunkIndex = usize;

/// Implements a cache for the Merkle tree backing a SSZ type.
#[derive(Clone)]
pub struct Cache(Arc<Inner>);

impl Deref for Cache {
    type Target = Inner;

    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}

impl Cache {
    pub fn new(element_count: usize) -> Self {
        Self(Arc::new(Inner::new(element_count)))
    }
}

pub struct Inner {
    state: Mutex<State>,
}

struct State {
    valid: BitVec<usize>,
    data: Vec<u8>,
}

impl Inner {
    pub fn new(element_count: usize) -> Self {
        let state = State { valid: BitVec::repeat(false, element_count), data: vec![] };
        Self { state: Mutex::new(state) }
    }

    pub fn invalidate_cache_element(&self, index: usize) {
        let mut state = self.state.lock().unwrap();
        state.valid.set(index, false);
    }
}
