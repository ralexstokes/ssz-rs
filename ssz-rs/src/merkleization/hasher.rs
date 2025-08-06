use super::BYTES_PER_CHUNK;

use ::sha2::{Digest, Sha256};

#[inline]
fn hash_chunks_sha256(left: impl AsRef<[u8]>, right: impl AsRef<[u8]>) -> [u8; BYTES_PER_CHUNK] {
    let mut hasher = Sha256::new();
    hasher.update(left.as_ref());
    hasher.update(right.as_ref());
    hasher.finalize_reset().into()
}

/// Hash N consecutive (left,right) pairs.
/// `in_pairs.len()  == 64 * N`
/// `out_hashes.len() == 32 * N`
#[inline(always)]
pub fn hash_pairs_bulk(in_pairs: &[u8], out_hashes: &mut [u8]) {
    debug_assert!(in_pairs.len() % 64 == 0);
    debug_assert!(out_hashes.len() * 2 == in_pairs.len());

    let num_pairs = in_pairs.len() / 64;
    if num_pairs == 0 {
        return;
    }

    // Call the SIMD-optimized bulk hash function directly.
    #[cfg(feature = "hashtree")]
    {
        hashtree::init();
        hashtree::hash(out_hashes, in_pairs, num_pairs);
    }

    #[cfg(not(feature = "hashtree"))]
    {
        let mut h = Sha256::new();
        for (blk, out) in in_pairs.chunks_exact(64).zip(out_hashes.chunks_exact_mut(32)) {
            h.update(blk);
            out.copy_from_slice(&h.finalize_reset());
        }
    }
}

/// Function that hashes 2 [BYTES_PER_CHUNK] (32) len byte slices together. Depending on the feature
/// flags, this will either use:
/// - sha256 (default)
/// - sha256 with assembly support (with the "sha2-asm" feature flag)
#[inline]
pub fn hash_chunks(left: impl AsRef<[u8]>, right: impl AsRef<[u8]>) -> [u8; BYTES_PER_CHUNK] {
    debug_assert!(left.as_ref().len() == BYTES_PER_CHUNK);
    debug_assert!(right.as_ref().len() == BYTES_PER_CHUNK);

    return hash_chunks_sha256(left, right);
}
