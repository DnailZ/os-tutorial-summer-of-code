

struct Solution();


use std::collections::HashMap;


impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut maps = HashMap::<i32, usize>::new();
        let m = nums.iter().enumerate().find_map(move |(i, e)| {
            maps.insert(*e, i);
            maps.get(&(target - e)).copied().and_then(|j|
                if i == j {None}
                else {Some((i , j))}
            )
        }).unwrap();

        vec![m.0 as _, m.1 as _]
    }
}

#[cfg(test)]
mod test{
    use super::*;
    use std::assert_eq;
    #[test]
    fn t() {
        assert_eq!(
            Solution::two_sum(vec![3,2,4], 6),
            vec![2,1]
        );
    }
}