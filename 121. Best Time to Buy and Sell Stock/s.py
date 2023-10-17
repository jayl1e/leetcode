class Solution:
    def maxProfit(self, prices: List[int]) -> int:
        lowest = prices[0]
        profit = 0
        for p in prices:
            profit = max(profit, p-lowest)
            lowest = min(lowest, p)
        return profit
