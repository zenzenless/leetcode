/*
 * @lc app=leetcode.cn id=70 lang=golang
 *
 * [70] 爬楼梯
 */
package main
// @lc code=start
func climbStairs(n int) int {
	var climb func(int)int
	m:=make([]int,n+1)
	climb = func(left int)int {
		if m[left]!=0{
			return m[left]
		}
		if left == 1 {
			return 1
		}
		if left==2{
			return 2
		}
		a:=climb(left - 1)
		m[left-1]=a
		b:=climb(left - 2)
		m[left-2]=b
		return a+b
	}
	
	return climb(n)
}

// @lc code=end

