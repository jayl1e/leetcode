def count_nest_array(array, k, n):
    ans = []
    def cnt_sub(sub, depth):
        left, right = 0, 0
        tsum = 0
        while right < len(sub):
            cur = sub[right]
            if isinstance(cur, list):
                cnt_sub(cur, depth+1)
                left = right + 1
                right = left
                tsum = 0
            elif right - left == k:
                tsum -= sub[left]
                left += 1
            else:
                tsum += cur
                right += 1
                if right - left ==k and tsum == n:
                    while len(ans) <= depth:
                        ans.append(0)
                    ans[depth] += 1
    cnt_sub(array, 0)
    return ans

if __name__=="__main__":
    print(count_nest_array([1,2,[3,4],5,6,7], 1, 7))