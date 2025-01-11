use std::collections::HashMap;

use crate::starter::Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut res: Vec<i32> = vec![];
        let mut delta_map = HashMap::new();
        nums.iter().enumerate().for_each(|(i, n)| {
            delta_map.insert(target - n, i);
        });

        for (i, n) in nums.iter().enumerate() {
            if let Some(val) = delta_map.get(n) {
                if *val == i {
                    continue;
                }
                res = vec![*val as i32, i as i32];
            }
        }
        res
    }
}
