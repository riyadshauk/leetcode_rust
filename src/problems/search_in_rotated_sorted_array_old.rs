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

*/

pub fn solution(nums: Vec<i32>, target: i32) -> i32 {
    println!("nums: {:?}, target: {:?}", nums, target);
    let mut lo: usize = 0;
    let mut hi: usize = nums.len() - 1;

    while lo < hi {
        let mid = lo + (hi - lo) / 2;
        if target == nums[mid] {
            return mid as i32;
        }
        if target > nums[hi] {
            // need to check against mid though!
            hi = mid - 1;
        }
        if target < nums[lo] {
            lo = mid + 1;
        }
        // if target > nums[mid] {
        //     if target > nums[hi] { // was rotated on left side
        //         hi = mid - 1;
        //     }
        //     // else {
        //     //     //      0, 1,2,3,4,5,6,7
        //     //     // ex: [9,10,1,2,3,4,5,8], target = 7
        //     //     // not greater than nums[hi] means it can't be rotated to front, so check right side
        //     //     lo = mid + 1;
        //     // }
        //     if target < nums[lo] { // was rotated on right side
        //         //      0, 1,2,3,4,5,6,7
        //         // ex: [9,10,1,2,3,4,5,8], target = 7
        //         // means it has to have wrapped around/rotated to other side of array (on right)
        //         lo = mid + 1;
        //     }
        //     // else {
        //     //     //      0, 1,2,3,4,5,6,7
        //     //     // ex: [9,10,1,2,3,4,5,8], target = 10
        //     //     // means we have to check left portion of array
        //     //     hi = mid - 1;
        //     // }
        // } else {
        //     if target > nums[hi] {
        //         //      0,1,2,3,4,5,6
        //         // ex: [1,2,3,4,0], target = 2
        //         hi = mid - 1;
        //     }
        //     // else {
        //     //     //      0,1,2,3,4,5,6,7
        //     //     // ex: [5,6,7,8,1,2,3,4], target = 2
        //     //     lo = mid + 1;
        //     // }
        //     if target < nums[lo] {
        //         //      0,1,2,3,4,5,6,7
        //         // ex: [5,6,7,8,1,2,3,4], target = 2
        //         lo = mid + 1;
        //     }
        //     // else {
        //     //     // less than mid, greater than lo, means go left
        //     //     hi = mid - 1;
        //     // }
        // }
    }

    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(solution(vec![4, 5, 6, 7, 0, 1, 2], 0), 4);
    }
}
