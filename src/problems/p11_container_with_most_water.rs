use super::Solution;

/*
 * @lc app=leetcode id=11 lang=rust
 *
 * [11] Container With Most Water
 */

// @lc code=start
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut area = 0;
        let (mut l, mut r) = (0, height.len()-1);

        while l<r {
            area = area.max((r-l) as i32*height[r].min(height[l]));

            if height[r] < height[l] {
                r -= 1;
            } else {
                l += 1;
            }
        }

        area
    }
}
// @lc code=end

