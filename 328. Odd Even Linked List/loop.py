# Definition for singly-linked list.
from typing import Optional
class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next

class Solution:
    def oddEvenList(self, head: Optional[ListNode]) -> Optional[ListNode]:
        if head is None or head.next is None:
            return head
        odd_tail = head
        even_tail = head.next
        even_head = even_tail
        while even_tail and even_tail.next:
            odd_tail.next, even_tail.next = odd_tail.next.next, even_tail.next.next
            odd_tail, even_tail = odd_tail.next, even_tail.next
        odd_tail.next = even_head
        return head
       

def to_linklist(l: list):
    head = None
    for v in reversed(l):
        head = ListNode(v, head)
    return head

def to_array(h: Optional[ListNode]):
    rt = []
    while h is not None:
        rt.append(h.val)
        h = h.next
    return rt
    

if __name__ == "__main__":
    s=Solution()
    rt = s.oddEvenList(to_linklist([2,1,3,5,6,4,7]))
    print(to_array(rt))