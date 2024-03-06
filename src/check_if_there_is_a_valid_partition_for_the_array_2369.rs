/*
 * @lc app=leetcode.cn id=2369 lang=rust
 * @lcpr version=30117
 *
 * [2369] 检查数组是否存在有效划分
 */

// @lcpr-template-start



// @lcpr-template-end
// @lc code=start
use std::collections::VecDeque;
impl Solution {
    pub fn valid_partition(nums: Vec<i32>) -> bool {

        let mut queue: VecDeque<usize> = VecDeque::new();
        let len=nums.len();
        let mut last_p = 100;
        queue.push_back(0);
        while let Some(current) = queue.pop_front() {
            if last_p==current{
                continue;
            }
            last_p=current;
            
            if current==len&&current!=0{
                return true
            }
            if  len- current<2{
                continue;
            }
            if nums[current] == nums[current + 1] {
                queue.push_back(current+2);
                if len- current>2&&nums[current + 1] == nums[current + 2]{
                    queue.push_back(current+3);
                    continue;
                }
            }
            if len- current>2&&nums[current] + 1 == nums[current + 1] && nums[current + 1] + 1 == nums[current + 2]{
                queue.push_back(current+3);
            }
            
        }
        false
    }

}
// current 剩余切片头位于数组的位置

// @lc code=end
#[inline]
fn can_cut_3_nums(nums: &Vec<i32>, current: usize) -> bool {
    if nums.len() - current < 3 {
        return false;
    };
    // 1 1 1
    if nums[current] == nums[current + 1] && nums[current + 1] == nums[current + 2] {
        return true;
    };
    // 1 2 3
    nums[current] + 1 == nums[current + 1] && nums[current + 1] + 1 == nums[current + 2]
}
#[inline]
fn can_cut_2_nums(nums: &Vec<i32>, current: usize) -> bool {
    if nums.len() - current < 2 {
        return false;
    };
    nums[current] == nums[current + 1]
}
struct Solution {}
#[test]
fn test_valid_partition(){
    let b=Solution::valid_partition(vec![4,4,4,5,6]);
    println!("{b}")
}
/*
// @lcpr case=start
// [4,4,4,5,6]\n
// @lcpr case=end

// @lcpr case=start
// [1,1,1,2]\n
// @lcpr case=end

 */
