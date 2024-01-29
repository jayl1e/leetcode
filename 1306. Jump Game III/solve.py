from queue import deque
class Solution:
    def canReach(self, arr: List[int], start: int) -> bool:
        n = len(arr)
        dp = [False]*n
        q = deque()
        dp[start] = True
        q.append(start)
        while q:
            visit = q.popleft()
            if arr[visit] == 0:
                return True
            left = visit-arr[visit]
            if left >= 0 and not dp[left]:
                dp[left] = True
                q.append(left)
            right = visit+arr[visit]
            if right < n and not dp[right]:
                dp[right] = True
                q.append(right)
        return False
