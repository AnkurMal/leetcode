use super::Solution;

/*
 * @lc app=leetcode id=1 lang=rust
 *
 * [1] Two Sum
 */

// @lc code=start
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut result = Vec::new();

        'outer: for i in 0..nums.len() - 1 {
            for (j, item) in nums.iter().enumerate().skip(i + 1) {
                if nums[i] + item == target {
                    result = vec![i as i32, j as i32];
                    break 'outer;
                }
            }
        }

        result
    }
}
// @lc code=end
