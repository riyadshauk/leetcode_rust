use std::{cell::RefCell, rc::Rc};

use crate::utils::binary_tree;

// convert_array_to_binary_tree is for ease of initializing binary trees, because who wants to type out several lines of procedural code just to instantiate a desired binary tree?!... In the spirit of binary tree inputs for Leetcode problems.
pub fn convert_array_to_binary_tree(
    encoded_tree: Vec<i32>,
) -> Option<Rc<RefCell<binary_tree::TreeNode>>> {
    return None;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        // minNum represents the minimum allowable number in the BT (this is due to constraint in Rust of not being able to have None/null as mixed type in Vector of integers)
        let minNum = -100;
        convert_array_to_binary_tree([1, 2, 3, minNum - 1, 5, minNum - 1, 4].to_vec());
        panic!("Not yet implemented!")
    }
}
