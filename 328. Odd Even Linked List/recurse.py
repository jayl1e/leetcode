# Definition for singly-linked list.
from typing import Optional
class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next

class Solution:
    def oddEvenList(self, head: Optional[ListNode]) -> Optional[ListNode]:
        odd, odd_tail, even = self.split(head)
        if odd is not None:
            odd_tail.next = even
            return odd
        else:
            return even
    def split(self, head):
        if head is None:
            return None, None, None
        if head.next is None:
            return head, head, None
        odd, odd_tail, even = self.split(head.next.next)
        this_even = head.next
        head.next = odd
        this_even.next = even
        return head, odd_tail or head, this_even
       

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