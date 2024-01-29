class Solution:
    def findMaxAverage(self, nums: List[int], k: int) -> float:
        left,right = 0,k-1
        n = len(nums)
        cur_sum = sum(nums[:k])
        max_sum = cur_sum
        while right+1 < n:
            right += 1
            cur_sum += nums[right]
            cur_sum -= nums[left]
            left += 1
            max_sum = max(max_sum, cur_sum)
        return max_sum/k