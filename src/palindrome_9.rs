/*
 * @lc app=leetcode.cn id=9 lang=rust
 *
 * [9] 回文数
 */

// @lc code=start
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let s=x.to_string();
        let mut v=s.chars().collect::<Vec<char>>();
        for i in 0..v.len()/2 {
            if v[i]!=v[v.len()-1-i] {
                return false;
            }
        }
        true
    }
}
// @lc code=end

struct Solution;