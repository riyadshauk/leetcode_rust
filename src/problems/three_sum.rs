/*

idea:

prevI: -1

prevJ: null

prevK: null

uniqueTriplets = [[-1,0,1]]

[-4,-1,0,0,1,2]
         i
           j
             k

*/
pub fn solution(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut unique_triplets: Vec<Vec<i32>> = Vec::new();

    nums.sort_unstable();

    for i in 0..(nums.len() - 2) {
        if i > 0 && nums[i] == nums[i - 1] {
            continue;
        }
        let mut j = i + 1;
        let mut k = nums.len() - 1;
        while j < k {
            if j != i + 1 && nums[j] == nums[j - 1] {
                j += 1;
                continue; // must loop to ignore all such dups before continuing to sum
            }
            if k < nums.len() - 1 && nums[k] == nums[k + 1] {
                k -= 1;
                continue; // must loop to ignore all such dups before continuing to sum
            }
            let sum = nums[i] + nums[j] + nums[k];
            dbg!(sum, i, j, k);
            if sum < 0 {
                j += 1
            } else if sum > 0 {
                k -= 1
            } else {
                unique_triplets.push(vec![nums[i], nums[j], nums[k]]);
                j += 1;
            }
        }
    }

    unique_triplets
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(
            solution(vec![-1, 0, 1, 2, -1, -4]),
            vec![[-1, -1, 2], [-1, 0, 1]]
        );
        let empty: Vec<Vec<i32>> = Vec::new();
        assert_eq!(solution(vec![0, 1, 1]), empty);
        assert_eq!(solution(vec![0, 0, 0]), vec![[0, 0, 0]]);
        assert_eq!(solution(vec![0, 0, 0, 0]), vec![[0, 0, 0]]); // failed TC in Leetcode submission 1
        assert_eq!(
            solution(vec![-4, -2, -2, -2, 0, 1, 2, 2, 2, 3, 3, 4, 4, 6, 6]),
            vec![
                [-4, -2, 6],
                [-4, 0, 4],
                [-4, 1, 3],
                [-4, 2, 2],
                [-2, -2, 4],
                [-2, 0, 2]
            ]
        ); // failed TC in Leetcode submission 2
    }
}
