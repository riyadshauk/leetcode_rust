use std::collections::HashSet;

pub fn solution(s: String) -> String {}

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
