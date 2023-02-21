/*
 * @lc app=leetcode.cn id=4 lang=rust
 *
 * [4] 寻找两个正序数组的中位数
 */

// @lc code=start
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut all = nums2;
        all.extend(nums1);
        // for i in nums1 {
        //     all.push(i);
        // }
        all.sort();
        let len = all.len();
        if len % 2 == 0 {
            return (all[len / 2] as f64 + all[len / 2 - 1] as f64) / 2.0;
        } else {
            all[len / 2] as f64
        }
    }
}
// @lc code=end

struct Solution;
