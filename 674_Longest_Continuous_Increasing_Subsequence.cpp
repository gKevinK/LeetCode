class Solution {
public:
    int findLengthOfLCIS(vector<int>& nums) {
        int m = 0, c = 0;
        for (int i = 0; i < nums.size(); i++) {
            if (i > 0 && nums[i] > nums[i - 1])
                c++;
            else
                c = 1;
            m = max(m, c);
        }
        return m;
    }
};