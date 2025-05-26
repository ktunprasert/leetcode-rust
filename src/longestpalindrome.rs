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
        counts[to_key(l, r)] += 1;
    }

    let mut has_central_double = false;

    for i in 0..26 {
        for j in i..26 {
            let key = to_key(i, j);

            if i == j {
                let count = counts[key];
                if count == 0 {
                    continue;
                }

                len += (count / 2) * 4;

                if count % 2 == 1 {
                    has_central_double = true;
                }

                continue;
            }

            let count_ab = counts[key];
            let count_ba = counts[to_key(j, i)];

            let pairs = count_ab.min(count_ba);

            len += pairs * 4;
        }
    }

    if has_central_double {
        len += 2;
    }

    len
}
