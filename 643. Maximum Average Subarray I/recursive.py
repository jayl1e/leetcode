from typing import List
class Solution:
    def findMaxAverage(self, nums: List[int], k: int) -> float:
        return self.max_subst(nums, k)[0]/k
    def max_subst(self, nums, k):
        if len(nums) == k:
            return sum(nums), sum(nums[:k-1]), nums[k-1]
        head = nums[0]
        r_max, r_left, r_tail = self.max_subst(nums[1:], k)
        return max(r_max, head+r_left), r_left - r_tail + head, nums[k-1] 


if __name__ == "__main__":
    print(Solution().findMaxAverage([1,12,-5,-6,50,3],4))