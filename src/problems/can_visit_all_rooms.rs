use std::collections::{HashSet, LinkedList};

pub fn solution(rooms: Vec<Vec<i32>>) -> bool {
    // one-pass graph traversal, O(n) space/time
    if rooms.len() == 0 {
        return true;
    }

    let mut visited: HashSet<usize> = HashSet::new();
    let mut q: LinkedList<usize> = LinkedList::new();
    visited.insert(0);
    for key in rooms.get(0).unwrap() {
        q.push_back(*key as usize);
    }
    while !q.is_empty() {
        let key = q.pop_front().unwrap();
        if !visited.contains(&key) && key < rooms.len() {
            visited.insert(key);
            for k in rooms.get(key).unwrap() {
                q.push_back(*k as usize);
            }
        }
    }

    visited.len() == rooms.len()
}

#[cfg(test)]
mod tests {
    use crate::utils::helper_macros::macros::vec_2d;

    use super::*;

    #[test]
    fn basic() {
        println!("assert_eq!(solution(vec_2d![[1], [2], [3], []]), true);");
        assert_eq!(solution(vec_2d![[1], [2], [3], []]), true);
        println!("assert_eq!(solution(vec_2d![[1, 3], [3, 0, 1], [2], [0]]), false);");
        assert_eq!(solution(vec_2d![[1, 3], [3, 0, 1], [2], [0]]), false);
        println!("assert_eq!(solution(vec_2d![[2], [], [1]]), true);");
        assert_eq!(solution(vec_2d![[2], [], [1]]), true);
    }
}
