use std::{cell::RefCell, rc::Rc};

use crate::{utils::binary_tree::TreeNode, utils::convert_array_to_binary_tree};

pub fn solution(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    return [].to_vec();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        solution(None);
        panic!("Not yet implemented!")
    }
}
