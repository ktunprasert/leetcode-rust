use crate::Solution;

impl Solution {
    pub fn is_palindrome(mut x: i32) -> bool {
        if x < 10 {
            return x >= 0;
        }

        let mut ns = [-1; 10];
        let mut len = 0;

        while x > 0 {
            ns[len] = x % 10;
            len += 1;
            x /= 10;
        }

        for i in 0..len / 2 {
            if ns[i] != ns[len - i - 1] {
                return false;
            }
        }

        true
    }
}
