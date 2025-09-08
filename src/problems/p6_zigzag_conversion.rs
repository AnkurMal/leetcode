use super::Solution;

/*
 * @lc app=leetcode id=6 lang=rust
 *
 * [6] Zigzag Conversion
 */

// @lc code=start
impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s;
        }

        let num_rows = num_rows as usize;
        let fluc = num_rows * 2 - 2;

        let s = s.as_bytes();
        let mut new = vec![];

        let mut i = 0;
        while let Some(&byte) = s.get(i * fluc) {
            new.push(byte);
            i += 1;
        }

        for i in 1..num_rows - 1 {
            let mut next = i;
            let mut flip = fluc - i * 2;

            while let Some(&byte) = s.get(next) {
                new.push(byte);
                next += flip;
                flip = fluc.abs_diff(flip);
            }
        }

        i = num_rows - 1;
        while let Some(&byte) = s.get(i) {
            new.push(byte);
            i += fluc;
        }

        String::from_utf8(new).unwrap()
    }
}
// @lc code=end
