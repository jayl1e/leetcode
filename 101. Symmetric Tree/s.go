package main

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func isSymmetric(root *TreeNode) bool {
	return isMirror(root.Left, root.Right)
}

func isMirror(p1 *TreeNode, p2 *TreeNode) bool {
	if p1 != nil && p2 != nil {
		return p1.Val == p2.Val && isMirror(p1.Left, p2.Right) && isMirror(p1.Right, p2.Left)
	}
	if p1 == p2 {
		return true
	}
	return false
}
