/*
 * @lc app=leetcode.cn id=18 lang=rust
 *
 * [18] 四数之和
 */

// @lc code=start
impl Solution {
    pub fn four_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut ns = nums.clone();
        let mut answer:Vec<Vec<i32>> = vec![];
        ns.sort();

        let len = ns.len();
        let mut last_a:Option<i32> =None;
        for a in 0..len {
            if let Some(v)=last_a{
                if v==ns[a]{
                    continue;
                }
            }
            last_a=Some(ns[a]);
            let mut last_b:Option<i32> =None;
            for b in (a + 1)..len {

                if let Some(v)=last_b{
                    if v==ns[b]{
                        continue;
                    }
                }
                last_b=Some(ns[b]);
                let mut last_c:Option<i32> =None;
                for c in (b + 1)..len {
                    if let Some(v)=last_c{
                        if v==ns[c]{
                            continue;
                        }
                    }
                    last_c=Some(ns[c]);
                    let mut last_d:Option<i32> =None;
                    for d in (c + 1)..len {
                        if let Some(v)=last_d{
                            if v==ns[d]{
                                continue;
                            }
                        }
                        let sum=ns[a].checked_add(ns[b]).
                        map(|s|s.checked_add(ns[c])).unwrap_or(None)
                        .map(|ss|ss.checked_add(ns[d])).unwrap_or(None);
                        if let None=sum{
                            break;
                        }
                        let sum=sum.unwrap();
                        if sum > target {
                            break;
                        }
                        
                        if sum == target {
                            if let Some(v)=last_d{
                                if v==ns[d]{
                                    break;
                                }
                            }
                            answer.push(vec![ns[a], ns[b], ns[c], ns[d]]);
                        }
                        last_d=Some(ns[d]);
                    }
                }
            }
        }
        return answer;
    }
}
// @lc code=end
struct Solution;

#[test]

fn test_four_sum() {
    let r = Solution::four_sum(vec![1000000000,1000000000,1000000000,1000000000], -294967296);
    println!("{:?}", r)
}
