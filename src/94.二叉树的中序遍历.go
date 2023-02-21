/*
 * @lc app=leetcode.cn id=94 lang=golang
 *
 * [94] 二叉树的中序遍历
 */
package main

// @lc code=start
/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */
func inorderTraversal(root *TreeNode) []int {
	m:=make([]int,0)
	stack:=make([]*TreeNode,0)
	stack = append(stack, root)
	for len(stack)!=0{
		last:=stack[len(stack)-1]
		stack=stack[:len(stack)-1]
		m = append(m, last.Val)
		if last.Right!=nil{
			
		}
	}
	return m
}

// @lc code=end

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}
