use leetcode_rust::*;
use rstest::*;

#[rstest]
#[case(vec![], 0, "empty input")]
#[case(vec!["lc", "cl", "gg"], 6, "pairs and a central palindrome")]
#[case(vec!["ab", "ty", "yt", "lc", "cl", "ab"], 8, "multiple pairs, non-paired items")]
#[case(vec!["cc", "ll", "xx"], 2, "multiple self-palindromic words, one central")]
#[case(vec!["dd", "dd"], 4, "pair of identical self-palindromic words")]
#[case(vec!["aa", "bb", "aa"], 6, "pair of self-palindromic words and another central")]
#[case(vec!["zz"], 2, "single self-palindromic word (central)")]
#[case(vec!["oo", "pp", "qq", "rr"], 2, "multiple self-palindromic, only one central")]
#[case(vec!["ff", "ff", "ff"], 6, "odd number of identical self-palindromic words")]
#[case(vec!["ab", "cd"], 0, "no pairs, no self-palindromes")]
#[case(vec![
    "aa","aa","aa", "bb", "aa"
], 10, "pair of self-palindromic words and another central")]
#[case(vec!["em","pe","mp","ee","pp","me","ep","em","em","me"], 14, "hehe")]
#[case(vec!["qo","fo","fq","qf","fo","ff","qq","qf","of","of","oo","of","of","qf","qf","of"], 14, "hoho")]
fn test_longest_palindrome_cases(
    #[case] input_words: Vec<&str>,
    #[case] expected: i32,
    #[case] _description: &str,
) {
    let converted: Vec<String> = input_words.into_iter().map(|s| String::from(s)).collect();

    assert_eq!(longest_palindrome(converted), expected);
}

// fn
