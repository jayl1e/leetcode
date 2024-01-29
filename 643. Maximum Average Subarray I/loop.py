class Solution:
    def findMaxAverage(self, nums: List[int], k: int) -> float:
        return self.max_subst(nums, k, sum(nums[:k]),  sum(nums[:k]))/k
    def max_subst(self, nums, k, max_sum, left):
        l = 0
        n = len(nums)
        while True:
            if n -l == k:
                return max_sum
            left = left-nums[l]+nums[l+k]
            max_sum = max(max_sum, left)
            l += 1