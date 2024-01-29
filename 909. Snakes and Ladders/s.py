from typing import *
from collections import deque
class Solution:
    def snakesAndLadders(self, board: List[List[int]]) -> int:
        q=deque()
        q.append(1)
        dist = 0
        board[-1][0]=0
        n = len(board)
        target = n*n

        def to_xy(p):
            roff = (p-1)//n
            r = n-1-roff
            offset = (p-1)%n
            if roff & 0x1:
                return r, n-1-offset
            else:
                return r, offset

        while q:
            dist += 1
            for _ in range(len(q)):
                p = q.popleft()
                for nxt in range(p+1,min(p+6, target)+1):
                    if nxt == target:
                        return dist
                    nxtr,nxtc = to_xy(nxt)
                    nnxt = board[nxtr][nxtc]
                    if nnxt == 0:
                        continue
                    elif nnxt>0:
                        if nnxt == target:
                            return dist
                        nxtr,nxtc = to_xy(nnxt)
                        if board[nxtr][nxtc] == 0:
                            continue
                        board[nxtr][nxtc] = 0
                        q.append(nnxt)
                    else:
                        board[nxtr][nxtc] = 0
                        q.append(nxt)
        return -1

if __name__ == "__main__":
    board = [[-1,1,1,1],[-1,7,1,1],[16,1,1,1],[-1,1,9,1]]
    board2 = [[-1,-1,-1,46,47,-1,-1,-1],[51,-1,-1,63,-1,31,21,-1],[-1,-1,26,-1,-1,38,-1,-1],[-1,-1,11,-1,14,23,56,57],[11,-1,-1,-1,49,36,-1,48],[-1,-1,-1,33,56,-1,57,21],[-1,-1,-1,-1,-1,-1,2,-1],[-1,-1,-1,8,3,-1,6,56]]
    ans = Solution().snakesAndLadders(board2)
    assert ans == 4, f"{ans} != 4"

