/*
 * @lc app=leetcode.cn id=20 lang=rust
 *
 * [20] 有效的括号
 */
enum paris {}
// @lc code=start
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let v: Vec<char> = s.chars().collect();
        let mut stack: Vec<char> = vec![];
        for i in 0..v.len() {
            stack.push(v[i]);
            if stack.len() >= 2 {
                let a = stack[stack.len() - 2];
                let b = stack[stack.len() - 1];
                if (a == '(' && b == ')') || (a == '[' && b == ']') || (a == '{' && b == '}') {
                    stack.pop();
                    stack.pop();
                }
            }
        }
        if stack.len() == 0 {
            return true;
        }
        return false;
    }
}
// @lc code=end
struct Solution;

#[test]
fn test_valid_paris_20() {
    assert_eq!(Solution::is_valid(String::from("()")), true);
    assert_eq!(Solution::is_valid(String::from("()[]{}")), true);
    assert_eq!(Solution::is_valid(String::from("(]")), false);
    assert_eq!(Solution::is_valid(String::from("([)]")), false);
    assert_eq!(Solution::is_valid(String::from("{[]}")), true);
}
