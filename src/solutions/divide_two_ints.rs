
pub struct Solution {
    pub name: Option<String>,
    pub explanation: Option<String>,
    pub code: Option<String>,
    pub args: Option<Vec<String>>,
    pub examples: Option<Vec<Vec<i32>>>,
}

// need to use traits
impl Solution {
    pub fn get_info() -> Solution {
        let code = include_str!("divide_two_ints.code");

        Solution {
            name: Some("Divide Two Integers".into()),
            explanation: Some("this is a explanation".into()),
            code: Some(code.into()),
            args: Some(vec!["dividend".into(), "divisor".into(), "answer".into()]),
            examples: Some(vec![vec![5, 5, 1], vec![25, -5, -5], vec![25, -5, 0]]),
        }
    }

    pub fn test(dividend: i32, divisor: i32, answer: i32) -> bool {
        let temp = Solution::divide(dividend, divisor);
        if temp == answer {
            return true;
        }
        false
    }

    pub fn divide(mut dividend: i32, mut divisor: i32) -> i32 {
        // handles an edge case
        if dividend == -2147483648 && divisor == -1 {
            return 2147483647;
        }

        // if negative and positive answer will be negative
        let mut positives = 0;
        if dividend.is_positive() {
            dividend = -dividend;
            positives += 1;
        }
        if divisor.is_positive() {
            divisor = -divisor;
            positives += 1;
        }

        let mut quotient: i32 = 0;
        let mut doubled: i32 = divisor;
        if -2147483648 - divisor < divisor {
            while doubled + doubled > dividend {
                if quotient == 0 {
                    quotient = 2;
                } else {
                    quotient += quotient;
                }

                doubled += doubled;
                if -2147483648 - doubled > doubled {
                    break;
                }
            }
            if doubled != divisor {
                dividend -= doubled;
            }
        }

        // subtract the divisor the dividend until it reaches the floor
        while dividend - divisor < 0 {
            dividend -= divisor;
            quotient += 1;
        }

        if dividend == divisor {
            quotient += 1;
        }

        if positives == 1 {
            quotient = -quotient;
        }
        quotient
    }
}

