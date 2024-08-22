/*
        0,1,2,3,4,5,6,7
nums = [4,5,6,7,8,0,1,2], target = 0
        l     m       h



 0,1,2,3,4,5,6,7
[4,5,6,7,0,1,2]
 l     m     h
         l m h

 0,1,2,3,4,5,6,7
[4,5,6,7,0,1,2], t = 0
 l     m     h
         l m h

 0,1,2,3,4,5,6,7
[4,5,6,7,0,1,2], t = 0
 l     m     h
         l m h
         lh

[1,3], 4
 m
 lh

 0,1,2,3,4,5,6,7
[4,5,6,7,8,1,2,3], target = 8
 l     m       h
         l m
*/

pub fn solution(nums: Vec<i32>, target: i32) -> i32 {
    println!("nums: {:?}, target: {:?}", nums, target);
    let mut lo: usize = 0;
    let mut hi: usize = nums.len() - 1;

    while lo <= hi {
        let mid = lo + (hi - lo) / 2;
        if target == nums[mid] {
            return mid as i32;
        }
        if target < nums[mid] {
            // can either be right or left, due to rotation!
            // sub-problem: figuring out which way to go based (maybe based on nums[lo] and nums[hi])
            if nums[lo] > target {
                // have to go right, because the smaller portion of array must have been rotated on right side
                // lo = mid + 1
                // target = 5, [9,5,6,7,8] -> nums[lo] > target, nums[mid] < nums[lo], nums[mid] > target
                // if nums[mid] < nums[lo] {
                lo = mid + 1; // go right (lower part of array was rotated to end)
            } else {
                if mid > 0 {
                    // protect against intneger underflow
                    if nums[hi] == target {
                        return hi as i32;
                    }
                    hi = mid - 1; // go left
                } else {
                    return -1;
                }
            }
        } else {
            // nums[lo] <= target + target < nums[mid], since array is sorted, means target lies between nums[lo] and nums[mid] in array
            // suppose this isn't the case. This means that target lies on right side of array, which means there was some rotation pushing the target there. But we know the array is sorted.
            // want to construct sorted array where target can be on other side (after rotation).
            // target = 5, [1,7,3,4,5] --> means target can be on left or right side.
            // how do we know if target is on the right side? -> if right side was rotated
            // sub-problem: detect if right is rotated, then go right, else go left
            // hi = mid - 1;
            if nums[hi] < target {
                // target = 5, [1,7,3,4,5] <-- wrong example first time!
                // [4,5,6,7,8,1,2,3], target = 8
                //          l m   h  --> gives -1 because doesn't check nums[lo]!
                if nums[lo] == target {
                    return lo as i32;
                }
                lo = mid + 1; // go right
            } else {
                if mid > 0 {
                    // protect against integer underflow
                    if nums[hi] == target {
                        return hi as i32;
                    }
                    hi = mid - 1; // go left (higher part of array was rotated left)
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
        // provided by leetcode on first submit:
        assert_eq!(solution(vec![1], 2), -1); // integer underflow
        assert_eq!(solution(vec![1, 3], 4), -1); // integer underflow again!
        assert_eq!(solution(vec![4, 5, 6, 7, 8, 1, 2, 3], 8), 4); // (used wrong example when implementing first time!)
    }
}
