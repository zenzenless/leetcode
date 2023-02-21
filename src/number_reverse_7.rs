/*
 * @lc app=leetcode.cn id=7 lang=rust
 *
 * [7] 整数反转
 */

// @lc code=start
impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut s = x.to_string();
        if x < 0 {
            s = s[1..].to_string();
        };

        let cs = s.chars().rev().collect::<String>();
        let mut r = cs.parse::<i32>().unwrap_or(0);
        if x < 0 {
            r = -r;
        };
        return r;
    }
}
// @lc code=end

struct Solution;

#[test]
fn test_reverse() {
    assert_eq!(Solution::reverse(1534236469), 0);
    assert_eq!(Solution::reverse(-123), -321);
    assert_eq!(Solution::reverse(120), 21);
}

#[test]
fn test_num_cover() {
    let s = "321".to_string();
    let n = s.parse::<i32>().unwrap();
    println!("{}", n);
    assert_eq!(n, 321);
}
