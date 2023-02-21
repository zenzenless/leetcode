/*
 * @lc app=leetcode.cn id=13 lang=golang
 *
 * [13] 罗马数字转整数
 */
package main
// @lc code=start
var mapping map[byte]int=map[byte]int{
	'I':1,
	'V':5,
	'X':10,
	'L':50,
	'C':100,
	'D':500,
	'M':1000,
}
func romanToInt(s string) int {
	b:=[]byte(s)
	var last int
	var sum int
	for i:=len(b)-1;i>=0;i--{
		n:=mapping[b[i]]
		if n<last{
			sum-=n
		}else{
			sum+=n
		}
		last=n

	}
	return sum
}
// @lc code=end

