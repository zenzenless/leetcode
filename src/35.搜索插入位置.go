/*
 * @lc app=leetcode.cn id=35 lang=golang
 *
 * [35] 搜索插入位置
 */
package main
// @lc code=start
func searchInsert(nums []int, target int) int {
	for i:=0;i<len(nums);i++{
		if nums[i]==target{
			return i
		}
		if nums[i]>target{
			return i
		}
	}
	return len(nums)
}
// @lc code=end

