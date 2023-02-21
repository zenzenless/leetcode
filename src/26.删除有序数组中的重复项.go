package main
/*
 * @lc app=leetcode.cn id=26 lang=golang
 *
 * [26] 删除有序数组中的重复项
 */

// @lc code=start
func removeDuplicates(nums []int) int {
	tail:=1
	last:=nums[0]
	for i:=1;i<len(nums);i++{
		if nums[i]==last{
			continue
		}
		last=nums[i]
		nums[tail]=last
		tail++
	}
	return tail
}
// @lc code=end

