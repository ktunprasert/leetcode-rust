use leetcode_rust::Solution;
use rstest::*;

#[rstest]
// #[case(vec![1,2,3], 4, vec![0,2], "hi")]
#[case(vec![2, 7, 11, 15], 9, vec![0,1], "ok")]
#[case(vec![0,4,3,0], 0, vec![0, 3], "ok")]
fn test_two_sum_cases(
    #[case] input_nums: Vec<i32>,
    #[case] target: i32,
    #[case] mut expected: Vec<i32>,
    #[case] _description: &str,
) {
    let mut result = Solution::two_sum(input_nums, target);

    result.sort();
    expected.sort();

    assert_eq!(result, expected); // Example assertion, adjust as needed
}
