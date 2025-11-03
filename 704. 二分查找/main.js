/**
 * @param {number[]} nums
 * @param {number} target
 * @return {number}
 */
var search = function (nums, target) {
  let left = 0,
    right = nums.length - 1;
  while (left <= right) {
    let mid = Math.floor((right - left) / 2) + left;
    const num = nums[mid];
    if (num == target) return mid;
    if (num < target) {
      left = mid + 1;
    } else {
      right = mid - 1;
    }
  }
  return -1;
};

var search2 = function (nums, target) {
  // 双重for循环
};
