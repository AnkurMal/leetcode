use super::Solution;

/*
 * @lc app=leetcode id=9 lang=rust
 *
 * [9] Palindrome Number
 */

// @lc code=start
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let x = x.to_string();

        x == x.chars().rev().collect::<String>()
    }
}
// @lc code=end
