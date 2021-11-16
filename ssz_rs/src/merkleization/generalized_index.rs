// From: https://users.rust-lang.org/t/logarithm-of-integers/8506/5
const fn num_bits<T>() -> usize {
    std::mem::size_of::<T>() * 8
}

fn log_2(x: usize) -> u32 {
    assert!(x > 0);
    num_bits::<usize>() as u32 - x.leading_zeros() - 1
}

type N = usize;

fn get_power_of_two_ceil(x: N) -> N {
    if x <= 1 {
        1
    } else if x == 2 {
        2
    } else {
        2 * get_power_of_two_ceil((x + 1) / 2)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct GeneralizedIndex(pub N);

impl Default for GeneralizedIndex {
    fn default() -> Self {
        Self(1)
    }
}

impl GeneralizedIndex {
    pub fn get_path_length(&self) -> usize {
        log_2(self.0) as usize
    }

    pub fn get_bit(&self, position: usize) -> bool {
        self.0 & (1 << position) > 0
    }

    pub fn sibling(&self) -> Self {
        Self(self.0 ^ 1)
    }

    pub fn child_left(&self) -> Self {
        Self(self.0 * 2)
    }

    pub fn child_right(&self) -> Self {
        Self(self.0 * 2 + 1)
    }

    pub fn parent(&self) -> Self {
        Self(self.0 / 2)
    }
}
