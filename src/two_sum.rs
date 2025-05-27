use std::collections::HashMap;

use crate::Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        if nums.len() == 2 {
            return vec![0, 1];
        }

        let mut diff: HashMap<i32, usize> = HashMap::default();

        for (i, n) in nums.into_iter().enumerate() {
            if let Some(&j) = diff.get(&n) {
                if i != j {
                    return vec![i as i32, j as i32];
                }
            }

            diff.insert(target - n, i);
        }

        vec![]
    }
}
