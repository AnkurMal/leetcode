use super::Solution;

/*
 * @lc app=leetcode id=14 lang=rust
 *
 * [14] Longest Common Prefix
 */

// @lc code=start
impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut end = 0;
        let vec = strs
            .iter()
            .map(|x| x.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        'outer: loop {
            let &first = vec[0].get(end).unwrap_or(&' ');

            for i in vec.iter() {
                if let Some(&ch) = i.get(end) {
                    if first != ch {
                        break 'outer;
                    }
                } else {
                    break 'outer;
                }
            }

            end += 1;
        }

        strs[0][..end].into()
    }
}
// @lc code=end
