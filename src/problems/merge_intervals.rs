use std::collections::HashMap;

/*

idea:

[[1, 3], [8, 10], [2, 6], [15, 18]]
  i
          j

...

first, sort by start-time:

[[1, 3], [2, 6], [8, 10], [15, 18]]
  i
          j

*/
pub fn solution(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut merged_intervals: Vec<Vec<i32>> = Vec::new();

    // let mut pq: HashMap<_, _> = HashMap::new();

    merged_intervals
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::helper_macros::macros::*; // provides vec_2d!

    #[test]
    fn basic() {
        assert_eq!(
            solution(vec_2d![[1, 3], [2, 6], [8, 10], [15, 18]]),
            vec_2d![[1, 6], [8, 10], [15, 18]]
        );
        assert_eq!(solution(vec_2d![[1, 4], [4, 5]]), vec_2d![[1, 5]]);
    }
}
