use super::Solution;

/*
 * @lc app=leetcode id=4 lang=rust
 *
 * [4] Median of Two Sorted Arrays
 */

// @lc code=start
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut nums1 = nums1;
        nums1.extend(nums2);
        nums1.sort();

        let len = nums1.len();
        if len % 2 != 0 {
            let index = len.div_ceil(2) - 1;
            nums1[index] as f64
        } else {
            let index = len / 2;
            (nums1[index] + nums1[index - 1]) as f64 / 2.
        }
    }
}
// @lc code=end
