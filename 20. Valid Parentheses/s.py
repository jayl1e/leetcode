class Solution:
    def isValid(self, s: str) -> bool:
        stack = []
        pairs = {'}':'{', ']':'[', ')':'('}
        for c in s:
            if c in pairs.values():
                stack.append(c)
            elif c in pairs:
                if not stack:
                    return False
                if stack[-1] == pairs[c]:
                    stack.pop()
                else:
                    return False
            else:
                raise Exception("bad char")
        return not stack