
use std::fmt::Display;
use sha2::{Sha256, Digest};

#[derive(Debug)]
pub struct MerkleTree {
    pub tree: Vec<Vec<String>>,
    // Depth of tree
    pub depth: usize
}

impl Display for MerkleTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        write!(
            f,
            "MerkleTree depth {}: \n1: {:?}\n2: {:?}\n3: {:?}\n",
            self.depth, self.tree[0], self.tree[1], self.tree[2]
            // "{:?}\n{:?}\n{:?}\n",
            // self.tree[0], self.tree[1], self.tree[2]
        )
    }
}

impl MerkleTree {
    // Take a list of hashes and build full merkle tree
    pub fn build(leaves: Vec<String>) -> MerkleTree {
        let depth: usize = MerkleTree::find_depth(leaves.len());
        
        // Row 0
        let mut tree: Vec<Vec<String>> = vec!();
        tree.push(leaves);

        // Build each row of Merkle tree
        for row in 0..depth-1 {
            let mut next_row: Vec<String> = vec!();
            // Hash concaternation of pairs of items on current row to build next row
            for i in (0..tree[row].len()).step_by(2) {
                next_row.push(hash((tree[row][i].clone() + &tree[row][i+1]).as_ref()));
            }
            // println!("next row: {:?}", next_row);
            tree.push(next_row);
        }

        MerkleTree {
            tree,
            depth
        }
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
        let hashes: Vec<String> = inputs.into_iter().map(|x| hash(x.as_ref())).collect();
        // println!("hashes: {:?}", hashes);
        
        // need to clone because concaternation is done in first item
        // let concat = hashes[0].clone() + &hashes[1];
        // println!("concat: {:?}", concat);
        
        // println!("hash concat: {:#?}", hash(concat.as_ref()));

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
}
