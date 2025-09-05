use super::Solution;

/*
 * @lc app=leetcode id=10 lang=rust
 *
 * [10] Regular Expression Matching
 */

// @lc code=start
use regex::Regex;

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        let mut p = p;
        p.insert(0, '^');
        p.push('$');

        let re = Regex::new(&p).unwrap();
        re.is_match(&s)
    }
}
// @lc code=end

