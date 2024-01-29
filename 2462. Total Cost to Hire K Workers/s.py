from typing import List
from itertools import chain
class Solution:
    def totalCost(self, costs: List[int], k: int, candidates: int) -> int:
        rt = 0
        for _ in range(k):
            right_index = max(0,len(costs) - candidates)
            hire = min(chain(zip(costs[:candidates],range(candidates)), zip(costs[right_index:], range(right_index,right_index+candidates))))
            rt += hire[0]
            costs.pop(hire[1])
        return rt

if __name__ == "__main__":
    rt = Solution().totalCost([17,12,10,2,7,2,11,20,8],3,4)
    print(rt)
