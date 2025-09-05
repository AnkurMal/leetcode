use super::Solution;

/*
 * @lc app=leetcode id=7 lang=rust
 *
 * [7] Reverse Integer
 */

// @lc code=start
impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut neg = false;
        if x<0 {neg = true};

        let x = x.abs().to_string();
        let mut x = x.chars().rev().collect::<String>();

        if neg {
            x.insert(0, '-');
        }

        x.parse::<i32>().unwrap_or(0)
    }
}
// @lc code=end

