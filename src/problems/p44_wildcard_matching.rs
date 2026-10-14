use super::Solution;

use std::collections::HashMap;

impl Solution {
    pub fn is_match_p44(s: String, p: String) -> bool {
        let s_bytes = s.as_bytes();
        let p_bytes = p.as_bytes();
        let mut memo: HashMap<(usize, usize), bool> = HashMap::new();

        Self::dfs(s_bytes, p_bytes, 0, 0, &mut memo)
    }

    fn dfs(
        s: &[u8],
        p: &[u8],
        i: usize,
        j: usize,
        memo: &mut HashMap<(usize, usize), bool>,
    ) -> bool {
        if let Some(&cached) = memo.get(&(i, j)) {
            return cached;
        }

        if j == p.len() {
            let res = i == s.len();
            memo.insert((i, j), res);
            return res;
        }

        if i == s.len() {
            let res = p[j..].iter().all(|&c| c == b'*');
            memo.insert((i, j), res);
            return res;
        }

        let result = if p[j] == b'*' {
            Self::dfs(s, p, i, j + 1, memo) || Self::dfs(s, p, i + 1, j, memo)
        } else if p[j] == b'?' || p[j] == s[i] {
            Self::dfs(s, p, i + 1, j + 1, memo)
        } else {
            false
        };

        memo.insert((i, j), result);
        result
    }
}
