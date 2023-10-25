from typing import *
class Solution:
    def permute(self, nums: List[int]) -> List[List[int]]:
        ans = []
        if len(nums)<=1:
            return [nums]
        for _ in range(len(nums)):
            n = nums.pop(0)
            subs = self.permute(nums)
            for s in subs:
                s.append(n)
            ans.extend(subs)
            nums.append(n)
        return ans

def main():
    Solution().permute([1,2])

main()
