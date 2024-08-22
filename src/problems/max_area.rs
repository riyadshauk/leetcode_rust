/*

brute force: O(n^2) time, O(1) space
- loop through all heights, calculcate: Math.min(height[j], height[i]) * (j - i), keep track of max.

 0,1,2,3,4,5,6,7,8
[1,8,6,2,5,4,8,3,7]
-> 7*7 = 49

consider sliding window, or PQ (somehow).

 0,1,2,3,4,5,6,7,8,9
[1,8,6,2,5,4,8,3,9,7]
   i
                 j

 0,1,2,3,4,5,6,7,8,9
[1,8,6,2,9,4,8,3,3,7]
         i
                   j

curMax = 1 * 1, 6 * 1, 3 * 8 = 24, 5 * 7 = 35

move i to bigger of the two


 0,1,2,3,4,5,6,7,8,9
[1,8,6,2,9,4,8,3,3,7]
 i
                   j

idea: keep track of biggest seen so far (O(n) space, can optimize by reordering pointers to go from out in!)

====

next thought: memo

maxSeenAtHeightSoFar <-- seems like a PQ (which would take more space and more time than sliding window)...

manual test:

 0,1,2,3,4,5,6,7,8,9
[1,8,6,2,9,4,8,3,3,7]
   i
                   j

*/

use std::cmp;

pub fn solution(heights: Vec<i32>) -> i32 {
    let mut max = 0;

    let mut i = 0;
    let mut j = heights.len() - 1;

    while i < j {
        max = cmp::max(cmp::min(heights[i], heights[j]) * (j - i) as i32, max);
        if heights[i] < heights[j] {
            i += 1;
        } else if heights[i] > heights[j] {
            j -= 1;
        } else {
            i += 1;
            j -= 1;
        }
    }

    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(solution(vec![1, 8, 6, 2, 9, 4, 8, 3, 3, 7]), 56);
        assert_eq!(solution(vec![1, 1]), 1);
        assert_eq!(solution(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
    }
}
