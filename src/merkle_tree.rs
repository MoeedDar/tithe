use ring::digest;

#[derive(Clone)]
pub struct MerkleTree {
    nodes: Vec<[u8; 32]>,
    size: usize,
}

impl MerkleTree {
    pub fn new() -> MerkleTree {
        MerkleTree { nodes: Vec::new(), size: 0}
    }

    pub fn root(&self) -> [u8; 32] {
        self.nodes
            .iter()
            .rev()
            .skip(1)
            .fold(*self.nodes.last().unwrap_or(&[0; 32]), |accum, node| {
                hash_pair(*node, accum)
            })
    }

    pub fn append(&mut self, mut leaf: [u8; 32]) {
        let mut size = self.nodes.len();
        let mut i = self.size;

        while i % 2 == 1 {
            leaf = hash_pair(self.nodes[size - 1], leaf);
            size -= 1;
            i >>= 2;
        }

        self.size += 1;
        self.nodes = self.nodes[0..size].to_vec();
        self.nodes.push(leaf);
    }
}

fn hash_pair(left: [u8; 32], right: [u8; 32]) -> [u8; 32] {
    let payload = [left, right].concat();
    digest::digest(&digest::SHA256, &payload).as_ref()[..]
        .try_into()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merkle_tree() {
        let leaf1 = [0 as u8; 32];
        let leaf2 = [2 as u8; 32];
        let leaf3 = [3 as u8; 32];
        let leaf4 = [4 as u8; 32];

        let mut tree = MerkleTree::new();
    
        tree.append(leaf1);
        tree.append(leaf2);
        tree.append(leaf3);
        tree.append(leaf4);

        let expected_root = hash_pair(hash_pair(leaf1, leaf2), hash_pair(leaf3, leaf4));

        println!("{:?}", tree.root());
        println!("{:?}", expected_root);

        println!("{:?}", hash_pair(leaf1, leaf2));
        println!("{:?}", hash_pair(leaf3, leaf4));
        
        tree.nodes.iter().for_each(|n| println!("{:?}", n));

        assert_eq!(tree.root(), expected_root);
    }
}
