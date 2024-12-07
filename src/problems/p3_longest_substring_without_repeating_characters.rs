use super::Solution;

/*
 * @lc app=leetcode id=3 lang=rust
 *
 * [3] Longest Substring Without Repeating Characters
 */

// @lc code=start
impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut max_len = 0;
        let mut char_vec = vec![];
        let str_len = s.len();

        for i in 0..str_len {
            if max_len < (str_len-i) {
                char_vec.clear();
                let mut len = 0;

                for j in s.chars().skip(i) {
                    if !char_vec.contains(&j) {
                        char_vec.push(j);
                        len += 1;
                    }
                    else {
                        break;
                    }
                }
                if len>max_len {
                    max_len = len
                }
            }
            else {
                break;
            }
        }

        max_len as i32
    }
}
// @lc code=end

