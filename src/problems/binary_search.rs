/*

      0,1,2,3,4,5
(vec![1,1,2,3,3,3], 2), 1
      l m   h
          l     h
            m
          h

      0,1,2,3
(vec![1,1,2,3], 2)
      l m   h
          l h
          m

Alt answer in Leetcode after submit:

pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let mut lo = 0;
    let mut hi: i32 = (nums.len() - 1) as i32;

    while lo <= hi {
        let mid = (lo + hi) / 2;
        if nums[mid as usize] == target {
            return mid;
        }
        if nums[mid as usize] < target {
            lo = mid + 1;
        } else {
            hi = mid - 1;
        }
    }
    -1;
}

*/

pub fn solution(nums: Vec<i32>, target: i32) -> i32 {
    let mut lo: usize = 0;
    let mut hi: usize = nums.len() - 1;

    while lo <= hi {
        let mid = lo + (hi - lo) / 2;

        if nums[mid] < target {
            if mid == nums.len() - 1 {
                return -1;
            }
            lo = mid + 1;
        } else if nums[mid] > target {
            if mid == 0 {
                return -1;
            }
            hi = mid - 1;
        } else {
            return mid as i32;
        }
    }

    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        println!("assert_eq!(solution(vec![1, 2, 3], 2), 1);");
        assert_eq!(solution(vec![1, 2, 3], 2), 1);
        println!("assert_eq!(solution(vec![1, 1, 2, 3, 3, 3], 2), 2);");
        assert_eq!(solution(vec![1, 1, 2, 3, 3, 3], 2), 2);
        println!("assert_eq!(solution(vec![1, 1, 2, 3], 2), 2);");
        assert_eq!(solution(vec![1, 1, 2, 3], 2), 2);
        println!("assert_eq!(solution(vec![-5, 1, 1, 2, 4, 5, 8, 10], 2), 3);");
        assert_eq!(solution(vec![-5, 1, 1, 2, 4, 5, 8, 10], 2), 3); // my own test, below are the provided LC tests, just running them for first time after implementing...
        println!("assert_eq!(solution(vec![5], -5), -1);");
        assert_eq!(solution(vec![5], -5), -1);
    }
}
