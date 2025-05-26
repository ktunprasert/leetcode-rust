pub fn longest_palindrome(words: Vec<String>) -> i32 {
    if words.len() == 0 {
        return 0;
    }

    let mut counts: [i32; 26 * 26] = [0; 26 * 26];
    let mut len = 0i32;

    fn to_key(a: u8, b: u8) -> usize {
        a as usize * 26 + b as usize
    }

    for w in words.iter() {
        let b = w.as_bytes();
        let (l, r) = (b[0] - b'a', b[1] - b'a');

        let key = to_key(l, r);
        let inv_key = to_key(r, l);

        if counts[inv_key] > 0 {
            counts[inv_key] -= 1;
            len += 4;
        } else {
            counts[key] += 1;
        }
    }

    len + (0..26 * 26)
        .step_by(27)
        .find(|&i| counts[i as usize] > 0)
        .map(|_| 2)
        .unwrap_or(0)
}
