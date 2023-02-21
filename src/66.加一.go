/*
 * @lc app=leetcode.cn id=66 lang=golang
 *
 * [66] 加一
 */
package main
// @lc code=start
func plusOne(digits []int) []int {
	l:=len(digits)
	digits[l-1]=digits[l-1]+1
	if digits[l-1]>=10{
		digits[l-1]=0
		if l>1{
			plusOne(digits[:l-1])
		}
		
	}
	if digits[0]==0{
		return append([]int{1},digits...)
	}
	return digits
}
// @lc code=end

