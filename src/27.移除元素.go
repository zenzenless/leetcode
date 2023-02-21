/*
 * @lc app=leetcode.cn id=27 lang=golang
 *
 * [27] 移除元素
 */
package main
// @lc code=start
func removeElement(nums []int, val int) int {
	lenght:=len(nums)
	for i:=0;i<lenght;i++{
		if nums[i]==val{
			for j:=i;j<len(nums);j++{
				if j+1==lenght{
					break
				}
				nums[j]=nums[j+1]
			}
			i--
			lenght--
		}
	}
	return lenght
}
// @lc code=end

