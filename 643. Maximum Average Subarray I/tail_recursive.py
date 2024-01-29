class Solution:
    def findMaxAverage(self, nums: List[int], k: int) -> float:
        return self.max_subst(nums, k, sum(nums[:k]),  sum(nums[:k]))/k
    def max_subst(self, nums, k, max_sum, left):
        if len(nums) == k:
            return max_sum
        left = left-nums[0]+nums[k]
        max_sum = max(max_sum, left)
        return self.max_subst(nums[1:],k, max_sum, left)