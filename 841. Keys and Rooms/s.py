from typing import List
class Solution:
    def canVisitAllRooms(self, rooms: List[List[int]]) -> bool:
        visited = [False] * len(rooms)
        def dfs(room):
            if visited[room]:
                return
            visited[room] = True
            for key in rooms[room]:
                dfs(key)
        dfs(0)
        return not (False in visited)

if __name__ == "__main__":
	s = Solution()
	rt = s.canVisitAllRooms([[1],[2],[3],[]])
	print(rt)
