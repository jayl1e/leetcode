#include <vector>
using namespace std;
class Solution {
public:
    bool canJump(vector<int>& nums) {
        int n = nums.size();
        vector<bool> dp(n);
        dp[n-1] = true;
        for(int i=n-2;i>=0;i--){
            for (int j = i+1; j<=i+nums[i] && j<n; j++)
            {
                if (dp[j]==true){
                    dp[i]=true;
                    break;
                }
            }
        }
        return dp[0];
    }
};