use divide_two_ints::Solution;

#[test]
fn two_positives() {
    assert_eq!(Solution::divide(15, 3), 5);
    assert_eq!(Solution::divide(100, 9), 11);
    assert_eq!(Solution::divide(1738, 27), 64);
}

#[test]
fn negative_and_positive() {
    assert_eq!(Solution::divide(-15, 3), -5);
    assert_eq!(Solution::divide(100, -9), -11);
    assert_eq!(Solution::divide(1738, -27), -64);
    assert_eq!(Solution::divide(-1738, 27), -64);
}

#[test]
fn two_negatives() {
    assert_eq!(Solution::divide(-15, -3), 5);
    assert_eq!(Solution::divide(-100, -9), 11);
    assert_eq!(Solution::divide(-1738, -27), 64);
    assert_eq!(Solution::divide(-1738, -27), 64);
}

#[test]
fn one() {
    assert_eq!(Solution::divide(1, 1), 1);
    assert_eq!(Solution::divide(-1, 1), -1);
    assert_eq!(Solution::divide(1, -1), -1);
    assert_eq!(Solution::divide(-1, -1), 1);
}

#[test]
fn max() {
    assert_eq!(Solution::divide(-2147483648, -1), 2147483647);
    assert_eq!(Solution::divide(2147483647, 3), 715827882);
}

#[test]
fn edge() {
    //assert_eq!(Solution::divide(1004958205, -2137325331), 0);
    //assert_eq!(Solution::divide(1076233784, -1766978113), 0);
    //assert_eq!(Solution::divide(-2147483648, -2147483648), 0);
}
