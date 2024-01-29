from queue import deque
class Solution:
    def canReach(self, arr: List[int], start: int) -> bool:
        n = len(arr)
        dp = [False]*n
        def dfs(pos):
            if pos >=n or pos<0 or dp[pos]:
                return False
            dp[pos] = True
            if arr[pos] == 0:
                return True
            return dfs(pos-arr[pos]) or dfs(pos+arr[pos])
        return dfs(start)
