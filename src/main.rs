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
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        // let (mut left, mut right) = (0_i32, nums.len() as i32 - 1);
        let mut left = 0;
        let mut right: i32 = nums.len() as i32 - 1;

        while left <= right {
            let mid = (right - left) / 2 + left;
            let num = nums[mid as usize];
            if num == target {
                return mid;
            }
            if num > target {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }

        return -1;
    }
}
