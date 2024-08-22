/*
guess: cap = cmp::max(weights)
cap = 10
cap = 11
cap = 12
cap = 13

 0,1,2,3,4,5
[1,10,1,2,1,4,8,5], 2.... ship_cap: 10 = cmp::max(ship_cap, weight[i]) (max-cap: 32), 21, 15
 1  1 1 1 1 2 2 3
 1  1 1 1 1 2 2 2
 1  1 1 1 1 2 2 3
 1  1 1 1 1 2 2 2
 1  1 1 1 1 2 2 3

              i
              j

lo: 10 -> 15 -> 16 -> 17
hi: 32 -> 21 -> 18 -> 17
cap: 21 -> 15 -> 18 -> 16 -> 17 -> 16
cur_day_weight: 4
days_needed: 2

 when I increased cap, keep going forward... make sure less days is ok... (3)

 cases:
 - need to make cap larger to fit into days (constraint)
 - need to increase cap to fit each weigth (cap >= weight[i] for each i) (constraint)
 - need cap to be as small as possible (constraint)


Might be useful to consider
acc_sum[i]

====

1. Sum of all weights
2. Take max element as starting cap
3.1 Use binary search from cap through 500 (max size) * 5*10^4 (max num of elems)
3.2 while looping through entire array and updating cap (end when cap yields sol <= days... but want min, so keep going smaller also using binary search until cap yields sold > days...)

run-time = log_2(25e6)*len(weights)
*/

fn binary_sarch_minimal(weights: Vec<i32>, days: i32) -> i32 {
    let mut lo = weights.iter().fold(std::i32::MIN, |acc, b| acc.max(*b));
    let mut hi = weights.iter().sum(); // 25 * 10 ^ 6;
    let mut cap; // cap represents cap, which is the traditional 'mid' in BS algo
    while lo < hi {
        // <= ? [todo]
        cap = (hi - lo) / 2 + lo; // avoid integer overflow by subtracting from hi, rounds down (I think)
        let mut cur_day_weight = 0;
        let mut days_needed_acc = 1;
        for weight in &weights {
            cur_day_weight += *weight;
            if cur_day_weight > cap {
                // case when more days would be required than provided 'days', so need to make cap larger
                days_needed_acc += 1;
                cur_day_weight = *weight;
            }
            if days_needed_acc > days {
                lo = cap + 1; // we know cap is definitely too low in this case, so skip it.
            }
        }
        if days_needed_acc <= days {
            // means cap is maybe too high, so need to try smaller cap (until exhaust all potential caps in range [lo,hi])
            hi = cap;
        }
    }
    (hi - lo) / 2 + lo // recalculate cap value one last time before returning (in case off from last iteration of loop)
}

pub fn solution(weights: Vec<i32>, days: i32) -> i32 {
    binary_sarch_minimal(weights, days)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(solution(vec![1, 10, 1, 2, 1, 4, 8, 5], 2), 17); // my own test, below are the provided LC tests, just running them for first time after implementing...
        assert_eq!(solution(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 5), 15);
        assert_eq!(solution(vec![3, 2, 2, 4, 1, 4], 3), 6);
        assert_eq!(solution(vec![1, 2, 3, 1, 1], 4), 3);
    }
}
