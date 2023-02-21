/*
 * @lc app=leetcode.cn id=83 lang=golang
 *
 * [83] 删除排序链表中的重复元素
 */
package main

// @lc code=start
/**
 * Definition for singly-linked list.
 * type ListNode struct {
 *     Val int
 *     Next *ListNode
 * }
 */
func deleteDuplicates(head *ListNode) *ListNode {
	h:=head
	last:=head
	m:=make([]bool,201)
	for head!=nil{
		if m[head.Val+100]{
			//发现重复
			last.Next=head.Next
			head=head.Next
			continue
		}
		last=head
		m[head.Val+100]=true
		head=head.Next
	}
	return h
}

// @lc code=end