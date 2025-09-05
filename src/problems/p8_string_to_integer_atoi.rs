use super::Solution;

/*
 * @lc app=leetcode id=8 lang=rust
 *
 * [8] String to Integer (atoi)
 */

// @lc code=start
impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut is_neg = 1;
        let mut seen_int = false;
        let s = s.trim();

        let mut start = 0;
        let mut bound = s.len();

        for (i, ch) in s.chars().enumerate() {
            if i == 0 {
                if ch == '-' {
                    is_neg = -1;
                    start += 1;
                } else if ch == '+' {
                    is_neg = 1;
                    start += 1;
                } else if !ch.is_ascii_digit() {
                    bound = 0;
                    break;
                }
            } else if !ch.is_ascii_digit() {
                bound = i;
                break;
            }

            if ch.is_ascii_digit() && ch == '0' {
                if !seen_int {
                    start += 1;
                }
            } else if ch.is_ascii_digit() && ch != '0' {
                seen_int = true;
            }
        }

        if s[start..bound].len() > 11 {
            return if is_neg < 0 { i32::MIN } else { i32::MAX };
        }

        let ret = s[start..bound].parse::<i64>().unwrap_or(0) * is_neg;
        if ret < i32::MIN as i64 {
            i32::MIN
        } else if ret > i32::MAX as i64 {
            i32::MAX
        } else {
            ret as i32
        }
    }
}
// @lc code=end
