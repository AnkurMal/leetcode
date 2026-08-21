use super::Solution;

impl Solution {
    pub fn count_and_say(n: i32) -> String {
        let mut str = "1".to_string();

        for _ in 0..n-1 {
            let mut new = String::new();
            let mut stb = str.as_bytes();
            let mut counter = 0;
            let mut current = stb[0];

            for &i in stb {
                if i == current {
                    counter += 1;
                } else {
                    new.push(char::from_digit(counter, 10).unwrap());
                    new.push(current as char);

                    counter = 1;
                    current = i;
                }
            }

            new.push(char::from_digit(counter, 10).unwrap());
            new.push(current as char);
            str = new;
        }

        str
    }
}
