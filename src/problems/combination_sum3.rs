use std::collections::{HashSet, LinkedList};

use crate::utils::helper_macros::macros::vec_2d;

fn helper(k: i32, n: i32, sum: Vec<i32>) -> Vec<i32> {
    if n == 0 {
        // required base case, as 0 is not a valid input to this problem (we've found sum to here, with <= k terms...)
        return sum;
    }

    if k == 0 {
        // straighforward, since we must decrease k by one on each recursive call.
        return sum;
    }

    vec![]
}

pub fn solution(k: i32, n: i32) -> Vec<Vec<i32>> {
    // hint: back-tracking, 1 <= k <= 9, 1 <= n <= 60 --> (so just solve for correctness)
    let mut combination_sum: Vec<Vec<i32>> = Vec::new();

    /*

    brainstorming...

    need a number to add to get to sum...

    how do I keep track of partial sums? What if some partial sums already have answers (that we possibly computed once before) – is that something that would mean anything for this problem?

    how would I do this manually? Need to think with a more concrete example...

    k = 2, n = 4

    x + y = 4 (want all distinct (x, y) pairs that satisfy this equation... -- we have k = 2 variables denoted with x and y here)
    y = 4 - x
    x = 4 - y

    We can simply iterate over 1 to n
    y = 3, 2, 1 (when x = 1, 2, 3)
    x = 3, 2, 1... same...

    hm... let's try a bigger example, to not get confused by only 2 variables...

    k = 3, n = 9
    x + y + z = 9
    x = 9 - y - z

    Simplest solution would be to have two nested loops (three loops total), and keep checking the sums.
    Then there would be a problem of duplicate tuples, so convert to string, sort topologically, and store in HS

     */

    combination_sum
}

#[cfg(test)]
mod tests {
    use crate::utils::helper_macros::macros::vec_2d;

    use super::*;

    #[test]
    fn basic() {
        println!("assert_eq!(solution(3, 7), vec_2d![[1,2,4]]);");
        assert_eq!(solution(3, 7), vec_2d![[1, 2, 4]]);
        println!("assert_eq!(solution(3, 9), vec_2d![[1,2,6],[1,3,5],[2,3,4]]);");
        assert_eq!(solution(3, 9), vec_2d![[1, 2, 6], [1, 3, 5], [2, 3, 4]]);
        println!("assert_eq!(solution(4, 1), vec_2d![[]]);");
        assert_eq!(solution(4, 1), vec_2d![[]]);
        // some of my own examples below, while understanding the problem...
        println!("assert_eq!(solution(1, 4), vec_2d![[4]]);");
        assert_eq!(solution(1, 4), vec_2d![[4]]); // k == 1 edge case
        println!("assert_eq!(solution(2, 4), vec_2d![[1, 3], [2, 2]]);");
        assert_eq!(solution(2, 4), vec_2d![[1, 3], [2, 2]]); // simple k == 2 case
    }
}
