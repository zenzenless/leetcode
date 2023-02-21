/*
 * @lc app=leetcode.cn id=67 lang=golang
 *
 * [67] 二进制求和
 */
package main

// @lc code=start
func addBinary(a string, b string) string {
	if len(a) > len(b) {
		tmp := []byte(a)
		for i := len(b) - 1; i >= 0; i-- {
			if b[i] == '1' {
				//a的倒数第i位
				n:=len(tmp)-len(b)+i
				tmp = pOne(tmp, n)
			}
		}
		return string(tmp)
	}

	tmp := []byte(b)
	for i := len(a) - 1; i >= 0; i-- {
		if a[i] == '1' {
			n:=len(tmp)-len(a)+i
			tmp = pOne(tmp, n)
		}
	}
	return string(tmp)

}
func pOne(s []byte, cur int) []byte {
	if cur < 0 {
		return append([]byte{'1'}, s...)
	}
	if s[cur] == '1' {
		s[cur] = '0'

		return pOne(s, cur-1)
	} else {
		s[cur] = '1'
	}
	return s
}

// @lc code=end
