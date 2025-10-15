use super::Solution;

/*
 * @lc app=leetcode id=30 lang=rust
 *
 * [30] Substring with Concatenation of All Words
 */

// @lc code=start
use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        let mut res: HashSet<i32> = HashSet::new();
        if s.len() < words[0].len() {
            return res.into_iter().collect();
        }

        let mut freq: HashMap<&str, i32> = HashMap::new();

        let words_len = words[0].len();

        for i in &words {
            *freq.entry(i).or_insert(0) += 1;
        }
        let mat = freq.values().sum::<i32>();

        for i in 0..words_len {
            let mut in_freq = freq.clone();
            let (mut start, mut end) = (i, i + words_len);
            let mut in_start = start;
            let mut in_match = 0;

            while !(s.len() - in_start < words.len() * words_len && in_match == 0) {
                if let Some(val) = in_freq.get_mut(&s[in_start..end]) {
                    if *val == 0 {
                        start += words_len;
                        in_start = start;
                        end = in_start + words_len;
                        in_match = 0;
                        in_freq = freq.clone();

                        continue;
                    }

                    *val -= 1;
                    in_match += 1;

                    if in_match == mat {
                        res.insert(start as i32);

                        start += words_len;
                        in_start = start;
                        end = in_start + words_len;
                        in_match = 0;
                        in_freq = freq.clone();

                        continue;
                    }
                } else {
                    start = end;
                    in_freq = freq.clone();
                    in_match = 0;
                }

                in_start += words_len;
                end += words_len;
            }
        }

        res.into_iter().collect()
    }
}
// @lc code=end
