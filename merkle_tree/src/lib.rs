#[macro_use] 
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

pub mod interface;

use std::fmt::Display;
use sha2::{Digest, Sha256};

#[derive(Debug)]
pub struct MerkleTree {
    /// Binary tree represented as a 2-dimensional vector in which the outer vector represents each row and inner vector represents the nodes on that row   
    /// Note that leaf nodes are stored in row 0 and the root node in row (tree.len()-1)
    pub tree: Vec<Vec<String>>,
    pub leaves: usize
}

impl Display for MerkleTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        write!(
            f,
            "MerkleTree depth {}: \n1: {:?}\n2: {:?}\n3: {:?}\n",
            self.tree.len(), self.tree[0], self.tree[1], self.tree[2]
            // "{:?}\n{:?}\n{:?}\n",
            // self.tree[0], self.tree[1], self.tree[2]
        )
    }
}

impl MerkleTree {
    /// Take a list of leaf hashes and build full merkle tree
    pub fn build(leaves: &Vec<String>) -> MerkleTree {
        let depth: usize = MerkleTree::find_depth(leaves.len());
        
        // Row 0
        let mut tree: Vec<Vec<String>> = Vec::with_capacity(depth);
        tree.push(leaves.clone());

        // Build each row of Merkle tree
        for row in 0..depth-1 {
            let mut next_row: Vec<String> = vec!();
            // Hash concaternation of pairs of items on current row to build next row
            for i in (0..tree[row].len()).step_by(2) {
                let concat = concat_string(&tree[row][i],&tree[row][i+1]);
                next_row.push(hash(concat.as_ref()));
            }
            // println!("next row: {:?}", next_row);
            tree.push(next_row);
        }

        MerkleTree {
            tree,
            leaves: leaves.len()
        }
    } 

    /// Return index for item in each row of the tree from a particular leaf to root
    pub fn find_path_leaf_to_root(&self, leaf_index: usize) -> Vec<usize> {
        let mut path = Vec::with_capacity(self.tree.len());
        path.push(leaf_index);
        for row in 0..self.tree.len()-2 {
            path.push(MerkleTree::find_parent_of_node(path[row]))
        }
        path
    }

    /// The parent index of a child node is the quotient of the child index when divided by 2
    pub fn find_parent_of_node(index: usize) -> usize {
        index.div_euclid(2)
    }

    /// The sibling node index of a given node is:
    ///  If node index is even: the previous in the row
    ///  If node index is odd: the next in the row
    pub fn find_node_sibling(index: usize) -> usize {
        if index % 2 == 0 {
            return index + 1;
        } 
        return index - 1;
    }

    /// Create a vector of hashes which are the nodes required to rebuild the root hash from the queried index
    pub fn prove(&self, index: usize) -> Vec<String> {
        if index > (self.leaves) {
            panic!("Index too large. Tree contains {} nodes.", self.tree.len())
        }
        // First find the indicies of each node in path from leaf to root
        let path_to_root = self.find_path_leaf_to_root(index);

        // Next find the sibling node to each node in the path to root
        let sibling_path: Vec<usize> = path_to_root.into_iter().map(|x| MerkleTree::find_node_sibling(x)).collect();

        // The proof vector then is a hash from each row at index in sibling_path vector
        let mut proof = Vec::new();
        for row in 0..self.tree.len()-1 {
            proof.push(self.tree[row][sibling_path[row]].clone())
        }
        proof
    }

    fn find_depth(num_items: usize) -> usize {
        (num_items.next_power_of_two().ilog2()+1).try_into().unwrap()
    }

}

// Sha256 hash a message. Return as hex string
pub fn hash(message: &[u8]) -> String {
    let hash = Sha256::digest(message);
    base16ct::lower::encode_string(&hash)
}

/// Take two Strings and copy them into a new String which is a concaternation of them  
/// Strings are ordered lexigraphically
fn concat_string(string1: &String, string2: &String) -> String {
    if string1 > string2 {
        return string1.clone() + &string2;
    }
    string2.clone() + &string1
}

/// Take a root hash, item hash and proof and return true if proof validates the item hash in merkle tree with given root
pub fn verify(root_hash: &String, item_hash: &String, proof: &Vec<String>) -> bool {
    // TODO: client side: First hash the file and verify first hash
    // if file_hash != proof[0] {
    //     false
    // }

    let mut current_hash = item_hash.clone();
    for i in 0..proof.len() {
        current_hash = hash(concat_string(&current_hash, &proof[i].clone()).as_ref());
    } 
    current_hash == *root_hash
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn playground() {
        let inputs: Vec<String> = vec!(String::from("0"),String::from("1"),String::from("2"),String::from("3"));
        let hashes: Vec<String> = inputs.into_iter().map(|x: String| hash(x.as_ref())).collect();
        let merkle_tree = MerkleTree::build(&hashes);
        println!("{}", merkle_tree);

        let proof = merkle_tree.prove(0usize);
        println!("proof: {:?}", proof);

        let res = verify(&merkle_tree.tree[merkle_tree.tree.len()-1][0], &hashes[0], &proof);
        assert_eq!(res, true);
    }

    #[test]
    fn test_build_and_verify() {
        let inputs: Vec<String> = vec!(String::from("0"),String::from("1"),String::from("2"),String::from("3"));
        let hashes: Vec<String> = inputs.into_iter().map(|x| hash(x.as_ref())).collect();
        let merkle_tree = MerkleTree::build(&hashes);
        // Data generated manually
        assert_eq!(merkle_tree.tree[0][0], String::from("5feceb66ffc86f38d952786c6d696c79c2dbc239dd4e91b46729d73a27fb57e9"));
        assert_eq!(merkle_tree.tree[0][1], String::from("6b86b273ff34fce19d6b804eff5a3f5747ada4eaa22f1d49c01e52ddb7875b4b")); 
        assert_eq!(merkle_tree.tree[0][2], String::from("d4735e3a265e16eee03f59718b9b5d03019c07d8b6c51f90da3a666eec13ab35")); 
        assert_eq!(merkle_tree.tree[0][3], String::from("4e07408562bedb8b60ce05c1decfe3ad16b72230967de01f640b7e4729b49fce")); 
        assert_eq!(merkle_tree.tree[1][0], String::from("dbdbf4fb684471f421fb255100e433c77fd1aac71c7a3739e9897168aec67ec1")); 
        assert_eq!(merkle_tree.tree[1][1], String::from("70311d9d203b2d7e4ff70d7fce219f82a4fcf73a110dc80187dfefb7c6e4bb87"));
        assert_eq!(merkle_tree.tree[2][0], String::from("a48572b1744e5e3f3473c9eaa91f73be774712d2b207c34eb537023af0ec6528"));

        let root_hash = merkle_tree.tree[merkle_tree.tree.len()-1][0].clone();

        // Build all proof and verify
        let mut proof = merkle_tree.prove(0usize);
        assert!(verify(&root_hash, &hashes[0], &proof));
        proof = merkle_tree.prove(1usize);
        assert!(verify(&root_hash, &hashes[1], &proof));
        proof = merkle_tree.prove(2usize);
        assert!(verify(&root_hash, &hashes[2], &proof));
        proof = merkle_tree.prove(3usize);
        assert!(verify(&root_hash, &hashes[3], &proof));
    }

    #[test]
    fn test_build_leaves_not_power_of_2() {

    }

    #[test]
    fn test_prove() {
        let inputs: Vec<String> = vec!(String::from("0"),String::from("1"),String::from("2"),String::from("3"));
        let hashes: Vec<String> = inputs.into_iter().map(|x| hash(x.as_ref())).collect();
        let merkle_tree = MerkleTree::build(&hashes);
        assert_eq!(merkle_tree.prove(0), Vec::from([merkle_tree.tree[0][1].clone(), merkle_tree.tree[1][1].clone()]));
        assert_eq!(merkle_tree.prove(1), Vec::from([merkle_tree.tree[0][0].clone(), merkle_tree.tree[1][1].clone()]));
        assert_eq!(merkle_tree.prove(2), Vec::from([merkle_tree.tree[0][3].clone(), merkle_tree.tree[1][0].clone()]));
        assert_eq!(merkle_tree.prove(3), Vec::from([merkle_tree.tree[0][2].clone(), merkle_tree.tree[1][0].clone()]));
        assert_eq!(merkle_tree.prove(4), Vec::from([merkle_tree.tree[0][2].clone(), merkle_tree.tree[1][0].clone()]));
    }

    #[test]
    fn test_find_path_leaf_to_root() {
        let merkle_tree = MerkleTree {
            tree: vec![Vec::new();4],
            leaves: 4
        };
        // test data generated manually
        assert_eq!(merkle_tree.find_path_leaf_to_root(0), Vec::from([0,0,0]));
        assert_eq!(merkle_tree.find_path_leaf_to_root(1), Vec::from([1,0,0]));
        assert_eq!(merkle_tree.find_path_leaf_to_root(2), Vec::from([2,1,0]));
        assert_eq!(merkle_tree.find_path_leaf_to_root(3), Vec::from([3,1,0]));
        assert_eq!(merkle_tree.find_path_leaf_to_root(4), Vec::from([4,2,1]));
        assert_eq!(merkle_tree.find_path_leaf_to_root(5), Vec::from([5,2,1]));
        assert_eq!(merkle_tree.find_path_leaf_to_root(6), Vec::from([6,3,1]));
        assert_eq!(merkle_tree.find_path_leaf_to_root(7), Vec::from([7,3,1]));
    }

    #[test]
    fn test_depth() {
        assert_eq!(MerkleTree::find_depth(1usize), 1usize); // 2**0
        assert_eq!(MerkleTree::find_depth(2usize), 2usize); // 2**1
        assert_eq!(MerkleTree::find_depth(3usize), 3usize);
        assert_eq!(MerkleTree::find_depth(4usize), 3usize); // 2**2
        assert_eq!(MerkleTree::find_depth(5usize), 4usize);
        assert_eq!(MerkleTree::find_depth(8usize), 4usize); // 2**3
        assert_eq!(MerkleTree::find_depth(9usize), 5usize);
        assert_eq!(MerkleTree::find_depth(16usize), 5usize); // 2**4
        assert_eq!(MerkleTree::find_depth(17usize), 6usize);
        assert_eq!(MerkleTree::find_depth(32usize), 6usize); // 2**5
        assert_eq!(MerkleTree::find_depth(33usize), 7usize);
    }

    #[test]
    fn test_find_node_sibling() {
        assert_eq!(MerkleTree::find_node_sibling(0usize), 1usize);
        assert_eq!(MerkleTree::find_node_sibling(1usize), 0usize);
        assert_eq!(MerkleTree::find_node_sibling(5usize), 4usize);
        assert_eq!(MerkleTree::find_node_sibling(6usize), 7usize);
    }
}
