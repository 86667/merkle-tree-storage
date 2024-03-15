use std::fmt::Display;
use sha2::{digest::block_buffer::Error, Digest, Sha256};

#[derive(Debug)]
pub struct MerkleTree {
    /// Binary tree represented as a 2-dimensional vector in which the outer vector represents each row and inner vector represents the nodes on that row   
    /// Note that leaf nodes are stored in row 0 and the root node in row (tree.len()-1)
    pub tree: Vec<Vec<String>>,
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
    pub fn build(leaves: Vec<String>) -> MerkleTree {
        let depth: usize = MerkleTree::find_depth(leaves.len());
        
        // Row 0
        let mut tree: Vec<Vec<String>> = Vec::with_capacity(depth);
        tree.push(leaves);

        // Build each row of Merkle tree
        for row in 0..depth-1 {
            let mut next_row: Vec<String> = vec!();
            // Hash concaternation of pairs of items on current row to build next row
            for i in (0..tree[row].len()).step_by(2) {
                let concat = MerkleTree::concat_string(&tree[row][i],&tree[row][i+1]);
                next_row.push(hash(concat.as_ref()));
            }
            // println!("next row: {:?}", next_row);
            tree.push(next_row);
        }

        MerkleTree {
            tree
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
        if index > (self.tree.len() - 1) {
            panic!("Index too large. Tree contains {} nodes.", self.tree.len())
        }
        // First find the indicies of each node in path from leaf to root
        let path_to_root = self.find_path_leaf_to_root(index);
        println!("{:?}", path_to_root);

        // Next find the sibling node to each node in the path to root
        let sibling_path: Vec<usize> = path_to_root.into_iter().map(|x| MerkleTree::find_node_sibling(x)).collect();
        println!("{:?}", sibling_path);

        // The proof vector then is a hash from each row at index in sibling_path vector
        let mut proof = Vec::new();
        for row in 0..self.tree.len()-1 {
            proof.push(self.tree[row][sibling_path[row]].clone())
        }
        proof
    }

    fn concat_string(string1: &String, string2: &String) -> String {
        string1.clone() + &string2
    }

    fn find_depth(num_items: usize) -> usize {
        (num_items.next_power_of_two().ilog2()+1).try_into().unwrap()
    }

}


pub fn hash(message: &[u8]) -> String {
    let hash = Sha256::digest(message);
    base16ct::lower::encode_string(&hash)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn playground() {
        let inputs: Vec<String> = vec!(String::from("0"),String::from("1"),String::from("2"),String::from("3"));
        let hashes: Vec<String> = inputs.into_iter().map(|x: String| hash(x.as_ref())).collect();
        let merkle_tree = MerkleTree::build(hashes);
        println!("{}", merkle_tree);
    }

    #[test]
    fn test_build() {
        let inputs: Vec<String> = vec!(String::from("0"),String::from("1"),String::from("2"),String::from("3"));
        let hashes: Vec<String> = inputs.into_iter().map(|x| hash(x.as_ref())).collect();
        let merkle_tree = MerkleTree::build(hashes);
        // Generated manually
        assert_eq!(merkle_tree.tree[0][0], String::from("5feceb66ffc86f38d952786c6d696c79c2dbc239dd4e91b46729d73a27fb57e9"));
        assert_eq!(merkle_tree.tree[0][1], String::from("6b86b273ff34fce19d6b804eff5a3f5747ada4eaa22f1d49c01e52ddb7875b4b")); 
        assert_eq!(merkle_tree.tree[0][2], String::from("d4735e3a265e16eee03f59718b9b5d03019c07d8b6c51f90da3a666eec13ab35")); 
        assert_eq!(merkle_tree.tree[0][3], String::from("4e07408562bedb8b60ce05c1decfe3ad16b72230967de01f640b7e4729b49fce")); 
        assert_eq!(merkle_tree.tree[1][0], String::from("fa13bb36c022a6943f37c638126a2c88fc8d008eb5a9fe8fcde17026807feae4")); 
        assert_eq!(merkle_tree.tree[1][1], String::from("70311d9d203b2d7e4ff70d7fce219f82a4fcf73a110dc80187dfefb7c6e4bb87"));
        assert_eq!(merkle_tree.tree[2][0], String::from("862532e6a3c9aafc2016810598ed0cc3025af5640db73224f586b6f1138385f4"));
    }

    #[test]
    fn test_prove() {
        let inputs: Vec<String> = vec!(String::from("0"),String::from("1"),String::from("2"),String::from("3"));
        let hashes: Vec<String> = inputs.into_iter().map(|x| hash(x.as_ref())).collect();
        let merkle_tree = MerkleTree::build(hashes);
        assert_eq!(merkle_tree.prove(0), Vec::from([merkle_tree.tree[0][1].clone(), merkle_tree.tree[1][1].clone()]));
        assert_eq!(merkle_tree.prove(1), Vec::from([merkle_tree.tree[0][0].clone(), merkle_tree.tree[1][1].clone()]));
        assert_eq!(merkle_tree.prove(2), Vec::from([merkle_tree.tree[0][3].clone(), merkle_tree.tree[1][0].clone()]));
        assert_eq!(merkle_tree.prove(3), Vec::from([merkle_tree.tree[0][2].clone(), merkle_tree.tree[1][0].clone()]));
        assert_eq!(merkle_tree.prove(4), Vec::from([merkle_tree.tree[0][2].clone(), merkle_tree.tree[1][0].clone()]));
    }

    #[test]
    fn test_find_path_leaf_to_root() {
        let merkle_tree = MerkleTree {
            tree: vec![Vec::new();4]
        };
        // test data generated manually
        assert_eq!(merkle_tree.find_path_leaf_to_root(0), Vec::from([0,0,0,0]));
        assert_eq!(merkle_tree.find_path_leaf_to_root(1), Vec::from([1,0,0,0]));
        assert_eq!(merkle_tree.find_path_leaf_to_root(2), Vec::from([2,1,0,0]));
        assert_eq!(merkle_tree.find_path_leaf_to_root(3), Vec::from([3,1,0,0]));
        assert_eq!(merkle_tree.find_path_leaf_to_root(4), Vec::from([4,2,1,0]));
        assert_eq!(merkle_tree.find_path_leaf_to_root(5), Vec::from([5,2,1,0]));
        assert_eq!(merkle_tree.find_path_leaf_to_root(6), Vec::from([6,3,1,0]));
        assert_eq!(merkle_tree.find_path_leaf_to_root(7), Vec::from([7,3,1,0]));
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
