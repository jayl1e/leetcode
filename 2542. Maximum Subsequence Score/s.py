from typing import *
import heapq
class Solution:
    def maxScore(self, nums1: List[int], nums2: List[int], k: int) -> int:
        pairs = sorted(zip(nums2, nums1), reverse=True)
        q=[t[1] for t in pairs[:k]]
        s1 = sum(q)
        ans = s1*pairs[k-1][0]
        heapq.heapify(q)
        for n2,n1 in pairs[k:]:
            ts1 = (s1-q[0]+n1)
            ians = ts1*n2
            if ians > ans:
                ans = ians
                s1 = ts1
                heapq.heapreplace(q, n1)
        return ans

if __name__ == "__main__":
	nums1 = [1,3,3,2]
	nums2 = [2,1,3,4]
	k = 3
	expected = 12
	act = Solution().maxScore(nums1,nums2,k)
	assert(expected == act)
