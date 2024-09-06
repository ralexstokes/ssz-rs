#[cfg(feature = "hashtree")]
use std::sync::Once;

use super::BYTES_PER_CHUNK;

#[cfg(not(feature = "hashtree"))]
use ::sha2::{Digest, Sha256};

#[cfg(feature = "hashtree")]
static INIT: Once = Once::new();

/// Function that hashes 2 [BYTES_PER_CHUNK] (32) len byte slices together. Depending on the feature flags,
/// this will either use:
/// - sha256 (default)
/// - sha256 with assembly support (with the "sha2-asm" feature flag)
/// - hashtree (with the "hashtree" feature flag)
pub fn hash_chunks(left: impl AsRef<[u8]>, right: impl AsRef<[u8]>) -> [u8; BYTES_PER_CHUNK] {
    #[cfg(feature = "hashtree")]
    {
        // Initialize the hashtree library (once)
        INIT.call_once(|| {
            hashtree::init();
        });

        debug_assert!(left.as_ref().len() == BYTES_PER_CHUNK);
        debug_assert!(right.as_ref().len() == BYTES_PER_CHUNK);

        let mut out = [0u8; BYTES_PER_CHUNK];

        let mut chunks = Vec::with_capacity(2 * BYTES_PER_CHUNK);
        chunks.extend_from_slice(left.as_ref());
        chunks.extend_from_slice(right.as_ref());

        // NOTE: hashtree "chunks" are 64 bytes long, not 32. That's why we
        // specify "1" as the chunk count.
        hashtree::hash(&mut out, chunks.as_slice(), 1);

        out
    }

    #[cfg(not(feature = "hashtree"))]
    {
        let mut hasher = Sha256::new();
        hasher.update(left.as_ref());
        hasher.update(right.as_ref());
        hasher.finalize_reset().into()
    }
}
