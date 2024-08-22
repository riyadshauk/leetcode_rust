/*

*/

// not yet solved... TBC a different time!
pub fn solution(nums: Vec<i32>, target: i32) -> i32 {
    println!("nums: {:?}, target: {:?}", nums, target);
    let mut lo: usize = 0;
    let mut hi: usize = nums.len() - 1;

    while lo <= hi {
        let mid = lo + (hi - lo) / 2;
        if target == nums[mid] {
            return mid as i32;
        }
        if nums[mid] > target {
            // normally we unconditionally search left here
            if nums[lo] <= target {
                // use nums[mid] here in condition??
                if mid > 0 {
                    // guard against integer overflow by going negative on unsigned array index
                    hi = mid - 1; // search left
                } else {
                    return -1;
                }
            } else {
                lo = mid + 1; // search right
            }
        } else {
            // normally, we unconditionally search right here, everything below kind of symmetrical of above logic
            if nums[hi] >= target {
                // use nums[mid] here in condition??
                lo = mid + 1; // go right
            } else {
                if mid > 0 {
                    // guard against integer overflow by going negative on unsigned array index
                    hi = mid - 1; // search left
                } else {
                    return -1;
                }
            }
        }
    }

    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(solution(vec![4, 5, 6, 7, 0, 1, 2], 0), 4);
        assert_eq!(solution(vec![4, 5, 0, 1, 2], 5), 1);
        assert_eq!(solution(vec![2], 1), -1);
        assert_eq!(solution(vec![2], 3), -1);
        assert_eq!(solution(vec![1, 3], 4), -1);
        assert_eq!(solution(vec![4, 5, 6, 7, 8, 1, 2, 3], 8), 4);
        assert_eq!(solution(vec![3, 4, 5, 6, 1, 2], 1), 4);
    }
}
