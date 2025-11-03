use core::num;
use std::{collections::HashMap, hash::Hasher};

fn main() {
    println!("Hello, world!");
}

fn two_nums(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let n = nums.len();
    for i in 0..n {
        for j in (i + 1)..n {
            if nums[i] + nums[j] == target {
                return vec![i as i32, j as i32];
            }
        }
    }
    return vec![];
}

use std::{collections::HashMap, hash::Hasher};
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut seen: HashMap<i32, usize> = HashMap::new();
        for (i, &x) in nums.iter().enumerate() {
            let need = target - x;
            if let Some(&j) = seen.get(&need) {
                return vec![j as i32, i as i32];
            }
            seen.insert(x, i);
        }
        return vec![];
    }
}
