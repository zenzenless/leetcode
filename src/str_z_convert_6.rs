/*
 * @lc app=leetcode.cn id=6 lang=rust
 *
 * [6] Z 字形变换
 */

// @lc code=start
impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s;
        }
        let mut rows = Vec::new();
        for _ in 0..num_rows {
            rows.push(Vec::new());
        }
        let num_rows = num_rows as usize;
        let mut pos = Vec::new();
        let mut i = 0;
        for i in 0..num_rows {
            pos.push(i);
        }
        for i in (1..num_rows - 1).rev() {
            pos.push(i);
        }
        // num_rows =[0,1,2,1]
        let len = pos.len();
        let sv = s.chars().collect::<Vec<char>>();

        for i in 0..sv.len() {
            let p = i % len;
            rows[pos[p]].push(sv[i]);
        }
        let mut r = String::new();
        for i in rows.iter() {
            r.push_str(&i.iter().collect::<String>());
        }
        return r;
    }
}
// @lc code=end
struct Solution;

#[test]
fn test_convert() {
    let s = "PAYPALISHIRING".to_string();
    let r = Solution::convert(s, 3);
    println!("{}", r);
    assert!(r == "PAHNAPLSIIGYIR".to_string());
}
