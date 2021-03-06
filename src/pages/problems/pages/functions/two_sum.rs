use super::super::two_sum::TwoSumExamples;

impl TwoSumExamples {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for (i, x) in nums.iter().enumerate() {
            for (j, y) in nums.iter().enumerate() {
                if i != j && x + y == target {
                    return vec![i as i32, j as i32];
                }
            }
        }
        return vec![-1, -1];
    }
}
