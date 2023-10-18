def isMirror(left,right):
        if left is right:
            return True
        if left is not None and right is not None:
            return left.val == right.val and isMirror(left.left, right.right) and isMirror(left.right, right.left)
        return False
class Solution:
    def isSymmetric(self, root: Optional[TreeNode]) -> bool:
        return isMirror(root.left, root.right)