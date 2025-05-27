use leetcode_rust::*;
use rstest::*;

#[rstest]
#[case(121, true)]
#[case(100, false)]
#[case(112, false)]
#[case(1122, false)]
#[case(1000, false)]
#[case(12, false)]
#[case(11, true)]
#[case(10, false)]
#[case(0, true)]
#[case(12345, false)]
#[case(1234321, true)]
#[case(-10, false)]
#[case(-1, false)]
#[case(-11231231, false)]
fn test_num_palindrome_cases(
    #[case] num: i32,
    #[case] expected: bool,
) {
    assert_eq!(Solution::is_palindrome(num), expected);
}


