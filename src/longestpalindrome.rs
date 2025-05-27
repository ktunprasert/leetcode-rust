pub fn longest_palindrome(words: Vec<String>) -> i32 {
    let words = std::mem::ManuallyDrop::new(words);
    let mut lookup = [[0_u8; 26]; 26];
    let mut result = 0;

    for word in words.iter() {
        let [a, b] = word.as_bytes() else { panic!() };
        let [a, b] = [a, b].map(|c| (c - 97) as usize);

        let tmp = lookup[b][a] > 0;
        result += i32::from(tmp) * 4;
        lookup[b][a] -= u8::from(tmp);
        lookup[a][b] += u8::from(!tmp);
    }

    for c in 0..26 {
        if lookup[c][c] > 0 {
            result += 2;
            break;
        }
    }

    result
}
