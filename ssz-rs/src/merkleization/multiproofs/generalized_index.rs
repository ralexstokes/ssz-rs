// From: https://users.rust-lang.org/t/logarithm-of-integers/8506/5
const fn num_bits<T>() -> usize {
    std::mem::size_of::<T>() * 8
}

fn log_2(x: usize) -> u32 {
    assert!(x > 0);
    num_bits::<usize>() as u32 - x.leading_zeros() - 1
}

pub(crate) fn get_power_of_two_ceil(x: usize) -> usize {
    match x {
        x if x <= 1 => 1,
        2 => 2,
        x => 2 * get_power_of_two_ceil((x + 1) / 2),
    }
}

pub type GeneralizedIndex = usize;

pub fn get_path_length(index: GeneralizedIndex) -> usize {
    log_2(index) as usize
}

pub fn get_bit(index: GeneralizedIndex, position: usize) -> bool {
    index & (1 << position) > 0
}

pub fn sibling(index: GeneralizedIndex) -> GeneralizedIndex {
    index ^ 1
}

pub fn child_left(index: GeneralizedIndex) -> GeneralizedIndex {
    index * 2
}

pub fn child_right(index: GeneralizedIndex) -> GeneralizedIndex {
    index * 2 + 1
}

pub fn parent(index: GeneralizedIndex) -> GeneralizedIndex {
    index / 2
}
