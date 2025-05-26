pub fn longest_palindrome(words: Vec<String>) -> i32 {
    if words.len() == 0 {
        return 0;
    }

    words
        .iter()
        .map(|w| w.as_bytes())
        .map(|b| (b[0] - b'a', b[1] - b'a'))
        .map(|(a, b)| ((a, b), (b, a)))
        .fold(Some((0i32, [[0; 26]; 26])), |opt, ((a, b), (ra, rb))| {
            let (mut len, mut counts) = opt.unwrap();
            if counts[ra as usize][rb as usize] > 0 {
                counts[ra as usize][rb as usize] -= 1;
                len += 4;
            } else {
                counts[a as usize][b as usize] += 1;
            }

            (len, counts).into()
        })
        .map(|(len, counts)| {
            len + (0..26)
                .find(|i| counts[*i as usize][*i as usize] > 0)
                .map(|_| 2)
                .unwrap_or(0)
        })
        .unwrap()
}
