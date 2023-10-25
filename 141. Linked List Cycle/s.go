package main

// Definition for singly-linked list.
type ListNode struct {
	Val  int
	Next *ListNode
}

func hasCycle1(head *ListNode) bool {
	fast, slow := head, head
	for fast != nil && fast.Next != nil {
		fast = fast.Next.Next
		slow = slow.Next
		if fast == slow {
			return true
		}
	}
	return false
}

func hasCycle(head *ListNode) bool {
	visited := make(map[*ListNode]struct{})

	for head != nil {
		if _, ok := visited[head]; ok {
			return true
		}
		visited[head] = struct{}{}
		head = head.Next
	}
	return false
}
