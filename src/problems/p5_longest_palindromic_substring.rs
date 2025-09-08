use super::Solution;

/*
 * @lc app=leetcode id=5 lang=rust
 *
 * [5] Longest Palindromic Substring
 */

// @lc code=start
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let mut longest: &[u8] = &[];
        let bytes = s.as_bytes();

        for i in 0..s.len() {
            let (mut l, mut r) = (i, i);

            while r < s.len() && bytes[l] == bytes[r] {
                if r - l + 1 > longest.len() {
                    longest = &bytes[l..=r];
                }

                if l == 0 {
                    break;
                }
                r += 1;
                l -= 1;
            }

            (l, r) = (i, i + 1);
            while r < s.len() && bytes[l] == bytes[r] {
                if r - l + 1 > longest.len() {
                    longest = &bytes[l..=r];
                }

                if l == 0 {
                    break;
                }
                r += 1;
                l -= 1;
            }
        }

        String::from_utf8(longest.to_vec()).unwrap()
    }
}
// @lc code=end
