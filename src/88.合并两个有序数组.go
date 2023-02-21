/*
 * @lc app=leetcode.cn id=88 lang=golang
 *
 * [88] 合并两个有序数组
 */
package main

// @lc code=start
func merge(nums1 []int, m int, nums2 []int, n int) {
	s := make([]int, m+n)
	si := 0
	i := 0
	j := 0
	for i < m && j < n {
		if nums1[i] < nums2[j] {
			s[si] = nums1[i]
			i++
		} else {
			s[si] = nums2[j]
			j++
		}
		si++
	}
	if i != m {
		for i < m {
			s[si] = nums1[i]
			si++
			i++
		}
	}
	if j != n {
		for ; j < n; j++ {
			s[si] = nums2[j]
			si++
		}
	}
	copy(nums1, s)
}

// @lc code=end
