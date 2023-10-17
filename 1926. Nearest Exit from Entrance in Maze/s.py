from typing import List
from collections import deque
class Solution:
    def nearestExit(self, maze: List[List[str]], entrance: List[int]) -> int:
        nr, nc = len(maze), len(maze[0])
        visited =[[False]*nc for i in range(nr)]
        move = 0
        q = deque()
        q.append((entrance[0], entrance[1]))
        while q:
            size = len(q)
            for _ in range(size):
                (pr,pc) = q.popleft()
                if pr<0 or pr>=nr or pc<0 or pc>=nc:
                    continue
                if  maze[pr][pc] == '+' or visited[pr][pc]:
                    continue
                visited[pr][pc] = True
                if pr == 0 or pr==nr-1 or pc ==0 or pc ==nc-1:
                    if pr!=entrance[0] or pc != entrance[1]:
                        return move
                q.extend([(pr-1, pc), (pr+1, pc),(pr, pc-1),(pr,pc+1)])
            move += 1
        return -1

