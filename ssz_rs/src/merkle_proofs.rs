use crate::merkleization::{hash_nodes, Root};
use sha2::{Digest, Sha256};

pub fn is_valid_merkle_branch<'a, Node: Into<Root> + 'a>(
    leaf: &Node,
    branch: impl Iterator<Item = &'a Node>,
    depth: usize,
    index: usize,
    root: &Root,
) -> bool {
    let mut value: Root = leaf.into();

    let mut hasher = Sha256::new();
    for i in 0..depth {
        let next_node = match branch.next() {
            Some(node) => node,
            None => return false,
        };
        if (index / 2usize.pow(i as u32)) % 2 != 0 {
            hash_nodes(
                &mut hasher,
                next_node.into().as_ref(),
                &value.as_ref(),
                &mut value.as_ref(),
            );
        } else {
            hash_nodes(
                &mut hasher,
                &value.as_ref(),
                next_node.into().as_ref(),
                &mut value.as_ref(),
            );
        }
    }
    value == *root
}
