use crate::Solution;

impl Solution {
    pub fn is_palindrome(mut x: i32) -> bool {
        if x < 10 {
            return x >= 0;
        }

        dbg!(x);

        let mut cnt = [0; 10];
        let mut found_at = [-1i32; 10];
        // let mut nums: &[i32] = &[];
        let mut i = 0;

        while x > 0 {
            let key = (x % 10) as usize;

            if cnt[key] > 0 {
                cnt[key] -= 1;
                found_at[key] = -1;
                // nums.as_slice
                // nums.push(key as i32);
                // nums.
            } else {
                cnt[key] += 1;
                found_at[key] = i;
            }

            i += 1;
            x /= 10;
        }

        let counted: i32 = cnt.iter().sum();

        // if counted == 1 {
        //     // check middle
        //     true
        // } else {
        //     counted <= 1
        // }

        // dbg!(&cnt);
        // dbg!(&found_at);
        for z in 0..10 {
            if found_at[z] < 0 || cnt[z] == 0 {
                continue;
            }

            if i % 2 == 0 {
                return counted <= 1;
            }

            dbg!(cnt[z]);
            dbg!(found_at[z]);
            return counted <= 1 && found_at[z] == ((i - 1) / 2);
        }

        true
    }
}
