from typing import List, Tuple
class Vertex:
    __slots__ = ('children')
    def __init__(self):
        self.children:List[Tuple[float,Vertex]] = []
    def add_child(self, weight:float, child):
        self.children.append((weight, child))

def dfs(visited, divident, divider):
    if divident is None or divider is None:
        return None
    if divident in visited:
        return None
    visited.add(divident)
    if divident is divider:
        return 1
    for (weight, child) in divident.children:
        v = dfs(visited,child, divider)
        if v is not None:
            return weight*v
    return None
    

class Solution:
    def calcEquation(self, equations: List[List[str]], values: List[float], queries: List[List[str]]) -> List[float]:
        graph = {}
        for ((divident, divider), quotient) in zip(equations, values):
            divident = graph.setdefault(divident, Vertex())
            divider = graph.setdefault(divider, Vertex())
            divident.add_child(quotient, divider)
            divider.add_child(1/quotient, divident)
        rt = []
        for (divident, divider) in queries:
            divident = graph.get(divident, None)
            divider = graph.get(divider, None)
            v = dfs(set(), divident, divider)
            if v is None:
                v = -1.0
            rt.append(v)
        return rt

if __name__ == "__main__":
    equations= [["a","b"],["b","c"]]
    values = [2.0,3.0]
    queries = [["a","c"],["b","a"],["a","e"],["a","a"],["x","x"]]
    rt = Solution().calcEquation(equations, values, queries)
    print(rt)