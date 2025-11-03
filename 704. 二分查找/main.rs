impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let left: i32 = 0;
        let right: i32 = nums.len() - 1;

        while left <= right {
            let mid = (right - left) / 2 + left;
            let num = nums[mid];
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
