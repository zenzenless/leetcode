/*
 * @lc app=leetcode.cn id=1078 lang=rust
 *
 * [1078] Bigram 分词
 */

// @lc code=start
impl Solution {
    pub fn find_ocurrences(text: String, first: String, second: String) -> Vec<String> {
        let mut r=vec![];
        
        let worlds:Vec<&str> =text.split_ascii_whitespace().collect();
        let first=first.as_str();
        let second=second.as_str();
        for i in 0..worlds.len(){
            if worlds[i]==first{    
                if  i<worlds.len()-1{
                    if worlds[i+1]==second{
                        if i<worlds.len()-2{
                            r.push(worlds[i+2].to_string());
                        }
                    }
                }              
            }
        } 
        return  r;
    }
  
}
// @lc code=end

struct Solution;