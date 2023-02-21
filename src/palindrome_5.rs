/*
 * @lc app=leetcode.cn id=5 lang=rust
 *
 * [5] 最长回文子串
 */

// @lc code=start
impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let sv = s.chars().collect::<Vec<char>>();
        let mut window = s.len();
        let mut head = 0;
        while head != sv.len() {
            if head + window > sv.len() {
                window -= 1;
                head = 0;
                continue;
            }
            if Self::is_palindrome(&sv[head..head + window]) {
                return sv[head..head + window].iter().collect::<String>();
            }
            head += 1;
        }

        return "".to_string();
    }
    fn is_palindrome(v: &[char]) -> bool {
        for i in 0..v.len() / 2 {
            if v[i] != v[v.len() - 1 - i] {
                return false;
            }
        }
        true
    }
}

// @lc code=end

struct Solution;

#[test]
fn show_palindrome() {
    let s = "babad".to_string();
    let r = Solution::longest_palindrome(s);
    assert!(r == "bab".to_string());
}
