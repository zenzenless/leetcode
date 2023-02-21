/*
 * @lc app=leetcode.cn id=58 lang=golang
 *
 * [58] 最后一个单词的长度
 */
package main
// @lc code=start
func lengthOfLastWord(s string) int {
	b:=[]byte(s)
	lastWorldLen:=0
	reset:=false
	for i:=0;i<len(b);i++{
		if b[i]==' '{
			reset=true
			continue
		}
		if reset{
			reset=false
			lastWorldLen=1
		}else{
			
			lastWorldLen++
		}
	}
	return lastWorldLen
}
// @lc code=end

