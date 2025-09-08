use super::Solution;

/*
 * @lc app=leetcode id=13 lang=rust
 *
 * [13] Roman to Integer
 */

// @lc code=start
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut res = 0;
        let s = s.chars().collect::<Vec<char>>();
        let mut i = 0;

        while i < s.len() {
            let rm = s[i];
            if let Some(&ch) = s.get(i + 1) {
                if (rm == 'I' && (ch == 'V' || ch == 'X'))
                    || (rm == 'X' && (ch == 'L' || ch == 'C'))
                    || (rm == 'C' && (ch == 'D' || ch == 'M'))
                {
                    res += map_value(ch) - map_value(rm);

                    i += 2;
                    continue;
                } else {
                    res += map_value(rm);
                }
            } else {
                res += map_value(rm);
            }

            i += 1;
        }

        res
    }
}

fn map_value(val: char) -> i32 {
    match val {
        'I' => 1,
        'V' => 5,
        'X' => 10,
        'L' => 50,
        'C' => 100,
        'D' => 500,
        _ => 1000,
    }
}
// @lc code=end
