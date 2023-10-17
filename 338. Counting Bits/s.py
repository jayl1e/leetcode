from typing import List
class Solution:
    def countBits(self, n: int) -> List[int]:
        rt = [0]*(n+1)
        for i in range(1,n+1):
            rt[i] = (i&1) + rt[i>>1]
        return rt

if __name__ == "__main__":
    s=Solution()
    rt = s.countBits(2)
    print(rt)
