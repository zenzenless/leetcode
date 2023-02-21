/*
 * @lc app=leetcode.cn id=19 lang=golang
 *
 * [19] 删除链表的倒数第 N 个结点
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
func removeNthFromEnd(head *ListNode, n int) *ListNode {
	if head.Next==nil{
		return nil
	}
	h:=head
	p :=&ListNode{
		Val: 0,
		Next: h,
	}
	n--
	for ;head.Next!=nil&&n!=0;n--{
		head=head.Next
	}

	for head.Next!=nil{
		p=p.Next
		head=head.Next
	}
	if p.Next==h{
		return p.Next.Next
	}
	p.Next=p.Next.Next
	
	return h
}
// @lc code=end


type ListNode struct {
    Val int
    Next *ListNode
}
