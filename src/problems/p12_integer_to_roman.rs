use super::Solution;

/*
 * @lc app=leetcode id=12 lang=rust
 *
 * [12] Integer to Roman
 */

// @lc code=start
impl Solution {
    pub fn int_to_roman(num: i32) -> String {
        let mut num = num;
        let mut len = get_digits(num);
        let mut rm = String::new();

        while num > 0 {
            let mut loc = String::new();
            let exp = 10_i32.pow(len);
            let mut start = num / exp;
            let mut val = start * exp;

            while val > 0 {
                if [4, 9].contains(&start) {
                    let (v, new_val) = get_diff_49(val);
                    val = new_val;
                    loc.push(map_value(v).unwrap());

                    if let Some(ch) = map_value(val) {
                        loc.insert(0, ch);
                        break;
                    }
                } else {
                    let (v, new_val) = get_diff(val);
                    val = new_val;
                    loc.push(map_value(v).unwrap());

                    if let Some(ch) = map_value(val) {
                        loc.push(ch);
                        break;
                    }
                }

                start = val / 10_i32.pow(get_digits(val))
            }

            rm.push_str(&loc);
            num %= exp;
            len = len.wrapping_sub(1);
        }

        rm
    }
}

pub fn get_digits(num: i32) -> u32 {
    if num == 0 {
        return 0;
    }
    let mut len = 0;
    let mut num = num;

    while num > 0 {
        len += 1;
        num /= 10;
    }

    len - 1
}

fn map_value(val: i32) -> Option<char> {
    match val {
        1 => Some('I'),
        5 => Some('V'),
        10 => Some('X'),
        50 => Some('L'),
        100 => Some('C'),
        500 => Some('D'),
        1000 => Some('M'),
        _ => None,
    }
}

fn get_diff_49(val: i32) -> (i32, i32) {
    match val {
        v if v >= 500 => (1000, 1000 - val),
        v if v >= 100 => (500, 500 - val),
        v if v >= 50 => (100, 100 - val),
        v if v >= 10 => (50, 50 - val),
        v if v >= 5 => (10, 10 - val),
        v if v >= 1 => (5, 5 - val),
        _ => (0, 0),
    }
}

fn get_diff(val: i32) -> (i32, i32) {
    match val {
        v if v >= 1000 => (1000, val - 1000),
        v if v >= 500 => (500, val - 500),
        v if v >= 100 => (100, val - 100),
        v if v >= 50 => (50, val - 50),
        v if v >= 10 => (10, val - 10),
        v if v >= 5 => (5, val - 5),
        v if v >= 1 => (1, val - 1),
        _ => (0, 0),
    }
}
// @lc code=end
