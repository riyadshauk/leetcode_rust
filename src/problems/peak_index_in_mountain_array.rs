/*

 0,1,2,3,4
[1,2,3,2,1] -> 2

note: arr[idx] happens to be largest element in array... (observation) ... not useful, though.

high-level idea:
- do binary search for the point at which the peak happens <- base/return case
1. peak happens, return it
2. increasing, lo = mid + 1 (consider higher up the mountain)
3. decreasing part of list, hi = mid - 1 (consider farther down the mountain)

how to check if peak happens? -> need to read arr[mid - 1], arr[mid], arr[mid + 1] each time.

  0, 1, 2, 3, 4, 5,  6, 7, 8
[18,29,38,59,98,100,99,98,90]
 lo           m           hi
                 lo  m    hi


*/

pub fn solution(arr: Vec<i32>) -> i32 {
    let mut lo: usize = 0;
    let mut hi: usize = arr.len() - 1;

    while lo < hi {
        // guarantees that we have lo < mid < hi, saves extra checking (over lo <= hi)
        // shouldn't ever be equal to hi, but shouldn't get there due to problem contraint that peak exists, hopefully
        let mid = lo + (hi - lo) / 2;
        let (first, second, third) = (arr[mid - 1], arr[mid], arr[mid + 1]);
        // println!("first: {first}, second: {second}, third: {third}");
        if first < second && second > third {
            return mid as i32;
        }

        if first < second {
            lo = mid;
        } else {
            hi = mid;
        }
    }

    -1 // shouldn't ever get here, guaranteed to be mountain array => guaranteed to have a peak
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        println!("assert_eq!(solution(vec![1, 2, 3, 2, 1]), 2);");
        assert_eq!(solution(vec![1, 2, 3, 2, 1]), 2);
        println!("assert_eq!(solution(vec![0, 2, 1, 0]), 1);");
        assert_eq!(solution(vec![0, 2, 1, 0]), 1);
        println!("assert_eq!(solution(vec![0, 10, 5, 2]), 1);");
        assert_eq!(solution(vec![0, 10, 5, 2]), 1); // below are submission-related TCs
        println!("assert_eq!(solution(vec![18,29,38,59,98,100,99,98,90]), 5);");
        assert_eq!(solution(vec![18, 29, 38, 59, 98, 100, 99, 98, 90]), 5);
    }
}
