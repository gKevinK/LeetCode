class Solution {
public:
    int findMaxConsecutiveOnes(vector<int>& nums) {
        int r = 0;
        for (int i = 0, j = 0; i < nums.size(); i++) {
            if (nums[i] == 0)
                j = i + 1;
            else
                r = max(r, i - j + 1);
        }
        return r;
    }
};